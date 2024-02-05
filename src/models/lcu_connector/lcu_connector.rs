use std::ptr::null_mut;
use regex::Regex;
use super::lcu_connector_info::LcuConnectorInfo;

#[cfg(target_os = "windows")]
use windows::{
    core::{PCSTR, PWSTR},
    Wdk::System::Threading::{
        NtQueryInformationProcess,
        PROCESSINFOCLASS,
    },
    Win32::{
        *,
        Foundation::*,
        Security::*,
        UI::Shell::*,
        System::{
            Threading::*,
            Diagnostics::{
                *,
                Debug::*,
                ToolHelp::*,
            },
        },
    },
};
#[cfg(target_os = "windows")]
use std::{
    ptr::*,
    mem::*,
    collections::HashMap,
    ffi::OsString,
    os::windows::ffi::OsStringExt,
};

#[derive(Debug, serde::Deserialize)]
struct LCUSettings {
    product_install_full_path: String
}
struct LcuConnector;

impl LcuConnector {

    #[cfg(target_os = "linux")]
    fn get_client_info() -> Option<LcuConnectorInfo> {
        let home_dir = simple_home_dir::home_dir()?;
        let lockfile = home_dir.join("/Games/league-of-legends/drive_c/Riot Games/League of Legends/lockfile");

        let lockfile_contents = std::fs::read_to_string(lockfile).ok()?;
        Some(lockfile_contents.into())
    }

    #[cfg(not(target_os = "linux"))]
    fn get_client_info() -> Option<LcuConnectorInfo> {

        unsafe {
            let client_process_id = Self::find_process_id()?;
            println!("{client_process_id:?}");
            let process_args = Self::get_process_args(client_process_id);

            let priority_regex = Regex::new(
                r#""--priority-launch-path=(.*?)""#
            ).unwrap();

            let priority_captures = process_args.iter()
                .flat_map(|arg| priority_regex.captures(arg.as_str()))
                .collect::<Vec<_>>();
            let priority_capture = priority_captures.get(0);

            let league_path = match priority_capture {
                Some(capture) => capture.extract::<1>().1[0].to_string(),
                None => Self::get_league_path(process_args)
            };

            let yaml_file = std::fs::File::open(league_path).unwrap();
            let yaml: LCUSettings  = serde_yaml::from_reader(yaml_file).unwrap();

            let mut lockfile_path = std::path::PathBuf::new();
            lockfile_path.push(yaml.product_install_full_path);
            lockfile_path.push("lockfile");

            let lockfile = std::fs::read_to_string(lockfile_path).ok()?;
            Some(lockfile.into())
        }
    }

    #[cfg(target_os = "windows")]
    fn client_process_name() -> String { "RiotClientServices.exe".to_string() }
    #[cfg(target_os = "macos")]
    fn client_process_name() -> String { "RiotClientServices".to_string() }

    #[cfg(target_os = "windows")]
    unsafe fn find_process_id() -> Option<u32> {
        let client_process_name = Self::client_process_name();

        let mut process_hashmap: HashMap<u32, String> = HashMap::new();

        let mut process: PROCESSENTRY32W = zeroed();
        process.dwSize = size_of::<PROCESSENTRY32W>() as u32;

        let handle = CreateToolhelp32Snapshot(
            TH32CS_SNAPALL,
            0
        ).ok()?;

        if Process32FirstW(handle, &mut process).is_ok() {
            while Process32NextW(handle, &mut process).is_ok() {
                let process_name = std::ffi::OsString::from_wide(&process.szExeFile)
                    .into_string()
                    .ok()?;

                process_hashmap.insert(
                    process.th32ProcessID,
                    process_name,
                );
            }
        }
        CloseHandle(handle);

        process_hashmap.into_iter()
            .filter(|(_, name)| name.starts_with(&client_process_name))
            .map(|(pid, _)| pid)
            .collect::<Vec<u32>>()
            .pop()
    }

    #[cfg(target_os = "windows")]
    unsafe fn get_process_args(pid: u32) -> Vec<String> {
        let process_info_class = zeroed::<PROCESSINFOCLASS>();
        let mut process_information = zeroed::<PROCESS_BASIC_INFORMATION>();
        let mut process_structure = zeroed::<PEB>();
        let mut process_params = zeroed::<RTL_USER_PROCESS_PARAMETERS>();

        let process_handle = OpenProcess(
            PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ,
            BOOL(0),
            pid,
        ).unwrap();

        // I do not like this solution, given the warning on msf docs
        NtQueryInformationProcess(
            process_handle,
            process_info_class,
            addr_of_mut!(process_information) as _,
            size_of::<PROCESS_BASIC_INFORMATION>() as _,
            null_mut(),
        ).ok().unwrap();

        ReadProcessMemory(
            process_handle,
            process_information.PebBaseAddress as _,
            addr_of_mut!(process_structure) as _,
            size_of::<PEB>(),
            None,
        ).unwrap();

        ReadProcessMemory(
            process_handle,
            process_structure.ProcessParameters as _,
            addr_of_mut!(process_params) as _,
            size_of::<RTL_USER_PROCESS_PARAMETERS>(),
            None,
        ).unwrap();

        // Normally, we would use a vec here. However, casting to usize causes
        // a very weird bug where the size is incorrect and the memory is completely
        // fucked. We can't use a runtime value for an array, but it's doubtful
        // 1024 values will ever be overrun.
        // This might provide more insight: https://github.com/rust-lang/rust/issues/97463
        let mut buffer: [u16; 1024] = [0; 1024];
        let buffer_base_address = process_params.CommandLine.Buffer.0;
        let length = process_params.CommandLine.MaximumLength;

        ReadProcessMemory(
            process_handle,
            buffer_base_address as _,
            addr_of_mut!(buffer) as _,
            length as _,
            None,
        ).unwrap();


        let command_line_params = match buffer.iter().position(|&p| p == 0) {
            Some(position) => OsString::from_wide(&buffer[..position]),
            None => OsString::from_wide(&buffer),
        }.into_string().unwrap();

        println!("{command_line_params}");
        CloseHandle(process_handle);

        command_line_params.split(" ")
            .skip(1)
            .map(String::from)
            .collect::<Vec<String>>()
    }

    #[cfg(target_os = "windows")]
    unsafe fn get_league_path(process_args: Vec<String>) -> String {
        let launch_product_regex = Regex::new(
            r#"--launch-product=(.*)"#
        ).unwrap();
        let launch_product_captures = process_args.iter()
            .flat_map(|arg| launch_product_regex.captures(arg.as_str()))
            .collect::<Vec<_>>();
        let launch_product_capture = launch_product_captures.get(0).unwrap();
        let launch_product = launch_product_capture.extract::<1>().1[0].to_string();

        let launch_patchline_regex = Regex::new(
            r#"--launch-patchline=(.*)"#
        ).unwrap();
        let launch_patchline_captures = process_args.iter()
            .flat_map(|arg| launch_patchline_regex.captures(arg.as_str()))
            .collect::<Vec<_>>();
        let launch_patchline_capture = launch_patchline_captures.get(0).unwrap();
        let launch_patchline = launch_patchline_capture.extract::<1>().1[0].to_string();

        let mut handle: HANDLE = zeroed();
        let folder_type = FOLDERID_ProgramData;
        let program_data = SHGetKnownFolderPath(
            addr_of!(folder_type) as _,
            KNOWN_FOLDER_FLAG(0),
            handle,
        ).unwrap();

        let program_data = OsString::from_wide(
            program_data.as_wide()
        ).into_string().unwrap();

        let mut filepath = std::path::PathBuf::new();
        filepath.push(program_data);
        filepath.push("Riot Games");
        filepath.push("Metadata");
        filepath.push(format!("{launch_product}.{launch_patchline}"));
        filepath.push(format!("{launch_product}.{launch_patchline}.product_settings.yaml"));

        filepath.to_str().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::lcu_connector::lcu_connector::LcuConnector;

    #[test]
    fn test_connection() {
        let client_info = LcuConnector::get_client_info().unwrap();
        println!("{client_info:?}")
    }
}
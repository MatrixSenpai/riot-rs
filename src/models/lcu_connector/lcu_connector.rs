use crate::models::lcu_connector::lcu_connector_info::LcuConnectorInfo;

struct LcuConnector {
    uri: Option<String>,
    port: Option<String>,
}

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

    }

    #[cfg(target_os = "windows")]
    fn client_process_name() -> String { "RiotClientServices.exe".to_string() }
    #[cfg(target_os = "macos")]
    fn client_process_name() -> String { "RiotClientServices".to_string() }

}


#[derive(Debug)]
pub(super) struct LcuConnectorInfo {
    pub username: String,
    pub process_name: String,
    pub pid: usize,
    pub port: usize,
    pub password: String,
    pub protocol: String,
    pub address: String,
}
impl From<String> for LcuConnectorInfo {
    fn from(value: String) -> Self {
        let parts = value.split(":")
            .map(String::from)
            .collect::<Vec<String>>();

        Self {
            username: "riot".to_string(),
            process_name: parts[0].clone(),
            pid: parts[1].clone().parse().unwrap(),
            port: parts[2].clone().parse().unwrap(),
            password: parts[3].clone(),
            protocol: parts[4].clone(),
            address: "127.0.0.1".to_string(),
        }
    }
}

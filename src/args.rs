use std::time::Duration;
use std::net::{IpAddr};

pub struct Config{
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub timeout: Duration,
}

impl Config
{
    pub fn from_args(raw: std::env::Args) -> Result<Config, String>
    {
        let arguments : Vec<String>= raw.skip(1).collect();

        if arguments.is_empty()
        {
            return Err("usage: portscan <ip> [end_port]".to_string());
        }
        let ip = arguments[0]
        .parse::<IpAddr>()
        .map_err(|_error| "usage: portscan <ip> [end_port]".to_string())?;

        let port= if arguments.get(1).is_none()
        {
            1024
        }
        else {
            arguments[1].parse::<u16>().map_err(|_| "usage: portscan <ip> [end_port]".to_string())?
        };

        let ports = (1..=port).collect();

        Ok(Config {
            ip,
            ports,
            timeout: Duration::from_millis(200)
        })
    }
}
use std::thread::{self};
use std::time::Duration;
use std::net::{IpAddr, TcpStream, SocketAddr};

pub fn scan_port(ip: IpAddr, port: u16, timeout : Duration) -> Option<u16>
{
    let addr = SocketAddr::from((ip, port));
    let res = TcpStream::connect_timeout(&addr, timeout);
    res.is_ok().then_some(port)
}

pub fn scan_range(ip: IpAddr, ports: &[u16], timeout: Duration) -> Vec<u16>
{
    ports.iter()
    .filter_map(|s| scan_port(ip, *s, timeout))
    .collect()
}

pub fn scan_range_threaded(ip: IpAddr, ports: &[u16], timeout: Duration) -> Vec<u16>
{
    ports
    .iter()
    .filter_map(|&s|{
                    let handler= thread::spawn(move ||{
                    scan_port(ip, s, timeout)});
                    handler.join().unwrap_or(Some(s))
        }
    )
    .collect()
}

#[cfg(test)]
mod test
{
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn closed_ports()
    {
        assert_eq!(scan_port(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), 9, Duration::from_millis(200)), None);
    }

    #[test]
    fn scan_range_closed_ports()
    {
        let vector : Vec<u16> = vec![];
        assert_eq!(scan_range(IpAddr::V4(Ipv4Addr::new(127,0,0,1)), &[1,2,3,4,5], Duration::from_millis(200)), vector);
    }
}
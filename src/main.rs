use portscan::args::*;
use portscan::scanner;
use std::process::exit;
use std::time::Instant;

fn main() {
    let args = std::env::args();
    let raw = match Config::from_args(args)
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("portscan: {e}");
            exit(1);
        }
    };

    let start = Instant::now();

    let scan_res = scanner::scan_range(raw.ip, &raw.ports, raw.timeout);

    println!("{:?}", start.elapsed());
    if scan_res.is_empty()
    {
        println!("no open ports found");
    }
    for iter in scan_res
    {
        println!("{}:{} open",raw.ip, iter);
    }

    
}

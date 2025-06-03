pub mod domain_resolver;
pub mod port_scan;
use std::io;
use std::net::IpAddr;

use port_scan::Mode;



fn main() {
    let mut input = String::new();
    let mut mode_input=String::new();
    
    println!("Enter the Ip address or website to scan:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    // aba ip halesi mode sodhni
    println!("Enter the mode (full/fast):");
    // mode input lini
    io::stdin()
        .read_line(&mut mode_input)
        .expect("Failed to read line");
    
    let mode_input=mode_input.trim().to_ascii_lowercase();
    let mode=match mode_input.as_str(){
        "full"=>Mode::Full,
        "fast"=>Mode::Fast,
        _=> {
            println!("Invalid mode selected, defaulting to Fast mode");
            Mode::Fast
        }
    };
    
    println!("scanning the ports of {input}");
    
    match input_resolver(input) {
        Ok(ip) => {
            println!("IP: {}", ip);
            port_scan::scan_open_ports(ip,mode);
        }
        Err(e) => println!("Failed to resolve IP {}", e),
    }
}

fn input_resolver(input: &str) -> Result<IpAddr, io::Error> {
    let valid_ip = domain_resolver::to_ip_addr(input)?;
    //testing ma use garya hai talako line
    // println!("Resolved IP:{}", valid_ip);
    Ok(valid_ip)
}


use crate::prelude::*;
pub fn input_resolver(input: &str) -> Result<Vec<Ipv4Addr>, io::Error> {
  
  if input.contains('/'){
    match parse_cidr(input)
    {
        Ok(hosts) => Ok(hosts),
        Err(e) => Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
    }
  }
  else {
     match domain_to_ipv4_addr(input){
        Ok(ip) => Ok(vec![ip]),
        Err(e) => Err(e),
     }
  }
  

    }

pub fn parse_cidr(cidr: &str) -> Result<Vec<Ipv4Addr>, String> {
    match cidr.parse::<Ipv4Network>() {
        Ok(network) => {
            println!("Parsed CIDR: {}", network);

            let size = network.size();
            
            if size > 1 {
                println!("The network has {} hosts.", size);
            }
            let hosts=network.iter().collect::<Vec<Ipv4Addr>>();
            Ok(hosts)
        },
        Err(_) => Err(format!(
            "Invalid CIDR notation: {}  .Valid CIDR format is 192.168.1.0/24"
       ,cidr )),
    }
}


pub fn domain_to_ipv4_addr(domain: &str) -> Result<Ipv4Addr, io::Error> {
    match Ipv4Addr::from_str(domain) {
        Ok(ip) => Ok(ip),
        Err(_) => {
            let ips: Vec<IpAddr> = lookup_host(domain)?;

            for ip in ips {
                if let IpAddr::V4(v4) = ip {
                    return Ok(v4);
                }
            }

            Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to resolve ips",
            ))
        }
    }
}

use dns_lookup::lookup_host;
use std::net;
use std::io;
use std::str::FromStr;
pub fn to_ip_addr(domain:& str )->Result<net::IpAddr, io::Error>{
    match net::IpAddr::from_str(domain){
        Ok(ip)=>Ok(ip),
        Err(_)=>{
            let mut ips=lookup_host(domain)?;
            ips.pop().ok_or_else(|| io::Error::new(io::ErrorKind::Other,"Failed to resilve ips"))
        }
    }
}


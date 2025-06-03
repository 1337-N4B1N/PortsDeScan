use std::net::{IpAddr, SocketAddr, TcpStream};
use std::{ thread};
use std::time::Duration;
use std::collections::HashSet;
pub enum Mode{
    Full,
    Fast,
     
}
const COMMON_PORTS: &[u16] = &[
    20, 21, 22, 23, 25, 53, 67, 68, 69, 80, 110, 123, 137, 138, 139, 143, 161, 162, 179, 389, 443,
    445, 465, 514, 515, 587, 631, 993, 995, 1080, 1433, 1521, 1723, 2049, 2082, 2083, 2181, 2483,
    2484, 3306, 3389, 3690, 4333, 5432, 5900, 5984, 6379, 6667, 7001, 8000, 8080, 8443, 8888, 9200,
    11211, 27017,
];
const MAX_THREADS: usize = 2000;

pub fn scan_open_ports(ip: IpAddr,mode:Mode) {
    let port_range:Vec<u16>=match mode{
        Mode::Full => (1..=65535).collect(),
        Mode::Fast=>{
             let mut ports = HashSet::new();
            
        //    tyo contains garda chai O(n) time complexity ley garda slow bhyo iterate garna and some ports were missed so we used HashSet
            for port in 1..=1000 {
                ports.insert(port);
            }
            
          
            for &port in COMMON_PORTS {
                ports.insert(port);
            }
            
            
            let mut port_vec: Vec<u16> = ports.into_iter().collect();
            port_vec.sort();
            port_vec
        
        },
        
    };
    let timeout = Duration::from_millis(3000); 
    println!("scanning ports {}",port_range.len());
    //thread::spawn returns a JoinHandle, wwhich is the datatype for the vector handles here..
        for chunk in port_range.chunks(MAX_THREADS) {
    let handles: Vec<_> = chunk.iter().map(|&port| {
        let ip = ip.clone();
        thread::spawn(move || {
            let socket = SocketAddr::new(ip, port);
            if TcpStream::connect_timeout(&socket, timeout).is_ok() {
                println!("Port {} is open", port);
            }
        })
    }).collect();

    for handle in handles {
        let _ = handle.join();
    }
}
}

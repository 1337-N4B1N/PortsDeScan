mod address_resolver;
mod map_services;
mod port_scan;
mod prelude;
mod input;
use crate::prelude::*; // Imports everything marked pub use in prelude.rs

fn main() {
    // network_resolver::test_cidr_parsing();
   let target= get_target_input();
let mode=get_mode_input();
    println!("scanning the ports of {target} in {:?} mode", mode);

    match input_resolver(&target) {
        Ok(ips) => {
            println!("We need to scan {} IPs", ips.len());
            for ip in ips.iter() {
                  println!(
                    "========================================================================================="
                );
                println!("Scanning IP: {}", ip);

                scan_open_ports(*ip, mode);
              
            }
        }
        Err(e) => println!("Failed to resolve IP {}", e),
    }
}

use crate::prelude::*;
#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Full,
    Fast,
}

const MAX_THREADS: usize = 2000;

pub fn scan_open_ports(ip: Ipv4Addr, mode: Mode) {
    let port_range: Vec<u16> = match mode {
        Mode::Full => (1..=65535).collect(),
        Mode::Fast => {
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
        }
    };

    let timeout = Duration::from_millis(3000);
    println!("Scanning {} ports", port_range.len());
    let open_ports_count=Arc::new(AtomicUsize::new(0));

    //thread::spawn returns a JoinHandle, wwhich is the datatype for the vector handles here..
    for chunk in port_range.chunks(MAX_THREADS) {
        let handles: Vec<_> = chunk
            .iter()
            .map(|&port| {
                let ip = ip;
                let open_ports_count=Arc::clone(&open_ports_count);
                thread::spawn(move || {
                    let socket = SocketAddrV4::new(ip, port);
                    let socket_addr = socket.into();
                    if TcpStream::connect_timeout(&socket_addr, timeout).is_ok() {
                        let service = get_service_name(port);
                        println!("Port {} is open   - {}", port, service);
                        open_ports_count.fetch_add(1, Ordering::Relaxed);
                    }
                })
            })
            .collect();

        for handle in handles {
            let _ = handle.join();
        }
        
        let total=open_ports_count.load(Ordering::Relaxed);
       if total==0{
        println!("No open ports found.");
       }
       else{
        println!("\n{} open ports found.", total);
       }

    }
    }


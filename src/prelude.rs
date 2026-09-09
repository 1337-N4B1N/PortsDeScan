

//! Commonly used standard-library imports shared across the crate.
pub use std::io::{self};
pub use std::net::{IpAddr,Ipv4Addr, SocketAddrV4, TcpStream};
pub use std::time::Duration;
pub use std::{thread, collections::HashSet};
pub use ipnetwork::{ Ipv4Network};
pub use dns_lookup::lookup_host;
pub use std::str::FromStr;
pub use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
pub use std::sync::Arc;

// Re-exports pub fn, pub struct, and pub enum  of this crate
pub use crate::port_scan::Mode;
pub use crate::map_services::{COMMON_PORTS, get_service_name};
pub use crate::address_resolver::{input_resolver, parse_cidr, domain_to_ipv4_addr};
pub use crate::port_scan::scan_open_ports;
pub use crate::input::{get_target_input, get_mode_input};
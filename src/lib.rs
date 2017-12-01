//! The crate simply parses `/etc/resolv.conf` file and creates a config object
//!
//! # Examples
//!
//! ## Parsing a config from a string
//! ```rust
//! extern crate resolv_conf;
//!
//! use std::net::{Ipv4Addr, Ipv6Addr};
//! use resolv_conf::{Ip, Config, Network};
//!
//! fn main() {
//!     let config_str = "
//! options ndots:8 timeout:8 attempts:8
//!
//! domain example.com
//! search example.com sub.example.com
//!
//! nameserver 2001:4860:4860::8888
//! nameserver 2001:4860:4860::8844
//! nameserver 8.8.8.8
//! nameserver 8.8.4.4
//!
//! options rotate
//! options inet6 no-tld-query
//!
//! sortlist 130.155.160.0/255.255.240.0 130.155.0.0";
//!
//!     // Parse the config.
//!     let parsed_config = Config::parse(&config_str).expect("Failed to parse config");
//!
//!     // We can build configs manually as well, either directly or with Config::new()
//!     let expected_config = Config {
//!         nameservers: vec![
//!             Ip::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888), None),
//!             Ip::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8844), None),
//!             Ip::V4(Ipv4Addr::new(8, 8, 8, 8)),
//!             Ip::V4(Ipv4Addr::new(8, 8, 4, 4)),
//!         ],
//!         search: vec![String::from("example.com"), String::from("sub.example.com")],
//!         sortlist: vec![
//!             Network::V4(Ipv4Addr::new(130, 155, 160, 0), Ipv4Addr::new(255, 255, 240, 0)),
//!             Network::V4(Ipv4Addr::new(130, 155, 0, 0), Ipv4Addr::new(255, 255, 0, 0)),
//!         ],
//!         debug: false,
//!         ndots: 8,
//!         timeout: 8,
//!         attempts: 8,
//!         rotate: true,
//!         no_check_names: false,
//!         inet6: true,
//!         ip6_bytestring: false,
//!         ip6_dotint: false,
//!         edns0: false,
//!         single_request: false,
//!         single_request_reopen: false,
//!         no_tld_query: true,
//!         use_vc: false,
//!     };
//!
//!     // We can compare configurations, since resolv_conf::Config implements Eq
//!     assert_eq!(parsed_config, expected_config);
//! }
//! ```
//!
//! ## Parsing a file
//!
//! ```rust
//! use std::io::Read;
//! use std::fs::File;
//!
//! extern crate resolv_conf;
//!
//! fn main() {
//!     // Read the file
//!     let mut buf = Vec::with_capacity(4096);
//!     let mut f = File::open("/etc/resolv.conf").unwrap();
//!     f.read_to_end(&mut buf).unwrap();
//!
//!     // Parse the buffer
//!     let cfg = resolv_conf::Config::parse(&buf).unwrap();
//!
//!     // Print the config
//!     println!("---- Parsed /etc/resolv.conf -----\n{:#?}\n", cfg);
//! }
//! ```

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

#[macro_use]
extern crate quick_error;

mod grammar;
mod ip;
mod config;

pub use grammar::ParseError;
pub use ip::{AddrParseError, Ip, Network};
pub use config::Config;

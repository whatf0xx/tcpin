use std::path::PathBuf;
use clap::Parser;

static IP: &'static str = "0.0.0.0";
static PORT: &'static str = "1024";

#[derive(Parser, Debug)]
struct CLArgs {
    executable: String,
    ip: Option<String>,
    port: Option<String>
}

pub struct TcpinArgs {
    pub executable: PathBuf,
    pub address: String
}

impl TcpinArgs {
    pub fn parse_and_collect() -> TcpinArgs {
        let clargs = CLArgs::parse();

        let ip = match clargs.ip {
            Some(ip) => ip,
            None => String::from(IP)
        };
    
        let port = match clargs.port {
            Some(port) => port,
            None => String::from(PORT)
        };

        let executable = PathBuf::from(clargs.executable);
        let address: String = format!("{}:{}", ip, port);

        TcpinArgs {
            executable,
            address
        }
    }
}

// if !exe_path.is_executable() {
    //     println!("Couldn't find executable '{}', tried looking at {:?}.", {&args.executable}, {exe_path});
    // } else {
    //     println!("Verified that '{:?}' can be run.", {exe_path});
    // }
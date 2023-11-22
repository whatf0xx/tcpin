use std::{
    fmt,
    env::{args, current_exe},
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    process::Command
};
use regex::Regex;

static IP: &'static str = "0.0.0.0";
static PORT: &'static str = "1024";


fn main() {
    let args: Vec<String> = args().collect();
    println!("{:?}", args);
    println!("{:?}", current_exe());
    let fpath: &str = &args.get(0).unwrap();
    let path_pattern = Regex::new(&format!(r"{}$", fpath)).unwrap();
    if path_pattern.is_match(&current_exe().unwrap().to_str().unwrap()) {
        println!("Matched the first arg ok");
    } else{
        panic!("First arg failed to match the current exe path!");
    }

    let addr: String = format!("{}:{}", IP, PORT);
    let listener = TcpListener::bind(addr).unwrap();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        if let Err(handling_error) = handle_connection(stream) {
            println!("Ran into an unexpected error, {}.", handling_error);
        }
    }
}

fn handle_connection(stream: TcpStream) -> Result<(), HandlingError> {
    let buf_reader = BufReader::new(stream);
    let mut found_errors: bool = false;
    let mut counter: i32 = 0;
    buf_reader.lines().for_each(
        |line| match line {
            Ok(txt) => {Command::new("echo")
                            .arg(txt)
                            .spawn()
                            .expect("Failed to execute command");
                        counter += 1},
            Err(err) => {println!("Ran into a problem, looks like {:?}", err);
                         found_errors = true},
        }
    );
    if found_errors {Err(HandlingError::BufferError)} else {Ok(())}
}

enum HandlingError {
    BufferError,

}

impl std::fmt::Display for HandlingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BufferError => write!(f, "BufferError"),
        }
    }
}
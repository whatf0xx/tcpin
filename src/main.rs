use std::{
    fmt,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1024").unwrap();

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
            Ok(txt) => {println!("{}: {}", counter, txt);
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
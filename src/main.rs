use std::net::TcpListener;

mod parse;
use parse::TcpinArgs;

mod handle;
use handle::handle_listener;

fn main() {
    let args =  TcpinArgs::parse_and_collect();
    let executable_path = args.executable;
    let addr = args.address;

    match TcpListener::bind(&addr) {
        Ok(listener) => { handle_listener(listener, &executable_path).unwrap() },
        Err(_) => println!("Some error handling should go here!")
    }
}

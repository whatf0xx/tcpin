use std::{
    path::Path,
    net::{TcpListener, TcpStream},
    thread,
    process::{Command, Stdio},
    os::fd::{AsRawFd, FromRawFd}
};

pub fn handle_listener(listener: TcpListener, executable_path: &Path) -> Result<(), String> {
    thread::scope(|s| {
        for connection in listener.incoming() {  
            match connection {
                Ok(stream) => { s.spawn(move || {
                            pin_to_stream(stream, &executable_path).unwrap();
                        }
                    );
                },
                Err(message) => println!("Unable to handle connection, {:?}", message)
            }
        }
            });
    Ok(())
}

fn pin_to_stream(stream: TcpStream, executable_path: &Path) -> Result<(), String> {
    let fd = stream.as_raw_fd();

        let _pinned_task = Command::new(&executable_path)
            .stdin(unsafe { Stdio::from_raw_fd(fd) })
            .stdout(unsafe { Stdio::from_raw_fd(fd) })
            .status()
            .unwrap();
        Ok(())
}
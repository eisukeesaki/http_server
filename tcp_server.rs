use std::process;
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;
use std::env;

const DEBUG: &str = "[DEBUG]";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Terminating due to insufficent number of arguments.");
        println!("usage: ./app <ipaddr> <port>");
        process::exit(1);
    }

    let ipaddr = &args[1];
    let port = &args[2];
    let socket = format!("{}:{}", ipaddr, port);

    let listener = match TcpListener::bind(socket) {
        Ok(l) => {
            println!("{} Listener bound to socket.", DEBUG);
            l
        },
        Err(e) => {
            eprintln!("Failed to bind listener to socket. {}", e);
            process::exit(1);
        }
    };

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                #[cfg(feature="debug")]
                {
                    match stream.peer_addr() {
                        Ok(client_addr) => println!("{} Connected to {:?}",
                            DEBUG, client_addr),
                        Err(e) => eprintln!("Failed to obtain client address. {}",
                            e),
                    };
                }

                let response = "Hello.";
                match stream.write_all(response.as_bytes()) {
                    Ok(()) => {
                        #[cfg(feature="debug")]
                        {
                            println!("{} Sent welcome message to user.\n", DEBUG);
                        }
                    },
                    Err(e) => eprintln!("Failed to respond to client. {}", e),
                }

                loop {
                    let mut buff = [0; 1024];
                    match stream.read(&mut buff) {
                        Ok(bytes_read) => {
                            let message =
                                String::from_utf8_lossy(&buff[0..bytes_read]);
                            print!("[Client]\n{}\n", message);
                        },
                        Err(e) => { // not tested
                            eprintln!("Couldn't read from stream. {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to connect to client. {}", e);
                process::exit(1);
            }
        }
    }
}


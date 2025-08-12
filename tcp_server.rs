fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("Terminating due to insufficent number of arguments.");
        println!("usage: ./app <ipaddr> <port>");
        std::process::exit(1);
    }

    let ipaddr = &args[1];
    let port = &args[2];
    let socket = format!("{}:{}", ipaddr, port);

    // let listener = match std::net::TcpListener::bind(socket) {
    //     Ok(l) => {
    //         println!("Listener bound to socket.");
    //         l
    //     },
    //     Err(e) => {
    //         eprintln!("Failed to bind listener to socket. {}", e);
    //         std::process::exit(1);
    //     }
    // };
    //
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             match stream.peer_addr() {
    //                 Ok(client_addr) => {
    //                     println!("Connected to {:?}", client_addr);
    //                     // how can we send string to client?
    //                 },
    //                 Err(e) => {
    //                     eprintln!("Failed to obtain client address. {}", e);
    //                 }
    //             };
    //         },
    //         Err(e) => {
    //             eprintln!("Failed to connect to client. {}", e);
    //             std::process::exit(1);
    //         }
    //     }
    // }
}


use std::sync::{Arc, Mutex};
use std::io::prelude::*;
use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tokio::prelude::future::ok;

#[derive(Debug, Default)]
struct Counts {
    lines: i32,
    words: i32,
    bytes: i32,
}

impl Counts {
    fn add(&self, other: Counts) -> Counts {
        Counts {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            bytes: self.bytes + other.bytes,
        }
    }
}


fn handle_connection(stream: TcpStream) -> Counts {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    let mut in_word = false;

    stream.bytes().for_each(|byte_result| {
        let byte = byte_result.unwrap();
        if !byte.is_ascii_whitespace() && !in_word {
            in_word = true;
            words += 1;
        }
        if byte.is_ascii_whitespace() {
            in_word = false;
        }
        if byte == b'\n' {
            lines += 1;
        }
        bytes += 1;
    });
    let c = Counts { lines: lines, words: words, bytes: bytes };
    println!("connection counts: {:?}", c);
    c
}

fn main() {
//    let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();

    let global_counts = Arc::new(Mutex::new(Counts::default()));

    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming()
        .map_err(|e| eprintln!("Failed to accept: {}", e))
        .for_each(|socket| {
            println!("Connection from {}", socket.peer_addr().unwrap());
//            tokio::spawn()
            ok(())
        });

    tokio::run(server);
    
//    pool.install(|| {
//        for stream in listener.incoming() {
//            let stream = stream.unwrap();
//            println!("connection established");
//            let gcc = global_counts.clone();
//            pool.spawn(move || {
//                let con_counts = handle_connection(stream);
//                {
//                    let mut gc = gcc.lock().unwrap();
//                    *gc = gc.add(con_counts);
//                    println!("global counts: {:?}", gc);
//                }
//            });
//        }
//    })
}

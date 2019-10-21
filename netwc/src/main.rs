use std::sync::{Arc, Mutex};
use tokio::prelude::*;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tokio::codec::{BytesCodec, Decoder};

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


fn handle_connection(stream: TcpStream) -> Box<Future<Item=Counts, Error=()> + Send> {
    eprintln!("Connection from {}", stream.peer_addr().unwrap());
    let lines = Arc::new(Mutex::new(0));
    let words = Arc::new(Mutex::new(0));
    let bytes_count = Arc::new(Mutex::new(0));

    let mut in_word = false;

    let lines_mutator = lines.clone();
    let words_mutator = words.clone();
    let bytes_count_mutator = bytes_count.clone();
    let data_handler = BytesCodec::new().framed(stream).for_each(move |bytes| {
        bytes.iter().for_each(|byte| {
            if !byte.is_ascii_whitespace() && !in_word {
                in_word = true;
                *words_mutator.lock().unwrap() += 1;
            }
            if byte.is_ascii_whitespace() {
                in_word = false;
            }
            if *byte == b'\n' {
                *lines_mutator.lock().unwrap() += 1;
            }
            *bytes_count_mutator.lock().unwrap() += 1;
        });
        Ok(())
    }).then(move |_result| {
        Ok(Counts { lines: *lines.lock().unwrap(), words: *words.lock().unwrap(), bytes: *bytes_count.lock().unwrap() })
    });

    Box::new(data_handler)
}

fn main() {
    let global_counts = Arc::new(Mutex::new(Counts::default()));

    let addr = "127.0.0.1:9000".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let server = listener.incoming()
        .map_err(|e| eprintln!("Failed to accept: {}", e))
        .for_each(move |socket| {
            let gcc = Arc::clone(&global_counts);
            tokio::spawn(handle_connection(socket).and_then(move |counts| {
                let mut gc = gcc.lock().unwrap();
                *gc = gc.add(counts);
                println!("global counts: {:?}", gc);
                Ok(())
            }))
        });

    tokio::run(server);
}

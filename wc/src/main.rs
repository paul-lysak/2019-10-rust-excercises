use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::io::BufReader;

#[derive(Debug, Default)]
struct Counts {
    lines: i32,
    words: i32,
    bytes: i32,
}

impl Counts {
    fn add(self, other: Counts) -> Counts {
        Counts {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            bytes: self.bytes + other.bytes,
        }
    }
}

//trait ByteIterator: Iterator {
//    //    type Item = u8;
//    type Item = Result<u8, io::Error>;
//}

fn wc<T: Read>(in_bytes: io::Bytes<T>) -> Counts {
//fn wc(in_bytes: &ByteIterator) -> Counts {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    let mut in_word = false;

    in_bytes.for_each(|byte_result| {
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
    return Counts { lines: lines, words: words, bytes: bytes };
}

fn main() {
    let args = env::args();

    let file_names = args.skip(1);
    let counts = if file_names.len() == 0 {
        let text_in = io::stdin();
        wc(text_in.bytes())
    } else {
        file_names.map(|file_name| {
//            wc(fs::File::open(file_name).unwrap().bytes())
            wc(BufReader::new(fs::File::open(file_name).unwrap()).bytes())
//            wc(fs::read(file_name).unwrap().bytes())
//            c.add(wc(fs::read(file_name).unwrap()))
        }).fold(Counts::default(), |acc, c| {
            acc.add(c)
        })
    };

    dbg!(counts);
}

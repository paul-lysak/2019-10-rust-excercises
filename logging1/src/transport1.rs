extern crate pretty_env_logger;
//#[macro_use] extern crate log;

fn main() {
    println!("Hello, transport1!");

    pretty_env_logger::init();
    lib::example("transport1");
}


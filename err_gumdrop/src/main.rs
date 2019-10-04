#[macro_use] extern crate gumdrop;
use gumdrop::Options;

#[derive(Options, Debug)]
struct CmdLine {
    items: i32,

    #[options(free)]
    free: Vec<String>,
}

fn main() {
    let cmd_line= CmdLine::parse_args_default_or_exit();
    println!("Command line args: {:?}", cmd_line);
}

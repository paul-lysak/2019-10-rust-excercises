use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CmdLine {
    #[structopt(short = "i", long)]
    items: i32
}

fn main() {
    let cmd_line= CmdLine::from_args();
    println!("Command line args: {:?}", cmd_line);
}

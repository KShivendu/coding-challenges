use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    // Name to print
    #[arg(short, long)]
    name: String,

    // Number of times to print the message
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}

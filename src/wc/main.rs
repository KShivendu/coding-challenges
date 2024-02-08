use clap::Parser;
use clap::CommandFactory;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    // File path to read
    #[clap(short, long)]
    file: String,

    // Calculate the number of bytes in the file
    #[clap(short, long)]
    bytes: bool,

    // Calculate the number of lines in the file
    #[clap(short, long)]
    lines: bool,

    // Calculate the number of words in the file
    #[clap(short, long)]
    words: bool,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    if args.bytes {
        let bytes = std::fs::read(&args.file).unwrap().len();
        println!("{} {}", bytes, args.file);
    } else if args.lines {
        let lines = std::fs::read_to_string(&args.file).unwrap().lines().count();
        println!("{} {}", lines, args.file);
    } else if args.words {
        let words = std::fs::read_to_string(&args.file).unwrap().split_whitespace().count();
        println!("{} {}", words, args.file);
    } else {
        let mut cmd = Args::command();
        cmd.print_help(); // FIXME: Find a cleaner solution to print help
    }
}

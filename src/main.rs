use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    
    // let _pattern = std::env::args().nth(1).expect("no pattern given");
    // let _path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: _pattern,
    //     path: std::path::PathBuf::from(_path),
    // };

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

}

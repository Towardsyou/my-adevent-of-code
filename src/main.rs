// use clap::Parser;

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     /// Network port to use
//     port: u16,
// }

// fn main() {
//     let cli = Cli::parse();

//     println!("PORT = {}", cli.port);
// }

// #[test]
// fn verify_cli() {
//     use clap::CommandFactory;
//     Cli::command().debug_assert();

//     let cli = Cli::parse_from(&["--port", "1234"]);
//     assert_eq!(cli.port, 1234);
//     let cli = Cli::parse_from(&["-p", "1234"]);
//     assert_eq!(cli.port, 1234);
// }

use std::io::Error;

mod advent;

fn main() -> Result<(), Error> {
    // advent::day1::main()
    advent::day5::main()
}

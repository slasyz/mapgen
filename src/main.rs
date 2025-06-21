mod cli;

use clap::{CommandFactory, Parser};
use cli::Cli;

fn main() {
	let cli = Cli::parse();
	if cli.watch && cli.output.is_none() {
		eprintln!("Error: watch mode requires an output file.\n");
		// print usage and exit 1
		Cli::command().print_help().unwrap();
		std::process::exit(1);
	}

	println!("{:?}", cli);
}

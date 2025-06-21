mod cli;
pub mod parser; // TODO: "pub" is temporary
pub mod files;

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

	let depth = if cli.one_level {
		1
	} else if cli.depth.is_some() {
		cli.depth.unwrap()
	} else {
		999
	};

	let files = files::get_files(&cli.sources, depth).unwrap();

	for file in files {
		println!("--------------------------------");
		println!("File: {}", file.display());
		println!("--------------------------------");

		let mut reader = std::fs::File::open(file).unwrap();
		let result = parser::process::process(None, &mut reader, &mut std::io::stdout());
		if result.is_err() {
			eprintln!("Error: {}", result.err().unwrap());
			std::process::exit(1);
		}
	}
	println!("--------------------------------");
}

mod cli;
mod files;
mod parser;

use std::io;
use std::path::{Path, PathBuf};

use clap::{CommandFactory, Parser};
use cli::Cli;

use crate::parser::Language;

fn header(file: &Path) -> String {
	format!("--------------------------------\nFile: {}\n--------------------------------\n", file.display())
}

fn once(files: Vec<PathBuf>, writer: &mut impl io::Write) {
	for file in files {
		writer.write_all(header(&file).as_bytes()).unwrap();

		let language = Language::from_extension(file.extension().unwrap_or_default().to_str().unwrap());

		let mut reader = std::fs::File::open(file).unwrap();
		let result = parser::process::process(language, &mut reader, writer);
		if result.is_err() {
			eprintln!("Error: {}", result.err().unwrap());
			std::process::exit(1);
		}
		writer.write_all(b"\n").unwrap();
	}
}

fn watch(_files: Vec<PathBuf>, _writer: &mut impl io::Write) {
	panic!("Not implemented");
}

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

	let mut writer: Box<dyn io::Write> = match cli.output {
		Some(output) => {
			let file = std::fs::File::create(output).unwrap();
			Box::new(file)
		}
		None => Box::new(std::io::stdout()),
	};

	if cli.watch {
		watch(files, &mut writer);
	} else {
		once(files, &mut writer);
	}
}

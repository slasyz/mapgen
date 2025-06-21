use std::io;

use tree_sitter::{Node};

pub enum Language {
	Rust,
	Go,
	Js,
}

impl Language {
	pub fn from_extension(ext: &str) -> Option<Self> {
		match ext {
			"rs" => Some(Self::Rust),
			"go" => Some(Self::Go),
			"js" => Some(Self::Js),
			_ => None,
		}
	}
}

impl std::fmt::Debug for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

pub fn process_rust_code(code: &str) -> String {
	let mut parser = tree_sitter::Parser::new();
	parser
		.set_language(&tree_sitter_rust::LANGUAGE.into())
		.expect("Error loading Rust grammar");
	let tree = parser.parse(code, None).unwrap();

	let mut output = String::new();
	let mut last_end = 0;

	process_node_with_whitespace(tree.root_node(), code, &mut output, &mut last_end);

	// Add any remaining text after the last node
	if last_end < code.len() {
		output.push_str(&code[last_end..]);
	}

	output
}

fn process_node_with_whitespace(node: Node, code: &str, output: &mut String, last_end: &mut usize) {
	let start = node.start_byte();
	let end = node.end_byte();

	// Add any whitespace/text between the last processed position and this node
	if start > *last_end {
		// println!("adding whitespace: ***{:?}***", &code[*last_end..start]);
		output.push_str(&code[*last_end..start]);
		*last_end = start;
	}

	match node.kind() {
		"function_item" => {
			// For functions, find where the body starts
			let mut cursor = node.walk();
			cursor.goto_first_child();

			let mut body_start = end; // fallback

			// Find the block (function body)
			loop {
				let child = cursor.node();
				if child.kind() == "block" {
					body_start = child.start_byte();
					break;
				}

				if !cursor.goto_next_sibling() {
					break;
				}
			}

			// Include everything up to the body
			output.push_str(&code[start..body_start]);
			// Replace body with placeholder
			output.push_str("{ ... }");

			*last_end = end;
		}
		_ => {
			// For non-function nodes, process children recursively
			if node.child_count() == 0 {
				// Leaf node - include its text
				let text = &code[start..end];
				// println!("adding leaf node: ***{:?}***", text);
				output.push_str(text);
				*last_end = end;
			} else {
				// Process all children
				let mut cursor = node.walk();
				cursor.goto_first_child();

				loop {
					process_node_with_whitespace(cursor.node(), code, output, last_end);

					if !cursor.goto_next_sibling() {
						break;
					}
				}
			}
		}
	}
}


pub fn parse(
	language: Language,
	reader: &mut impl io::Read,
	writer: &mut impl io::Write,
) -> io::Result<()> {
	let mut code = String::new();
	reader.read_to_string(&mut code)?;

	match language {
		Language::Rust => {
			let processed = process_rust_code(&code);
			writer.write_all(processed.as_bytes())?;
			Ok(())
		}
		_ => {
			// Fallback for other languages is to just copy the content
			writer.write_all(code.as_bytes())
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;
	use std::path::Path;

	#[test]
	fn test_parse_files() {
		let test_data_dir = Path::new("src/parser/test_data");
		
		// Find all .in files
		let entries = fs::read_dir(test_data_dir).expect("Failed to read test_data directory");
		
		for entry in entries {
			let entry = entry.expect("Failed to read directory entry");
			let path = entry.path();
			
			if let Some(extension) = path.extension() {
				if extension == "in" {
					// Extract language and number from filename (e.g., "rs-1.in" -> "rs", "1")
					let filename = path.file_name().unwrap().to_str().unwrap();
					let parts: Vec<&str> = filename.split('-').collect();
					
					if parts.len() == 2 {
						let lang_str = parts[0];
						let num = parts[1].trim_end_matches(".in");
						
						let language = Language::from_extension(lang_str).unwrap();
						let mut input = fs::File::open(&path).expect("Failed to open input file");

						let out_path = test_data_dir.join(format!("{}-{}.out", lang_str, num));
						let expected = fs::read_to_string(&out_path).expect("Failed to read expected output");
						
						// Process the input
						let mut output = Vec::new();
						parse(language, &mut input, &mut output).expect("Failed to parse");
						
						let actual = String::from_utf8(output).expect("Failed to convert output to string");
						
						// Compare results
						assert_eq!(actual, expected, "Mismatch for file {}", filename);
					}
				}
			}
		}
	}
}

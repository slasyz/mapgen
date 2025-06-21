pub enum Language {
	Rust,
	Go,
	JavaScript,
	Python,
}

pub struct LanguageGrammar {
	pub function: &'static str,
	pub function_body: &'static str,
	pub replacement: &'static str,
}

impl Language {
	pub fn from_extension(ext: &str) -> Option<Self> {
		match ext {
			"rs" => Some(Self::Rust),
			"go" => Some(Self::Go),
			"js" => Some(Self::JavaScript),
			"py" => Some(Self::Python),
			_ => None,
		}
	}

	pub fn get_tree_sitter_language(&self) -> Option<tree_sitter::Language> {
		match self {
			Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
			Language::Go => Some(tree_sitter_go::LANGUAGE.into()),
			Language::JavaScript => Some(tree_sitter_javascript::LANGUAGE.into()),
			Language::Python => Some(tree_sitter_python::LANGUAGE.into()),
		}
	}

	pub fn get_grammar(&self) -> Option<LanguageGrammar> {
		match self {
			Language::Rust => Some(LanguageGrammar {
				function: "function_item",
				function_body: "block",
				replacement: "{ ... }",
			}),
			Language::Go => Some(LanguageGrammar {
				function: "function_declaration",
				function_body: "block",
				replacement: "{ ... }",
			}),
			Language::JavaScript => Some(LanguageGrammar {
				function: "function_declaration",
				function_body: "statement_block",
				replacement: "{ ... }",
			}),
			Language::Python => Some(LanguageGrammar {
				function: "function_definition",
				function_body: "block",
				replacement: "...",
			}),
		}
	}
}

impl std::fmt::Debug for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Language::Rust => write!(f, "Rust"),
			Language::Go => write!(f, "Go"),
			Language::JavaScript => write!(f, "JavaScript"),
			Language::Python => write!(f, "Python"),
		}
	}
}

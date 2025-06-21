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

	pub fn get_tree_sitter_language(&self) -> Option<tree_sitter::Language> {
		match self {
			Language::Rust => Some(tree_sitter_rust::LANGUAGE.into()),
			// Language::Go => tree_sitter_go::language(),
			// Language::Js => tree_sitter_javascript::language(),
			_ => None,
		}
	}
}

impl std::fmt::Debug for Language {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Language::Rust => write!(f, "Rust"),
			Language::Go => write!(f, "Go"),
			Language::Js => write!(f, "JavaScript"),
		}
	}
}

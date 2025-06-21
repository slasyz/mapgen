/// Returns a list of files in the directory.
///
/// If depth is 0, return nothing.
/// If path is a file, return it anyway.
/// If depth is 1, return the list of files in the directory only.
/// If depth is greater than 1, return the list of files in the directory and its subdirectories recursively.
fn traverse_dir(path: &std::path::Path, depth: usize) -> Result<Vec<std::path::PathBuf>, String> {
	if depth == 0 {
		return Ok(vec![]);
	}

	if !path.exists() {
		return Err(format!("Path does not exist: {}", path.display()));
	}
	if path.is_file() {
		return Ok(vec![path.to_path_buf()]);
	}
	if !path.is_dir() {
		return Err(format!("Path is neither file nor directory: {}", path.display()));
	}
	
	let mut files: Vec<std::path::PathBuf> = Vec::new();
	
	for entry in std::fs::read_dir(path).map_err(|e| e.to_string())? {
		let entry = entry.map_err(|e| e.to_string())?;
		let entry_path = entry.path();
		
		if entry_path.is_file() {
			files.push(entry_path);
		} else if entry_path.is_dir() && depth > 1 {
			files.extend(traverse_dir(&entry_path, depth - 1)?);
		}
	}
	
	Ok(files)
}

// Expands glob patterns, and after that keeps files and traverses directories recursively.
pub fn get_files(sources: &[String], depth: usize) -> Result<Vec<std::path::PathBuf>, String> {
	let mut all_files: Vec<std::path::PathBuf> = Vec::new();
	
	for source in sources {
		// Expand glob pattern
		let glob_results = glob::glob(source).map_err(|e| format!("Invalid glob pattern '{}': {}", source, e))?;
		
		for glob_result in glob_results {
			let path = glob_result.map_err(|e| format!("Glob error: {}", e))?;
			
			// Use traverse_dir to handle both files and directories
			let files = traverse_dir(&path, depth)?;
			all_files.extend(files);
		}
	}
	
	Ok(all_files)
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	// Create test file structure:
	// /tmp/traverse_test/
	// ├── file1.txt
	// ├── file2.txt
	// ├── subdir1/
	// │   ├── file3.txt
	// │   └── subdir2/
	// │       └── file4.txt
	// └── subdir3/
	//     └── file5.txt
	fn setup_test_directory() -> std::path::PathBuf {
		// Create temporary directory
		let temp_dir = std::env::temp_dir().join("traverse_test");
		if temp_dir.exists() {
				fs::remove_dir_all(&temp_dir).unwrap();
		}
		fs::create_dir_all(&temp_dir).unwrap();

		// Create test file structure
		fs::write(temp_dir.join("file1.txt"), "content1").unwrap();
		fs::write(temp_dir.join("file2.txt"), "content2").unwrap();
		let subdir1 = temp_dir.join("subdir1");
		fs::create_dir_all(&subdir1).unwrap();
		fs::write(subdir1.join("file3.txt"), "content3").unwrap();
		let subdir2 = subdir1.join("subdir2");
		fs::create_dir_all(&subdir2).unwrap();
		fs::write(subdir2.join("file4.txt"), "content4").unwrap();
		let subdir3 = temp_dir.join("subdir3");
		fs::create_dir_all(&subdir3).unwrap();
		fs::write(subdir3.join("file5.txt"), "content5").unwrap();

		temp_dir
	}

	fn cleanup_test_directory(temp_dir: &std::path::PathBuf) {
		if temp_dir.exists() {
			fs::remove_dir_all(temp_dir).unwrap();
		}
	}

	#[test]
	fn test_traverse_dir() {
		let temp_dir = setup_test_directory();

		let test_cases = vec![
			(0, vec![]),
			(1, vec!["file1.txt", "file2.txt"]),
			(2, vec!["file1.txt", "file2.txt", "file3.txt", "file5.txt"]),
			(3, vec!["file1.txt", "file2.txt", "file3.txt", "file4.txt", "file5.txt"]),
		];

		for (depth, expected_files) in test_cases {
			let result = traverse_dir(&temp_dir, depth).unwrap();
			assert_eq!(result.len(), expected_files.len());
			for file in result {
				assert!(expected_files.contains(&file.file_name().unwrap().to_str().unwrap()));
			}
		}

		// Test with file path
		let file_path = temp_dir.join("file1.txt");
		let result = traverse_dir(&file_path, 1).unwrap();
		assert_eq!(result.len(), 1);
		assert_eq!(result[0], file_path);

		// Test with non-existent path
		let non_existent = temp_dir.join("does_not_exist");
		let result = traverse_dir(&non_existent, 1);
		assert!(result.is_err());

		// Sources, depth
		let test_cases = vec![
			(vec!["**/*.txt"], 1, vec!["file1.txt", "file2.txt", "file3.txt", "file4.txt", "file5.txt"]),
			(vec!["*.txt"], 1, vec!["file1.txt", "file2.txt"]),
			(vec![""], 1, vec!["file1.txt", "file2.txt"]),
			(vec![""], 0, vec![]),
			(vec!["file1.txt"], 1, vec!["file1.txt"]),
			(vec!["*"], 1, vec!["file1.txt", "file2.txt", "file3.txt", "file5.txt"]),
		];

		for (patterns, depth, expected_files) in test_cases {
			let patterns_abs: Vec<String> = patterns.iter().map(|p| format!("{}/{}", temp_dir.display(), p)).collect();
			let files = get_files(&patterns_abs, depth).unwrap();
			assert_eq!(files.len(), expected_files.len());
			for file in files {
				assert!(expected_files.contains(&file.file_name().unwrap().to_str().unwrap()));
			}
		}

		// Cleanup
		cleanup_test_directory(&temp_dir);
	}
}

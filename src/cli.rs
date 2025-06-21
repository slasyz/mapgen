use clap::Parser;

/// A tool for generating a map of a code repository.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Paths to files or directories to generate map for.
    /// Can be a single file, a directory, or a glob pattern.
    #[arg(required = false)]
    pub sources: Vec<String>,

    /// Output file to write map to.
    /// If not specified, map will be printed to stdout.
    #[arg(short, long)]
    pub output: Option<String>,

    /// Max depth of the traversal.
    #[arg(short = 'd', long, value_name = "N", conflicts_with = "one_level")]
    pub depth: Option<usize>,

    /// Only direct children of specified directory.  The same as --depth 1.
    #[arg(short = '1', conflicts_with = "depth")]
    pub one_level: bool,

    /// Watch for changes in sources and re-generate map on change.
    /// Can only be used with the output file specified.
    #[arg(short, long)]
    pub watch: bool,
} 

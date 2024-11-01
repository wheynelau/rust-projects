use clap::Parser;

/// Struct for reddit parser
#[derive(Parser, Debug)]
#[clap(
    author,
    version,
    long_about = "Reddit data parser, will parse the reddit data into a graph structure",
)]
pub struct Cli {
    #[clap(short, long, help="Input to prefix, assumes submissions and comments.zst exists.",
    value_hint=clap::ValueHint::DirPath)]
    pub prefix: String,
    #[clap(short, long, help = "Output folder for the JSONL files, will write the jsonl as subreddit.jsonl",
    value_hint=clap::ValueHint::DirPath)]
    pub output: Option<String>,
    #[clap(
        short,
        long,
        help = "Tokenizer name: Accepts huggingface <org>/<name> or a path to tokenizer.json\nIf not provided, will split and count words"
    )]
    pub tokenizer: Option<String>,
    #[clap(
        long,
        default_value = "reddit",
        help="Source of the forum"
    )]
    source: String,
}

impl Cli {
    pub fn get_output(&self) -> String {
        self.output.clone().unwrap_or_else(|| format!("{}.jsonl", self.prefix))
    }
}
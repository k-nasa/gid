pub(crate) use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "k-nasa <htilcs1115@gmail.com>",
    about = "Issue graphical tool"
)]
pub(crate) struct CliArgs {
    #[clap(short, long)]
    pub organization: String,

    #[clap(short, long)]
    pub repository: String,

    #[clap(short, long)]
    pub issue_number: u64,
}

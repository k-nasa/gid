pub(crate) use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "0.1.0",
    author = "k-nasa <htilcs1115@gmail.com>",
    about = "Help project managers and project owners with easy-to-understand views of github issue dependencies."
)]
pub(crate) struct CliArgs {
    #[clap(short, long)]
    pub organization: String,

    #[clap(short, long)]
    pub repository: String,

    #[clap(short, long)]
    pub issue_number: u64,
}

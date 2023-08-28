#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub(crate) enum Commands {
    Upload(Upload),
}

#[derive(clap::Args, Debug)]
pub(crate) struct Upload {
    /// The file to upload
    #[clap(short, long)]
    pub file: std::path::PathBuf,

    /// The S3 path to upload to
    #[clap(short, long)]
    pub s3_path: String,
}

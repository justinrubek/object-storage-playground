use crate::{commands::Commands, error::Result};
use clap::Parser;
use s3::{creds::Credentials, region::Region, Bucket};

mod commands;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = commands::Args::parse();
    match args.command {
        Commands::Upload(upload) => {
            let file = tokio::fs::read(upload.file).await?;

            let token_id = std::env::var("STORAGE_TOKEN_ID")?;
            let secret_token = std::env::var("STORAGE_SECRET_TOKEN")?;

            let bucket = Bucket::new(
                "scratch-files",
                Region::Custom {
                    region: "us-east-1".to_owned(),
                    endpoint: "http://alex:9000".to_owned(),
                },
                Credentials::new(Some(&token_id), Some(&secret_token), None, None, None)?,
            )?
            .with_path_style();

            let s3_path = upload.s3_path;

            let response = bucket.put_object(s3_path, &file).await?;
            tracing::info!(?response);
        }
    }

    Ok(())
}

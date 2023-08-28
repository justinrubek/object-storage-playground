#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    S3(#[from] s3::error::S3Error),
    #[error(transparent)]
    S3Credentials(#[from] s3::creds::error::CredentialsError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
}

pub type Result<T> = std::result::Result<T, Error>;

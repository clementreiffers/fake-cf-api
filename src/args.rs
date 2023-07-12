use clap::{arg, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct S3Params {
    #[arg(long)]
    pub(crate) s3_bucket_name: String,
    #[arg(long)]
    pub(crate) s3_endpoint: String,
    #[arg(long)]
    pub(crate) s3_region: String,
}

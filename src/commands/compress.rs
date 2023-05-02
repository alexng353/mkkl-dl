use super::*;
use crate::utils::compress::compress;

/// Compress a manga into a CBZ file
#[derive(Parser)]
pub struct Args {
    /// Force overwrite of existing files
    #[clap(short, long)]
    force: bool,

    /// Format of the output file (cbz, zip, etc.)
    #[clap(long)]
    format: String,
}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    let force = args.force;
    let format = if !args.format.clone().starts_with(".") {
        format!(".{}", args.format.clone())
    } else {
        args.format.clone()
    };

    compress(force, format).await?;

    Ok(())
}

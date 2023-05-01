/// Add a new plugin to your project
#[derive(Parser)]
pub struct Args {}

pub async fn command(args: Args, _json: bool) -> Result<()> {
    println!("hello world");

    Ok(())
}

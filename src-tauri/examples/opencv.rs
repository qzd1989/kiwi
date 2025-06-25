use opencv::core;

fn main() -> opencv::Result<()> {
    println!("OpenCV version: {}", core::get_version_string()?);
    Ok(())
}

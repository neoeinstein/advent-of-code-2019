use advent_of_code_2019::day08;
use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    let image = day08::Image::from_bytes(day08::PUZZLE_INPUT.trim(), day08::PUZZLE_DIMENSIONS);

    let (layer_num, checksum) = image.checksum();

    println!("Checksum (layer {}): {}", layer_num, checksum,);

    if log::log_enabled!(log::Level::Debug) {
        for (i, layer) in image.layers().enumerate() {
            log::debug!("Layer {}:\n{}", i, layer);
        }
    }

    log::debug!("Resolving… [from background forward]");
    let resolved = image.resolve_back();
    println!("Resolved (back to front):\n{}", resolved);

    log::debug!("Resolving… [from foreground backward]");
    let resolved = image.resolve();
    println!("Resolved (front to back):\n{}", resolved);

    Ok(())
}

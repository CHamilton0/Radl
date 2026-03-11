mod glamadelaide;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0")
        .build()?;

    let _ = glamadelaide::get_this_week(client);

    Ok(())
}

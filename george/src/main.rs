use std::thread::sleep;
use std::time::Duration;
use george::George;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new();

    george.start().await?;
    sleep(Duration::from_secs(5));
    george.fill_in("name input field", "Logan Keenan").await?;
    george.click("sign up button").await?;
    george.fill_in("email input field", "logan1@meshly.cloud").await?;

    george.stop().await?;

    Ok(())
}
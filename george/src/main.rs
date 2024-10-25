use george::George;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let george = George::new();

    george.fill_in("name input field", "Logan Keenan").await?;
    george.fill_in("email input field", "logan1@meshly.cloud").await?;
    george.click("sign up button").await?;

    Ok(())
}
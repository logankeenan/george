use std::time::Duration;
use george::George;


async fn test_fill_in_and_submit() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new();
    george.start().await?;
    george.execute(
        "firefox http://host.docker.internal:3001 --width=1024 --height=768 --display=:99",
        false,
    ).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;
    george.fill_in("input name field", "Ada Lovelace").await?;
    george.fill_in("input phone field", "5554443333").await?;
    george.fill_in("input email field", "ada@email.com").await?;
    george.click("First Programmer checkbox").await?;
    george.click("Programming radio label").await?;
    george.click("submit button").await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    george.coordinate_of("Success text").await?;

    george.stop().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    test_fill_in_and_submit().await.unwrap()
}

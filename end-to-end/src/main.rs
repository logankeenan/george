#[cfg(test)]
mod tests {
    use std::time::Duration;
    use george::George;

    #[tokio::test]
    async fn test_fill_in_and_submit() -> Result<(), Box<dyn std::error::Error>> {
        let mut george = George::new();
        george.start().await?;
        tokio::time::sleep(Duration::from_secs(5)).await;
        george.fill_in("input name field", "Ada Lovelace").await?;
        george.fill_in("input phone field", "5554443333").await?;
        george.fill_in("input email field", "ada@email.com").await?;
        george.click("First Programmer checkbox").await?;
        george.click("Programmer radio option").await?;
        george.click("submit button").await?;
        tokio::time::sleep(Duration::from_secs(3)).await;
        george.coordinate_of("Success text").await?;

        george.stop().await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The main function can remain empty if you're only running tests
    Ok(())
}

use std::time::Duration;
use george::George;


async fn fill_out_form_and_submit(george: &mut George) -> Result<(), Box<dyn std::error::Error>> {

    tokio::time::sleep(Duration::from_secs(5)).await;
    george.fill_in("input Name text field", "Ada Lovelace").await?;
    george.fill_in("input Phone text field", "5554443333").await?;
    george.fill_in("input Email text field", "ada@email.com").await?;
    george.click("checkbox labeled First Programmer").await?;
    george.click("center of the radio button labeled Programming").await?;
    george.click("blue submit button").await?;
    tokio::time::sleep(Duration::from_secs(3)).await;
    george.coordinate_of("Success text").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new();
    george.start().await?;

    let mut success_count = 0;
    let mut failure_count = 0;

    let iterations = 20;
    for i in 1..=iterations {
        println!("Starting iteration {}", i);

        george.execute(
            "firefox http://host.docker.internal:3001 --width=1024 --height=768 --display=:99",
            false,
        ).await?;

        match fill_out_form_and_submit(&mut george).await {
            Ok(_) => {
                println!("Iteration {}: Success", i);
                success_count += 1;
            }
            Err(e) => {
                println!("Iteration {}: Failure - {}", i, e);
                failure_count += 1;
            }
        }

        george.execute("pkill firefox", false).await?;
    }

    println!("\nCompleted {} iterations.", iterations);
    println!("Success count: {}", success_count);
    println!("Failure count: {}", failure_count);

    george.stop().await?;

    Ok(())
}
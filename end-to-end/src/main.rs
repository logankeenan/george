use george::George;


async fn fill_out_form_and_submit(george: &mut George) -> Result<(), Box<dyn std::error::Error>> {

    george.wait_until_text_is_visible("End-to-End Test").await?;
    george.fill_in("input Name text field", "Ada Lovelace").await?;
    george.fill_in("input Phone text field", "5554443333").await?;
    george.fill_in("input Email text field", "ada@email.com").await?;
    george.click("checkbox labeled First Programmer").await?;
    george.click("radio button labeled Programming").await?;
    george.click("blue submit button").await?;
    george.wait_until_text_is_visible("Success").await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new("http://logan-server:8000");
    george.start().await?;

    let mut success_count = 0;
    let mut failure_count = 0;

    let iterations = 5;
    for i in 1..=iterations {
        println!("Starting iteration {}", i);

        george.open_firefox("http://host.docker.internal:3001").await?;

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

        george.close_firefox().await?;
    }

    println!("\nCompleted {} iterations.", iterations);
    println!("Success count: {}", success_count);
    println!("Failure count: {}", failure_count);

    george.stop().await?;

    Ok(())
}
use std::time::Duration;
use george::George;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new();
    george.start().await?;
    george.execute(
        "firefox http://host.docker.internal:3001 --width=1024 --height=768 --display=:99",
        false,
    ).await?;

    tokio::time::sleep(Duration::from_secs(5)).await;

    println!("Interactive George Coordinate Finder");
    println!("Enter 'quit' to exit the program");

    loop {
        print!("Enter prompt: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        match george.coordinate_of_raw(input).await {
            Ok((x, y)) => println!("Coordinates: ({}, {})", x, y),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    println!("Exiting program");
    george.stop().await?;

    Ok(())
}
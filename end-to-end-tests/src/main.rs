

#[tokio::test]
async fn test_fill_out_form() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use george_ai::George;

    dotenv::dotenv().ok();

    let vision_llm_url = env::var("VISION_LLM_URL")
        .expect("VISION_LLM_URL must be set in .env file");
    let auth_token = env::var("VISION_LLM_AUTH_TOKEN")
        .expect("VISION_LLM_AUTH_TOKEN must be set in .env file");
    let local_ip = env::var("LOCAL_IP")
        .unwrap_or_else(|_| "host.docker.internal".to_string());

    let mut george = George::new(&vision_llm_url);
    george.set_vision_llm_auth_token(&auth_token);
    george.start().await?;

    george.open_chrome(&format!("http://{}:3001", local_ip)).await?;

    george.wait_until_text_is_visible("End-to-End Test").await?;
    george.fill_in("input Name text field", "Ada Lovelace").await?;
    george.fill_in("input Phone text field", "5554443333").await?;
    george.fill_in("input Email text field", "ada@email.com").await?;
    george.click("checkbox labeled First Programmer").await?;
    george.click("radio button labeled Programming").await?;
    george.click("blue submit button").await?;

    george.wait_until_text_is_visible("Success").await?;

    george.close_chrome().await?;
    george.stop().await?;

    Ok(())
}

fn main() {}
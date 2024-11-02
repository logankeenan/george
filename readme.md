# George

George is an API leveraging AI to make it easy to control a computer with natural language.


```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new("https://your-molmo-llm.com");
    george.start().await?;
    george.open_firefox("https://some-website.com").await?;
    george.click("sign in link").await?;
    george.fill_in("input Email text field", "your@email.com").await?;
    george.fill_in("input Password text field", "super-secret").await?;
    george.click("sign in button").await?;
    george.stop().await?
}
```


https://github.com/user-attachments/assets/4c0a2d85-7eb3-4851-93ff-817c05658776


## Getting Start
Prerequisites
* Rust
* Docker
* A server running [Molmo-7B-D-0924](https://huggingface.co/allenai/Molmo-7B-D-0924). See the Docker + vllm example [script](./scripts/start-molmo.sh)

## Roadmap
* Improve Documentation
* Remove all my person stuff from the repo
* Improve debugging and logging
  * Remove all the println statements
  * Provide the inputs/outputs for each LLM interactive in an easily debuggable format
* Add an actual test suite to the end-to-end project
  * Includes running these tests in CI
* Create bindings for other languages
  * Ruby
  * Python
  * JavaScript/Typescript
  * Others?
* Create a UI to help build out the selectors. It can be time consuming to come up with an accurate selector. 


### Why the name George?

This is George. Most of the time he does what he's supposed to, but sometimes he doesn't do the
right thing at all. He's a living embodiment of current AI expectations.
![George the Dog](./dog.jpeg)

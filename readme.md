# George

George is an API leveraging AI to make it easy to control a computer with natural language.

Unlike traditional frameworks which rely on predefined static selectors, this API uses AI vision to interpret the screen
like a human would, executing basic computer commands (mouse, keyboard) to interact with elements. This makes it more
resilient to UI changes and able to automate interfaces that traditional tools can't handle.

The key to George's reliable automation is providing clear, descriptive references to UI elements - such as "blue submit
button" or "email input field". Once these natural descriptions are established, the AI consistently identifies and
interacts with the correct elements, regardless of underlying code changes.

George runs all in an isolated Docker container - though custom
Docker images can be used for specific needs.

### Example

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

## Getting Started

### Prerequisites

* Rust
* Docker
* [Molmo-7B-D-0924](https://huggingface.co/allenai/Molmo-7B-D-0924).
  details

### Setting up Molmo

George uses Molmo, a vision-based LLM, to identify UI elements by converting natural language descriptions into screen
coordinates which are then used to execute computer interactions.

You can try the online [demo](https://molmo.allenai.org/) and ask for the point coordinates of an element in an
image.

#### Docker

To run Molmo within Docker, you can use the following command which requires a 24GB VRAM GPU:

```bash
docker run -d --name molmo_container --runtime=nvidia --gpus all \
  -v ~/.cache/huggingface:/root/.cache/huggingface \
  -p 8000:8000 \
  --ipc=host \
  vllm/vllm-openai:latest \
  --model allenai/Molmo-7B-D-0924 \
  --trust-remote-code
```

See this [script](https://github.com/logankeenan/george/blob/main/scripts/vllm-install-deps.sh) to easily install Docker
with Nvidia support on Ubuntu

#### Bare Metal

Alternatively, you can run Molmo on bare metal, which can reduce the GPU memory consumption down to ~18GB or even ~12GB
by leveraging [bitsandbytes](https://github.com/bitsandbytes-foundation/bitsandbytes). Here are some example projects:

* Molmo example  [server](https://github.com/logankeenan/molmo-server)
* Modified Molmo Python [server](https://github.com/logankeenan/molmo-benchmarks/blob/main/main.py#L47) with
  bitsandbytes option

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

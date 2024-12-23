# George

[![Crates.io](https://img.shields.io/crates/v/george-ai)](https://crates.io/crates/george-ai)


George is an API leveraging AI to make it easy to control a computer with natural language.

Unlike traditional frameworks which rely on predefined static selectors, this API uses AI vision to interpret the
screen. This makes it more resilient to UI changes and able to automate interfaces that traditional tools can't handle.

https://github.com/user-attachments/assets/534bfcf8-13c6-45cf-83b3-98804f9aa432


### Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut george = George::new("https://your-molmo-llm.com");
    george.start().await?;
    george.open_chrome("https://some-website.com").await?;
    george.click("sign in link").await?;
    george.fill_in("input Email text field", "your@email.com").await?;
    george.fill_in("input Password text field", "super-secret").await?;
    george.click("sign in button").await?;
    george.close_chrome().await?;
    george.stop().await?;
}
```

## Getting Started

### Prerequisites

* Rust
* Docker
* [Molmo-7B-D-0924](https://huggingface.co/allenai/Molmo-7B-D-0924).
  details

### Setting up Molmo

George uses Molmo, a vision-based LLM, to identify UI elements by converting natural language descriptions into screen
coordinates which are then used to execute computer interactions.

You can try the online Molmo [demo](https://molmo.allenai.org/) and ask for the point coordinates of an element in an
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

#### Cloud

You can run Molmo with [Runpod.io](https://runpod.io?ref=myyk6f6x) via their vllm pod template. See the video below for
a demo ([youtube](https://youtu.be/x84Lxl40s-A)):

https://github.com/user-attachments/assets/8c38169c-bc54-4128-a409-985ef4a2c1de

template override:

```
--host 0.0.0.0 --port 8000 --model allenai/Molmo-7B-D-0924 --trust-remote-code --api-key your-api-key
```

## Roadmap

* Create a UI to help build out the selectors. It can be time-consuming to come up with an accurate selector.
* Improve debugging and logging
* Create bindings for other languages
    * Ruby
    * Python
    * JavaScript/Typescript
    * Others?


### Why the name George?

This is George. Most of the time he does what he's supposed to, but sometimes he doesn't do the
right thing at all. He's a living embodiment of current AI expectations.
![dog](https://github.com/user-attachments/assets/e23081b1-c966-49b9-83d5-a2f1dc8429f3)


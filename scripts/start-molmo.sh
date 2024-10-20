#!/bin/bash

ssh logan@logan-server << EOF
docker run -d --name molmo_container --runtime=nvidia --gpus all \
  -v ~/.cache/huggingface:/root/.cache/huggingface \
  -p 8000:8000 \
  --ipc=host \
  vllm/vllm-openai:latest \
  --model allenai/Molmo-7B-D-0924 \
  --trust-remote-code
EOF
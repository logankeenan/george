#!/bin/bash

# Running molmo with vllm locally

ssh logan@logan-server << EOF
# Check if the container exists
if [ \$(docker ps -a -q -f name=molmo_container) ]; then
  # Stop and remove the existing container
  docker stop molmo_container
  docker rm molmo_container
fi

# Run the new container
docker run -d --name molmo_container --runtime=nvidia --gpus all \
  -v ~/.cache/huggingface:/root/.cache/huggingface \
  -p 8000:8000 \
  --ipc=host \
  vllm/vllm-openai:latest \
  --model allenai/Molmo-7B-D-0924 \
  --trust-remote-code
EOF
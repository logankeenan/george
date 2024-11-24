#!/bin/bash

# stopping molmo running with vllm locally

ssh logan@logan-server << EOF
docker stop molmo_container
EOF
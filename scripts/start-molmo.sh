docker run --runtime=nvidia --gpus all \
  -v ~/.cache/huggingface:/root/.cache/huggingface \
  -p 8000:8000 \
  --ipc=host \
  vllm/vllm-openai:latest \
  --model allenai/Molmo-7B-D-0924 \
  --trust-remote-code


##Ubuntu
#base64 -w 0 /home/logan/Pictures/Screenshots/test.png > image_base64.txt
#
##Mac
#base64 -w 0 -i /Users/logankeenan/test-image_60.png > image_base64.txt
#
#curl --location 'http://192.168.50.200:8000/v1/chat/completions' \
#--header 'Content-Type: application/json' \
#--header 'Authorization: Bearer token' \
#--data '{
#     "model": "allenai/Molmo-7B-D-0924",
#    "messages": [
#      {
#        "role": "user",
#        "content": [
#            {"type" : "text", "text": "Find the sign up button and return the coordinates. The response should only be in the follow format: [x, y]"},
#            {"type": "image_url", "image_url": {"url": "data:image/jpeg;base64,'"$(cat image_base64.txt)"'"}}
#        ]
#      }
#    ]
#  }'
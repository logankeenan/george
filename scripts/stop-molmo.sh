#!/bin/bash

ssh logan@logan-server << EOF
docker stop molmo_container
EOF
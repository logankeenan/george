sudo apt  install docker.io -y
sudo usermod -aG docker $USER
sudo chmod 666 /var/run/docker.sock


sudo apt  install curl
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg \
  && curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
    sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
    sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

sudo apt-get update
sudo apt-get install -y nvidia-container-toolkit
sudo apt-get install -y nvidia-docker2
sudo bash -c 'echo "{\"runtimes\": {\"nvidia\": {\"path\": \"nvidia-container-runtime\",\"runtimeArgs\": []}}}" > /etc/docker/daemon.json'
sudo systemctl restart docker

# 8000 will be used for molmo
sudo ufw allow 8000
sudo ufw enable

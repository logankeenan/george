sudo apt update
sudo apt install -y build-essential

# Used for making screenshots with the tests
sudo apt-get install -y libxcb1-dev
sudo apt-get install -y xvfb

# needed for openssl compile
sudo apt install libssl-dev

# need to compile x11 based desktop automation
sudo apt install libxdo-dev -y
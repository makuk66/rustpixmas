# rustpixmas -- A simple rust program to twinkle Christmas tree

I bought a [3D Xmas Tree for Raspberry Pi](https://thepihut.com/products/3d-xmas-tree-for-raspberry-pi)
and assembled it using [these instructions](https://thepihut.com/blogs/raspberry-pi-tutorials/3d-xmas-tree-for-raspberry-pi-assembly-instructions).

I then wrote a simple Rust program to make the lights flash.
I cross-compile from my Mac, using `./build.sh` to compile the code and deploy the binary.

To run it automatically on the Pi, I added a [simple systemd service](rustpixmas.service).
Install with:
```
sudo cp rustpixmas.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable rustpixmas.service
sudo systemctl start rustpixmas.service
sudo journalctl -u rustpixmas.service
```

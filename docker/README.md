
Docker image for Rustpixmas
===========================

Usage
=====

To build:

```
./build.sh
```

To run on the Pi:

```
docker login
docker pull makuk66/rustpixmas:latest
docker run -d --restart unless-stopped --name rustpixmas --privileged --device /dev/gpiomem makuk66/rustpixmas:latest
```

About the image
===============

The [Dockerfile](./Dockerfile) is trivial. The base image is from [Balena](https://www.balena.io/docs/reference/base-images/base-images/) (previously resin.io).

Docker Notes
============

The Raspberry Pi I use use is a [Pi Zero W](https://www.raspberrypi.org/products/raspberry-pi-zero-w/), running the latest [Raspbian](https://www.raspberrypi.org/downloads/raspbian/) purchased late 2015.

Instructions for installing Docker on the Raspberry Pi are [in the manual](https://docs.docker.com/install/linux/docker-ce/debian/), which mentions what does and doesn't work on Raspbian. The installation instructions on [this blog post](https://blog.docker.com/2019/03/happy-pi-day-docker-raspberry-pi/) are the same but a little easier to follow. But, that didn't work: `containerd` crashed. `gdb` showed `containerd` crashed with `SIGILL`, similar to [moby/issues/29347](https://github.com/moby/moby/issues/29347#issuecomment-306171942). I found helpful information in [moby/issues/38175](https://github.com/moby/moby/issues/38175). Installing `18.06.3` fixed it.

```
root@raspberrypi:~# cat /proc/cpuinfo  |grep model
model name	: ARMv6-compatible processor rev 7 (v6l)

root@raspberrypi:~# cat /etc/apt/sources.list.d/docker.list
deb [arch=armhf] https://download.docker.com/linux/raspbian stretch stable

root@raspberrypi:~# cat > /etc/apt/preferences.d/docker-ce <<'EOM'
Package: docker-ce
Pin: version 18.06.*
Pin-Priority: 1000
EOM

root@raspberrypi:~# apt-get install -y -qq --no-install-recommends docker-ce=18.06.3~ce~3-0~raspbian
```

As an alternative to Raspbian, you could use [HypriotOS](https://blog.hypriot.com/getting-started-with-docker-on-your-arm-device/).

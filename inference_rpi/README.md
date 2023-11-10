# Inference Raspberry Pi

The Raspberry Pi linker should only work on Linux, so it need to cross compile the inference_rpi on Linux.

## Zero

following: [https://www.freecodecamp.org/news/embedded-rust-programming-on-raspberry-pi-zero-w/](https://www.freecodecamp.org/news/embedded-rust-programming-on-raspberry-pi-zero-w/)

### 1. add target

```bash
rustup target add arm-unknown-linux-gnueabihf
```

### 2. config linker

download tools

```
git clone https://github.com/raspberrypi/tools $HOME/rpi_tools
```

config linker, edit: ` ~/.cargo/config`, remember to change the path to your own path

```bash
[target.arm-unknown-linux-gnueabihf]
linker = "/xxxxxxx/rpi_tools/arm-bcm2708/arm-rpi-4.9.3-linux-gnueabihf/bin/arm-linux-gnueabihf-gcc"
```

### 3. cross compile

```bash
cargo build --release --target=arm-unknown-linux-gnueabihf
```

### 4. copy to pi

```bash
scp target/arm-unknown-linux-gnueabihf/release/inference_rpi pi@192.168.1.199:~/inference_rpi
```

### 5. run

```bash
./inference_rpi
```


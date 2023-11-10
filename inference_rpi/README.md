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

## FAQ

### Zero issue

```bash
   Compiling ort v2.0.0 (https://github.com/pykeio/ort.git?rev=9a631f1#9a631f1d)
   Compiling derive_builder v0.12.0
error: failed to run custom build command for `ort v2.0.0 (https://github.com/pykeio/ort.git?rev=9a631f1#9a631f1d)`

Caused by:
  process didn't exit successfully: `/home/parallels/inference/target/release/build/ort-cbb482a22f43397f/build-script-build` (exit status: 101)
  --- stdout
  [ort] strategy: "unknown"
  cargo:rerun-if-env-changed=ORT_STRATEGY

  --- stderr
  thread 'main' panicked at /home/parallels/.cargo/git/checkouts/ort-675c4f2a84c4db27/9a631f1/build.rs:168:18:
  Microsoft does not provide ONNX Runtime downloads for triplet: linux-arm-unaccelerated; you may have to use the `system` strategy instead
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
```

Mapping source code:

```
#[cfg(feature = "download-binaries")]
impl OnnxPrebuiltArchive for Triplet {
	fn as_onnx_str(&self) -> Cow<str> {
		match (&self.os, &self.arch, &self.accelerator) {
			(Os::Windows, Architecture::X86, Accelerator::None)
			| (Os::Windows, Architecture::X86_64, Accelerator::None)
			| (Os::Windows, Architecture::Arm, Accelerator::None)
			| (Os::Windows, Architecture::Arm64, Accelerator::None)
			| (Os::Linux, Architecture::X86_64, Accelerator::None)
			| (Os::MacOS, Architecture::Arm64, Accelerator::None) => format!("{}-{}", self.os.as_onnx_str(), self.arch.as_onnx_str()).into(),
			// for some reason, arm64/Linux uses `aarch64` instead of `arm64`
			(Os::Linux, Architecture::Arm64, Accelerator::None) => format!("{}-{}", self.os.as_onnx_str(), "aarch64").into(),
			// for another odd reason, x64/macOS uses `x86_64` instead of `x64`
			(Os::MacOS, Architecture::X86_64, Accelerator::None) => format!("{}-{}", self.os.as_onnx_str(), "x86_64").into(),
			(Os::Windows, Architecture::X86_64, Accelerator::Gpu) | (Os::Linux, Architecture::X86_64, Accelerator::Gpu) => {
				format!("{}-{}-{}", self.os.as_onnx_str(), self.arch.as_onnx_str(), self.accelerator.as_onnx_str()).into()
			}
			_ => panic!(
				"Microsoft does not provide ONNX Runtime downloads for triplet: {}-{}-{}; you may have to use the `system` strategy instead",
				self.os.as_onnx_str(),
				self.arch.as_onnx_str(),
				self.accelerator.as_onnx_str()
			)
		}
	}

```

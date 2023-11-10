# EdgeInference

[![Rust](https://github.com/unit-mesh/edge-inference/actions/workflows/rust.yml/badge.svg)](https://github.com/unit-mesh/edge-inference/actions/workflows/rust.yml)

EdgeInference is a gRPC-based server tailored for running ONNX models on edge devices, such as Android, iOS, or other
microcontroller units (MCUs).

- Onnx Runtime

Platform:

- Android, iOS
- Linux, Windows, Mac,
- Raspberry Pi, MCU

## Todos

- [ ] GRPC server with [tonic](https://github.com/hyperium/tonic)
- [ ] Onnx Runtime
- [ ] Tokenizer
- [ ] Flexible Configuration: Easily configurable via command-line parameters, including listening port, batch size,
  thread count, and others.

## Usecases

- [ ] SearchEverywhere: Search for anything, anywhere, anytime.
- [ ] AutoComplete: Auto-complete your code, your life.
- [ ] Summarization: Summarize your text, your life.

## Resources

### MCU

ToSpike

- [ ] Raspberry Pi Zero W, Flash: 512 MB, SRAM: 512 MB
    - Official: [Raspberry Pi Zero W](https://www.raspberrypi.com/products/raspberry-pi-zero/)
    - [Using Rust to Control a Raspberry Pi Zero W Rover](https://disconnected.systems/blog/rust-powered-rover/)

Not working:

- Arduino M0 Pro, Flash: 256 KB, SRAM: 32Kb
    - Official: [Arduino M0 Pro](https://docs.arduino.cc/retired/boards/arduino-m0-pro)
    - Rust's [cortex-m-quickstart](https://github.com/rust-embedded/cortex-m-quickstart)]
- Raspberry Pi Zero ? see in [inference_rpi](inference_rpi/README.md)

## License

This project is licensed under the MIT License, See [LICENSE](LICENSE) for the full license text.

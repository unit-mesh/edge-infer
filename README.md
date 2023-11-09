# Inference

Inference is a gRPC-based inference server designed to support ONNX models. It offers flexible configuration options,
allowing users to specify parameters such as the listening port, model file, batch size, and more. The server is built
on a specific model architecture with numerous input and output nodes to cater to complex inference requirements.

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

## License

This project is licensed under the MIT License, See [LICENSE](LICENSE) for the full license text.

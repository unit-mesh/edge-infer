#  Edge Inference Core

> EdgeInfer enables efficient edge intelligence by running small AI models, including embeddings and OnnxModels, on
> resource-constrained devices like Android, iOS, or MCUs for real-time decision-making.

Usage:

```rust
let model = std::fs::read("model/model.onnx").unwrap();
let tokenizer_data = std::fs::read("model/tokenizer.json").unwrap();

let semantic = init_semantic(model, tokenizer_data).unwrap();
let embedding = semantic.embed("hello world").unwrap();
assert_eq!(embedding.len(), 128);
```

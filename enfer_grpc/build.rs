use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("tokenizer_descriptor.bin"))
        .compile(&["proto/tokenizer/tokenizer.proto"], &["proto"])
        .unwrap();


    build_json_codec_service();
}

fn build_json_codec_service() {
    let encode_service = tonic_build::manual::Service::builder()
        .name("Tokenizer")
        .package("json.tokenizer")
        .method(
            tonic_build::manual::Method::builder()
                .name("encode")
                .route_name("Encode")
                .input_type("crate::common::EncodeRequest")
                .output_type("crate::common::EncodeReply")
                .codec_path("crate::common::JsonCodec")
                .build(),
        )
        .build();

    tonic_build::manual::Builder::new().compile(&[encode_service]);
}

TARGET_DIR :=  target

android:
	@make bindgen-kotlin
	cd examples/android && ./gradlew cargoBuild --info

# refs to https://mozilla.github.io/uniffi-rs/tutorial/foreign_language_bindings.html
bindgen-kotlin:
	cd inference_core && \
	cargo run --bin uniffi-bindgen generate src/inference.udl \
	  --language kotlin -o ../examples/android/app/src/main/java \

prepare-android:
	rustup toolchain install stable
	rustup target add x86_64-linux-android
	rustup target add x86_64-unknown-linux-gnu
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add i686-linux-android

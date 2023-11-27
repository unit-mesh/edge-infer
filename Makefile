TARGET_DIR :=  target

android:
	cd examples/android && ./gradlew cargoBuild

# refs to https://mozilla.github.io/uniffi-rs/tutorial/foreign_language_bindings.html
bindgen-kotlin:
	cargo uniffi-bindgen generate enfer_core/src/inference.udl --language kotlin --out-dir examples/android/app/src/main/java \

prepare-android:
	rustup target add x86_64-linux-android
	rustup target add aarch64-linux-android
	rustup target add armv7-linux-androideabi
	rustup target add i686-linux-android

bindgen-swift:
	export ORT_STRATEGY=system
	export ORT_LIB_LOCATION=vendor/lib
	cargo uniffi-bindgen generate enfer_core/src/inference.udl --language swift --out-dir examples/ios/InferenceCore/InferenceCore
	export ORT_STRATEGY=download
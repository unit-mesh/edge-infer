TARGET_DIR :=  target

android:
	@make bindgen-kotlin
	cd examples/android && ./gradlew cargoBuild --info

# refs to https://mozilla.github.io/uniffi-rs/tutorial/foreign_language_bindings.html
bindgen-kotlin:
	cd inference_core && \
	cargo run --bin uniffi-bindgen generate src/inference.udl \
	  --language kotlin -o ../examples/android/app/src/main/java \

.PHONY: build-universal release-universal install-universal clean

# Build universal binary for both x86_64 and aarch64 Mac
build-universal:
	@echo "Building for x86_64-apple-darwin..."
	cargo build --release --target x86_64-apple-darwin
	@echo "Building for aarch64-apple-darwin..."
	cargo build --release --target aarch64-apple-darwin
	@mkdir -p target/universal
	@echo "Creating universal binary..."
	lipo -create \
		target/x86_64-apple-darwin/release/life-progress-cli \
		target/aarch64-apple-darwin/release/life-progress-cli \
		-output target/universal/life-progress-cli
	@echo "Universal binary created at target/universal/life-progress-cli"
	@file target/universal/life-progress-cli

release-universal: build-universal

install-universal: build-universal
	@echo "Installing universal binary to ~/.cargo/bin/"
	cp target/universal/life-progress-cli ~/.cargo/bin/life-progress-cli
	chmod +x ~/.cargo/bin/life-progress-cli
	@echo "Installed to ~/.cargo/bin/life-progress-cli"

clean:
	cargo clean
	rm -rf target/universal

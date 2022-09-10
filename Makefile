.PHONY: release

macvm:
	cargo build --example macvm
	codesign -f --entitlement virtualization_rs.entitlements -s - target/debug/examples/macvm

debug:
	cargo build --example simplevm
	codesign -f --entitlement virtualization_rs.entitlements -s - target/debug/examples/simplevm

release:
	cargo build --release --example simplevm
	cargo build --release --example macvm
	codesign -f --entitlement virtualization_rs.entitlements -s - target/release/examples/simplevm
	codesign -f --entitlement virtualization_rs.entitlements -s - target/release/examples/macvm

check:
	cargo check

clean:
	cargo clean


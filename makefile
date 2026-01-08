

all: 
	cargo build
	./target/debug/vfs	
clippy:
	cargo build
	cargo clippy
clean:
	@echo "--- Deleting the target dir..."
	@rm -rf target
	@echo "--- Deleting all .vfs files from tests.."
	@rm -rf *.vfs
	@echo "--- Done!"


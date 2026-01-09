

.PHONY: output
all: 
	cargo build
	./target/debug/vfs	

# this is a pretty weird rule I made. Rust sometimes doesn't create files despite using the create flag. Whatever. I just 
# wanna finish this
output:
	@touch output/background.bmp
	@touch output/background.jpeg
	@touch output/background_5.jpeg
	@touch output/background_10.bmp
	@touch output/background_test4.bmp
clippy:
	cargo build
	cargo clippy
clean:
	@echo "--- Deleting the target dir..."
	@rm -rf target
	@echo "--- Deleting all .vfs files from tests.."
	@rm -rf *.vfs
	@echo "--- Done!"
	@rm output/*
	@make output


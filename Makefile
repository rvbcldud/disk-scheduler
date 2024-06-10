BINARY_NAME=DiskSim
TARGET_DIR=target/debug
DEST_DIR=.

all: build copy

build:
	cargo build

copy: 
	cp $(TARGET_DIR)/$(BINARY_NAME) $(DEST_DIR)/$(BINARY_NAME)

clean:
	cargo clean
	rm -f $(DEST_DIR)/$(BINARY_NAME)
PROJECT_NAME = BeagleBone-LightMeter
ARCH = armv7-unknown-linux-gnueabihf
NFS_DIR = $(HOME)/NFS_BBG/public/myApps

build:
	cargo build --target="$(ARCH)"
	cp target/$(ARCH)/debug/$(PROJECT_NAME) $(NFS_DIR)


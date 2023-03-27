main:
	cargo build --release
	scp ./target/armv5te-unknown-linux-musleabi/release/ev3dprinter ev3:~
	ssh ev3 "./ev3dprinter"

upload:
	scp ./target/armv5te-unknown-linux-musleabi/release/ev3dprinter robot@ev3dev.local:~

release:
	cargo build --release
	scp ./target/armv5te-unknown-linux-musleabi/release/ev3dprinter robot@ev3dev.local:~

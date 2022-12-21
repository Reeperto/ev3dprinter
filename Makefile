upload:
	scp ./target/armv5te-unknown-linux-musleabi/release/ev3dprinter robot@ev3dev.local:~

release:
	cargo build --release
	scp ./target/armv5te-unknown-linux-musleabi/release/ev3dprinter robot@ev3dev.local:~

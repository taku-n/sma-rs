all: sma-rs.ex5 sma-rs.dll

sma-rs.ex5: sma-rs.mq5
	-metaeditor64.exe /compile:sma-rs.mq5 /log:log.log
	cat log.log
	rm log.log

sma-rs.dll: target/x86_64-pc-windows-gnu/release/sma_rs.dll
	cp target/x86_64-pc-windows-gnu/release/sma_rs.dll sma-rs.dll

target/x86_64-pc-windows-gnu/release/sma_rs.dll: src/lib.rs
	cargo build --release

test: src/lib.rs
	cargo test --target x86_64-unknown-linux-gnu

clean:
	-rm target/x86_64-pc-windows-gnu/debug/sma_rs.dll
	-rm target/x86_64-pc-windows-gnu/release/sma_rs.dll

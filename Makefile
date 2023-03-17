
build:
	cargo build --release --target x86_64-unknown-linux-musl

t: build
	echo "asdf\ta\ts,df\nas\tdfa\tsdf" | target/x86_64-unknown-linux-musl/release/ttable

tc: build
	echo "aaaa,bbbb,ccc,dddd\neee,fff,gg" | target/x86_64-unknown-linux-musl/release/ttable -d ','

tt:build
	cat test/test_tab.txt |  target/x86_64-unknown-linux-musl/release/ttable -a

tcc: build
	cat test/test_coma.csv| target/x86_64-unknown-linux-musl/release/ttable -d ','

tss: build
	cat test/test_whitespace.txt | target/x86_64-unknown-linux-musl/release/ttable -d ' '

h:build
	target/x86_64-unknown-linux-musl/release/ttable --help

install:build
	@cp /mnt/e/projects/git_repo/ttable/target/x86_64-unknown-linux-musl/release/ttable /home/zhengxc/zhengxcProtableConfig/bin/ && \
	cp /mnt/e/projects/git_repo/ttable/target/x86_64-unknown-linux-musl/release/ttable /home/zhengxc/zhengxcProtableConfig/bin/tt && \
	chmod a+x /home/zhengxc/zhengxcProtableConfig/bin/ttable && \
	chmod a+x /home/zhengxc/zhengxcProtableConfig/bin/tt && \
	echo "Installing to /home/zhengxc/zhengxcProtableConfig/bin/ OK!" || echo "Installation Failed!"
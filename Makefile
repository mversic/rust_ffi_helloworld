.PHONY: build clean fuzz test

build:
	gcc -o hello src/main.c src/hello.c

clean:
	rm -f hello
	cd hello-rs; $(MAKE) clean
	cd hello-sys; $(MAKE) clean

fuzz:
	cd hello-rs; $(MAKE) fuzz

test:
	cd hello-rs; $(MAKE) test

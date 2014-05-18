
RUSTC=rustc
RFLAGS='-L.'

all: compile
	

compile:
	$(RUSTC) $(RFLAGS) lib.rs

test: compile
	$(RUSTC) $(RFLAGS) --test test.rs
	./test

clean:
	rm *.rlib

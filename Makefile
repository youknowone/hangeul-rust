
RC=rustc
RDOC=rustdoc
RLIBFLAGS=-O -L.
RFLAGS=-O -L.

all: compile
	

compile:
	$(RC) $(RLIBFLAGS) lib.rs

test: compile
	$(RC) $(RFLAGS) --test test.rs
	./test

clean:
	rm *.rlib

doc:
	$(RDOC) lib.rs

doc-clean:
	rm -rf doc


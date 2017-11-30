LANG ?= Rust
DEST ?= ./bin
SRC ?= solutions
INPUT ?= input
OUTPUT ?= output

all: rust

rust: day1

day1:
	LANG=$(LANG) DAY=1 make test

$(DEST):
	mkdir -p $(DEST)

build: $(DEST)
ifeq ($(LANG), Rust)
	rustc $(SRC)/day$(DAY).rs -o $(DEST)/day$(DAY) -O
endif

test: build
	$(DEST)/day$(DAY) < $(INPUT)/day$(DAY).txt > $(DEST)/day$(DAY).out
	diff -u $(OUTPUT)/day$(DAY).txt $(DEST)/day$(DAY).out

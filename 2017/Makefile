LANG ?= Rust
DEST ?= ./bin
SRC ?= solutions
INPUT ?= input
OUTPUT ?= output

all: day1

day1:
	DAY=1 make day

day:
	DAY=$(DAY)a make test
	DAY=$(DAY)b make test

$(DEST):
	mkdir -p $(DEST)

build: $(DEST)
ifeq ($(LANG), Rust)
	rustc $(SRC)/day$(DAY).rs -o $(DEST)/day$(DAY) -O
else
	@echo "Invalid Language: $(LANG)"
	exit 1
endif

test: build
	$(DEST)/day$(DAY) < $(INPUT)/day$(DAY).txt > $(DEST)/day$(DAY).out
	diff -u $(OUTPUT)/day$(DAY).txt $(DEST)/day$(DAY).out

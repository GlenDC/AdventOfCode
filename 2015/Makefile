LANG ?= C99
DEST ?= ./bin
SRC ?= solutions
INPUT ?= input
OUTPUT ?= output

# C-Related Options
CSTD_DIR = $(SRC)/cstd
CSTD_FILES = $(CSTD_DIR)/algorithms.c

day1:
	LANG=$(LANG) DAY=1 make test

day2:
	LANG=$(LANG) DAY=2 make test

$(DEST):
	mkdir -p $(DEST)

build: $(DEST)
ifeq ($(LANG), C99)
	CC -std=c99 -Wall $(CSTD_FILES) -o $(DEST)/day$(DAY) $(SRC)/day$(DAY).c
endif

test: build
	$(DEST)/day$(DAY) < $(INPUT)/day$(DAY).txt > $(DEST)/day$(DAY).out
	diff -u $(OUTPUT)/day$(DAY).txt $(DEST)/day$(DAY).out

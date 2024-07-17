.PHONY: build vendor json

HACK_CLANG := env LD_LIBRARY_PATH=/usr/local/opt/llvm/lib/ 
#HACK_CLANG := 

#RUSTFLAGS := "-Awarnings -Z time-passes"
RUSTFLAGS := -Awarnings
#CARGO     := env CARGO_MSG_LIMIT=15 CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 
CARGO    := env CARGO_BUILD_JOBS=12 NUM_JOBS=12 cargo 
BUILD     := build --verbose

#FEATURES := --features "mp3 lame"
#FEATURES := --features "aloe-data-structures"

default: build

#----------------------------------------------[todo]
ACTIVE_PACKAGE := aloe-simd-register-demo

active:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) -p $(ACTIVE_PACKAGE) $(FEATURES)

build:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) $(FEATURES)

vendor:
	RUSTFLAGS=$(RUSTFLAGS) $(CARGO) vendor

json:
	$(HACK_CLANG) RUSTFLAGS=$(RUSTFLAGS) $(CARGO) $(BUILD) $(FEATURES) --quiet --message-format=json 2> /dev/null | jq --slurp

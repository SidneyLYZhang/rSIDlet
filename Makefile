NAME    := rsidlet
BINARY  := sidlet

# --- Platform detection ---
ifneq (,$(findstring Windows,$(OS)))
    BIN_EXT     := .exe
    INSTALL_DIR ?= $(LOCALAPPDATA)\Programs\$(NAME)
    INSTALL_CMD := powershell -ExecutionPolicy Bypass -File install.ps1
    RM_DIR      := cmd /c rmdir /s /q
    RM_FILE     := cmd /c del /q
    PKG_EXT     := zip
else
    BIN_EXT     :=
    INSTALL_DIR ?= $(HOME)/.local/bin
    INSTALL_CMD := bash install.sh
    RM_DIR      := rm -rf
    RM_FILE     := rm -f
    PKG_EXT     := tar.xz
endif

# --- Colors for help output ---
ifneq (,$(findstring Windows,$(OS)))
    COLOR_RESET  :=
    COLOR_BOLD   :=
    COLOR_GREEN  :=
    COLOR_YELLOW :=
else
    COLOR_RESET  := \033[0m
    COLOR_BOLD   := \033[1m
    COLOR_GREEN  := \033[32m
    COLOR_YELLOW := \033[33m
endif

# --- Default target: help ---
.DEFAULT_GOAL := help

.PHONY: help
help:
	@echo "$(COLOR_BOLD)$(NAME) - Rust figlet implementation$(COLOR_RESET)"
	@echo ""
	@echo "$(COLOR_BOLD)Usage:$(COLOR_RESET) make [target]"
	@echo ""
	@echo "$(COLOR_BOLD)Build:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)build$(COLOR_RESET)        Build release binary              (cargo build --release)"
	@echo "  $(COLOR_GREEN)debug$(COLOR_RESET)        Build debug binary                (cargo build)"
	@echo ""
	@echo "$(COLOR_BOLD)Test & Lint:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)test$(COLOR_RESET)         Run all tests                     (cargo test)"
	@echo "  $(COLOR_GREEN)check$(COLOR_RESET)        Check compilation (no codegen)    (cargo check)"
	@echo "  $(COLOR_GREEN)clippy$(COLOR_RESET)       Run clippy lints                  (cargo clippy)"
	@echo "  $(COLOR_GREEN)fmt$(COLOR_RESET)          Format source code                (cargo fmt)"
	@echo "  $(COLOR_GREEN)fmt-check$(COLOR_RESET)    Check code formatting              (cargo fmt --check)"
	@echo ""
	@echo "$(COLOR_BOLD)Run:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)run$(COLOR_RESET)          Run with release build            (cargo run --release)"
	@echo "  $(COLOR_GREEN)rund$(COLOR_RESET)         Run with debug build              (cargo run)"
	@echo ""
	@echo "$(COLOR_BOLD)Doc:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)doc$(COLOR_RESET)          Generate documentation            (cargo doc)"
	@echo "  $(COLOR_GREEN)doc-open$(COLOR_RESET)     Generate and open docs            (cargo doc --open)"
	@echo ""
	@echo "$(COLOR_BOLD)Release:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)package$(COLOR_RESET)      Build and create release archive"
	@echo "  $(COLOR_GREEN)publish$(COLOR_RESET)      Publish to crates.io              (cargo publish)"
	@echo ""
	@echo "$(COLOR_BOLD)Install:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)install$(COLOR_RESET)      Install binary to system"
	@echo "  $(COLOR_GREEN)uninstall$(COLOR_RESET)    Remove installed binary"
	@echo ""
	@echo "$(COLOR_BOLD)Clean:$(COLOR_RESET)"
	@echo "  $(COLOR_GREEN)clean$(COLOR_RESET)        Remove build artifacts            (cargo clean)"
	@echo ""

# --- Build targets ---
.PHONY: build release
build: release

release:
	cargo build --release
	@echo "Built: target/release/$(BINARY)$(BIN_EXT)"

.PHONY: debug
debug:
	cargo build
	@echo "Built: target/debug/$(BINARY)$(BIN_EXT)"

# --- Test & Lint targets ---
.PHONY: test
test:
	cargo test

.PHONY: check
check:
	cargo check

.PHONY: clippy
clippy:
	cargo clippy -- -D warnings

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: fmt-check
fmt-check:
	cargo fmt --check

# --- Run targets ---
.PHONY: run
run:
	cargo run --release

.PHONY: rund
rund:
	cargo run

# --- Doc targets ---
.PHONY: doc
doc:
	cargo doc

.PHONY: doc-open
doc-open:
	cargo doc --open

# --- Release targets ---
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/version.*=.*"\(.*\)"/\1/')

.PHONY: package
package: build
	@echo "Packaging $(NAME)-$(VERSION)-local.$(PKG_EXT)..."
ifeq ($(PKG_EXT),zip)
	powershell -Command "Compress-Archive -Path 'target/release/$(BINARY)$(BIN_EXT)', 'fonts' -DestinationPath '$(NAME)-$(VERSION)-local.zip' -Force"
else
	tar -cJf "$(NAME)-$(VERSION)-local.tar.xz" -C target/release "$(BINARY)" -C ../.. fonts/
endif
	@echo "Created: $(NAME)-$(VERSION)-local.$(PKG_EXT)"

.PHONY: publish
publish: test
	cargo publish

.PHONY: publish-dry-run
publish-dry-run:
	cargo publish --dry-run

# --- Clean targets ---
.PHONY: clean
clean:
	cargo clean
	@echo "Cleaned build artifacts"

# --- Install targets ---
.PHONY: install
install: build
	$(INSTALL_CMD)

.PHONY: uninstall
uninstall:
	-@$(RM_FILE) "$(INSTALL_DIR)/$(BINARY)$(BIN_EXT)"
	-@$(RM_DIR) "$(INSTALL_DIR)/fonts"
	@echo "Uninstalled from $(INSTALL_DIR)"
	@echo "You may need to manually remove $(INSTALL_DIR) from your PATH."

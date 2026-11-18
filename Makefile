# SPDX-License-Identifier: Apache-2.0 OR MIT
# Local gates. `make check` runs what the CI workflow runs, in order, and
# is the one command that validates a change. `make docs-lint` runs the
# Docs lint workflow's checks.

CARGO ?= cargo
CLIPPY_FLAGS := -D warnings -W clippy::cognitive_complexity -W clippy::too_many_lines

.PHONY: check fmt fmt-check lint build test coverage doc deny complexity versions package-check docs-lint clean

check: fmt-check lint build test coverage doc deny versions package-check complexity

fmt:
	$(CARGO) fmt

fmt-check:
	$(CARGO) fmt --check

lint:
	$(CARGO) clippy --locked --all-targets --all-features -- $(CLIPPY_FLAGS)

build:
	$(CARGO) build --locked --all-targets --all-features

test:
	$(CARGO) test --locked --all-features

# 100% of lines and regions in src/, or the target fails. Needs
# cargo-llvm-cov (`cargo install cargo-llvm-cov --locked`) and the
# llvm-tools-preview component. The unit tests alone reach 100%.
coverage:
	$(CARGO) llvm-cov --locked --all-features --lib --fail-under-lines 100 --fail-under-regions 100

doc:
	RUSTDOCFLAGS="-D warnings" $(CARGO) doc --locked --no-deps --all-features

# Needs cargo-deny (`cargo install cargo-deny --locked`).
deny:
	$(CARGO) deny --locked check

# Cyclomatic 10, cognitive 15, Halstead difficulty 30 and 60 lines per
# function, 500 lines per file, on every tracked .rs file. Needs
# rust-code-analysis-cli 0.0.25.
complexity:
	python3 -I scripts/complexity-gate.py

versions:
	scripts/verify-release-versions

package-check:
	scripts/check-package.sh

# Markdown style, spelling and REUSE: npx, codespell and reuse.
docs-lint:
	npx --yes markdownlint-cli2@0.23.3 "**/*.md"
	codespell
	reuse lint

clean:
	$(CARGO) clean


lint: action-lint clippy

##
## Rust
##

# If we're running in github actions and cargo-action-fmt is installed, then add
# a command suffix that formats errors.
_fmt := if env_var_or_default("GITHUB_ACTIONS", "") != "true" { "" } else {
    ```
    if command -v cargo-action-fmt >/dev/null 2>&1; then
        echo "--message-format=json | cargo-action-fmt"
    fi
    ```
}

toolchain := ""

_cargo := "cargo" + if toolchain != "" { " +" + toolchain } else { "" }

fetch:
    {{ _cargo }} fetch --locked

clippy: fetch
    {{ _cargo }} clippy --frozen {{ _fmt }}

build: fetch
    {{ _cargo }} build --frozen --release --target x86_64-unknown-linux-musl {{ _fmt }}

version := ```
    cargo metadata --format-version=1 \
        | jq -r '.packages[] | select(.name == "j5j") | .version' \
        | head -n 1
    ```

package: build
    tar -czf target/j5j-v{{ version }}-x86_64-unknown-linux-musl.tar.gz \
        -C target/x86_64-unknown-linux-musl/release j5j

##
## Etc
##

# Format actionlint output for Github Actions if running in CI.
_actionlint-fmt := if env_var_or_default("GITHUB_ACTIONS", "") != "true" { "" } else {
  '{{range $err := .}}::error file={{$err.Filepath}},line={{$err.Line}},col={{$err.Column}}::{{$err.Message}}%0A```%0A{{replace $err.Snippet "\\n" "%0A"}}%0A```\n{{end}}'
}

# Lints all GitHub Actions workflows
action-lint:
    actionlint {{ if _actionlint-fmt != '' { "-format '" + _actionlint-fmt + "'" } else { "" } }} .github/workflows/*

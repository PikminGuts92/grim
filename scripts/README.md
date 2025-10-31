## Setup Python Environment
```bash
python -m venv .venv
# . .venv/Scripts/activate # Using Git Bash on Windows
source .venv/bin/activate
pip install maturin
```

## Run (Development)
```bash
# Adds pikaxe api as python module to current env
maturin develop -m ./core/pikaxe/Cargo.toml --all-features
```

## Build and Install
```bash
# Build pikaxe api as python module in target/wheels
maturin build -m ./core/pikaxe/Cargo.toml --all-features -r

# Use '--force-reinstall' to override pre-existing install
# Note: File name may be different depending on build target
pip install ./target/wheels/pikaxe-0.1.0-cp310-none-win_amd64.whl --force-reinstall
```
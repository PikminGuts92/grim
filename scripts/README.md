## Setup Python Environment
```bash
python -m venv .env
. .env/Scripts/activate # Using Git Bash on Windows
pip install maturin
```

## Run (Development)
```bash
# Adds grim api as python module to current env
maturin develop -m ./core/grim/Cargo.toml --all-features
```

## Build and Install
```bash
# Build grim api as python module in target/wheels
maturin build -m ./core/grim/Cargo.toml --all-features -r

# Use '--force-reinstall' to override pre-existing install
# Note: File name may be different depending on build target
pip install ./target/wheels/grim-0.1.0-cp310-none-win_amd64.whl --force-reinstall
```
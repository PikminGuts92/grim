setup-python:
    uv venv --allow-existing
    uv pip install maturin
    maturin develop -m ./core/pikaxe/Cargo.toml --all-features
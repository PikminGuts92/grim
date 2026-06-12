build-dev:
    cargo build --all-features

setup-python:
    uv venv --allow-existing
    uv pip install maturin
    maturin develop -m ./core/pikaxe/Cargo.toml --all-features

run-create-kr-lipsync input_path output_path midi_path:
    cargo run --bin scene_tool -- milo2kr {{input_path}} {{output_path}} -m {{midi_path}}
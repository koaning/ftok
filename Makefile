rust-install:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

codespace: 
	python -m pip install uv
	uv venv
	uv pip install maturin


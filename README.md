# ftok

> fast tokenizer ... soft of

These are the commands that I used on the CLI to get started:

```python
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
maturin new fasttok --mixed --src
uv run maturin develop --uv
uv pip install -e .
```

You should be able to run the benchmark after running all that. 
# Readme

Basic POC mixing:

1. Use `cargo workspace` (`Cargo.toml`)
2. Create a rust library to access the [Kraken](https://docs.kraken.com/api/docs/rest-api/add-order) public API (in `kraken`) with `reqwest` + `tokio` + `serde`).
3. Add a rust test for the function (test module `kraken/src/tests/` declared at the end of `kraken/src/kraken.rs`)
4. Add a main example program using the function and compiled independently (`kraken/src/bin`)
5. Develop Python bindings (in `pykraken`) with `pyo3` and `pyo3-asyncio`.
6. Declare dependencies between workspace's members (`kraken` -> `pykraken` in `pykraken/Cargo.toml`)
7. Use `maturin` to build the python bindings project (`pykraken/pyproject.toml`).
8. Write a python test for the module (`pykraken/pykraken_tests.py`)
9. Use `tox` to automate all the tests because the python tests cannot be automated with cargo (`tox.ini`)


## Testing the process

Normal build just do `cargo build` and equivalents.

Automatically run all the tests: `tox`

The python tests use `maturin`, so, a virtual environment may be needed to run the python manually.

```
cd pykraken
python -m venv .venv # .venv is recognized automatically by maturin
maturin develop
.venv/bin/python pykraken_tests.py
```



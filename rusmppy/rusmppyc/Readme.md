# Rusmppyc

An async [SMPP v5](https://smpp.org/SMPP_v5.pdf) `Python` client powered by `Rust`.

## Develop

- Install [`maturin`](https://www.maturin.rs/installation.html)

- Create a virtual environment:

  ```bash
  python3 -m venv venv
  source venv/bin/activate
  ```

- Generate the `pyi` stubs:

  ```bash
  cargo run --bin stub-gen
  ```

- Generate the bindings:

  ```bash
  maturin develop
  ```

- The bindings are now available in the virtual environment. You can test them by running:

  ```bash
  python3 -c "import rusmppyc; print(rusmppyc.__version__)"
  ```

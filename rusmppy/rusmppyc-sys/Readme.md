# Rusmppyc-sys

Raw `Python` bindings for `Rusmppc`

## Develop

- Install [`maturin`](https://www.maturin.rs/installation.html)

- Create a virtual environment:

  ```bash
  python3 -m venv venv
  source venv/bin/activate
  ```
  
- Generate the bindings:

  ```bash
  maturin develop
  ```

- The bindings are now available in the virtual environment. You can test them by running:

  ```bash
  python3 -c "import rusmppyc_sys; print(rusmppyc_sys.__version__)"
  ```

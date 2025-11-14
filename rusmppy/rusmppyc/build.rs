fn main() {
    // Use the same cfgs as pyo3. We need cfg(PyPy) and cfg(GraalPy)
    // Issue: [Rusmpp #102](https://github.com/Rusmpp/Rusmpp/issues/102)
    pyo3_build_config::use_pyo3_cfgs();
}

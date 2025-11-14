fn main() {
    println!("cargo::rustc-check-cfg=cfg(PyO3_PyPy)");
    println!("cargo::rustc-check-cfg=cfg(PyO3_GraalPy)");

    // set by maturin
    if let Ok(sig) = std::env::var("PYO3_ENVIRONMENT_SIGNATURE") {
        println!("cargo:info=PYO3_ENVIRONMENT_SIGNATURE = {sig}");

        let sig_lower = sig.to_lowercase();

        if sig_lower.starts_with("pypy") {
            println!("cargo:info=Detected PyPy environment");
            println!("cargo:rustc-cfg=PyO3_PyPy");
        }

        if sig_lower.starts_with("graalpy") {
            println!("cargo:info=Detected GraalPy environment");
            println!("cargo:rustc-cfg=PyO3_GraalPy");
        }
    }
}

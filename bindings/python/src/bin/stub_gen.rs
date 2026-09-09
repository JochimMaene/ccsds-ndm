//! Generates `ccsds_ndm/__init__.pyi` from the PyO3 bindings.
//!
//! With no argument the stub is written in place. With a directory argument the stub is
//! written under that directory instead, which lets `just stubs-check` compare generated
//! output against the committed file without touching it.

fn main() -> pyo3_stub_gen::Result<()> {
    let mut info = ccsds_ndm_py::stub_info()?;
    if let Some(out_dir) = std::env::args().nth(1) {
        info.python_root = std::path::PathBuf::from(out_dir);
    }
    info.generate()?;
    Ok(())
}

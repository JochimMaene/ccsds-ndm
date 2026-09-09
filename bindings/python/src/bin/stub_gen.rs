fn main() -> pyo3_stub_gen::Result<()> {
    ccsds_ndm_py::stub_info()?.generate()?;
    Ok(())
}

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
    // PyO3 simple enums expose class attributes, but do not implement Python's enum.Enum API.
    let path = info.python_root.join("ccsds_ndm/__init__.pyi");
    let source = std::fs::read_to_string(&path)?;
    let mut enum_name = None;
    let mut output = String::with_capacity(source.len());
    for line in source.lines() {
        if let Some(name) = line
            .strip_prefix("class ")
            .and_then(|s| s.strip_suffix("(enum.Enum):"))
        {
            enum_name = Some(name);
            output.push_str(&format!("class {name}:\n"));
        } else {
            if !line.is_empty() && !line.starts_with(' ') {
                enum_name = None;
            }
            if let (Some(name), Some(member)) = (
                enum_name,
                line.strip_prefix("    ")
                    .and_then(|s| s.strip_suffix(" = ...")),
            ) {
                output.push_str(&format!("    {member}: typing.ClassVar[{name}]\n"));
            } else if line != "import enum" {
                output.push_str(line);
                output.push('\n');
            }
        }
    }
    std::fs::write(path, output)?;
    Ok(())
}

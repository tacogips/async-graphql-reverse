use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;

pub fn fmt_file<P: AsRef<str>>(file_path: P) -> Result<()> {
    let output = Command::new("rustfmt")
        .arg("--edition=2018")
        .arg("--config=normalize_doc_attributes=true")
        .arg(file_path.as_ref())
        .spawn()
        .expect("rustfmt failed");
    output.wait_with_output()?;
    Ok(())
}

pub fn pathbuf_to_str(pathbuf: &PathBuf) -> String {
    pathbuf.as_path().display().to_string()
}

use crate::{config::RsbpError, Result};
use std::{
    fs::File,
    io::{LineWriter, Write},
    path::PathBuf,
};

/// Start url in browser.
pub fn start_url(url: &str) -> Result<()> {
    open::that(&url).map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    Ok(())
    // if let Ok(process) = std::process::Command::new("ls")
    //     .args(&["-l", "/home/wolfgang"])
    //     .spawn()
    // {
    //     if let Ok(output) = process.wait_with_output() {
    //         print!("{:?}", output.stdout.as_slice());
    //     };
    // };
}

/// Save lines in a file an optionnally open it.
pub fn save_file(lines: &Vec<String>, path: &String, file: &String, open: bool) -> Result<()> {
    if lines.is_empty() || (path.is_empty() && file.is_empty()) {
        return Ok(());
    }
    let pathfile = combine_path(path.as_str(), file.as_str());
    let f = File::create(pathfile.as_str())
        .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    let mut f = LineWriter::new(f);
    for l in lines {
        f.write_all(l.as_bytes())
            .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
        f.write_all(b"\r\n")
            .map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    }
    if open {
        open::that(&pathfile).map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
    }
    Ok(())
}

/// Combine two path elements.
pub fn combine_path(p1: &str, p2: &str) -> String {
    let mut pb = PathBuf::from(p1);
    pb.push(PathBuf::from(p2));
    let path = pb.into_os_string().into_string().unwrap_or(p1.to_string());
    path
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    /// Combine path elements.
    #[test]
    fn combine() {
        let p1 = String::from("a");
        let p2 = String::from("b");
        let mut pb = PathBuf::from(p1.as_str());
        pb.push(PathBuf::from(p2.as_str()));
        pb.set_extension("ext");
        let path = pb.into_os_string().into_string().unwrap_or(p1);
        println!("{:?}", path);
    }
}

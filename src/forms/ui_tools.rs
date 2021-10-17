use crate::{base::functions, config::RsbpError, Result};

pub fn start_url(url: &str) -> Result<()> {
    functions::mach_nichts();
    let _r =
        webbrowser::open(url).map_err(|err| RsbpError::error_string(err.to_string().as_str()))?;
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

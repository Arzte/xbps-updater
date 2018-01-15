use failure::Error;

pub fn check_for_updates(pkg: &str) -> Result<String, Error> {
    let output = ::std::process::Command::new("/home/doctors/git/void-packages/xbps-src")
            .arg("update-check")
            .arg(pkg)
            .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

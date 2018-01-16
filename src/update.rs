use failure::Error;

pub fn check_for_updates(pkg: &str) -> Result<String, Error> {
    let output = ::std::process::Command::new("/home/doctors/git/void-packages/xbps-src")
            .arg("update-check")
            .arg(pkg)
            .output()?;
    let output_stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let output_stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if output_stderr.is_empty() {
        return Ok(output_stdout.split(" ").last().unwrap().split("-").last().unwrap().to_owned());
    // TODO: Properly check output to see if no update was needed/Or no update was found/Or other error
    } else if output_stdout.is_empty() {
        return Ok("".to_owned())
    } else {
        bail!("Unknown ./xbps-src output".to_owned());
    }
}

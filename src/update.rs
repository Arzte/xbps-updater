use failure::Error;

#[derive(Fail, Debug)]
pub enum xbps_src_error {
    #[fail(display = "Already latest version")]
    latest_version,
    #[fail(display = "{}", _0)]
    no_version(String),
    #[fail(display = "Unexpected xbps-src output: {}", _0)]
    unknown_error(String),
}
pub fn check_update(pkg: &str) -> Result<String, xbps_src_error> {
    let output = ::std::process::Command::new("/home/doctors/git/void-packages/xbps-src")
            .arg("update-check")
            .arg(pkg)
            .output()
            .unwrap();
    let output_stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let output_stdout = String::from_utf8_lossy(&output.stdout).to_string();

    if output_stdout.contains("->") {
        return Ok(output_stdout.split(" ").last().unwrap().split("-").last().unwrap().to_owned());
    // TODO: Properly check output to see if no update was needed/Or no update was found/Or other error
    } else if output_stdout.is_empty() {
        return Ok("".to_owned())
    } else {
        match output_stderr.as_ref() {
            "NO VERSION found for" => return Err(xbps_src_error::no_version(output_stderr)),
            "" => return Err(xbps_src_error::latest_version),
            _ => return Err(xbps_src_error::unknown_error(output_stderr)),
    }
}
}

pub update_package(pkg: &str) -> Result<String, Error> {
    check_update(pkg);
}

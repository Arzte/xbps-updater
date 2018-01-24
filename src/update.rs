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

#[derive(Fail, Debug)]
pub enum xbps_updater_error {
    #[fail(display = "xbps-src error")]
    xbps_src_error,
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
        Ok(output_stdout.split(" ").last().unwrap().split("-").last().unwrap().to_owned())
    } else {
        match output_stderr.as_ref() {
            "NO VERSION found for" => Err(xbps_src_error::no_version(output_stderr)),
            "" => Err(xbps_src_error::latest_version),
            _ => Err(xbps_src_error::unknown_error(output_stderr)),
        }
    }
}

pub fn update_package(pkg: &str) -> Result<String, Error> {
    let update = check_update(pkg);
    if update.is_ok() {
        let output = ::std::process::Command::new("xgensum")
            .arg(format!("/home/doctors/git/void-packages/srcpkgs/{}", pkg))
            .output()
            .unwrap();
        let output_stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok(output_stderr)
    } else {
        match update.err() {
            xbps_src_error::no_version => xbps_src_error::no_version,
            xbps_src_error::latest_version => xbps_src_error::latest_version,
            xbps_src_error::unknown_error => xbps_src_error::unknown_error,
        }
    }
}

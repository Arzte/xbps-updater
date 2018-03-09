use failure::Error;

// Error stuffs, makes life easier... I think.
#[derive(Fail, Debug)]
pub enum XbpsSrcError {
    #[fail(display = "Already latest version")]
    LatestVersion,
    #[fail(display = "{}", _0)]
    NoVersion(String),
    #[fail(display = "Unexpected xbps-src output: {}", _0)]
    UnknownError(String),
}

// Uses xbps-src to check for a update, if there is one, return the new version, else give a
// "error" of why its not a never version.
pub fn check_update(pkg: &str) -> Result<String, XbpsSrcError> {
    let output = ::std::process::Command::new("/home/doctors/git/void-packages/xbps-src")
        .arg("update-check")
        .arg(pkg)
        .output()
        .unwrap();
    // This is needed (even though its redundant looking) because output.stout is a string utf8
    // lossy string, but since we need to use it twice, it's easier to pass a reference and
    // "convert" it twice. (Because otherwise it gets consumed by from_utf8_lossy)
    let output_stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let output_stdout = String::from_utf8_lossy(&output.stdout).to_string();

    if output_stdout.contains("->") {
        // TDLR; Takes the output of xbps-src and gets the new version, far and away the laziest and
        // probably worst way to do it, but it works.
        Ok(output_stdout.split(" ").last().unwrap().split("-").last().unwrap().to_owned())
    } else {
        // matching strings looks weird, so I match references instead, blame rust.
        match output_stderr.as_ref() {
            "NO VERSION found for" => Err(XbpsSrcError::NoVersion(output_stderr)),
            "" => Err(XbpsSrcError::LatestVersion),
            _ => Err(XbpsSrcError::UnknownError(output_stderr)),
        }
    }
}

// If there is a update it update the package, otherwise it returns nothing.
// TODO: Return () if there is no update, otherwise update the package version and checksum
pub fn update_package(pkg: &str) -> Result<String, XbpsSrcError> {
    let update = check_update(pkg);

    // TODO: Update version of a program in its file
    // FIXME: Get data for both a error and a OK result.
    if let Err(e) = update {
        Err(e)
    } else {
        let output = ::std::process::Command::new("xgensum")
            .arg("-i")
            .arg(format!("/home/doctors/git/void-packages/srcpkgs/{}", pkg))
            .output()
            .unwrap();
        // We want to do more than just gather output later, this is almost a placeholder.
        // (so please don't mind the assigning it only to return it a line later bit.
        let output_stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Ok(output_stderr)
    }
}

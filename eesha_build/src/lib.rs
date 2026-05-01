//! # Eesha Build
//!
//! This is a crate to help with getting started with using eesha as a webview without building it yourself
//!
//! ## Example
//!
//! To use it, first add it to your build dependency, and in your build script:
//!
//! ```no_run
//! fn main() {
//!     eesha_build::download_and_extract_eesha("output_directory").unwrap();
//! }
//! ```

use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

const EESHA_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Decompress the archive to the output directory, this should resulting in a eesha(.exe) in that directory
pub fn decompress_archive<P1: AsRef<Path>, P2: AsRef<Path>>(
    archive_path: P1,
    output_directory: P2,
) -> Result<(), io::Error> {
    fs::create_dir_all(&output_directory)?;
    if Command::new("tar")
        .current_dir(&output_directory)
        .arg("-xf")
        .arg(archive_path.as_ref())
        .status()?
        .success()
    {
        Ok(())
    } else {
        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}

/// Download the pre-built eesha archive to the `OUT_DIR` and returns that path
pub fn download_archive<S: AsRef<str>>(base_url: S) -> Result<PathBuf, io::Error> {
    let target = env::var("TARGET").unwrap();
    let archive_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("eesha.tar.gz");
    if !archive_path.exists() {
        let base_url = base_url.as_ref();
        let download_url =
            format!("{base_url}/download/eesha-v{EESHA_VERSION}/eesha-{target}.tar.gz");
        let curl_start = Instant::now();
        println!("Try downloading eesha from {download_url}");
        if !Command::new("curl")
            .arg("-L")
            .arg("-f")
            .arg("-s")
            .arg("-o")
            .arg(&archive_path)
            .arg(download_url)
            .status()?
            .success()
        {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
        println!(
            "Successfully downloaded eesha archive in {} ms",
            curl_start.elapsed().as_millis()
        );
    }

    Ok(archive_path)
}

/// Download and extract the pre-built eesha executable to this directory
///
/// This function uses the base URL from [`default_archive_base_url`],
/// if you want more flexibility, use [`download_archive`] and then [`decompress_archive`] separately
pub fn download_and_extract_eesha<P: AsRef<Path>>(output_directory: P) -> Result<(), io::Error> {
    let archive_path = download_archive(default_archive_base_url())?;
    decompress_archive(&archive_path, output_directory)?;
    Ok(())
}

/// If you don't know where to put the eesha executable,
/// this function gives you the `target` directory
pub fn default_output_directory() -> PathBuf {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    // Not ideal, but there's no good way of getting the target directory
    out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Default archive base URL.
///
/// This can be overridden by setting the `EESHA_ARCHIVE_BASE_URL` environment variable.
/// The default points to a placeholder URL that must be configured before release builds.
/// TODO: Set up actual release infrastructure and update this URL accordingly.
pub fn default_archive_base_url() -> String {
    env::var("EESHA_ARCHIVE_BASE_URL").unwrap_or_else(|_| {
        // Placeholder URL - must be configured for actual releases
        "https://github.com/eesha-browser/eesha-release/releases".to_string()
    })
}

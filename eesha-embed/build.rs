use std::{env, path::PathBuf};

use eesha_build::{
    decompress_archive, default_archive_base_url, default_output_directory, download_archive,
};

fn main() {
    println!("cargo:rerun-if-env-changed=PRE_BUILT_EESHA");
    println!("cargo:rerun-if-env-changed=EESHA_ARCHIVE");

    if let Ok(pre_built_eesha_env) = env::var("PRE_BUILT_EESHA") {
        let output_directory = if pre_built_eesha_env == "true" {
            default_output_directory()
        } else {
            PathBuf::from(pre_built_eesha_env)
        };
        download_and_extract_eesha(output_directory).unwrap();
    };
}

pub fn download_and_extract_eesha(output_directory: PathBuf) -> Result<(), std::io::Error> {
    if let Ok(archive) = env::var("EESHA_ARCHIVE") {
        // If the archive variable is present, assume it's a URL base to download from.
        let archive = download_archive(&archive).unwrap_or(PathBuf::from(archive));
        // Panic directly since the archive is specified manually.
        decompress_archive(archive, output_directory)?;
    } else {
        let archive_path = download_archive(default_archive_base_url())?;
        decompress_archive(archive_path, output_directory)?;
    };

    Ok(())
}

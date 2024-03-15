// Copyright 2016-2019 Cargo-Bundle developers <https://github.com/burtonageo/cargo-bundle>
// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// The structure of a Debian package looks something like this:
//
// foobar_1.2.3_i386.deb   # Actually an ar archive
//     debian-binary           # Specifies deb format version (2.0 in our case)
//     control.tar.gz          # Contains files controlling the installation:
//         control                  # Basic package metadata
//         md5sums                  # Checksums for files in data.tar.gz below
//         postinst                 # Post-installation script (optional)
//         prerm                    # Pre-uninstallation script (optional)
//     data.tar.gz             # Contains files to be installed:
//         usr/bin/foobar                            # Binary executable file
//         usr/share/applications/foobar.desktop     # Desktop file (for apps)
//         usr/share/icons/hicolor/...               # Icon files (for apps)
//         usr/lib/foobar/...                        # Other resource files
//
// For cargo-bundle, we put bundle resource files under /usr/lib/package_name/,
// and then generate the desktop file and control file from the bundle
// metadata, as well as generating the md5sums file.  Currently we do not
// generate postinst or prerm files.

use super::{super::common, freedesktop};
use crate::Settings;
use anyhow::Context;
use flate2::{write::GzEncoder, Compression};
use heck::AsKebabCase;
use tar::HeaderMode;
use walkdir::WalkDir;

use std::{
  fs::{self, File},
  io::{self, Write},
  os::unix::fs::MetadataExt,
  path::{Path, PathBuf},
};

/// Bundles the project.
/// Returns a vector of PathBuf that shows where the DEB was created.
pub fn bundle_project(settings: &Settings) -> crate::Result<Vec<PathBuf>> {
  let arch = match settings.binary_arch() {
    "x86" => "i386",
    "x86_64" => "amd64",
    // ARM64 is detected differently, armel isn't supported, so armhf is the only reasonable choice here.
    "arm" => "armhf",
    "aarch64" => "arm64",
    other => other,
  };
  let package_base_name = format!(
    "{}_{}_{}",
    settings.main_binary_name(),
    settings.version_string(),
    arch
  );
  let package_name = format!("{package_base_name}.deb");

  let base_dir = settings.project_out_directory().join("bundle/deb");
  let package_dir = base_dir.join(&package_base_name);
  if package_dir.exists() {
    fs::remove_dir_all(&package_dir)
      .with_context(|| format!("Failed to remove old {package_base_name}"))?;
  }
  let package_path = base_dir.join(&package_name);

  log::info!(action = "Bundling"; "{} ({})", package_name, package_path.display());

  let (data_dir, _) = generate_data(settings, &package_dir)
    .with_context(|| "Failed to build data folders and files")?;
  common::copy_custom_files(&settings.deb().files, &data_dir)
    .with_context(|| "Failed to copy custom files")?;

  // Generate control files.
  let control_dir = package_dir.join("control");
  generate_control_file(settings, arch, &control_dir, &data_dir)
    .with_context(|| "Failed to create control file")?;
  generate_md5sums(&control_dir, &data_dir).with_context(|| "Failed to create md5sums file")?;

  // Generate `debian-binary` file; see
  // http://www.tldp.org/HOWTO/Debian-Binary-Package-Building-HOWTO/x60.html#AEN66
  let debian_binary_path = package_dir.join("debian-binary");
  create_file_with_data(&debian_binary_path, "2.0\n")
    .with_context(|| "Failed to create debian-binary file")?;

  // Apply tar/gzip/ar to create the final package file.
  let control_tar_gz_path =
    tar_and_gzip_dir(control_dir).with_context(|| "Failed to tar/gzip control directory")?;
  let data_tar_gz_path =
    tar_and_gzip_dir(data_dir).with_context(|| "Failed to tar/gzip data directory")?;
  create_archive(
    vec![debian_binary_path, control_tar_gz_path, data_tar_gz_path],
    &package_path,
  )
  .with_context(|| "Failed to create package archive")?;
  Ok(vec![package_path])
}

/// Generate the debian data folders and files.
pub fn generate_data(
  settings: &Settings,
  package_dir: &Path,
) -> crate::Result<(PathBuf, Vec<freedesktop::Icon>)> {
  // Generate data files.
  let data_dir = package_dir.join("data");
  let bin_dir = data_dir.join("usr/bin");

  for bin in settings.binaries() {
    let bin_path = settings.binary_path(bin);
    common::copy_file(&bin_path, bin_dir.join(bin.name()))
      .with_context(|| format!("Failed to copy binary from {bin_path:?}"))?;
  }

  copy_resource_files(settings, &data_dir).with_context(|| "Failed to copy resource files")?;

  settings
    .copy_binaries(&bin_dir)
    .with_context(|| "Failed to copy external binaries")?;

  let icons = freedesktop::copy_icon_files(settings, &data_dir)
    .with_context(|| "Failed to create icon files")?;
  freedesktop::generate_desktop_file(settings, &settings.deb().desktop_template, &data_dir)
    .with_context(|| "Failed to create desktop file")?;
  generate_changelog_file(settings, &data_dir)
    .with_context(|| "Failed to create changelog.gz file")?;

  Ok((data_dir, icons))
}

/// Generate the Changelog file by compressing, to be stored at /usr/share/doc/package-name/changelog.gz. See
/// https://www.debian.org/doc/debian-policy/ch-docs.html#changelog-files-and-release-notes
fn generate_changelog_file(settings: &Settings, data_dir: &Path) -> crate::Result<()> {
  if let Some(changelog_src_path) = &settings.deb().changelog {
    let mut src_file = File::open(changelog_src_path)?;
    let bin_name = settings.main_binary_name();
    let dest_path = data_dir.join(format!("usr/share/doc/{}/changelog.gz", bin_name));

    let changelog_file = common::create_file(&dest_path)?;
    let mut gzip_encoder = GzEncoder::new(changelog_file, Compression::new(9));
    io::copy(&mut src_file, &mut gzip_encoder)?;

    let mut changelog_file = gzip_encoder.finish()?;
    changelog_file.flush()?;
  }
  Ok(())
}

/// Generates the debian control file and stores it under the `control_dir`.
fn generate_control_file(
  settings: &Settings,
  arch: &str,
  control_dir: &Path,
  data_dir: &Path,
) -> crate::Result<()> {
  // For more information about the format of this file, see
  // https://www.debian.org/doc/debian-policy/ch-controlfields.html
  let dest_path = control_dir.join("control");
  let mut file = common::create_file(&dest_path)?;
  writeln!(file, "Package: {}", AsKebabCase(settings.product_name()))?;
  writeln!(file, "Version: {}", settings.version_string())?;
  writeln!(file, "Architecture: {arch}")?;
  // Installed-Size must be divided by 1024, see https://www.debian.org/doc/debian-policy/ch-controlfields.html#installed-size
  writeln!(file, "Installed-Size: {}", total_dir_size(data_dir)? / 1024)?;
  let authors = settings.authors_comma_separated().unwrap_or_default();

  writeln!(file, "Maintainer: {authors}")?;
  if let Some(section) = &settings.deb().section {
    writeln!(file, "Section: {}", section)?;
  }
  if let Some(priority) = &settings.deb().priority {
    writeln!(file, "Priority: {}", priority)?;
  } else {
    writeln!(file, "Priority: optional")?;
  }

  if !settings.homepage_url().is_empty() {
    writeln!(file, "Homepage: {}", settings.homepage_url())?;
  }
  let dependencies = settings.deb().depends.as_ref().cloned().unwrap_or_default();
  if !dependencies.is_empty() {
    writeln!(file, "Depends: {}", dependencies.join(", "))?;
  }
  let mut short_description = settings.short_description().trim();
  if short_description.is_empty() {
    short_description = "(none)";
  }
  let mut long_description = settings.long_description().unwrap_or("").trim();
  if long_description.is_empty() {
    long_description = "(none)";
  }
  writeln!(file, "Description: {short_description}")?;
  for line in long_description.lines() {
    let line = line.trim();
    if line.is_empty() {
      writeln!(file, " .")?;
    } else {
      writeln!(file, " {line}")?;
    }
  }
  file.flush()?;
  Ok(())
}

/// Create an `md5sums` file in the `control_dir` containing the MD5 checksums
/// for each file within the `data_dir`.
fn generate_md5sums(control_dir: &Path, data_dir: &Path) -> crate::Result<()> {
  let md5sums_path = control_dir.join("md5sums");
  let mut md5sums_file = common::create_file(&md5sums_path)?;
  for entry in WalkDir::new(data_dir) {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      continue;
    }
    let mut file = File::open(path)?;
    let mut hash = md5::Context::new();
    io::copy(&mut file, &mut hash)?;
    for byte in hash.compute().iter() {
      write!(md5sums_file, "{byte:02x}")?;
    }
    let rel_path = path.strip_prefix(data_dir)?;
    let path_str = rel_path.to_str().ok_or_else(|| {
      let msg = format!("Non-UTF-8 path: {rel_path:?}");
      io::Error::new(io::ErrorKind::InvalidData, msg)
    })?;
    writeln!(md5sums_file, "  {path_str}")?;
  }
  Ok(())
}

/// Copy the bundle's resource files into an appropriate directory under the
/// `data_dir`.
fn copy_resource_files(settings: &Settings, data_dir: &Path) -> crate::Result<()> {
  let resource_dir = data_dir.join("usr/lib").join(settings.main_binary_name());
  settings.copy_resources(&resource_dir)
}

/// Create an empty file at the given path, creating any parent directories as
/// needed, then write `data` into the file.
fn create_file_with_data<P: AsRef<Path>>(path: P, data: &str) -> crate::Result<()> {
  let mut file = common::create_file(path.as_ref())?;
  file.write_all(data.as_bytes())?;
  file.flush()?;
  Ok(())
}

/// Computes the total size, in bytes, of the given directory and all of its
/// contents.
fn total_dir_size(dir: &Path) -> crate::Result<u64> {
  let mut total: u64 = 0;
  for entry in WalkDir::new(dir) {
    total += entry?.metadata()?.len();
  }
  Ok(total)
}

/// Writes a tar file to the given writer containing the given directory.
fn create_tar_from_dir<P: AsRef<Path>, W: Write>(src_dir: P, dest_file: W) -> crate::Result<W> {
  let src_dir = src_dir.as_ref();
  let mut tar_builder = tar::Builder::new(dest_file);
  for entry in WalkDir::new(src_dir) {
    let entry = entry?;
    let src_path = entry.path();
    if src_path == src_dir {
      continue;
    }
    let dest_path = src_path.strip_prefix(src_dir)?;
    let stat = fs::metadata(src_path)?;
    let mut header = tar::Header::new_gnu();
    header.set_metadata_in_mode(&stat, HeaderMode::Deterministic);
    header.set_mtime(stat.mtime() as u64);

    if entry.file_type().is_dir() {
      tar_builder.append_data(&mut header, dest_path, &mut io::empty())?;
    } else {
      let mut src_file = fs::File::open(src_path)?;
      tar_builder.append_data(&mut header, dest_path, &mut src_file)?;
    }
  }
  let dest_file = tar_builder.into_inner()?;
  Ok(dest_file)
}

/// Creates a `.tar.gz` file from the given directory (placing the new file
/// within the given directory's parent directory), then deletes the original
/// directory and returns the path to the new file.
fn tar_and_gzip_dir<P: AsRef<Path>>(src_dir: P) -> crate::Result<PathBuf> {
  let src_dir = src_dir.as_ref();
  let dest_path = src_dir.with_extension("tar.gz");
  let dest_file = common::create_file(&dest_path)?;
  let gzip_encoder = GzEncoder::new(dest_file, Compression::default());
  let gzip_encoder = create_tar_from_dir(src_dir, gzip_encoder)?;
  let mut dest_file = gzip_encoder.finish()?;
  dest_file.flush()?;
  Ok(dest_path)
}

/// Creates an `ar` archive from the given source files and writes it to the
/// given destination path.
fn create_archive(srcs: Vec<PathBuf>, dest: &Path) -> crate::Result<()> {
  let mut builder = ar::Builder::new(common::create_file(dest)?);
  for path in &srcs {
    builder.append_path(path)?;
  }
  builder.into_inner()?.flush()?;
  Ok(())
}

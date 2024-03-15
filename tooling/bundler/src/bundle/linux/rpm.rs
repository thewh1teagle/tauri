// Copyright 2016-2019 Cargo-Bundle developers <https://github.com/burtonageo/cargo-bundle>
// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::Settings;

use anyhow::Context;
use rpm::{self, signature::pgp, Dependency, FileMode, FileOptions};
use std::{
  env,
  fs::{self, File},
  path::{Path, PathBuf},
};

use super::freedesktop;

/// Bundles the project.
/// Returns a vector of PathBuf that shows where the RPM was created.
pub fn bundle_project(settings: &Settings) -> crate::Result<Vec<PathBuf>> {
  let name = settings.main_binary_name();
  let version = settings.version_string();
  let release = settings.rpm().release.as_str();
  let epoch = settings.rpm().epoch;
  let arch = match settings.binary_arch() {
    "x86" => "i386",
    "arm" => "armhfp",
    other => other,
  };

  let summary = settings.short_description().trim();

  let package_base_name = format!("{name}-{version}-{release}.{arch}");
  let package_name = format!("{package_base_name}.rpm");

  let base_dir = settings.project_out_directory().join("bundle/rpm");
  let package_dir = base_dir.join(&package_base_name);
  if package_dir.exists() {
    fs::remove_dir_all(&package_dir)
      .with_context(|| format!("Failed to remove old {package_base_name}"))?;
  }
  fs::create_dir_all(&package_dir)?;
  let package_path = base_dir.join(&package_name);

  log::info!(action = "Bundling"; "{} ({})", package_name, package_path.display());

  let license = settings.license().unwrap_or_default();
  let mut builder = rpm::PackageBuilder::new(name, version, &license, arch, summary)
    .epoch(epoch)
    .release(release);

  if let Some(description) = settings.long_description() {
    builder = builder.description(description.trim())
  }

  // Add requirements
  for dep in settings.rpm().depends.as_ref().cloned().unwrap_or_default() {
    builder = builder.requires(Dependency::any(dep));
  }

  // Add binaries
  for bin in settings.binaries() {
    let src = settings.binary_path(bin);
    let dest = Path::new("/usr/bin").join(bin.name());
    builder = builder.with_file(src, FileOptions::new(dest.to_string_lossy()))?;
  }

  // Add external binaries
  for src in settings.external_binaries() {
    let src = src?;
    let dest = Path::new("/usr/bin").join(
      src
        .file_name()
        .expect("failed to extract external binary filename")
        .to_string_lossy()
        .replace(&format!("-{}", settings.target()), ""),
    );
    builder = builder.with_file(&src, FileOptions::new(dest.to_string_lossy()))?;
  }

  // Add resources
  if settings.resource_files().count() > 0 {
    let resource_dir = Path::new("/usr/lib").join(settings.main_binary_name());
    // Create an empty file, needed to add a directory to the RPM package
    // (cf https://github.com/rpm-rs/rpm/issues/177)
    let empty_file_path = &package_dir.join("empty");
    File::create(empty_file_path)?;
    // Then add the resource directory `/usr/lib/<binary_name>` to the package.
    builder = builder.with_file(
      empty_file_path,
      FileOptions::new(resource_dir.to_string_lossy()).mode(FileMode::Dir { permissions: 0o755 }),
    )?;
    // Then add the resources files in that directory
    for src in settings.resource_files() {
      let src = src?;
      let dest = resource_dir.join(tauri_utils::resources::resource_relpath(&src));
      builder = builder.with_file(&src, FileOptions::new(dest.to_string_lossy()))?;
    }
  }

  // Add Desktop entry file
  let (desktop_src_path, desktop_dest_path) =
    freedesktop::generate_desktop_file(settings, &settings.rpm().desktop_template, &package_dir)?;
  builder = builder.with_file(
    desktop_src_path,
    FileOptions::new(desktop_dest_path.to_string_lossy()),
  )?;

  // Add icons
  for (icon, src) in &freedesktop::list_icon_files(settings, &PathBuf::from("/"))? {
    builder = builder.with_file(src, FileOptions::new(icon.path.to_string_lossy()))?;
  }

  // Add custom files
  for (rpm_path, src_path) in settings.rpm().files.iter() {
    if src_path.is_file() {
      builder = builder.with_file(src_path, FileOptions::new(rpm_path.to_string_lossy()))?;
    } else {
      for entry in walkdir::WalkDir::new(src_path) {
        let entry_path = entry?.into_path();
        if entry_path.is_file() {
          let dest_path = rpm_path.join(entry_path.strip_prefix(src_path).unwrap());
          builder =
            builder.with_file(&entry_path, FileOptions::new(dest_path.to_string_lossy()))?;
        }
      }
    }
  }

  let pkg = if let Ok(raw_secret_key) = env::var("TAURI_SIGNING_RPM_KEY") {
    let mut signer = pgp::Signer::load_from_asc(&raw_secret_key)?;
    if let Ok(passphrase) = env::var("TAURI_SIGNING_RPM_KEY_PASSPHRASE") {
      signer = signer.with_key_passphrase(passphrase);
    }
    builder.build_and_sign(signer)?
  } else {
    builder.build()?
  };

  let mut f = fs::File::create(&package_path)?;
  pkg.write(&mut f)?;

  Ok(vec![package_path])
}

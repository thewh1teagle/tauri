// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{
  helpers::app_paths::{app_dir, tauri_dir},
  Result,
};

mod config;
mod frontend;
mod manifest;

pub fn command() -> Result<()> {
  let tauri_dir = tauri_dir();
  let app_dir = app_dir();

  let migrated = config::migrate(&tauri_dir)?;
  manifest::migrate(&tauri_dir)?;
  frontend::migrate(app_dir, &tauri_dir)?;

  // Add plugins
  for plugin in migrated.plugins {
    crate::add::command(crate::add::Options {
      plugin,
      branch: None,
      tag: None,
      rev: None,
    })?
  }

  Ok(())
}

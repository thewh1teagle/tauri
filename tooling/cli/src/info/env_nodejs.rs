// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::{ActionResult, SectionItem, VersionMetadata};
use colored::Colorize;

use crate::helpers::cross_command;

pub fn manager_version(package_manager: &str) -> Option<String> {
  cross_command(package_manager)
    .arg("-v")
    .output()
    .map(|o| {
      if o.status.success() {
        let v = String::from_utf8_lossy(o.stdout.as_slice()).to_string();
        Some(v.split('\n').next().unwrap().to_string())
      } else {
        None
      }
    })
    .ok()
    .unwrap_or_default()
}

pub fn items(metadata: &VersionMetadata) -> Vec<SectionItem> {
  let node_target_ver = metadata.js_cli.node.replace(">= ", "");

  vec![
    SectionItem::new().action(move || {
      cross_command("node")
        .arg("-v")
        .output()
        .map(|o| {
          if o.status.success() {
            let v = String::from_utf8_lossy(o.stdout.as_slice()).to_string();
            let v = v
              .split('\n')
              .next()
              .unwrap()
              .strip_prefix('v')
              .unwrap_or_default()
              .trim();
            ActionResult::Description(format!("node: {}{}", v, {
              let version = semver::Version::parse(v).unwrap();
              let target_version = semver::Version::parse(node_target_ver.as_str()).unwrap();
              if version < target_version {
                format!(
                  " ({}, latest: {})",
                  "outdated".red(),
                  target_version.to_string().green()
                )
              } else {
                "".into()
              }
            }))
          } else {
            ActionResult::None
          }
        })
        .ok()
        .unwrap_or_default()
    }),
    SectionItem::new().action(|| {
      manager_version("pnpm")
        .map(|v| format!("pnpm: {}", v))
        .into()
    }),
    SectionItem::new().action(|| {
      manager_version("yarn")
        .map(|v| format!("yarn: {}", v))
        .into()
    }),
    SectionItem::new().action(|| manager_version("npm").map(|v| format!("npm: {}", v)).into()),
    SectionItem::new().action(|| manager_version("bun").map(|v| format!("bun: {}", v)).into()),
  ]
}

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Plugin ACL types.

use std::{collections::BTreeMap, num::NonZeroU64};

use super::{Permission, PermissionSet};
use serde::{Deserialize, Serialize};

/// The default permission set of the plugin.
///
/// Works similarly to a permission with the "default" identifier.
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct DefaultPermission {
  /// The version of the permission.
  pub version: Option<NonZeroU64>,

  /// Human-readable description of what the permission does.
  pub description: Option<String>,

  /// All permissions this set contains.
  pub permissions: Vec<String>,
}

/// Permission file that can define a default permission, a set of permissions or a list of inlined permissions.
#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PermissionFile {
  /// The default permission set for the plugin
  pub default: Option<DefaultPermission>,

  /// A list of permissions sets defined
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub set: Vec<PermissionSet>,

  /// A list of inlined permissions
  #[serde(default)]
  pub permission: Vec<Permission>,
}

/// Plugin manifest.
#[derive(Debug, Serialize, Deserialize)]
pub struct Manifest {
  /// Default permission.
  pub default_permission: Option<PermissionSet>,
  /// Plugin permissions.
  pub permissions: BTreeMap<String, Permission>,
  /// Plugin permission sets.
  pub permission_sets: BTreeMap<String, PermissionSet>,
  /// The global scope schema.
  pub global_scope_schema: Option<serde_json::Value>,
}

impl Manifest {
  /// Creates a new manifest from the given plugin permission files and global scope schema.
  pub fn new(
    permission_files: Vec<PermissionFile>,
    global_scope_schema: Option<serde_json::Value>,
  ) -> Self {
    let mut manifest = Self {
      default_permission: None,
      permissions: BTreeMap::new(),
      permission_sets: BTreeMap::new(),
      global_scope_schema,
    };

    for permission_file in permission_files {
      if let Some(default) = permission_file.default {
        manifest.default_permission.replace(PermissionSet {
          identifier: "default".into(),
          description: default
            .description
            .unwrap_or_else(|| "Default plugin permissions.".to_string()),
          permissions: default.permissions,
        });
      }

      manifest.permissions.extend(
        permission_file
          .permission
          .into_iter()
          .map(|p| (p.identifier.clone(), p))
          .collect::<BTreeMap<_, _>>(),
      );

      manifest.permission_sets.extend(
        permission_file
          .set
          .into_iter()
          .map(|set| {
            (
              set.identifier.clone(),
              PermissionSet {
                identifier: set.identifier,
                description: set.description,
                permissions: set.permissions,
              },
            )
          })
          .collect::<BTreeMap<_, _>>(),
      );
    }

    manifest
  }
}

#[cfg(feature = "build")]
mod build {
  use proc_macro2::TokenStream;
  use quote::{quote, ToTokens, TokenStreamExt};
  use std::convert::identity;

  use super::*;
  use crate::{literal_struct, tokens::*};

  impl ToTokens for DefaultPermission {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let version = opt_lit_owned(self.version.as_ref().map(|v| {
        let v = v.get();
        quote!(::core::num::NonZeroU64::new(#v).unwrap())
      }));
      let description = opt_str_lit(self.description.as_ref());
      let permissions = vec_lit(&self.permissions, str_lit);
      literal_struct!(
        tokens,
        ::tauri::utils::acl::plugin::DefaultPermission,
        version,
        description,
        permissions
      )
    }
  }

  impl ToTokens for Manifest {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let default_permission = opt_lit(self.default_permission.as_ref());

      let permissions = map_lit(
        quote! { ::std::collections::BTreeMap },
        &self.permissions,
        str_lit,
        identity,
      );

      let permission_sets = map_lit(
        quote! { ::std::collections::BTreeMap },
        &self.permission_sets,
        str_lit,
        identity,
      );

      let global_scope_schema =
        opt_lit_owned(self.global_scope_schema.as_ref().map(json_value_lit));

      literal_struct!(
        tokens,
        ::tauri::utils::acl::manifest::Manifest,
        default_permission,
        permissions,
        permission_sets,
        global_scope_schema
      )
    }
  }
}

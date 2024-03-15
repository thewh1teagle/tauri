// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Access Control List types.

use serde::{Deserialize, Serialize};
use std::{num::NonZeroU64, str::FromStr, sync::Arc};
use thiserror::Error;
use url::Url;

use crate::platform::Target;

pub use self::{identifier::*, value::*};

/// Known filename of the permission schema JSON file
pub const PERMISSION_SCHEMA_FILE_NAME: &str = "schema.json";
/// Known ACL key for the app permissions.
pub const APP_ACL_KEY: &str = "__app-acl__";

#[cfg(feature = "build")]
pub mod build;
pub mod capability;
pub mod identifier;
pub mod manifest;
pub mod resolved;
pub mod value;

/// Possible errors while processing ACL files.
#[derive(Debug, Error)]
pub enum Error {
  /// Could not find an environmental variable that is set inside of build scripts.
  ///
  /// Whatever generated this should be called inside of a build script.
  #[error("expected build script env var {0}, but it was not found - ensure this is called in a build script")]
  BuildVar(&'static str),

  /// The links field in the manifest **MUST** be set and match the name of the crate.
  #[error("package.links field in the Cargo manifest is not set, it should be set to the same as package.name")]
  LinksMissing,

  /// The links field in the manifest **MUST** match the name of the crate.
  #[error(
    "package.links field in the Cargo manifest MUST be set to the same value as package.name"
  )]
  LinksName,

  /// IO error while reading a file
  #[error("failed to read file: {0}")]
  ReadFile(std::io::Error),

  /// IO error while writing a file
  #[error("failed to write file: {0}")]
  WriteFile(std::io::Error),

  /// IO error while creating a file
  #[error("failed to create file: {0}")]
  CreateFile(std::io::Error),

  /// [`cargo_metadata`] was not able to complete successfully
  #[cfg(feature = "build")]
  #[error("failed to execute: {0}")]
  Metadata(#[from] ::cargo_metadata::Error),

  /// Invalid glob
  #[error("failed to run glob: {0}")]
  Glob(#[from] glob::PatternError),

  /// Invalid TOML encountered
  #[error("failed to parse TOML: {0}")]
  Toml(#[from] toml::de::Error),

  /// Invalid JSON encountered
  #[error("failed to parse JSON: {0}")]
  Json(#[from] serde_json::Error),

  /// Invalid permissions file format
  #[error("unknown permission format {0}")]
  UnknownPermissionFormat(String),

  /// Invalid capabilities file format
  #[error("unknown capability format {0}")]
  UnknownCapabilityFormat(String),

  /// Permission referenced in set not found.
  #[error("permission {permission} not found from set {set}")]
  SetPermissionNotFound {
    /// Permission identifier.
    permission: String,
    /// Set identifier.
    set: String,
  },

  /// Unknown ACL manifest.
  #[error("unknown ACL for {key}, expected one of {available}")]
  UnknownManifest {
    /// Manifest key.
    key: String,
    /// Available manifest keys.
    available: String,
  },

  /// Unknown permission.
  #[error("unknown permission {permission} for {key}")]
  UnknownPermission {
    /// Manifest key.
    key: String,

    /// Permission identifier.
    permission: String,
  },
}

/// Allowed and denied commands inside a permission.
///
/// If two commands clash inside of `allow` and `deny`, it should be denied by default.
#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Commands {
  /// Allowed command.
  #[serde(default)]
  pub allow: Vec<String>,

  /// Denied command, which takes priority.
  #[serde(default)]
  pub deny: Vec<String>,
}

/// A restriction of the command/endpoint functionality.
///
/// It can be of any serde serializable type and is used for allowing or preventing certain actions inside a Tauri command.
///
/// The scope is passed to the command and handled/enforced by the command itself.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Scopes {
  /// Data that defines what is allowed by the scope.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub allow: Option<Vec<Value>>,
  /// Data that defines what is denied by the scope.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub deny: Option<Vec<Value>>,
}

impl Scopes {
  fn is_empty(&self) -> bool {
    self.allow.is_none() && self.deny.is_none()
  }
}

/// Descriptions of explicit privileges of commands.
///
/// It can enable commands to be accessible in the frontend of the application.
///
/// If the scope is defined it can be used to fine grain control the access of individual or multiple commands.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Permission {
  /// The version of the permission.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<NonZeroU64>,

  /// A unique identifier for the permission.
  pub identifier: String,

  /// Human-readable description of what the permission does.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,

  /// Allowed or denied commands when using this permission.
  #[serde(default)]
  pub commands: Commands,

  /// Allowed or denied scoped when using this permission.
  #[serde(default, skip_serializing_if = "Scopes::is_empty")]
  pub scope: Scopes,

  /// Target platforms this permission applies. By default all platforms are affected by this permission.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub platforms: Option<Vec<Target>>,
}

/// A set of direct permissions grouped together under a new name.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PermissionSet {
  /// A unique identifier for the permission.
  pub identifier: String,

  /// Human-readable description of what the permission does.
  pub description: String,

  /// All permissions this set contains.
  pub permissions: Vec<String>,
}

/// UrlPattern for [`ExecutionContext::Remote`].
#[derive(Debug, Clone)]
pub struct RemoteUrlPattern(Arc<urlpattern::UrlPattern>, String);

impl FromStr for RemoteUrlPattern {
  type Err = urlpattern::quirks::Error;

  fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
    let mut init = urlpattern::UrlPatternInit::parse_constructor_string::<regex::Regex>(s, None)?;
    if init.search.as_ref().map(|p| p.is_empty()).unwrap_or(true) {
      init.search.replace("*".to_string());
    }
    if init.hash.as_ref().map(|p| p.is_empty()).unwrap_or(true) {
      init.hash.replace("*".to_string());
    }
    if init
      .pathname
      .as_ref()
      .map(|p| p.is_empty() || p == "/")
      .unwrap_or(true)
    {
      init.pathname.replace("*".to_string());
    }
    let pattern = urlpattern::UrlPattern::parse(init)?;
    Ok(Self(Arc::new(pattern), s.to_string()))
  }
}

impl RemoteUrlPattern {
  #[doc(hidden)]
  pub fn as_str(&self) -> &str {
    &self.1
  }

  /// Test if a given URL matches the pattern.
  pub fn test(&self, url: &Url) -> bool {
    self
      .0
      .test(urlpattern::UrlPatternMatchInput::Url(url.clone()))
      .unwrap_or_default()
  }
}

impl PartialEq for RemoteUrlPattern {
  fn eq(&self, other: &Self) -> bool {
    self.0.protocol() == other.0.protocol()
      && self.0.username() == other.0.username()
      && self.0.password() == other.0.password()
      && self.0.hostname() == other.0.hostname()
      && self.0.port() == other.0.port()
      && self.0.pathname() == other.0.pathname()
      && self.0.search() == other.0.search()
      && self.0.hash() == other.0.hash()
  }
}

impl Eq for RemoteUrlPattern {}

/// Execution context of an IPC call.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum ExecutionContext {
  /// A local URL is used (the Tauri app URL).
  #[default]
  Local,
  /// Remote URL is trying to use the IPC.
  Remote {
    /// The URL trying to access the IPC (URL pattern).
    url: RemoteUrlPattern,
  },
}

#[cfg(test)]
mod tests {
  use crate::acl::RemoteUrlPattern;

  #[test]
  fn url_pattern_domain_wildcard() {
    let pattern: RemoteUrlPattern = "http://*".parse().unwrap();

    assert!(pattern.test(&"http://tauri.app/path".parse().unwrap()));
    assert!(pattern.test(&"http://tauri.app/path?q=1".parse().unwrap()));

    assert!(pattern.test(&"http://localhost/path".parse().unwrap()));
    assert!(pattern.test(&"http://localhost/path?q=1".parse().unwrap()));

    let pattern: RemoteUrlPattern = "http://*.tauri.app".parse().unwrap();

    assert!(!pattern.test(&"http://tauri.app/path".parse().unwrap()));
    assert!(!pattern.test(&"http://tauri.app/path?q=1".parse().unwrap()));
    assert!(pattern.test(&"http://api.tauri.app/path".parse().unwrap()));
    assert!(pattern.test(&"http://api.tauri.app/path?q=1".parse().unwrap()));
    assert!(!pattern.test(&"http://localhost/path".parse().unwrap()));
    assert!(!pattern.test(&"http://localhost/path?q=1".parse().unwrap()));
  }

  #[test]
  fn url_pattern_path_wildcard() {
    let pattern: RemoteUrlPattern = "http://localhost/*".parse().unwrap();
    assert!(pattern.test(&"http://localhost/path".parse().unwrap()));
    assert!(pattern.test(&"http://localhost/path?q=1".parse().unwrap()));
  }

  #[test]
  fn url_pattern_scheme_wildcard() {
    let pattern: RemoteUrlPattern = "*://localhost".parse().unwrap();
    assert!(pattern.test(&"http://localhost/path".parse().unwrap()));
    assert!(pattern.test(&"https://localhost/path?q=1".parse().unwrap()));
    assert!(pattern.test(&"custom://localhost/path".parse().unwrap()));
  }
}

#[cfg(feature = "build")]
mod build_ {
  use std::convert::identity;

  use crate::{literal_struct, tokens::*};

  use super::*;
  use proc_macro2::TokenStream;
  use quote::{quote, ToTokens, TokenStreamExt};

  impl ToTokens for ExecutionContext {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let prefix = quote! { ::tauri::utils::acl::ExecutionContext };

      tokens.append_all(match self {
        Self::Local => {
          quote! { #prefix::Local }
        }
        Self::Remote { url } => {
          let url = url.as_str();
          quote! { #prefix::Remote { url: #url.parse().unwrap() } }
        }
      });
    }
  }

  impl ToTokens for Commands {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let allow = vec_lit(&self.allow, str_lit);
      let deny = vec_lit(&self.deny, str_lit);
      literal_struct!(tokens, ::tauri::utils::acl::Commands, allow, deny)
    }
  }

  impl ToTokens for Scopes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let allow = opt_vec_lit(self.allow.as_ref(), identity);
      let deny = opt_vec_lit(self.deny.as_ref(), identity);
      literal_struct!(tokens, ::tauri::utils::acl::Scopes, allow, deny)
    }
  }

  impl ToTokens for Permission {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let version = opt_lit_owned(self.version.as_ref().map(|v| {
        let v = v.get();
        quote!(::core::num::NonZeroU64::new(#v).unwrap())
      }));
      let identifier = str_lit(&self.identifier);
      let description = opt_str_lit(self.description.as_ref());
      let commands = &self.commands;
      let scope = &self.scope;
      let platforms = opt_vec_lit(self.platforms.as_ref(), identity);

      literal_struct!(
        tokens,
        ::tauri::utils::acl::Permission,
        version,
        identifier,
        description,
        commands,
        scope,
        platforms
      )
    }
  }

  impl ToTokens for PermissionSet {
    fn to_tokens(&self, tokens: &mut TokenStream) {
      let identifier = str_lit(&self.identifier);
      let description = str_lit(&self.description);
      let permissions = vec_lit(&self.permissions, str_lit);
      literal_struct!(
        tokens,
        ::tauri::utils::acl::PermissionSet,
        identifier,
        description,
        permissions
      )
    }
  }
}

use std::collections::HashMap;

use tabled::Tabled;
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

/// ## ContextArg
///
/// `nanocl context` available arguments
///
#[derive(Debug, Parser)]
pub struct ContextArg {
  #[clap(subcommand)]
  pub command: ContextCommand,
}

/// ## ContextCommand
///
/// `nanocl context` available commands
///
#[derive(Debug, Subcommand)]
pub enum ContextCommand {
  /// List contexts
  #[clap(alias = "ls")]
  List,
  /// Set current context
  Use {
    /// Context name
    name: String,
  },
  /// Create a new context from a file
  From {
    /// Path to context file
    path: String,
  },
}

/// ## ContextEndpoint
///
/// A context endpoint definition
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextEndpoint {
  pub host: String,
}

/// ## ContextMetaData
///
/// A context metadata definition
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContextMetaData {
  pub description: String,
}

/// ## Context
///
/// A context definition is a user defined set of endpoints to manage remote nanocl clusters
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Context {
  pub name: String,
  pub meta_data: ContextMetaData,
  pub endpoints: HashMap<String, ContextEndpoint>,
}

/// Default value for a Context
impl std::default::Default for Context {
  fn default() -> Self {
    Self {
      name: "default".into(),
      meta_data: ContextMetaData {
        description: "Default context based on NANOCL_HOST env".into(),
      },
      endpoints: {
        let mut map = HashMap::new();
        map.insert(
          "Nanocl".into(),
          ContextEndpoint {
            host: std::env::var("NANOCL_HOST")
              .unwrap_or("unix:///run/nanocl/nanocl.sock".into()),
          },
        );
        map
      },
    }
  }
}

/// ## ContextRow
///
/// A row of the context table
///
#[derive(Clone, Debug, Tabled)]
pub struct ContextRow {
  /// Name of the context
  pub name: String,
  /// Description of the context
  pub description: String,
  /// Endpoint of the context
  pub endpoint: String,
  /// Current context indicator
  pub current: String,
}

/// Convert Context to ContextRow
impl From<Context> for ContextRow {
  fn from(context: Context) -> Self {
    let endpoint = context
      .endpoints
      .get("Nanocl")
      .expect("Expect context to have a Nanocl endpoint");
    Self {
      name: context.name,
      description: context.meta_data.description,
      endpoint: endpoint.host.clone(),
      current: "⨯".into(),
    }
  }
}

/// ## DockerContextMetaEndpoint
///
/// A docker context endpoint definition used to parse the docker context metadata endpoint
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContextMetaEndpoint {
  pub host: String,
}

/// ## DockerContextMeta
///
/// A docker context metadata definition used to parse the docker context metadata
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContextMeta {
  pub endpoints: HashMap<String, DockerContextMetaEndpoint>,
}

use crate::config::UserConfig;
use crate::models::{Context, ContextRow};

/// ## Context
///
/// Context is a struct that represents a nanocl context
/// A nanocl context is a configuration for a specific cluster
///
impl Context {
  pub fn new() -> Self {
    Self::default()
  }

  /// ## Ensure
  ///
  /// Ensure that the contexts directory exists in $HOME/.nanocl/contexts
  ///
  /// ## Return
  ///
  /// * [Result](Result) The result of the operation
  ///   * [Ok](()) The operation was successful
  ///   * [Err](Err) An error occured
  ///
  pub fn ensure() -> std::io::Result<()> {
    let home = std::env::var("HOME").map_err(|_| {
      std::io::Error::new(std::io::ErrorKind::Other, "Could not get $HOME")
    })?;
    let path = format!("{home}/.nanocl/contexts");
    std::fs::create_dir_all(path)?;
    Ok(())
  }

  /// ## Read
  ///
  /// Read a context from a file
  ///
  /// ## Arguments
  ///
  /// * [path](str) The path to the context file
  ///
  /// ## Return
  ///
  /// * [Result](Result) The result of the operation
  ///   * [Ok](Context) The context
  ///   * [Err](Err) An error occured
  ///
  pub fn read(path: &str) -> std::io::Result<Context> {
    let s = std::fs::read_to_string(path)?;
    let context = serde_yaml::from_str::<Context>(&s).map_err(|err| {
      std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("Could not parse context {path}: {err}"),
      )
    })?;
    Ok(context)
  }

  /// ## Read by name
  ///
  /// Read a context by name
  ///
  /// ## Arguments
  ///
  /// * [name](str) The name of the context
  ///
  /// ## Return
  ///
  /// * [Result](Result) The result of the operation
  ///   * [Ok](Context) The context
  ///   * [Err](Err) An error occured
  ///
  pub fn read_by_name(name: &str) -> std::io::Result<Context> {
    let home = std::env::var("HOME").map_err(|_| {
      std::io::Error::new(std::io::ErrorKind::Other, "Could not get $HOME")
    })?;
    let path = format!("{home}/.nanocl/contexts/{name}.yml");
    let context = Self::read(&path)?;
    Ok(context)
  }

  /// ## Write
  ///
  /// Write a context to a file
  ///
  /// ## Arguments
  ///
  /// * [context](Context) The context to write
  ///
  /// ## Return
  ///
  /// * [Result](Result) The result of the operation
  ///   * [Ok](()) The operation was successful
  ///   * [Err](Err) An error occured
  ///
  pub fn write(context: &Context) -> std::io::Result<()> {
    let home = std::env::var("HOME").map_err(|_| {
      std::io::Error::new(std::io::ErrorKind::Other, "Could not get $HOME")
    })?;
    let path = format!("{home}/.nanocl/contexts/{}.yml", context.name);
    let s = serde_yaml::to_string(&context).map_err(|err| {
      std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("Could not serialize context {}: {err}", context.name),
      )
    })?;
    std::fs::write(path, s)?;
    Ok(())
  }

  /// ## List
  ///
  /// List all contexts
  ///
  /// ## Return
  ///
  /// * [Result](Result) The result of the operation
  ///   * [Ok](Vec<ContextRow>) The list of contexts
  ///   * [Err](Err) An error occured
  ///
  pub fn list() -> std::io::Result<Vec<ContextRow>> {
    let home = std::env::var("HOME").map_err(|_| {
      std::io::Error::new(std::io::ErrorKind::Other, "Could not get $HOME")
    })?;
    let path = format!("{home}/.nanocl/contexts");
    let mut contexts = vec![ContextRow::from(Context::new())];
    for entry in std::fs::read_dir(path)? {
      let entry = entry?;
      let path = entry.path();
      let path = path.to_string_lossy().to_string();
      if let Ok(context) = Self::read(&path) {
        contexts.push(ContextRow::from(context));
      }
    }
    Ok(contexts)
  }

  /// ## Use
  ///
  /// Use a context
  ///
  /// ## Arguments
  ///
  /// * [name](str) The name of the context
  ///
  /// ## Return
  ///
  /// * [Result](Result) The result of the operation
  ///   * [Ok](()) The operation was successful
  ///   * [Err](Err) An error occured
  ///
  pub fn r#use(name: &str) -> std::io::Result<()> {
    let home = std::env::var("HOME").map_err(|_| {
      std::io::Error::new(std::io::ErrorKind::Other, "Could not get $HOME")
    })?;
    if name != "default" {
      Context::read_by_name(name).map_err(|err| {
        std::io::Error::new(
          std::io::ErrorKind::InvalidData,
          format!("Could not read context {name}: {err}"),
        )
      })?;
    }
    let path = format!("{home}/.nanocl/conf.yml");
    let mut config = UserConfig::new();
    config.current_context = name.to_string();
    let s = serde_yaml::to_string(&config).map_err(|err| {
      std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("Could not serialize config: {err}"),
      )
    })?;
    std::fs::write(path, s)?;
    Ok(())
  }
}

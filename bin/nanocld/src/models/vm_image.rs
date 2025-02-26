use diesel::prelude::*;

use nanocl_stubs::vm_image::VmImage;
use serde::{Serialize, Deserialize};

use crate::schema::vm_images;

/// ## VmImageDbModel
///
/// This structure represent a virtual machine image in the database.
/// A virtual machine image is a file that represent a virtual machine disk.
///
/// Two kind of virtual machine image are supported:
/// - Base: A base image is a virtual machine image that is not based on another image.
/// - Snapshot: A snapshot image is a virtual machine image that is based on a base image.
///
/// A `Snapshot` of a `Base` image will alway be use to create a virtual machine.
///
#[derive(
  Clone, Debug, Queryable, Identifiable, Insertable, Serialize, Deserialize,
)]
#[diesel(primary_key(name))]
#[diesel(table_name = vm_images)]
#[serde(rename_all = "PascalCase")]
pub struct VmImageDbModel {
  /// The name of the virtual machine image
  pub(crate) name: String,
  /// The created at date
  pub(crate) created_at: chrono::NaiveDateTime,
  /// The kind of the virtual machine image (Base, Snapshot)
  pub(crate) kind: String,
  /// The path of the virtual machine image
  pub(crate) path: String,
  /// The format of the virtual machine image
  pub(crate) format: String,
  /// The actual size of the virtual machine image
  pub(crate) size_actual: i64,
  /// The virtual size of the virtual machine image
  pub(crate) size_virtual: i64,
  /// The parent of the virtual machine image
  pub(crate) parent: Option<String>,
}

/// ## VmImageUpdateDbModel
///
/// This structure is used to update a virtual machine image in the database.
///
#[derive(Clone, Debug, AsChangeset)]
#[diesel(table_name = vm_images)]
pub struct VmImageUpdateDbModel {
  /// The actual size of the virtual machine image
  pub(crate) size_actual: i64,
  /// The virtual size of the virtual machine image
  pub(crate) size_virtual: i64,
}

/// ## QemuImgInfo
///
/// This structure is used to parse the output of the qemu-img info command.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct QemuImgInfo {
  /// The format of the virtual machine image
  pub(crate) format: String,
  /// The virtual size of the virtual machine image
  pub(crate) virtual_size: i64,
  /// The actual size of the virtual machine image
  pub(crate) actual_size: i64,
}

/// Helper to convert a `VmImageDbModel` to a `VmImage`
impl From<VmImageDbModel> for VmImage {
  fn from(db_model: VmImageDbModel) -> Self {
    Self {
      name: db_model.name,
      created_at: db_model.created_at,
      path: db_model.path,
      kind: db_model.kind,
      format: db_model.format,
      size_actual: db_model.size_actual,
      size_virtual: db_model.size_virtual,
    }
  }
}

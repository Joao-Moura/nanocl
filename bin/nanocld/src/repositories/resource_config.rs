use ntex::web;
use diesel::prelude::*;

use nanocl_stubs::resource::ResourceConfig;

use nanocl_utils::io_error::{IoError, FromIo, IoResult};

use crate::utils;
use crate::models::{Pool, ResourceConfigDbModel};

/// ## Create resource config
///
/// Create a resource config in database
///
/// ## Arguments
///
/// - [item](ResourceConfigDbModel) - Resource config item
/// - [pool](Pool) - Database connection pool
///
/// ## Returns
///
/// - [Result](Result) - The result of the operation
///  - [Ok](ResourceConfigDbModel) - Resource config created
/// - [Err](HttpResponseError) - Error during the operation
///
/// ## Examples
///
/// ```rust,norun
/// use nanocl_stubs::resource::ResourceConfigDbModel;
/// use crate::repositories;
///
/// let item = ResourceConfigDbModel {
///   // Fill data
/// };
/// let resource_config = repositories::resource_config::create(item, &pool).await;
/// ```
///
pub async fn create(
  item: &ResourceConfigDbModel,
  pool: &Pool,
) -> IoResult<ResourceConfigDbModel> {
  use crate::schema::resource_configs::dsl;

  let item = item.clone();
  let pool = pool.clone();

  let dbmodel = web::block(move || {
    let mut conn = utils::store::get_pool_conn(&pool)?;
    diesel::insert_into(dsl::resource_configs)
      .values(&item)
      .execute(&mut conn)
      .map_err(|err| err.map_err_context(|| "ResourceConfig"))?;
    Ok::<_, IoError>(item)
  })
  .await?;

  Ok(dbmodel)
}

/// ## Delete resource config by resource key
///
/// Delete all resource config by resource key
///
/// ## Arguments
///
/// - [key](&str) - Resource key
/// - [pool](Pool) - Database connection pool
///
/// ## Returns
///
/// - [Result](Result) - The result of the operation
///   - [Ok](()) - Resource config deleted
///   - [Err](HttpResponseError) - Error during the operation
///
/// ## Examples
///
/// ```rust,norun
/// use crate::repositories;
///
/// repositories::resource_config::delete_by_resource_key(&str::from("my-resource"), &pool).await;
/// ```
///
pub async fn delete_by_resource_key(key: &str, pool: &Pool) -> IoResult<()> {
  use crate::schema::resource_configs::dsl;

  let key = key.to_owned();
  let pool = pool.clone();

  web::block(move || {
    let mut conn = utils::store::get_pool_conn(&pool)?;
    diesel::delete(dsl::resource_configs.filter(dsl::resource_key.eq(key)))
      .execute(&mut conn)
      .map_err(|err| err.map_err_context(|| "ResourceConfig"))?;
    Ok::<_, IoError>(())
  })
  .await?;

  Ok(())
}

pub async fn list_by_resource(
  key: &str,
  pool: &Pool,
) -> IoResult<Vec<ResourceConfig>> {
  use crate::schema::resource_configs::dsl;

  let key = key.to_owned();
  let pool = pool.clone();

  let models = web::block(move || {
    let mut conn = utils::store::get_pool_conn(&pool)?;
    let items = dsl::resource_configs
      .order(dsl::created_at.desc())
      .filter(dsl::resource_key.eq(key))
      .load::<ResourceConfigDbModel>(&mut conn)
      .map_err(|err| err.map_err_context(|| "ResourceConfig"))?;
    Ok::<_, IoError>(items)
  })
  .await?;

  let models = models
    .into_iter()
    .map(ResourceConfig::from)
    .collect::<Vec<_>>();

  Ok(models)
}

pub async fn find_by_key(
  key: &uuid::Uuid,
  pool: &Pool,
) -> IoResult<ResourceConfig> {
  use crate::schema::resource_configs::dsl;

  let key = *key;
  let pool = pool.clone();

  let model = web::block(move || {
    let mut conn = utils::store::get_pool_conn(&pool)?;
    let item = dsl::resource_configs
      .filter(dsl::key.eq(key))
      .first::<ResourceConfigDbModel>(&mut conn)
      .map_err(|err| err.map_err_context(|| "ResourceConfig"))?;
    Ok::<_, IoError>(item)
  })
  .await?;

  Ok(ResourceConfig::from(model))
}

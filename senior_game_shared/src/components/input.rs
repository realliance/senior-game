use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;
use serde::Deserialize;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "b6de0f0e-33c3-4154-99d9-2d4d1cee167d"]
pub struct RawBinding(pub String);

#[derive(Default)]
pub struct RawBindingAssetLoader;

impl AssetLoader for RawBindingAssetLoader {
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
    Box::pin(async move {
      let custom_asset = String::from_utf8(bytes.to_vec())?;
      load_context.set_default_asset(LoadedAsset::new(RawBinding(custom_asset)));
      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["ron"]
  }
}

#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CreatePickSource;

#[derive(Clone, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CreatePickMesh;

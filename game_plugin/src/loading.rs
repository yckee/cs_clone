use csv::ReaderBuilder;
use serde::Deserialize;

use crate::GameState;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};

use bevy_asset_loader::AssetCollection;
use bevy_kira_audio::AudioSource;
// use bevy_ecs_tilemap::prelude::*;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct MapAsset {
    pub map: Vec<Vec<u32>>,
}

#[derive(Default)]
pub struct MapAssetLoader;

impl AssetLoader for MapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(bytes);
            let mut map: Vec<Vec<u32>> = Vec::new();

            for result in rdr.deserialize() {
                let record: Vec<u32> = result?;
                map.push(record);
            }

            let map_asset = MapAsset { map };
            load_context.set_default_asset(LoadedAsset::new(map_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<MapAsset>().add_asset_loader(MapAssetLoader);
        bevy_asset_loader::AssetLoader::new(GameState::Loading, GameState::Playing)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .with_collection::<MapAssets>()
            .build(app);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 6, rows = 2))]
    #[asset(path = "textures/templates/templates_map_one.png")]
    pub tileset: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 2, rows = 1))]
    #[asset(path = "textures/character_one/character_one_sprite_animation_stay.png")]
    pub player_stay: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 7, rows = 1))]
    #[asset(path = "textures/character_one/character_one_sprite_animation_walk.png")]
    pub player_walk: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 64., tile_size_y = 64., columns = 4, rows = 1))]
    #[asset(path = "textures/character_one/character_one_sprite_animation_jump.png")]
    pub player_jump: Handle<TextureAtlas>,

    #[asset(path = "textures/character_one/character_one_sprite.png")]
    pub player: Handle<Texture>,
}

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(path = "textures/maps/map_one.csv")]
    pub map_one: Handle<MapAsset>,
}

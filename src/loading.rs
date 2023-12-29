use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "textures/april.assets.ron",
                )
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(key = "april")]
    pub april: Handle<TextureAtlas>,
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/Level07_Scene01_Ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/Level07_Scene01_Buildings01.png")]
    pub buildings_fg: Handle<Image>,
    #[asset(path = "textures/Level07_Scene01_Buildings01.png")]
    pub buildings_bg: Handle<Image>,
    #[asset(path = "textures/Level07_Scene01_Sky.png")]
    pub sky: Handle<Image>,
    #[asset(path = "textures/Level07_Scene01_Trees.png")]
    pub tree: Handle<Image>,
}

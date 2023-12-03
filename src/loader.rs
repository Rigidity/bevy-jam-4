use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, ModelAssets>(GameState::Loading);
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ModelAssets {
    #[asset(path = "models/Low Poly Weapons Pack/Pistol.glb")]
    pub gun: Handle<Scene>,
}

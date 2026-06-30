use bevy::prelude::*;

#[derive(Resource,Debug,Default)]
pub struct SceneAssets {
    pub asteroid: Handle<WorldAsset>,
    pub spaceship: Handle<WorldAsset>,
    pub missiles: Handle<WorldAsset>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(Startup,load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load("asteroid.glb#Scene0"),
        spaceship: asset_server.load("spaceship.glb#Scene0"),
        missiles: asset_server.load("missile.gltf#Scene0"),
    }
}
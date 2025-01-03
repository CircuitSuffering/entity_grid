use bevy::{
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
};
use entity_grid::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes_override: Some(true),
            ..Default::default()
        }))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 2000.,
            ..default()
        })
        .add_plugins(EntityGridPlugin {
            settings: EntityGridSettings {
                up_offset: 0.0,
                cell_size: 0.5,
            },
        })
        .add_plugins((RemoteHttpPlugin::default(), RemotePlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/standing_freezer.glb")),
        ),
        GridPosition::new(1, 1),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/standing_freezer.glb")),
        ),
        GridPosition::new(0, 1),
    ));
    commands.spawn((
        SceneRoot(
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/standing_freezer.glb")),
        ),
        GridPosition::new(-1, 1),
    ));
}

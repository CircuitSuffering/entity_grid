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
        .add_plugins(EntityGridPlugin {
            settings: EntityGridSettings {
                up_offset: 0.0,
                cell_size: 256.,
            },
        })
        .add_plugins((RemoteHttpPlugin::default(), RemotePlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Camera {
        ..Default::default()
    }));
    commands.spawn((
        Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
        GridPosition::new(1, 1),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
        GridPosition::new(0, 1),
    ));
    commands.spawn((
        Sprite::from_image(asset_server.load("branding/bevy_bird_dark.png")),
        GridPosition::new(-1, 1),
    ));
}

fn update(#[cfg(debug_assertions)] mut gizmos: Gizmos) {
    #[cfg(debug_assertions)]
    gizmos
        .grid_2d(
            Isometry2d::IDENTITY,
            UVec2::new(9, 9),
            Vec2::new(256., 256.),
            // Dark gray
            LinearRgba::gray(0.05),
        )
        .outer_edges();
}

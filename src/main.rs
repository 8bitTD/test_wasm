use bevy::{color::palettes::basic, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);
    commands.spawn(AudioPlayer::new(
        asset_server.load("sounds/Windless Slopes.ogg"),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
        Transform::default().with_scale(Vec3::splat(128.)),
    ));
}
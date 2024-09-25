use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add camera.
    commands.spawn(Camera2dBundle::default());
    // then spawn Sprite.
    let img1: Handle<Image> = asset_server.load("img1.png");
    commands.spawn(SpriteBundle {
        texture: img1.clone(),
        transform: Transform {
            translation: Vec3 {
                z: 1.,
                x: 100.,
                ..default()
            },
            ..default()
        },
        ..default()
    });
    // add another Sprite with img2.png
    let img2: Handle<Image> = asset_server.load("img2.png");
    commands.spawn(SpriteBundle {
        texture: img2.clone(),
        ..default()
    });
}
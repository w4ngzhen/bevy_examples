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
    let icon: Handle<Image> = asset_server.load("icon.png");
    commands.spawn(SpriteBundle {
        texture: icon.clone(),
        ..default()
    });
}
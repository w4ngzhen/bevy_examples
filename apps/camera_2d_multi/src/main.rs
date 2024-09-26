use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::render::view::RenderLayers;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}


fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // add camera.
    commands.spawn(
        (
            RenderLayers::layer(1),
            Camera2dBundle {
                camera: Camera {
                    order: 1,
                    viewport: Some(Viewport {
                        physical_position: UVec2::new(50, 50),
                        physical_size: UVec2::new(400, 200),
                        ..default()
                    }),
                    ..default()
                },
                ..default()
            }
        )
    );
    // then spawn Sprite.
    let img1: Handle<Image> = asset_server.load("img1.png");
    commands.spawn((
        RenderLayers::layer(1),
        SpriteBundle {
            texture: img1.clone(),
            ..default()
        }));
    // layer 2 camera and img2.
    commands.spawn((
        RenderLayers::layer(2),
        Camera2dBundle {
            camera: Camera {
                order: 2,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(500, 300),
                    physical_size: UVec2::new(400, 200),
                    ..default()
                }),
                ..default()
            },
            ..default()
        })
    );
    // add another Sprite with img2.png
    let img2: Handle<Image> = asset_server.load("img2.png");
    commands.spawn((RenderLayers::layer(2), SpriteBundle {
        texture: img2.clone(),
        ..default()
    }));
}
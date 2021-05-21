use bevy::{pbr::AmbientLight, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tree = asset_server.load("plane.glb#Scene0");
    let tree_clone = tree.clone();
    commands
        .spawn_bundle((
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(tree);
        });

    /* commands
        .spawn_bundle((
            Transform::from_xyz(1.0, 0.0, 0.0),
            GlobalTransform::identity(),
        ))
        .with_children(|parent| {
            parent.spawn_scene(tree_clone);
        }); */

    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle::new_3d())
        .insert(FlyCamera::default());

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 3.0),
        ..Default::default()
    });
}

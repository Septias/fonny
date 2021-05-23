use bevy::{pbr::AmbientLight, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use map::{Config, TreeBuilderBuilder};
mod map;

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Config {
            tile_height: 2.0,
            tile_width: 1.0,
            trunk_ratio: 0.2,
        })
        .add_startup_system(setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let tree_builder = TreeBuilderBuilder::new().build(&mut meshes, &mut materials);

    for x in -50..50 {
        for z in -50..50 {
            if x % 2 == 0 && z % 2 == 0 {
                tree_builder.build_tree_at(
                    x as f32,
                    z as f32,
                    (z as f32 + 50.0) / 100.0 * 8.0,
                    &mut commands,
                );
            }
        }
    }

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 3.0),
        ..Default::default()
    });
}

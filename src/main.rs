use bevy::{pbr::AmbientLight, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use map::{generators::NoiseGenerator, WorldBuilder};
use noise::{NoiseFn, Perlin, Seedable};
mod map;

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 2.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
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
    let noise_fn = Perlin::new();
    noise_fn.set_seed(1979);
    let world_builder = WorldBuilder::new(100, 100, NoiseGenerator::new(noise_fn), 20);
    world_builder.build(&mut commands, &mut meshes, &mut materials);

    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(30.0, 30.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera::default());

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 30.0, 0.0),
        light: Light {
            intensity: 800.0,
            range: 120.0,
            ..Default::default()
        },
        ..Default::default()
    });
}

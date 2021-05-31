//! Everything about map gen
use crate::map::trees::TreeBuilderBuilder;
use bevy::prelude::*;
use trees::generators::Generator;
pub mod trees;

#[derive(Debug, Default)]
struct Tile {
    x: f32,
    z: f32,
}

pub struct WorldBuilder<T: Generator> {
    width: i32,
    depth: i32,
    generator_function: T,
    levels: u32,
}

impl<T: Generator> WorldBuilder<T> {
    pub fn new(width: i32, depth: i32, generator_function: T, levels: u32) -> Self {
        Self {
            width,
            depth,
            generator_function,
            levels,
        }
    }
    pub fn build(
        self,
        mut commands: &mut Commands,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        mut materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let tree_builder = TreeBuilderBuilder::new()
            .with_config(trees::Config {
                tile_height: 10.0,
                ..Default::default()
            })
            .with_levels(self.levels)
            .build(&mut meshes, &mut materials);

        let half_width = self.width / 2;
        let half_depth = self.depth / 2;
        for x in -half_width..half_width {
            for z in -half_depth..half_depth {
                if x % 2 == 0 && z % 2 == 0 {
                    let size = self.generator_function.compute(x, z) * self.levels as f32;
                    if size > 2.0 {
                        tree_builder.build_tree_at(x as f32, z as f32, size as f32, &mut commands);
                    }
                }
            }
        }
    }
}

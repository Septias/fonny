//! Tree generators
#![allow(dead_code)]
use noise::NoiseFn;
pub trait Generator {
    // compute the tree-height at a given position
    fn compute(&self, x: i32, z: i32) -> f32;
}

/// Generates a crater-like structure
pub struct CraterGenerator {
    /// width of map
    width: i32,
    /// depth of map
    depth: i32,
    /// Number of tree-levels
    levels: i32,
}

impl CraterGenerator {
    pub fn new(width: i32, depth: i32, levels: i32) -> Self {
        Self {
            width,
            depth,
            levels,
        }
    }
}
impl Generator for CraterGenerator {
    fn compute(&self, x: i32, z: i32) -> f32 {
        let max = ((self.width.pow(2) + self.depth.pow(2)) as f32).sqrt();
        let val = ((x.pow(2) + z.pow(2)) as f32).sqrt();
        val / max
    }
}

/// Generates a sin-wave structure
pub struct WaveGenerator {
    /// width of map
    width: i32,
    /// depth of map
    depth: i32,
    /// Number of tree-levels
    levels: i32,
    /// TODO
    frequency: f32,
}

impl Generator for WaveGenerator {
    fn compute(&self, x: i32, z: i32) -> f32 {
        let dist_to_center = ((x.pow(2) + z.pow(2)) as f32).sqrt();
        (dist_to_center * self.frequency).sin()
    }
}

impl WaveGenerator {
    pub fn new(width: i32, depth: i32, levels: i32, frequency: f32) -> Self {
        Self {
            width,
            depth,
            levels,
            frequency,
        }
    }
}
/// Generates a donut-like structure
pub struct DonutGenerator {
    /// width of map
    width: i32,
    /// depth of map
    depth: i32,
    /// radius at which the donut has it's peak
    radius: f32,
}

impl Generator for DonutGenerator {
    fn compute(&self, x: i32, z: i32) -> f32 {
        let val = ((x.pow(2) + z.pow(2)) as f32).sqrt();
        1.0 / ((val - self.radius).abs() / 3.0).max(1.0)
    }
}

impl DonutGenerator {
    pub fn new(width: i32, depth: i32, radius: f32) -> Self {
        Self {
            width,
            depth,
            radius,
        }
    }
}

/// Generates trees with a given [NoiseFn]
pub struct NoiseGenerator<T>
where
    T: NoiseFn<[f64; 2]>,
{
    function: T,
}

impl<T> Generator for NoiseGenerator<T>
where
    T: NoiseFn<[f64; 2]>,
{
    fn compute(&self, x: i32, z: i32) -> f32 {
        self.function
            .get([(x + 50) as f64 / 20.1, (z + 50) as f64 / 20.1]) as f32
    }
}

impl<T: NoiseFn<[f64; 2]>> NoiseGenerator<T> {
    pub fn new(function: T) -> Self {
        Self { function }
    }
}

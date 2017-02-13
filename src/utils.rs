pub fn scale(x: f32, source: (f32, f32), target: (f32, f32)) -> f32 {
    (target.1 - target.0) as f32 * (x - source.0) / (source.1 - source.0) + target.0 as f32
}

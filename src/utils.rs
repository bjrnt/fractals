pub fn scale(x: f32, interval: (f32, f32), allowed: (u32, u32)) -> f32 {
    (allowed.1 - allowed.0) as f32 * (x - interval.0) / (interval.1 - interval.0) + allowed.0 as f32
}
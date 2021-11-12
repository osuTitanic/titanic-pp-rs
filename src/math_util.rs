#[cfg(feature = "sliders")]
#[inline]
pub(crate) fn is_linear(
    p0: crate::parse::Pos2,
    p1: crate::parse::Pos2,
    p2: crate::parse::Pos2,
) -> bool {
    ((p1.x - p0.x) * (p2.y - p0.y) - (p1.y - p0.y) * (p2.x - p0.x)).abs() <= f32::EPSILON
}

#[cfg(feature = "osu")]
#[inline]
pub(crate) fn lerp(start: f64, end: f64, percent: f64) -> f64 {
    start + (end - start) * percent
}

pub const WIDTH: f32 = 432.;
pub const HEIGHT: f32 = 768.;
pub const RADIUS: f32 = 32.;
pub const SHADOW_MARGIN: f32 = 8.;
pub const WINDOW_WIDTH: f32 = WIDTH + SHADOW_MARGIN * 2.;
pub const WINDOW_HEIGHT: f32 = HEIGHT + SHADOW_MARGIN * 2.;

pub fn contains(x: f32, y: f32) -> bool {
    let x = x - SHADOW_MARGIN;
    let y = y - SHADOW_MARGIN;
    let dx = x - x.clamp(RADIUS, WIDTH - RADIUS);
    let dy = y - y.clamp(RADIUS, HEIGHT - RADIUS);
    (0. ..=WIDTH).contains(&x)
        && (0. ..=HEIGHT).contains(&y)
        && dx * dx + dy * dy <= RADIUS * RADIUS
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn corners_and_shadow_are_outside_but_edges_and_center_are_inside() {
        assert!(!contains(0., WINDOW_HEIGHT / 2.));
        assert!(!contains(SHADOW_MARGIN, SHADOW_MARGIN));
        assert!(contains(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2.));
        assert!(contains(SHADOW_MARGIN, WINDOW_HEIGHT / 2.));
        assert!(contains(WINDOW_WIDTH - SHADOW_MARGIN, WINDOW_HEIGHT / 2.));
    }
}

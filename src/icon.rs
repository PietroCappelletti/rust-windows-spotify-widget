/// Generates an RGBA pixel buffer for a simple, original app icon: a
/// rounded square with a teal-to-purple gradient and a white "play"
/// triangle centered on top. Drawn procedurally so the project doesn't
/// depend on an external image asset or risk reusing anyone else's logo.
pub fn generate_icon_rgba(size: u32) -> Vec<u8> {
    let size_f = size as f32;
    let radius = size_f * 0.22;
    let mut buf = vec![0u8; (size * size * 4) as usize];

    // Play triangle geometry, centered (nudged slightly right for
    // optical balance, since a symmetric triangle looks off-center).
    let tri_h = size_f * 0.42;
    let tri_w = size_f * 0.36;
    let cx = size_f / 2.0 + size_f * 0.03;
    let cy = size_f / 2.0;
    let p0 = (cx - tri_w / 2.0, cy - tri_h / 2.0);
    let p1 = (cx - tri_w / 2.0, cy + tri_h / 2.0);
    let p2 = (cx + tri_w / 2.0, cy);

    for y in 0..size {
        for x in 0..size {
            let idx = ((y * size + x) * 4) as usize;
            let fx = x as f32 + 0.5;
            let fy = y as f32 + 0.5;

            let bg_alpha = rounded_rect_coverage(fx, fy, size_f, radius);
            if bg_alpha <= 0.0 {
                continue; // fully transparent outside the rounded square
            }

            // Vertical gradient background: teal at top, purple at bottom.
            let t = fy / size_f;
            let (r, g, b) = if point_in_triangle((fx, fy), p0, p1, p2) {
                (255.0, 255.0, 255.0)
            } else {
                (
                    lerp(29.0, 98.0, t),
                    lerp(185.0, 60.0, t),
                    lerp(175.0, 190.0, t),
                )
            };

            buf[idx] = r as u8;
            buf[idx + 1] = g as u8;
            buf[idx + 2] = b as u8;
            buf[idx + 3] = (bg_alpha * 255.0) as u8;
        }
    }

    buf
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Returns 0.0..=1.0 coverage of a rounded square with the given corner
/// radius, with a roughly 1px antialiased edge so corners aren't jagged.
fn rounded_rect_coverage(x: f32, y: f32, size: f32, radius: f32) -> f32 {
    let dx = (x - size / 2.0).abs() - (size / 2.0 - radius);
    let dy = (y - size / 2.0).abs() - (size / 2.0 - radius);

    if dx <= 0.0 || dy <= 0.0 {
        return 1.0; // inside the straight edges/center, not near a corner
    }

    let dist = (dx * dx + dy * dy).sqrt();
    if dist <= radius - 0.5 {
        1.0
    } else if dist >= radius + 0.5 {
        0.0
    } else {
        radius + 0.5 - dist
    }
}

fn point_in_triangle(p: (f32, f32), a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
    let sign = |p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)| {
        (p1.0 - p3.0) * (p2.1 - p3.1) - (p2.0 - p3.0) * (p1.1 - p3.1)
    };

    let d1 = sign(p, a, b);
    let d2 = sign(p, b, c);
    let d3 = sign(p, c, a);

    let has_neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let has_pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;

    !(has_neg && has_pos)
}
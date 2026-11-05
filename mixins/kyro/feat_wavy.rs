// Capes as cloth, from WavyCapes.java: strips on springs, each chasing the one above.
fn kyro_feat_wavy(
    m: &Mob,
    alpha: f32,
    base: glam::Mat4,
    uv: [f32; 4],
    light: u32,
    data: &MobData,
    out: &mut Vec<Vertex>,
) -> core::ops::ControlFlow<()> {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::time::Instant;
    const POINTS: usize = 12;
    const WIDTH: f32 = 10.0;
    const HEIGHT: f32 = 16.0;
    struct Cloth {
        angle: [f32; POINTS],
        speed: [f32; POINTS],
        seen: Instant,
    }
    thread_local! {
        static CLOTHS: RefCell<HashMap<u32, Cloth>> = RefCell::new(HashMap::new());
    }
    /// The wave's extra angle at a strip: a ripple running down the cape, growing toward the hem.
    fn ripple(i: usize, time: f32, motion: f32, strength: f32) -> f32 {
        let along = (i + 1) as f32 / POINTS as f32;
        let amplitude = (0.06 + 0.16 * motion) * strength;
        amplitude * along * ((time * (0.18 + 0.22 * motion) - along * 7.0).sin() + 0.6)
    }
    if !crate::options_ui::kyro_ring::on("wavy_capes") {
        return core::ops::ControlFlow::Continue(());
    }
    let Some(model) = m.kind.layer("player#cape") else {
        return core::ops::ControlFlow::Continue(());
    };
    let s = anim::state(m, alpha);
    let mut poses = Poses::new(model);
    anim::setup(m, &s, &mut poses, &data.animations);
    let Some(body_matrix) = part_matrix(model, base, &poses, "body", 0.0) else {
        return core::ops::ControlFlow::Continue(());
    };
    let a = f64::from(alpha);
    let at = [
        m.previous.x + (m.position.x - m.previous.x) * a,
        m.previous.y + (m.position.y - m.previous.y) * a,
        m.previous.z + (m.position.z - m.previous.z) * a,
    ];
    let body = m.previous_body_yaw + wrap(m.body_yaw - m.previous_body_yaw) * alpha;
    let ticks = m.extra.fall_fly_ticks as f32 + alpha;
    let gliding = (ticks * ticks / 100.0).clamp(0.0, 1.0);
    let [flap, lean, _] = m.extra.avatar.cape(alpha, at, body, gliding);
    let top = (6.0 + lean / 2.0 + flap).clamp(0.0, 100.0).to_radians();
    let motion = (flap / 25.0).clamp(0.0, 1.0);
    let strength = crate::options_ui::kyro_ring::value("wavy_capes", "wave") as f32 / 100.0;
    let time = s.age;
    let angles = CLOTHS.with_borrow_mut(|cloths| {
        if cloths.len() > 128 {
            cloths.clear();
        }
        let now = Instant::now();
        let cloth = cloths.entry(m.id).or_insert(Cloth { angle: [top; POINTS], speed: [0.0; POINTS], seen: now });
        let mut elapsed = now.duration_since(cloth.seen).as_secs_f32();
        cloth.seen = now;
        if elapsed > 0.5 {
            // Not seen for a while: start at rest, no fling.
            cloth.angle = [top; POINTS];
            cloth.speed = [0.0; POINTS];
            elapsed = 0.0;
        }
        // Small fixed steps keep the springs steady at any frame rate.
        let steps = ((elapsed / (1.0 / 120.0)).ceil() as usize).min(8);
        let dt = if steps == 0 { 0.0 } else { elapsed / steps as f32 };
        for _ in 0..steps {
            for i in 0..POINTS {
                let above = if i == 0 { top } else { cloth.angle[i - 1] };
                let goal = above + ripple(i, time, motion, strength) - if i == 0 { 0.0 } else { ripple(i - 1, time, motion, strength) };
                // Softer toward the hem, so the bottom trails the top.
                let stiffness = 260.0 - 150.0 * i as f32 / POINTS as f32;
                let damping = 2.0 * stiffness.sqrt() * 0.75;
                cloth.speed[i] += ((goal - cloth.angle[i]) * stiffness - cloth.speed[i] * damping) * dt;
                cloth.angle[i] += cloth.speed[i] * dt;
                // Never into the body, never folded over the strip above, never past flat out.
                let low = if i == 0 { 0.0 } else { (cloth.angle[i - 1] - 0.35).max(0.0) };
                let high = 100f32.to_radians().min(above + 0.45);
                if cloth.angle[i] < low || cloth.angle[i] > high {
                    cloth.angle[i] = cloth.angle[i].clamp(low, high);
                    cloth.speed[i] = 0.0;
                }
            }
        }
        cloth.angle
    });
    // Where the cape hangs: 2 pixels behind the body's centre, turned to face backward.
    let t = body_matrix * glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, 2.0 / 16.0)) * glam::Mat4::from_rotation_y(PI);
    let turn = glam::Mat3::from_mat4(t);
    let mut ys = [0f32; POINTS + 1];
    let mut zs = [0f32; POINTS + 1];
    for i in 0..POINTS {
        ys[i + 1] = ys[i] + angles[i].cos() * HEIGHT / POINTS as f32;
        zs[i + 1] = zs[i] - angles[i].sin() * HEIGHT / POINTS as f32;
    }
    let mut quad = |corners: [[f32; 5]; 4], normal: [f32; 3]| {
        let n = (turn * glam::Vec3::from_array(normal)).normalize_or_zero().to_array().map(|v| (v.clamp(-1.0, 1.0) * 127.0) as i8);
        for [x, y, z, u, v] in corners {
            out.push(Vertex {
                normal: [n[0], n[1], n[2], -127],
                pos: t.transform_point3(glam::Vec3::new(x, y, z)).to_array(),
                uv: [uv[0] + (uv[2] - uv[0]) * u, uv[1] + (uv[3] - uv[1]) * v],
                layer: 0,
                color: 0xFFFF_FFFF,
                light,
                glint_uv: [0.0, 0.0],
            });
        }
    };
    let (x0, x1) = (-WIDTH / 2.0 / 16.0, WIDTH / 2.0 / 16.0);
    let (u, w) = (1.0 / 64.0, 1.0 / 32.0);
    for i in 0..POINTS {
        let (y0, y1) = (ys[i] / 16.0, ys[i + 1] / 16.0);
        let (back0, back1) = ((zs[i] - 1.0) / 16.0, (zs[i + 1] - 1.0) / 16.0);
        let (front0, front1) = (zs[i] / 16.0, zs[i + 1] / 16.0);
        let v0 = (1.0 + HEIGHT * i as f32 / POINTS as f32) / 32.0;
        let v1 = (1.0 + HEIGHT * (i + 1) as f32 / POINTS as f32) / 32.0;
        // The outer face (the cape's art, texture 1..11) and the inner face (12..22).
        quad([[x1, y0, back0, u, v0], [x0, y0, back0, 11.0 * u, v0], [x0, y1, back1, 11.0 * u, v1], [x1, y1, back1, u, v1]], [0.0, 0.0, -1.0]);
        quad([[x0, y0, front0, 12.0 * u, v0], [x1, y0, front0, 22.0 * u, v0], [x1, y1, front1, 22.0 * u, v1], [x0, y1, front1, 12.0 * u, v1]], [0.0, 0.0, 1.0]);
        // Edges: left (texture 0..1) and right (11..12).
        quad([[x0, y0, back0, 0.0, v0], [x0, y0, front0, u, v0], [x0, y1, front1, u, v1], [x0, y1, back1, 0.0, v1]], [-1.0, 0.0, 0.0]);
        quad([[x1, y0, front0, 11.0 * u, v0], [x1, y0, back0, 12.0 * u, v0], [x1, y1, back1, 12.0 * u, v1], [x1, y1, front1, 11.0 * u, v1]], [1.0, 0.0, 0.0]);
    }
    // Top and bottom edges.
    let (top0, end, y_end) = (zs[0] / 16.0, zs[POINTS] / 16.0, ys[POINTS] / 16.0);
    let thick = 1.0 / 16.0;
    quad([[x0, 0.0, top0 - thick, u, 0.0], [x1, 0.0, top0 - thick, 11.0 * u, 0.0], [x1, 0.0, top0, 11.0 * u, w], [x0, 0.0, top0, u, w]], [0.0, -1.0, 0.0]);
    quad([[x0, y_end, end, 11.0 * u, 0.0], [x1, y_end, end, 21.0 * u, 0.0], [x1, y_end, end - thick, 21.0 * u, w], [x0, y_end, end - thick, 11.0 * u, w]], [0.0, 1.0, 0.0]);
    core::ops::ControlFlow::Break(())
}

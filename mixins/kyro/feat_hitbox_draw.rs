// Hitboxes, from ReachHitbox.java: your color, red once your eyes are within reach of the box.
fn kyro_feat_hitbox_draw(
    gizmos: &mut Gizmos,
    min: [f64; 3],
    max: [f64; 3],
    position: [f64; 3],
    eye_height: f32,
    living: bool,
    view: (f32, f32),
) -> core::ops::ControlFlow<()> {
    use crate::options_ui::kyro_ring as ring;
    if !ring::on("reach_hitbox") {
        return core::ops::ControlFlow::Continue(());
    }
    // EntityType.ITEM: 0.25 wide, eyes at 0.2125. Its own switch, past the two filters below.
    let item = (max[0] - min[0] - 0.25).abs() < 0.001 && (eye_height - 0.2125).abs() < 0.001;
    if item {
        if ring::value("reach_hitbox", "items") != 1 {
            return core::ops::ControlFlow::Break(());
        }
    } else {
        if !living && ring::value("reach_hitbox", "living") == 1 {
            return core::ops::ControlFlow::Break(());
        }
        if !crate::options_ui::kyro_world::player_at(position) && ring::value("reach_hitbox", "players") == 1 {
            return core::ops::ControlFlow::Break(());
        }
    }
    let eye = crate::options_ui::kyro_world::eye();
    let near: f64 = (0..3).map(|i| (min[i] - eye[i]).max(eye[i] - max[i]).max(0.0).powi(2)).sum();
    let colored = ring::value("reach_hitbox", "reach") == 1;
    let color = if colored && near > 0.0 && near.sqrt() <= crate::options_ui::kyro_world::reach() {
        0xff000000 | ring::color_of(ring::value("reach_hitbox", "reach_color"))
    } else {
        (if colored { 0x70000000 } else { 0xff000000 }) | ring::color_of(ring::value("reach_hitbox", "color"))
    };
    let width = STROKE_WIDTH * ring::value("reach_hitbox", "thickness") as f32 / 100.0;
    match ring::value("reach_hitbox", "box_style") {
        1 => {
            // A flat outline through the box's middle, always turned to face you.
            let centre = [(min[0] + max[0]) / 2.0, (min[2] + max[2]) / 2.0];
            let toward = [centre[0] - eye[0], centre[1] - eye[2]];
            let length = (toward[0] * toward[0] + toward[1] * toward[1]).sqrt();
            let side = if length < 1e-3 { [1.0, 0.0] } else { [-toward[1] / length, toward[0] / length] };
            let half = (max[0] - min[0]) / 2.0;
            let at = |s: f64, y: f64| [centre[0] + side[0] * half * s, y, centre[1] + side[1] * half * s];
            let corners = [at(-1.0, min[1]), at(1.0, min[1]), at(1.0, max[1]), at(-1.0, max[1])];
            for i in 0..4 {
                gizmos.line(corners[i], corners[(i + 1) % 4], color, width);
            }
        }
        2 => {
            // Only the corners: a short piece of each edge at every corner.
            let reach = [(max[0] - min[0]) / 4.0, (max[1] - min[1]) / 6.0, (max[2] - min[2]) / 4.0];
            for i in 0..8 {
                let low = [i & 1 == 0, i & 2 == 0, i & 4 == 0];
                let corner = [0, 1, 2].map(|a| if low[a] { min[a] } else { max[a] });
                for axis in 0..3 {
                    let mut end = corner;
                    end[axis] += if low[axis] { reach[axis] } else { -reach[axis] };
                    gizmos.line(corner, end, color, width);
                }
            }
        }
        3 => {
            // Gizmos are lines: faint stripes across the sides stand in for a fill.
            gizmos.cuboid(min, max, color, width);
            let faint = color & 0x00ff_ffff | 0x3000_0000;
            for i in 1..10 {
                let y = min[1] + (max[1] - min[1]) * f64::from(i) / 10.0;
                gizmos.cuboid([min[0], y, min[2]], [max[0], y, max[2]], faint, width);
            }
        }
        _ => gizmos.cuboid(min, max, color, width),
    }
    if ring::value("reach_hitbox", "look") == 1 {
        let from = [position[0], position[1] + f64::from(eye_height), position[2]];
        let v = view_vector(view.0, view.1);
        gizmos.arrow(from, [from[0] + v[0] * 2.0, from[1] + v[1] * 2.0, from[2] + v[2] * 2.0], BLUE, STROKE_WIDTH);
    }
    core::ops::ControlFlow::Break(())
}

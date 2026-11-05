// 3D skin layers, from SkinLayers3D.java: the outer layer as raised pixels. A block per texel of
// each layer; the entity shader drops the see-through ones, so no skin pixels need reading.
fn kyro_feat_skin(m: &Mob, alpha: f32, base: glam::Mat4, atlas: Atlas, light: u32, data: &MobData, body_visible: bool, out: &mut Vec<Vertex>) {
    use crate::options_ui::kyro_ring as ring;
    const OUT: f32 = 0.6;
    if !ring::on("skin_layers_3d") || !avatar(m) || !body_visible || m.extra.spectator {
        return;
    }
    let far = ring::value("skin_layers_3d", "distance") as f32;
    let distance = base.w_axis.truncate().length();
    if distance > far {
        return;
    }
    // Further off, the walls between raised pixels are too small to see.
    let close = distance < 6.0;
    let (model_key, texture) = body(m);
    let Some(uv) = atlas(&texture).or_else(|| atlas(&m.kind.texture)) else {
        return;
    };
    let model = m.kind.layer(&model_key).cloned().unwrap_or_else(|| m.kind.model.clone());
    let s = anim::state(m, alpha);
    let mut poses = Poses::new(&model);
    anim::setup(m, &s, &mut poses, &data.animations);
    ring::set_raised();
    let arm = if model_key.contains("slim") { 3.0 } else { 4.0 };
    let overlay = u32::from(red_overlay(m));
    let color = body_color(m, alpha);
    // Part, the bit that hides it, box corner, box size, and its place on the skin.
    let parts: [(&str, u32, [f32; 3], [i32; 3], [i32; 2]); 6] = [
        ("hat", 6, [-4.0, -8.0, -4.0], [8, 8, 8], [32, 0]),
        ("jacket", 1, [-4.0, 0.0, -2.0], [8, 12, 4], [16, 32]),
        ("right_sleeve", 3, [1.0 - arm, -2.0, -2.0], [arm as i32, 12, 4], [40, 32]),
        ("left_sleeve", 2, [-1.0, -2.0, -2.0], [arm as i32, 12, 4], [48, 48]),
        ("right_pants", 5, [-2.0, 0.0, -2.0], [4, 12, 4], [0, 32]),
        ("left_pants", 4, [-2.0, 0.0, -2.0], [4, 12, 4], [0, 48]),
    ];
    for (name, bit, [x0, y0, z0], [w, h, d], [u, v]) in parts {
        if m.extra.hidden_model_parts & (1 << bit) != 0 {
            continue;
        }
        let Some(matrix) = part_matrix(&model, base, &poses, name, 0.0) else {
            continue;
        };
        let turn = glam::Mat3::from_mat4(matrix);
        let (x1, y1, z1) = (x0 + w as f32, y0 + h as f32, z0 + d as f32);
        // Columns, rows, texel start, the cell at (a, b), and the face's direction.
        type Cell = Box<dyn Fn(f32, f32) -> [f32; 3]>;
        let faces: [(i32, i32, i32, i32, Cell, [f32; 3]); 6] = [
            (w, h, u + d, v + d, Box::new(move |a, b| [x1 - a - 1.0, y0 + b, z0]), [0.0, 0.0, -1.0]),
            (w, h, u + 2 * d + w, v + d, Box::new(move |a, b| [x0 + a, y0 + b, z1 - 1.0]), [0.0, 0.0, 1.0]),
            (d, h, u, v + d, Box::new(move |a, b| [x0, y0 + b, z1 - a - 1.0]), [-1.0, 0.0, 0.0]),
            (d, h, u + d + w, v + d, Box::new(move |a, b| [x1 - 1.0, y0 + b, z0 + a]), [1.0, 0.0, 0.0]),
            (w, d, u + d, v, Box::new(move |a, b| [x0 + a, y0, z1 - b - 1.0]), [0.0, -1.0, 0.0]),
            (w, d, u + d + w, v, Box::new(move |a, b| [x0 + a, y1 - 1.0, z1 - b - 1.0]), [0.0, 1.0, 0.0]),
        ];
        for (columns, rows, fu, fv, cell, normal) in faces {
            // A side turned over 45 degrees away is out of sight, raised edge and all.
            let middle = matrix.transform_point3(glam::Vec3::from_array(cell(columns as f32 / 2.0, rows as f32 / 2.0)) / 16.0);
            if (turn * glam::Vec3::from_array(normal)).normalize_or_zero().dot(middle.normalize_or_zero()) > 0.7 {
                continue;
            }
            for a in 0..columns {
                for b in 0..rows {
                    let rim = a == 0 || b == 0 || a == columns - 1 || b == rows - 1;
                    let c = cell(a as f32, b as f32);
                    let mut min = c;
                    let mut max = [c[0] + 1.0, c[1] + 1.0, c[2] + 1.0];
                    for axis in 0..3 {
                        if normal[axis] < 0.0 {
                            max[axis] = c[axis];
                            min[axis] = c[axis] - OUT;
                        } else if normal[axis] > 0.0 {
                            min[axis] = c[axis] + 1.0;
                            max[axis] = c[axis] + 1.0 + OUT;
                        }
                    }
                    let (u0, v0) = ((fu + a) as f32 / 64.0, (fv + b) as f32 / 64.0);
                    let (u1, v1) = (u0 + 1.0 / 64.0, v0 + 1.0 / 64.0);
                    for axis in 0..3 {
                        for sign in [-1.0_f32, 1.0] {
                            if normal[axis] != 0.0 && sign != normal[axis] {
                                continue; // The side against the body never shows.
                            }
                            if normal[axis] == 0.0 && !close && !rim {
                                continue;
                            }
                            let (s_axis, t_axis) = ((axis + 1) % 3, (axis + 2) % 3);
                            let level = if sign > 0.0 { max[axis] } else { min[axis] };
                            let corners = [[min[s_axis], min[t_axis]], [max[s_axis], min[t_axis]], [max[s_axis], max[t_axis]], [min[s_axis], max[t_axis]]];
                            let texels = [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
                            let mut n = [0.0_f32; 3];
                            n[axis] = sign;
                            let packed = (turn * glam::Vec3::from_array(n)).normalize_or_zero().to_array().map(|v| (v.clamp(-1.0, 1.0) * 127.0) as i8);
                            for k in 0..4 {
                                let i = if sign > 0.0 { k } else { 3 - k };
                                let mut p = [0.0_f32; 3];
                                p[axis] = level;
                                p[s_axis] = corners[i][0];
                                p[t_axis] = corners[i][1];
                                out.push(Vertex {
                                    normal: [packed[0], packed[1], packed[2], -127],
                                    pos: matrix.transform_point3(glam::Vec3::from_array(p) / 16.0).to_array(),
                                    uv: [uv[0] + (uv[2] - uv[0]) * texels[i][0], uv[1] + (uv[3] - uv[1]) * texels[i][1]],
                                    layer: overlay,
                                    color,
                                    light,
                                    glint_uv: [0.0, 0.0],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

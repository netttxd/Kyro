// Low obstruction: a lower, fainter first-person fire overlay. Fire is the only quad with alpha 0xE5.
fn kyro_feat_fire(mut value: Vec<ScreenQuad>) -> Vec<ScreenQuad> {
    use crate::options_ui::kyro_ring as ring;
    if !ring::on("low_obstruction") {
        return value;
    }
    let drop = [0.0_f32, 0.2, 0.35][ring::value("low_obstruction", "fire").clamp(0, 2) as usize];
    let opacity = ring::value("low_obstruction", "fire_opacity") as f32 / 100.0;
    // The hand projection's clip-space height at depth 0.5, as hand_scale has it.
    let clip = 1.0 / (0.5 * 35.0_f32.to_radians().tan());
    for quad in &mut value {
        if (quad.vertices[0].color[3] - 229.0 / 255.0).abs() < 0.001 {
            for vertex in &mut quad.vertices {
                vertex.pos[1] -= drop * clip;
                vertex.color[3] *= opacity;
            }
        }
    }
    value
}

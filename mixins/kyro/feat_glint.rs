// Glint color: the glint textures recolored, brightness kept. Loaded with resources, so it shows next launch or reload.
fn kyro_feat_glint(mut value: (Vec<u8>, u32, u32), name: &str) -> (Vec<u8>, u32, u32) {
    use crate::options_ui::kyro_ring as ring;
    ring::ensure_loaded();
    if !ring::on("glint_color") {
        return value;
    }
    let own_armor = name.contains("armor") && ring::value("glint_color", "armor") == 1;
    let tint = ring::rgb(ring::color_of(ring::value("glint_color", if own_armor { "armor_color" } else { "color" })));
    let style = ring::value("glint_color", "glint_style");
    let light: Vec<f32> = value.0.chunks_exact(4).map(|p| f32::from(p[0].max(p[1]).max(p[2])) / 255.0).collect();
    for (pixel, light) in value.0.chunks_exact_mut(4).zip(light) {
        let light = match style {
            1 => light.sqrt() * 0.85,
            2 => 0.55,
            _ => light,
        };
        if style == 2 {
            pixel[3] = 255;
        }
        for channel in 0..3 {
            // The shaders square the glint, so the root comes out as the color you picked.
            pixel[channel] = ((tint[channel] as f32 / 255.0 * light).sqrt() * 255.0).min(255.0) as u8;
        }
    }
    value
}

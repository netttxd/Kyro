// Glint color: the glint textures recolored, brightness kept. Loaded with resources, so it shows next launch or reload.
fn kyro_feat_glint(mut value: (Vec<u8>, u32, u32), name: &str) -> (Vec<u8>, u32, u32) {
    let Some(dir) = std::env::current_dir().ok().map(|d| d.join("horizon-mods").join(".horizon-mod-data").join("kyro")) else {
        return value;
    };
    let read = |key: &str| std::fs::read_to_string(dir.join(key)).ok().map(|v| v.trim().to_owned());
    if read("on_glint_color").as_deref() != Some("true") {
        return value;
    }
    let own_armor = name.contains("armor") && read("set_glint_color_armor").as_deref() == Some("1");
    let index = read(if own_armor { "set_glint_color_armor_color" } else { "set_glint_color_color" }).and_then(|v| v.parse::<i32>().ok()).unwrap_or(1);
    let color = crate::options_ui::kyro_ring::color_of(index);
    let tint = [(color >> 16) & 255, (color >> 8) & 255, color & 255];
    let style = read("set_glint_color_glint_style").and_then(|v| v.parse::<u32>().ok()).unwrap_or(0);
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

// Hit color: the hurt flash, baked into the shader, so it is read from the saved setting at launch.
fn kyro_feat_hurt(value: String) -> String {
    let Some(dir) = std::env::current_dir().ok().map(|d| d.join("horizon-mods").join(".horizon-mod-data").join("kyro")) else {
        return value;
    };
    let read = |key: &str| std::fs::read_to_string(dir.join(key)).ok().map(|v| v.trim().to_owned());
    if read("on_hit_color").as_deref() != Some("true") {
        return value;
    }
    let index = read("set_hit_color_color").and_then(|v| v.parse::<i32>().ok()).unwrap_or(6);
    let color = crate::options_ui::kyro_ring::color_of(index);
    let line = format!(
        "const HURT_OVERLAY = vec4({}.0 / 255.0, {}.0 / 255.0, {}.0 / 255.0, 178.0 / 255.0);",
        (color >> 16) & 255,
        (color >> 8) & 255,
        color & 255
    );
    value.lines().map(|l| if l.starts_with("const HURT_OVERLAY") { line.as_str() } else { l }).collect::<Vec<_>>().join("\n") + "\n"
}

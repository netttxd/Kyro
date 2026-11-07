// Hit color: the hurt flash, baked into the shader, so it is read from the saved setting at launch.
fn kyro_feat_hurt(value: String) -> String {
    use crate::options_ui::kyro_ring as ring;
    ring::ensure_loaded();
    if !ring::on("hit_color") {
        return value;
    }
    let [r, g, b] = ring::rgb(ring::color_of(ring::value("hit_color", "color")));
    let line = format!("const HURT_OVERLAY = vec4({r}.0 / 255.0, {g}.0 / 255.0, {b}.0 / 255.0, 178.0 / 255.0);");
    value.lines().map(|l| if l.starts_with("const HURT_OVERLAY") { line.as_str() } else { l }).collect::<Vec<_>>().join("\n") + "\n"
}

// Block outline: thicker or thinner lines. The color is painted inside a closure, out of reach of a hook.
fn kyro_feat_outline(value: f32) -> f32 {
    use crate::options_ui::kyro_ring as ring;
    if ring::on("block_outline") { value * ring::value("block_outline", "thickness") as f32 / 100.0 } else { value }
}

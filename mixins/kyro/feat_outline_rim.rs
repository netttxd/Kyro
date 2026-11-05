// The rim around the block you look at.
fn kyro_feat_outline_rim(rims: &mut Vec<crate::game::native_hooks::NativeBlockRim>) {
    use horizon_client::options_ui::kyro_ring as ring;
    if let Some(pos) = crate::game::kyro_feat::take_outlined() {
        let alpha = (ring::value("block_outline", "opacity") as f32 * 2.55) as u32;
        let width = 2.5 * ring::value("block_outline", "thickness") as f32 / 100.0;
        rims.push(crate::game::native_hooks::NativeBlockRim { pos, argb: alpha << 24 | ring::color_of(ring::value("block_outline", "color")), width });
    }
}

// Shulker preview: room for a 9x3 grid in the tooltip of a filled shulker box.
fn kyro_feat_shulker(stack: &ItemStack, size: &mut Option<[f32; 2]>) {
    if horizon_client::options_ui::kyro_ring::on("shulker_preview")
        && stack.item.ends_with("shulker_box")
        && stack.extra.as_ref().is_some_and(|e| !e.container.is_empty())
    {
        *size = Some([162.0, 58.0]);
    }
}

// The grid replaces the text list of contents: only the name stays.
fn kyro_feat_shulker_lines(stack: &ItemStack, lines: &mut Vec<(Text, u32)>) {
    if horizon_client::options_ui::kyro_ring::on("shulker_preview")
        && stack.item.ends_with("shulker_box")
        && stack.extra.as_ref().is_some_and(|e| !e.container.is_empty())
    {
        lines.truncate(1);
    }
}

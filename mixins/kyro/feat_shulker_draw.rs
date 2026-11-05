// The grid itself, from ShulkerPreview.java.
fn kyro_feat_shulker_draw(game: &Game<'_>, stack: &ItemStack, batch: &mut HudBatch, origin: [f32; 2]) {
    let Some(extra) = stack.extra.as_ref().filter(|_| stack.item.ends_with("shulker_box")) else {
        return;
    };
    for index in 0..27 {
        let (x, y) = (origin[0] + (index % 9) as f32 * 18.0, origin[1] + (index / 9) as f32 * 18.0);
        game.native_fill(batch, [x, y, 17.0, 17.0], 0x50000000);
    }
    for (slot, item) in &extra.container {
        let (x, y) = (origin[0] + (*slot % 9) as f32 * 18.0, origin[1] + (*slot / 9) as f32 * 18.0);
        game.draw_item_model(batch, item, x + 1.0, y + 1.0);
        if item.count > 1 {
            let label = item.count.to_string();
            game.native_text(batch, &label, [x + 18.0 - game.native_text_width(&label), y + 10.0], 0xffffff, true);
        }
    }
}

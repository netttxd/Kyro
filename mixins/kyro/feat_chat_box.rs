// The search box, top right of the chat screen.
fn kyro_feat_chat_box(game: &crate::game::Game<'_>, batch: &mut HudBatch) {
    use horizon_client::options_ui::kyro_ring as ring;
    if !ring::chat_search_on() {
        return;
    }
    let (x, query, focus) = (batch.width - 134.0, ring::chat_query(), ring::chat_focus());
    game.native_fill(batch, [x, 4.0, 130.0, 14.0], 0xff000000);
    game.native_fill(batch, [x + 1.0, 5.0, 128.0, 12.0], 0xff0a1038);
    if query.is_empty() && !focus {
        game.native_text(batch, "Search", [x + 4.0, 7.0], 0x7884b8, false);
    } else {
        let shown = if focus { format!("{query}_") } else { query };
        game.native_text(batch, &shown, [x + 4.0, 7.0], 0xffffff, false);
    }
}

// The search box, top right of the chat screen.
fn kyro_feat_chat_box(game: &crate::game::Game<'_>, batch: &mut HudBatch) {
    if !horizon_client::options_ui::kyro_world::chat_search_on() {
        return;
    }
    let [x, y, w, h] = horizon_client::options_ui::kyro_world::chat_search_box(batch.width);
    let (query, focus) = (horizon_client::options_ui::kyro_world::chat_query(), horizon_client::options_ui::kyro_world::chat_focus());
    game.native_fill(batch, [x, y, w, h], 0xff000000);
    game.native_fill(batch, [x + 1.0, y + 1.0, w - 2.0, h - 2.0], 0xff0a1038);
    if query.is_empty() && !focus {
        game.native_text(batch, "Search", [x + 4.0, y + 3.0], 0x7884b8, false);
    } else {
        let shown = if focus { format!("{query}_") } else { query };
        game.native_text(batch, &shown, [x + 4.0, y + 3.0], 0xffffff, false);
    }
}

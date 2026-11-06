// The Action bar element: the game's overlay message, moved and scaled where the editor put it.
fn kyro_feat_titles(game: &crate::game::Game<'_>, batch: &mut HudBatch, bar_height: i32) -> core::ops::ControlFlow<()> {
    use horizon_client::options_ui::kyro_ring as ring;
    let (w, h) = (batch.width as i32, batch.height as i32);
    let partial = game.clock.fraction() as f32;
    let background = game.settings.text_background(0.0);
    let atlas = &game.gui.atlas;
    // Held back so a background can sit under what the game draws.
    let slot = batch.vertices.len();
    batch.fill(atlas, 0.0, 0.0, 0.0, 0.0, 0);
    let start = batch.vertices.len();
    if ring::on("action_bar") {
        let draws = game.titles.overlay_message(w, h, bar_height, partial, &atlas.font, background);
        hud_layout::paint(batch, atlas, &draws);
    }
    if batch.vertices.len() > start {
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for v in &batch.vertices[start..] {
            x0 = x0.min(v.pos[0]);
            y0 = y0.min(v.pos[1]);
            x1 = x1.max(v.pos[0]);
            y1 = y1.max(v.pos[1]);
        }
        let (x, y, scale, panel) = ring::place("action_bar", x1 - x0, y1 - y0, (x0, y0));
        for v in &mut batch.vertices[start..] {
            v.pos[0] += x - x0;
            v.pos[1] += y - y0;
        }
        if panel {
            ring::back_panel(&mut batch.vertices[slot..], [x - 3.0, y - 2.0, x1 - x0 + 6.0, y1 - y0 + 4.0]);
        }
        batch.scale_since(slot, x, y, scale, scale);
    } else {
        // Nothing to show: the editor still gets a box to drag.
        ring::place("action_bar", 182.0, 12.0, (((batch.width - 182.0) / 2.0).floor(), batch.height - 72.0));
    }
    let draws = game.titles.title(w, h, partial, &atlas.font, background);
    hud_layout::paint(batch, atlas, &draws);
    core::ops::ControlFlow::Break(())
}

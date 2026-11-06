// The Scoreboard element: the game's sidebar, drawn then moved and scaled where the editor put it.
fn kyro_feat_scoreboard(game: &crate::game::Game<'_>, batch: &mut HudBatch) -> core::ops::ControlFlow<()> {
    use horizon_client::options_ui::kyro_ring as ring;
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() {
        return core::ops::ControlFlow::Continue(());
    }
    if !ring::on("scoreboard") {
        return core::ops::ControlFlow::Break(());
    }
    // Held back so a background can sit under what the game draws.
    let slot = batch.vertices.len();
    batch.fill(&game.gui.atlas, 0.0, 0.0, 0.0, 0.0, 0);
    let start = batch.vertices.len();
    NESTED.set(true);
    game.draw_scoreboard(batch);
    NESTED.set(false);
    if batch.vertices.len() == start {
        batch.vertices.truncate(slot);
        return core::ops::ControlFlow::Break(());
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for v in &batch.vertices[start..] {
        x0 = x0.min(v.pos[0]);
        y0 = y0.min(v.pos[1]);
        x1 = x1.max(v.pos[0]);
        y1 = y1.max(v.pos[1]);
    }
    let (x, y, scale, panel) = ring::place("scoreboard", x1 - x0, y1 - y0, (x0, y0));
    for v in &mut batch.vertices[start..] {
        v.pos[0] += x - x0;
        v.pos[1] += y - y0;
    }
    if panel {
        ring::back_panel(&mut batch.vertices[slot..], [x - 2.0, y - 2.0, x1 - x0 + 4.0, y1 - y0 + 4.0]);
    }
    batch.scale_since(slot, x, y, scale, scale);
    core::ops::ControlFlow::Break(())
}

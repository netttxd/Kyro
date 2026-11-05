// Ping numbers: milliseconds where the bars go, in the same order as the rows.
fn kyro_feat_tab_ping(mut value: Vec<GuiDraw>, inputs: &TabInputs<'_>, font: &Font) -> Vec<GuiDraw> {
    if !crate::options_ui::kyro_ring::on("tab_ping") {
        return value;
    }
    let players = sorted_players(inputs.players, inputs.board);
    let mut rows = players.iter();
    for draw in &mut value {
        if let GuiDraw::Sprite { sprite, x, y, .. } = draw
            && sprite.starts_with("icon/ping_")
        {
            let (x, y) = (*x, *y);
            let Some(player) = rows.next() else {
                break;
            };
            if player.latency < 0 {
                continue; // Unknown keeps its icon.
            }
            let number = player.latency.min(9999).to_string();
            let color = match player.latency {
                0..100 => 0xff55ff55,
                100..250 => 0xffffff55,
                _ => 0xffff5555,
            };
            let width = font.width(&number) as i32;
            *draw = GuiDraw::Text { text: Text::literal(number), x: x + 10 - width, y, color, shadow: true };
        }
    }
    value
}

// Better chat: the sender's skin face on lines that opened with the spaces left for it.
fn kyro_feat_chat_heads(value: Vec<GuiDraw>) -> Vec<GuiDraw> {
    use crate::options_ui::kyro_ring as ring;
    if !ring::on("better_chat") || ring::value("better_chat", "heads") == 0 {
        return value;
    }
    let mut out = Vec::with_capacity(value.len());
    for draw in value {
        let face = match &draw {
            GuiDraw::Text { text, x, y, color, .. } => {
                let plain = text.plain();
                plain.starts_with("   ").then(|| ring::line_skin(&plain)).flatten().map(|skin| (skin, *x, *y, *color))
            }
            _ => None,
        };
        out.push(draw);
        if let Some((skin, x, y, color)) = face {
            crate::tab_list::face(&mut out, &skin, x, y, 8, true, false, color);
        }
    }
    out
}

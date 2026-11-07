// Right-click copy: remembers the message under the cursor, whole, even when it wrapped.
fn kyro_feat_chat_hover(this: &Chat, height: i32, x: f64, y: f64) {
    let row = this.line_at(height, x, y).map(|l| l.0.content.plain()).unwrap_or_default();
    let row = row.trim();
    let whole = this.messages.iter().map(|m| m.content.plain()).find(|m| !row.is_empty() && m.contains(row));
    crate::options_ui::kyro_world::set_chat_hover(whole.unwrap_or_else(|| row.to_owned()));
}

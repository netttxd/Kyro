// Chat search: while the box has text, the open chat shows only the lines that match.
fn kyro_feat_chat_filter(this: &Chat, height: i32, ticks: f32, focused: bool) -> core::ops::ControlFlow<Vec<GuiDraw>> {
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    let query = crate::options_ui::kyro_ring::chat_query().to_lowercase();
    if NESTED.get() || !focused || query.is_empty() {
        return core::ops::ControlFlow::Continue(());
    }
    let mut shown = Chat::default();
    shown.options = this.options;
    shown.smooth = this.smooth;
    shown.hide_indicator = this.hide_indicator;
    shown.lines = this.lines.iter().filter(|l| l.content.plain().to_lowercase().contains(&query)).cloned().collect();
    NESTED.set(true);
    let draws = shown.layout(height, ticks, focused);
    NESTED.set(false);
    core::ops::ControlFlow::Break(draws)
}

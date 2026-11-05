// Better chat: timestamps, repeats stacked with a count, a gold mark on lines with your name.
fn kyro_feat_chat(chat: &mut Chat, font: &crate::font::Font, message: &Message, focused: bool) -> core::ops::ControlFlow<()> {
    use crate::options_ui::kyro_ring as ring;
    use std::cell::{Cell, RefCell};
    thread_local! {
        static NESTED: Cell<bool> = const { Cell::new(false) };
        static LAST: RefCell<(String, u32)> = const { RefCell::new((String::new(), 0)) };
    }
    let plain = message.content.plain();
    if NESTED.get() || ring::chat_restoring() || !ring::on("better_chat") || plain.trim().is_empty() {
        return core::ops::ControlFlow::Continue(());
    }
    let words = ring::text_value("better_chat", "text_filter");
    let lower = plain.to_lowercase();
    if words.split(',').map(|w| w.trim().to_lowercase()).any(|w| !w.is_empty() && lower.contains(&w)) {
        return core::ops::ControlFlow::Break(());
    }
    let repeats = LAST.with_borrow_mut(|last| {
        if ring::value("better_chat", "stack") == 1 && last.0 == plain && last.1 > 0 && !chat.messages.is_empty() {
            last.1 += 1;
        } else {
            *last = (plain.clone(), 1);
        }
        last.1
    });
    if repeats > 1 {
        chat.messages.remove(0);
        chat.rewrap(font);
    }
    // Room for the sender's face, which the layout draws over these spaces.
    let skin = ring::take_line_skin(&plain).filter(|_| ring::value("better_chat", "heads") == 1);
    let mut shown = Text::literal(if skin.is_some() { "   " } else { "" });
    let own = ring::own_name();
    if ring::value("better_chat", "mentions") == 1 && !own.is_empty() && repeats == 1 && plain.to_lowercase().contains(&own.to_lowercase()) {
        shown = shown.append(Text::colored("\u{258c} ", 0xffaa00));
    }
    if ring::value("better_chat", "timestamps") == 1 {
        shown = shown.append(Text::colored(chrono::Local::now().format("%H:%M ").to_string(), 0x8c8c8c));
    }
    shown = shown.append(message.content.clone());
    if repeats > 1 {
        shown = shown.append(Text::colored(format!(" (x{repeats})"), 0x8c8c8c));
    }
    if let Some(skin) = skin {
        ring::remember_line(&shown.plain(), skin);
    }
    let mut out = message.clone();
    out.content = shown;
    NESTED.set(true);
    chat.push_message(font, out, focused);
    NESTED.set(false);
    core::ops::ControlFlow::Break(())
}

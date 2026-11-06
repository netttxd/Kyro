// Right Shift opens the ring over the world and closes it again.
fn kyro_ring_open(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::port_kyro as p;
    use horizon_client::options_ui::{Page, kyro_ring as m};
    if !pressed || !game.native_window() || !game.native_on(p::ID) {
        return;
    }
    if key != game.native_mod_key(p::ID, "menu", "key.keyboard.right.shift") {
        return;
    }
    let held = i32::from(game.modifiers.shift_key()) | i32::from(game.modifiers.control_key()) << 1 | i32::from(game.modifiers.alt_key()) << 2;
    if m::listening().is_some() || m::just_bound() || !m::bind_ok("menu", key, held) {
        return;
    }
    if game.native_playing() {
        game.capture(false);
        game.open_options_at(true, Page::Mods);
        game.ui.options.in_world = true;
        m::opened_by_key();
    } else if game.screen == crate::game::Screen::Options && m::by_key() {
        m::reset_list();
        let _ = game.options_key(winit::keyboard::KeyCode::Escape, None);
    }
}

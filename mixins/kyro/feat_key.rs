// The Zoom and Freelook keys, held.
fn kyro_feat_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::{kyro_feat as f, port_kyro as p};
    use horizon_client::options_ui::kyro_ring as ring;
    if !game.native_on(p::ID) {
        return;
    }
    let held = i32::from(game.modifiers.shift_key()) | i32::from(game.modifiers.control_key()) << 1 | i32::from(game.modifiers.alt_key()) << 2;
    if let Some(name) = ring::listening() {
        let modifier = ring::is_modifier(key);
        // Shift or AltGr may be on the way to a symbol, so only a modifier let go alone binds it.
        let picked = if modifier {
            if pressed {
                ring::hold_modifier(key);
                None
            } else {
                ring::released_alone(key).then_some((key, 0))
            }
        } else if !pressed {
            None
        } else if key == "key.keyboard.escape" {
            // The menu key can't be cleared: Esc just leaves it as it was.
            if name == "menu" {
                ring::cancel_bind();
                return;
            }
            Some(("key.keyboard.unknown", 0))
        } else {
            Some((key, held))
        };
        if let Some((key, modifiers)) = picked {
            game.settings.mods.keys.entry(p::ID.to_owned()).or_default().insert(name.to_owned(), key.to_owned());
            // The save must hold your own menu blur, not Glass's stand-in.
            let shown = game.settings.menu_blur;
            game.settings.menu_blur = ring::own_blur().unwrap_or(shown);
            game.settings.save(&crate::game::Game::options_path());
            game.settings.menu_blur = shown;
            ring::finish_bind(name, key, modifiers);
        }
        return;
    }
    let playing = game.native_playing();
    if pressed && playing && key == game.settings.keys.get("key.sprint") {
        f::sprint_tap();
    }
    if pressed && playing && ring::on("lyrics") {
        if key == game.native_mod_key(p::ID, "lyric_earlier", "key.keyboard.left.bracket") && ring::bind_ok("lyric_earlier", key, held) {
            f::nudge(50);
        } else if key == game.native_mod_key(p::ID, "lyric_later", "key.keyboard.right.bracket") && ring::bind_ok("lyric_later", key, held) {
            f::nudge(-50);
        }
    }
    if ring::on("zoom") && key == game.native_mod_key(p::ID, "zoom", "key.keyboard.c") && (!pressed || ring::bind_ok("zoom", key, held)) {
        f::zoom_key(playing, pressed, ring::value("zoom", "level") as usize);
    }
    if ring::on("freelook") && key == game.native_mod_key(p::ID, "freelook", "key.keyboard.left.alt") && (!pressed || ring::bind_ok("freelook", key, held)) {
        let server = ring::server().to_lowercase();
        let blocked = !server.is_empty() && ring::text_value("freelook", "text_blocked").split(',').any(|b| !b.trim().is_empty() && server.contains(&b.trim().to_lowercase()));
        let allowed = !blocked && (ring::value("freelook", "solo") == 0 || server.is_empty());
        // As Java: from first person the camera steps back while held, so your body stays in view.
        let was = f::looking();
        f::look_key(playing && allowed, pressed, game.yaw, game.pitch);
        let now = f::looking();
        if now && !was && game.camera_type.first_person() {
            f::set_stepped_back(true);
            game.camera_type = crate::game::perspective::CameraType::ThirdPersonBack;
        } else if !now && was && f::set_stepped_back(false) {
            game.camera_type = crate::game::perspective::CameraType::FirstPerson;
        }
    }
}

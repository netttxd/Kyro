// Each frame: saved switches in and out, and the pointer back after the ring closes.
fn kyro_ring_frame(game: &mut crate::game::Game<'_>) {
const PACK_MANIFEST: &str = r#"schema = 1
id = "native_kyro"
name = "Kyro"
version = "0.1.0"
api = "0.1"

[[post]]
file = "grade.wgsl"

[[post]]
file = "blur.wgsl"
"#;
const PACK_GRADE: &str = r#"// Color saturation: frame.mods[0] = saturation, brightness, contrast, gamma; mods[1] = vibrance, hue.
fn hue_turn(c: vec3<f32>, angle: f32) -> vec3<f32> {
    let k = vec3<f32>(0.57735, 0.57735, 0.57735);
    let s = sin(angle);
    let o = cos(angle);
    return c * o + cross(k, c) * s + k * dot(k, c) * (1.0 - o);
}

fn post(input: PostInput) -> vec4<f32> {
    let a = frame.mods[0];
    let b = frame.mods[1];
    if a.x == 0.0 && a.w == 0.0 {
        return input.color;
    }
    var c = max(input.color.rgb, vec3<f32>(0.0));
    c = hue_turn(c, b.y);
    let luma = dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
    c = mix(vec3<f32>(luma), c, a.x);
    let top = max(c.r, max(c.g, c.b));
    let low = min(c.r, min(c.g, c.b));
    c = mix(vec3<f32>(luma), c, 1.0 + b.x * (1.0 - (top - low)));
    c = (c - vec3<f32>(0.5)) * a.z + vec3<f32>(0.5) + vec3<f32>(a.y);
    c = pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / max(a.w, 0.01)));
    return vec4<f32>(c, input.color.a);
}
"#;
const PACK_BLUR: &str = r#"// Motion blur by velocity, after Natural Motion Blur (ItsPasi, LGPL-3.0): each pixel smears along
// how far it moved on screen since the last frame, found from its depth and the two cameras, so
// nothing ghosts. frame.mods[1].z is the strength, .w the shutter time in seconds (0: one frame).
fn blur_noise(p: vec2<f32>) -> f32 {
    return fract(52.9829189 * fract(0.06711056 * p.x + 0.00583715 * p.y));
}

fn post(input: PostInput) -> vec4<f32> {
    let strength = frame.mods[1].z;
    if strength <= 0.0 || frame.delta_time <= 0.0 {
        return input.color;
    }
    let size = vec2<f32>(textureDimensions(scene_depth));
    // The nearest depth around (reversed: bigger is nearer), so near edges keep their own motion.
    var depth = 0.0;
    for (var x = -1; x <= 1; x++) {
        for (var y = -1; y <= 1; y++) {
            depth = max(depth, depth_at(input.uv + vec2<f32>(f32(x), f32(y)) / size));
        }
    }
    // The sky has no distance: a tiny depth stands in, so it only moves with the camera's turning.
    let then = view_position(input.uv, max(depth, 0.000001)) + frame.camera_motion;
    let clip = frame.previous_view_projection * vec4<f32>(then, 1.0);
    if clip.w <= 0.0 {
        return input.color;
    }
    let before = vec2<f32>(clip.x / clip.w * 0.5 + 0.5, 0.5 - clip.y / clip.w * 0.5);
    var velocity = input.uv - before;
    let far = length(velocity);
    if far > 0.4 {
        velocity *= 0.4 / far;
    }
    let speed = min(far, 0.4);
    if speed < 0.0001 {
        return input.color;
    }
    let samples = clamp(i32(ceil(speed * 100.0)), 4, 64);
    // Above the shutter time a frame moves less, so the smear reaches back over more of them.
    let scale = strength * max(1.0, frame.mods[1].w / frame.delta_time);
    let step = scale * velocity / f32(samples);
    let seed = input.uv * size;
    var sum = vec3<f32>(0.0);
    for (var i = 0; i < samples; i++) {
        let n = f32(i);
        let jitter = blur_noise(seed + vec2<f32>(n, n * 1.4));
        let c = textureSampleLevel(previous, linear_sampler, input.uv + (n + jitter) * step, 0.0).rgb;
        sum += c * c;
    }
    return vec4<f32>(sqrt(sum / f32(samples)), input.color.a);
}
"#;
    use horizon_client::options_ui::kyro_ring as m;
    m::ensure_loaded();
    game.settings.extras.own_name_tag = m::on("own_nametag");
    horizon_core::kyro_shared::SHINY_POTIONS.store(m::on("shiny_potions"), std::sync::atomic::Ordering::Relaxed);
    horizon_client::options_ui::kyro_world::set_own_name(game.own_profile_name());
    // Glass uses the game's own menu blur, at its slider, and gives the option back on close.
    if let Some(blur) = m::glass_blur() {
        if m::own_blur().is_none() {
            m::set_own_blur(Some(game.settings.menu_blur));
        }
        game.settings.menu_blur = blur;
    } else if let Some(own) = m::own_blur() {
        game.settings.menu_blur = own;
        m::set_own_blur(None);
    }
    m::set_held(crate::game::kyro_feat::held_modifiers(game));
    if game.screen == crate::game::Screen::Options {
        for [name, _, _] in m::KEYS {
            m::set_key_name(name, &crate::game::kyro_feat::bound_key(game, name));
        }
    }
    if game.screen != crate::game::Screen::Chat {
        horizon_client::options_ui::kyro_world::chat_closed();
    }
    crate::game::kyro_feat::shield_delay(game);
    crate::game::kyro_feat::track_kills(game);
    // Your shield: the server's cooldown, the local one, or the disable event, whichever is known.
    let shield = [game.offhand.as_ref(), game.held()].into_iter().flatten().find(|s| s.item.ends_with(":shield")).map(|s| s.item.to_string());
    let wait = shield.map_or(0.0_f32, |item| game.cooldowns.percent(&item, 0.0)).max((game.shield_cooldown as f32 / 100.0).min(1.0)).max(crate::game::kyro_feat::shield_wait(crate::game::kyro_feat::YOU));
    horizon_client::options_ui::kyro_world::set_own_shield_color(horizon_client::options_ui::kyro_world::shield_color(wait));
    let held_shield = |s: Option<&crate::game::ItemStack>| s.is_some_and(|s| s.item.ends_with(":shield"));
    let (main, off) = (held_shield(game.held()), held_shield(game.offhand.as_ref()));
    if game.settings.main_hand_left {
        horizon_client::options_ui::kyro_world::set_shield_arms(main, off);
    } else {
        horizon_client::options_ui::kyro_world::set_shield_arms(off, main);
    }
    // A disabled shield can't stay up, even when no cooldown packet reached this client.
    if wait > 0.0 && game.using.and_then(|(slot, _)| game.stack_in(slot)).is_some_and(|s| s.item.ends_with(":shield")) {
        game.using = None;
    }
    // The menu draws no HUD: the session clock keeps running through it.
    if game.title.is_none() && game.player.is_some() {
        crate::game::kyro_feat::session_tick(&format!("{:?}", game.native_player().map(|p| p.dimension)));
    }
    horizon_client::options_ui::kyro_world::set_player_spots(game.mobs.iter().filter(|e| &*e.kind.id == "minecraft:player").map(|e| [e.position.x, e.position.y, e.position.z]).collect());
    horizon_client::options_ui::kyro_world::set_f3(game.keys.contains(&winit::keyboard::KeyCode::F3));
    // Keep chat: the last lines saved for next launch, and read back, greyed and faded, once a world is up.
    if m::on("better_chat") && m::value("better_chat", "keep") == 1 && game.player.is_some() {
        thread_local! {
            static LOG: std::cell::Cell<(bool, u64, Option<std::time::Instant>)> = const { std::cell::Cell::new((false, 0, None)) };
        }
        let file = || m::data_dir().map(|d| d.join("chat_log.txt"));
        let (restored, seen, at) = LOG.get();
        if !restored {
            let text = file().and_then(|f| std::fs::read_to_string(f).ok()).unwrap_or_default();
            let logging = std::mem::replace(&mut game.ui.chat.log, false);
            horizon_client::options_ui::kyro_world::set_chat_restoring(true);
            for line in text.lines().rev() {
                game.ui.chat.add(&game.gui.atlas, horizon_client::text::Text::colored(line, 0x9a9a9a), None, horizon_client::chat::ChatSource::Server, 0, false);
            }
            horizon_client::options_ui::kyro_world::set_chat_restoring(false);
            game.ui.chat.log = logging;
            LOG.set((true, game.ui.chat.received, None));
        } else if game.ui.chat.received != seen && at.is_none_or(|t| t.elapsed().as_secs() >= 5) {
            let lines: Vec<String> = game.ui.chat.recent_lines(100).iter().map(|l| l.trim().replace('\n', " ")).collect();
            if let Some(file) = file()
                && let Err(error) = std::fs::create_dir_all(file.parent().unwrap_or(&file)).and_then(|()| std::fs::write(&file, lines.join("\n")))
            {
                log::warn!("kyro chat log not written: {error}");
            }
            LOG.set((true, game.ui.chat.received, Some(std::time::Instant::now())));
        }
    }
    if horizon_client::options_ui::kyro_world::players_due() {
        let list = game.remote_tab_players().map_or(Vec::new(), |(players, _, _)| players.into_iter().map(|t| {
            let nick = t.display_name.as_ref().map(|d| horizon_client::scoreboard::comp_text(d).plain()).unwrap_or_default();
            let nick = nick.split_whitespace().last().unwrap_or("").trim_matches(|c: char| !c.is_alphanumeric() && c != '_').to_owned();
            (t.name, t.skin, t.uuid, nick)
        }).collect());
        horizon_client::options_ui::kyro_world::set_players(list);
    }
    if m::on("glint_color") {
        for (option, key) in [("glintSpeed", "speed"), ("glintStrength", "strength")] {
            let share = f64::from(m::value("glint_color", key)) / 100.0;
            if (game.settings.file.double(option) - share).abs() > 0.001 {
                let _ = game.settings.file.set_text(option, &share.to_string());
            }
        }
    }
    horizon_client::options_ui::kyro_world::set_reach(game.entity_reach);
    if game.title.is_some() {
        horizon_client::options_ui::kyro_world::server_left(); // Back at the menu: the next world may be singleplayer.
        crate::game::kyro_feat::world_reset();
        crate::game::kyro_feat::session_reset();
    }
    if let Some((name, key)) = m::take_picked() {
        crate::game::kyro_feat::save_key(game, name, key);
    }
    if m::take_close() {
        m::reset_list();
        let _ = game.options_key(winit::keyboard::KeyCode::Escape, None);
    }
    if let Some(eye) = game.eye_position() {
        horizon_client::options_ui::kyro_world::set_eye(eye);
    }
    // Color saturation and Motion blur run as Kyro's post pack, never over a pack you picked.
    const PACK: &str = "native-kyro";
    static WRITTEN: std::sync::Once = std::sync::Once::new();
    WRITTEN.call_once(|| {
        let dir = crate::game::Game::shader_packs_dir().join(PACK);
        let files = [("horizon-shaders.toml", PACK_MANIFEST), ("grade.wgsl", PACK_GRADE), ("blur.wgsl", PACK_BLUR)];
        let done = std::fs::create_dir_all(&dir).and_then(|()| files.iter().try_for_each(|(name, text)| std::fs::write(dir.join(name), text)));
        if let Err(error) = done {
            log::warn!("kyro post pack not written: {error}");
        }
    });
    let wanted = m::on("color_saturation") || m::on("motion_blur");
    let picked = game.settings.horizon.shader_pack.clone();
    if wanted && picked.is_empty() || !wanted && picked == PACK {
        game.settings.horizon.shader_pack = if wanted { PACK.to_owned() } else { String::new() };
        game.pack_ids = None;
        if let Some(renderer) = &mut game.renderer {
            renderer.drop_shader_packs();
        }
    }
    m::save_changed();
    // The glint's look lives in loaded resources, so it needs a reload, but only when it really
    // changed: Speed and Strength apply live, and a color put back needs nothing.
    static GLINT: std::sync::Mutex<Option<[i32; 5]>> = std::sync::Mutex::new(None);
    let glint = [i32::from(m::on("glint_color")), m::value("glint_color", "color"), m::value("glint_color", "glint_style"), m::value("glint_color", "armor"), m::value("glint_color", "armor_color")];
    if !m::by_key() {
        // What the game loaded with, before any menu changes it.
        if let Ok(mut loaded) = GLINT.lock() {
            loaded.get_or_insert(glint);
        }
    }
    if game.screen != crate::game::Screen::Options && m::by_key() {
        m::closed();
        if GLINT.lock().is_ok_and(|mut loaded| loaded.replace(glint).is_some_and(|before| before != glint)) {
            game.reload_packs();
        }
        if game.screen == crate::game::Screen::Playing {
            game.capture(true);
        }
    }
}

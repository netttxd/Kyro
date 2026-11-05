// Food saturation, 1:1 with AppleSkin (public domain, its icons included): the gold outlines, the
// checkered exhaustion, and the hunger, saturation and health held food would give, flashing.
// Drawn with the game's own bars, since anything earlier lands under them.
fn kyro_feat_food(
    game: &mut crate::game::Game<'_>,
    inputs: &horizon_client::hud_layout::HealthInputs,
    wi: i32,
    hi: i32,
    draws: &mut Option<Vec<horizon_client::hud_layout::GuiDraw>>,
) {
    use horizon_client::hud_layout::{Blend, GuiDraw, fill};
    use horizon_client::items::ConsumeEffect;
    use horizon_client::options_ui::kyro_ring as ring;
    const BAR: [&[(i32, i32, u32)]; 4] = [
        &[(6, 4, 0xffc2a100), (6, 5, 0xffc2a100), (5, 6, 0xff9f8609), (7, 6, 0xffc2a100), (8, 6, 0xffc2a100), (6, 7, 0xff9f8609), (8, 7, 0xffc2a100), (6, 8, 0xff9f8609), (7, 8, 0xff9f8609)],
        &[(3, 0, 0xffe9d262), (4, 1, 0xffddc241), (5, 2, 0xffc2a100), (6, 3, 0xffc2a100), (6, 4, 0xffc2a100), (6, 5, 0xffc2a100), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609), (7, 6, 0xffc2a100), (8, 6, 0xffc2a100), (6, 7, 0xff9f8609), (8, 7, 0xffc2a100), (6, 8, 0xff9f8609), (7, 8, 0xff9f8609)],
        &[(2, 0, 0xffe9d262), (3, 0, 0xffe9d262), (4, 1, 0xffddc241), (5, 2, 0xffc2a100), (6, 3, 0xffc2a100), (6, 4, 0xffc2a100), (2, 5, 0xff9f8609), (6, 5, 0xffc2a100), (3, 6, 0xff9f8609), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609), (7, 6, 0xffc2a100), (8, 6, 0xffc2a100), (6, 7, 0xff9f8609), (8, 7, 0xffc2a100), (6, 8, 0xff9f8609), (7, 8, 0xff9f8609)],
        &[(2, 0, 0xffe9d262), (3, 0, 0xffe9d262), (1, 1, 0xffddc241), (4, 1, 0xffddc241), (0, 2, 0xffc2a100), (5, 2, 0xffc2a100), (0, 3, 0xffc2a100), (6, 3, 0xffc2a100), (1, 4, 0xff9f8609), (6, 4, 0xffc2a100), (2, 5, 0xff9f8609), (6, 5, 0xffc2a100), (3, 6, 0xff9f8609), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609), (7, 6, 0xffc2a100), (8, 6, 0xffc2a100), (6, 7, 0xff9f8609), (8, 7, 0xffc2a100), (6, 8, 0xff9f8609), (7, 8, 0xff9f8609)],
    ];
    if !ring::on("food_saturation") || draws.is_some() {
        return;
    }
    let mut out = game.health_hud.layout(inputs, wi, hi);
    let (food, saturation) = (inputs.food, inputs.saturation);
    crate::game::kyro_feat::track_exhaustion(game, food, saturation, inputs.health);
    let exhaustion = crate::game::kyro_feat::exhaustion();
    // Where the game drew this frame, so all of this shakes with its bars; riding, the food bar is gone.
    let spots = |out: &[GuiDraw], name: &str| -> Vec<(i32, i32)> {
        out.iter()
            .filter_map(|d| match d {
                GuiDraw::Sprite { sprite, x, y, .. } if sprite.starts_with(name) => Some((*x, *y)),
                _ => None,
            })
            .collect()
    };
    let bar = spots(&out, "hud/food_empty");
    if bar.is_empty() {
        *draws = Some(out);
        return;
    }
    let mut hearts = spots(&out, "hud/heart/container");
    hearts.reverse();
    let sprite = |name: &'static str, (x, y): (i32, i32), alpha: f32| GuiDraw::Sprite {
        sprite: name.into(),
        x,
        y,
        w: 9,
        h: 9,
        region: None,
        color: ((alpha * 255.0) as u32) << 24 | 0xffffff,
        blend: Blend::Textured,
    };
    // One icon's outline, drawn by how much of its two points is filled.
    let gold = |out: &mut Vec<GuiDraw>, index: i32, share: f32, alpha: f32| {
        let Some(&(x, y)) = bar.get(index as usize) else {
            return;
        };
        let frame = if share >= 1.0 {
            3
        } else if share > 0.5 {
            2
        } else if share > 0.25 {
            1
        } else {
            0
        };
        for &(px, py, color) in BAR[frame] {
            let shade = (((color >> 24) as f32 * alpha) as u32) << 24 | color & 0xffffff;
            out.push(fill(x + px, y + py, x + px + 1, y + py + 1, shade));
        }
    };
    // Exhaustion under the icons, filling from the right; 4 takes a point of food.
    if ring::value("food_saturation", "exhaustion") == 1 {
        let (right, top) = (wi / 2 + 91, hi - 39);
        let width = ((exhaustion / 4.0).clamp(0.0, 1.0) * 81.0) as i32;
        let mut under = Vec::new();
        for row in 0..9 {
            for column in (0..width).filter(|c| (81 - width + c + row) % 2 == 1) {
                let x = right - width + column;
                under.push(fill(x, top + row, x + 1, top + row + 1, 0xbf282828));
            }
        }
        out.splice(0..0, under);
    }
    let level = saturation.clamp(0.0, 20.0);
    for index in 0..(level / 2.0).ceil() as i32 {
        gold(&mut out, index, level / 2.0 - index as f32, 1.0);
    }
    // The food in hand: what you'd get back from it.
    let eaten = game.held().into_iter().chain(game.offhand.iter()).find_map(|s| {
        let info = game.res.items().get(&s.item).filter(|i| i.food.is_some() && (food < 20 || i.can_always_eat))?;
        let (nutrition, gain) = info.food?;
        let mut rotten = false;
        let mut regeneration = 0.0_f32;
        for effect in info.consumable.iter().flat_map(|c| c.effects.iter()) {
            if let ConsumeEffect::ApplyEffects { effects, .. } = effect {
                for e in effects {
                    rotten |= matches!(e.id.trim_start_matches("minecraft:"), "hunger" | "poison" | "nausea" | "slowness" | "mining_fatigue" | "weakness" | "wither" | "blindness" | "darkness" | "levitation" | "unluck");
                    if e.id.ends_with("regeneration") {
                        regeneration += (e.duration / (50 >> e.amplifier.clamp(0, 31)).max(1)) as f32;
                    }
                }
            }
        }
        Some((nutrition as i32, gain, rotten, regeneration))
    });
    if let Some((nutrition, gain, rotten, regeneration)) = eaten {
        let beat = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0.0, |t| (t.as_millis() % 1600) as f32 / 1600.0);
        // Up and down over 32 ticks, resting at each end, to at most 65%.
        let flash = (if beat < 0.5 { -0.5 + beat * 4.0 } else { 1.5 - (beat - 0.5) * 4.0 }).clamp(0.0, 1.0) * 0.65;
        if ring::value("food_saturation", "preview") == 1 {
            let (empty, half, full) = if rotten {
                ("hud/food_empty_hunger", "hud/food_half_hunger", "hud/food_full_hunger")
            } else {
                ("hud/food_empty", "hud/food_half", "hud/food_full")
            };
            let modified = (food + nutrition).clamp(0, 20);
            for index in (food / 2).max(0)..(modified + 1) / 2 {
                if let Some(&spot) = bar.get(index as usize) {
                    out.push(sprite(empty, spot, flash * 0.25));
                    out.push(sprite(if index * 2 + 1 == modified { half } else { full }, spot, flash));
                }
            }
            let (new_food, new_saturation) = ((food + nutrition) as f32, saturation + gain);
            let gained = if new_saturation > new_food { new_food - saturation } else { gain };
            if saturation + gained >= 0.0 {
                let modified = (saturation + gained).clamp(0.0, 20.0);
                let start = if gained != 0.0 { (saturation / 2.0).max(0.0) as i32 } else { 0 };
                for index in start..(modified / 2.0).ceil() as i32 {
                    gold(&mut out, index, modified / 2.0 - index as f32, flash);
                }
            }
        }
        // shortcut: peaceful heals faster and is not ruled out, the client isn't told cleanly.
        let healing = game.potions.effects.list.iter().any(|e| ["poison", "wither", "regeneration"].iter().any(|n| e.id.ends_with(n)));
        if ring::value("food_saturation", "health") == 1 && food < 18 && !healing && inputs.health < inputs.max_health {
            let after = (food + nutrition).min(20);
            let natural = if after >= 18 { crate::game::kyro_feat::regen_health(after, (saturation + gain).min(after as f32), exhaustion) } else { 0.0 };
            let modified = (inputs.health + natural + regeneration).min(inputs.max_health);
            if inputs.health < modified {
                let (container, half, full) = if inputs.hardcore {
                    ("hud/heart/container_hardcore", "hud/heart/hardcore_half", "hud/heart/hardcore_full")
                } else {
                    ("hud/heart/container", "hud/heart/half", "hud/heart/full")
                };
                let whole = modified.ceil() as i32;
                for index in (inputs.health.ceil() as i32 / 2)..(whole + 1) / 2 {
                    if let Some(&spot) = hearts.get(index as usize) {
                        out.push(sprite(container, spot, flash * 0.25));
                        out.push(sprite(if index * 2 + 1 == whole { half } else { full }, spot, flash));
                    }
                }
            }
        }
    }
    *draws = Some(out);
}

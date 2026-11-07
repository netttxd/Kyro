// The HUD editor shows your real HUD; the menu screen would otherwise leave it out.
fn kyro_feat_editor_hud(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    if horizon_client::options_ui::kyro_ring::editing() {
        let mut corners = horizon_client::extras_hud::Corners::new(batch.width, batch.height, 0.0);
        game.native_hud(batch, &mut corners);
        game.draw_scoreboard(batch);
        game.draw_overlay_message(batch);
        game.draw_title(batch);
        // The hotbar too, so the screen looks as it does in play.
        let (x, y) = ((batch.width / 2.0).floor() - 91.0, batch.height - 22.0);
        batch.sprite_scaled(&game.gui.atlas, "gui/sprites/hud/hotbar", x, y, 182.0, 22.0, u32::MAX);
        batch.sprite_scaled(&game.gui.atlas, "gui/sprites/hud/hotbar_selection", x - 1.0 + game.selected as f32 * 20.0, y - 1.0, 24.0, 23.0, u32::MAX);
        // XP, armor, hearts and hunger above it, in survival.
        if !game.creative {
            let (wi, hi) = (batch.width as i32, batch.height as i32);
            let atlas = &game.gui.atlas;
            let width = |s: &str| horizon_client::hud::HudBatch::text_width(atlas, s) as i32;
            let draws = horizon_client::hud_layout::experience(wi, hi, game.experience.progress, game.experience.level, true, &width);
            horizon_client::hud_layout::paint(batch, atlas, &draws);
        }
        let armor = game.player.as_mut().map_or(0, |p| p.attribute("minecraft:armor").or_else(|_| p.attribute("armor")).unwrap_or(0.0) as i32);
        if armor > 0 && !game.creative {
            for i in 0..10 {
                let name = if armor > i * 2 + 1 {
                    "gui/sprites/hud/armor_full"
                } else if armor == i * 2 + 1 {
                    "gui/sprites/hud/armor_half"
                } else {
                    "gui/sprites/hud/armor_empty"
                };
                batch.sprite_scaled(&game.gui.atlas, name, x + i as f32 * 8.0, y - 27.0, 9.0, 9.0, u32::MAX);
            }
        }
        if let Some(p) = game.player.as_ref().filter(|_| !game.creative) {
            let (health, food) = (p.health.health() as i32, p.health.food.food);
            let atlas = &game.gui.atlas;
            for i in 0..10 {
                let hx = x + i as f32 * 8.0;
                let fx = x + 182.0 - 9.0 - i as f32 * 8.0;
                batch.sprite_scaled(atlas, "gui/sprites/hud/heart/container", hx, y - 17.0, 9.0, 9.0, u32::MAX);
                batch.sprite_scaled(atlas, "gui/sprites/hud/food_empty", fx, y - 17.0, 9.0, 9.0, u32::MAX);
                let heart = if health > i * 2 + 1 { "gui/sprites/hud/heart/full" } else { "gui/sprites/hud/heart/half" };
                if health > i * 2 {
                    batch.sprite_scaled(atlas, heart, hx, y - 17.0, 9.0, 9.0, u32::MAX);
                }
                let meat = if food as i32 > i * 2 + 1 { "gui/sprites/hud/food_full" } else { "gui/sprites/hud/food_half" };
                if food as i32 > i * 2 {
                    batch.sprite_scaled(atlas, meat, fx, y - 17.0, 9.0, 9.0, u32::MAX);
                }
            }
        }
        for slot in 0..9 {
            if let Some(stack) = game.inventory[slot].clone() {
                let sx = x + 3.0 + slot as f32 * 20.0;
                game.draw_item_model(batch, &stack, sx, y + 3.0);
                if stack.count > 1 {
                    let label = stack.count.to_string();
                    game.native_text(batch, &label, [sx + 17.0 - game.native_text_width(&label), y + 12.0], 0xffffff, true);
                }
            }
        }
    }
}

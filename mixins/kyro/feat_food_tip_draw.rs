// Food tooltip, as AppleSkin's: the game's hunger icons, then its small gold saturation ones,
// each row filled from the right so the icons face the right way.
fn kyro_feat_food_tip_draw(game: &crate::game::Game<'_>, stack: &crate::game::ItemStack, batch: &mut horizon_client::hud::HudBatch, origin: [f32; 2]) {
    use horizon_client::items::ConsumeEffect;
    const TIP: [&[(u8, u8, u32)]; 5] = [
        &[(2, 0, 0xff0a0a0a), (1, 1, 0xff0a0a0a), (2, 1, 0xff282828), (3, 1, 0xff0a0a0a), (0, 2, 0xff0a0a0a), (1, 2, 0xff282828), (2, 2, 0xff282828), (3, 2, 0xff282828), (4, 2, 0xff0a0a0a), (1, 3, 0xff0a0a0a), (2, 3, 0xff282828), (3, 3, 0xff282828), (4, 3, 0xffc2a100), (2, 4, 0xff0a0a0a), (3, 4, 0xff0a0a0a), (4, 4, 0xff282828), (5, 4, 0xffc2a100), (6, 4, 0xffc2a100), (4, 5, 0xff9f8609), (5, 5, 0xff282828), (6, 5, 0xffc2a100), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609)],
        &[(2, 0, 0xff0a0a0a), (1, 1, 0xff0a0a0a), (2, 1, 0xff282828), (3, 1, 0xff0a0a0a), (0, 2, 0xff0a0a0a), (1, 2, 0xff282828), (2, 2, 0xff282828), (3, 2, 0xff282828), (4, 2, 0xffc2a100), (1, 3, 0xff0a0a0a), (2, 3, 0xff282828), (3, 3, 0xff282828), (4, 3, 0xffc2a100), (2, 4, 0xff0a0a0a), (3, 4, 0xff9f8609), (4, 4, 0xff282828), (5, 4, 0xffc2a100), (6, 4, 0xffc2a100), (4, 5, 0xff9f8609), (5, 5, 0xff282828), (6, 5, 0xffc2a100), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609)],
        &[(2, 0, 0xff0a0a0a), (1, 1, 0xff0a0a0a), (2, 1, 0xff282828), (3, 1, 0xffddc241), (0, 2, 0xff0a0a0a), (1, 2, 0xff282828), (2, 2, 0xff282828), (3, 2, 0xff282828), (4, 2, 0xffc2a100), (1, 3, 0xff0a0a0a), (2, 3, 0xff282828), (3, 3, 0xff282828), (4, 3, 0xffc2a100), (2, 4, 0xff9f8609), (3, 4, 0xff9f8609), (4, 4, 0xff282828), (5, 4, 0xffc2a100), (6, 4, 0xffc2a100), (4, 5, 0xff9f8609), (5, 5, 0xff282828), (6, 5, 0xffc2a100), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609)],
        &[(2, 0, 0xffddc241), (1, 1, 0xffddc241), (2, 1, 0xff282828), (3, 1, 0xffddc241), (0, 2, 0xffc2a100), (1, 2, 0xff282828), (2, 2, 0xff282828), (3, 2, 0xff282828), (4, 2, 0xffc2a100), (1, 3, 0xff9f8609), (2, 3, 0xff282828), (3, 3, 0xff282828), (4, 3, 0xffc2a100), (2, 4, 0xff9f8609), (3, 4, 0xff9f8609), (4, 4, 0xff282828), (5, 4, 0xffc2a100), (6, 4, 0xffc2a100), (4, 5, 0xff9f8609), (5, 5, 0xff282828), (6, 5, 0xffc2a100), (4, 6, 0xff9f8609), (5, 6, 0xff9f8609)],
        &[(2, 0, 0xff0a0a0a), (1, 1, 0xff0a0a0a), (2, 1, 0xff282828), (3, 1, 0xff0a0a0a), (0, 2, 0xff0a0a0a), (1, 2, 0xff282828), (2, 2, 0xff282828), (3, 2, 0xff282828), (4, 2, 0xff0a0a0a), (1, 3, 0xff0a0a0a), (2, 3, 0xff282828), (3, 3, 0xff282828), (4, 3, 0xff0a0a0a), (2, 4, 0xff0a0a0a), (3, 4, 0xff0a0a0a), (4, 4, 0xff282828), (5, 4, 0xff0a0a0a), (6, 4, 0xff0a0a0a), (4, 5, 0xff0a0a0a), (5, 5, 0xff282828), (6, 5, 0xff0a0a0a), (4, 6, 0xff0a0a0a), (5, 6, 0xff0a0a0a)],
    ];
    let Some(info) = game.res.items().get(&stack.item) else {
        return;
    };
    let Some((nutrition, gain)) = info.food else {
        return;
    };
    let rotten = info.consumable.iter().flat_map(|c| c.effects.iter()).any(|effect| match effect {
        ConsumeEffect::ApplyEffects { effects, .. } => effects.iter().any(|e| matches!(e.id.trim_start_matches("minecraft:"), "hunger" | "poison" | "nausea" | "slowness" | "mining_fatigue" | "weakness" | "wither" | "blindness" | "darkness" | "levitation" | "unluck")),
        _ => false,
    });
    let atlas = &game.gui.atlas;
    let [x, y] = origin;
    let (full, half) = if rotten { ("gui/sprites/hud/food_full_hunger", "gui/sprites/hud/food_half_hunger") } else { ("gui/sprites/hud/food_full", "gui/sprites/hud/food_half") };
    let hunger = nutrition.div_ceil(2);
    let bars = if hunger > 10 { 1 } else { hunger };
    for index in 0..bars {
        let at = x + (bars - 1 - index) as f32 * 9.0;
        batch.sprite_scaled(atlas, "gui/sprites/hud/food_empty", at, y, 9.0, 9.0, u32::MAX);
        batch.sprite_scaled(atlas, if index * 2 + 1 == nutrition { half } else { full }, at, y, 9.0, 9.0, u32::MAX);
    }
    if hunger > 10 {
        crate::game::kyro_feat::lyric_text(game, batch, &format!("x{hunger}"), (x + 10.0, y + 2.0), 0.75, 0xaaaaaa, 1.0, true);
    }
    let amount = gain.abs();
    let wanted = (amount / 2.0).ceil() as i32;
    let bars = if wanted > 10 || wanted == 0 { 1 } else { wanted };
    for index in 0..bars {
        let point = (index * 2) as f32;
        let share = (amount - point) / 2.0;
        let frame = if share >= 1.0 {
            3
        } else if share > 0.5 {
            2
        } else if share > 0.25 {
            1
        } else if share > 0.0 {
            0
        } else {
            4
        };
        let alpha = if amount <= point { 0.5 } else { 1.0 };
        let at = x + (bars - 1 - index) as f32 * 7.0;
        for &(px, py, color) in TIP[frame] {
            let [a, r, g, b] = color.to_be_bytes();
            batch.fill(atlas, at + f32::from(px), y + 10.0 + f32::from(py), 1.0, 1.0, horizon_client::hud::rgba(r, g, b, (f32::from(a) * alpha) as u8));
        }
    }
    if wanted > 10 || wanted == 0 {
        crate::game::kyro_feat::lyric_text(game, batch, &format!("x{wanted}"), (x + 8.0, y + 11.0), 0.75, 0xaaaaaa, 1.0, true);
    }
}

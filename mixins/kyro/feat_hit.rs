// Your attack landed on a mob, on this client: the hit marker and the combo. Blocks do not count.
fn kyro_feat_hit(value: bool, game: &crate::game::Game<'_>) -> bool {
    // The return value only says a block broke at once, so it is false on a mob hit. A click during
    // the miss delay, or a spectator's, swings at nothing.
    if game.clicks.miss_time == 0 && !game.spectator() && game.mob_hit().is_some_and(|m| game.before_block(m.1)) {
        crate::game::kyro_feat::hit();
        if let Some(mob) = game.mob_hit().and_then(|m| game.mobs.get(m.0))
            && let Some(name) = game.remote.as_ref().and_then(|r| r.level_player_names.get(&(mob.id as i32)))
        {
            crate::game::kyro_feat::set_target_name(name);
        }
    }
    value
}

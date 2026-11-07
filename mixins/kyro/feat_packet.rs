// The server's world time feeds TPS; its crit animation colors the hit marker.
fn kyro_feat_packet(game: &mut crate::game::Game<'_>, packet: &C) {
    use crate::game::kyro_feat as f;
    match packet {
        C::SetTime(p) => f::tps_time(p.game_time),
        C::Animate(p) if p.action == 4 => f::crit(p.id),
        C::SwingAnimation(p) => f::swing(p.id),
        C::Login(_) => f::session_reset(),
        C::PlayerChat(p) => horizon_client::options_ui::kyro_world::set_chat_sender(p.sender),
        C::EntityEvent(p) if p.event == 3 => f::entity_died(p.id),
        // Complete Shield Fixes (MC-105068): shields sound when hit or disabled.
        C::EntityEvent(p) if p.event == 29 || p.event == 30 => {
            use horizon_client::options_ui::kyro_ring as ring;
            if p.event == 30 {
                // u32::MAX stands for you: the mob list doesn't hold your own entity.
                let own = game.remote.as_ref().is_some_and(|r| r.entity_id == p.id);
                f::shield_broke(if own { u32::MAX } else { p.id as u32 });
            }
            let at = game.mobs.iter().find(|m| m.id == p.id as u32).map(|m| [m.position.x, m.position.y, m.position.z]).or_else(|| game.native_player().map(|o| [o.x, o.y, o.z]));
            if let Some(at) = at.filter(|_| ring::on("shield_status") && ring::value("shield_status", "sounds") == 1) {
                let sound = if p.event == 29 { "minecraft:item.shield.block" } else { "minecraft:item.shield.break" };
                game.native_play_local_sound(sound, at, 1.0, 1.0, 0);
            }
        }
        _ => {}
    }
}

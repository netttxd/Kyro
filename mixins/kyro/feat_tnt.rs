// TNT timer: seconds left above lit TNT, green, then yellow, red in the last second.
fn kyro_feat_tnt(game: &Game<'_>, mut value: Vec<NameTag>) -> Vec<NameTag> {
    use horizon_client::options_ui::kyro_ring as ring;
    if ring::on("damage_numbers") {
        // DamageNumbers.java: the health your target lost rises from it and fades.
        use std::cell::RefCell;
        use std::time::Instant;
        thread_local! {
            static WATCHED: std::cell::Cell<Option<(u32, f32)>> = const { std::cell::Cell::new(None) };
            static POPS: RefCell<Vec<(glam::DVec3, String, Instant)>> = const { RefCell::new(Vec::new()) };
        }
        if let Some((index, _, _)) = game.mob_hit() {
            let m = &game.mobs[index];
            if let Some((id, health)) = WATCHED.get()
                && id == m.id
                && health - m.health > 0.01
                && crate::game::kyro_feat::combo() > 0
            {
                let side = (m.id % 7) as f64 / 7.0 - 0.5;
                let at = glam::DVec3::new(m.position.x + side * 0.6, m.position.y + 2.0, m.position.z);
                POPS.with_borrow_mut(|p| p.push((at, format!("{:.1} \u{2764}", (health - m.health) / 2.0), Instant::now())));
            }
            WATCHED.set(Some((m.id, m.health)));
        }
        POPS.with_borrow_mut(|pops| {
            pops.retain(|p| p.2.elapsed().as_secs_f32() < 0.9);
            for (at, text, born) in pops.iter() {
                let t = born.elapsed().as_secs_f64() / 0.9;
                value.push(NameTag {
                    at: *at + glam::DVec3::new(0.0, 0.7 * (1.0 - (1.0 - t) * (1.0 - t)), 0.0),
                    lines: vec![(Text::colored(text.clone(), 0xffffff), 0.0)],
                    light: 15,
                    discrete: false,
                    marks: Default::default(),
                });
            }
        });
    }
    if !ring::on("tnt_timer") {
        return value;
    }
    for t in &game.tnts {
        let seconds = (t.fuse as f32 / 20.0).max(0.0);
        let color = if seconds > 2.0 { 0x6be36b } else if seconds > 1.0 { 0xffd166 } else { 0xff5a5a };
        value.push(NameTag {
            at: glam::DVec3::new(t.position.x, t.position.y + 1.25, t.position.z),
            lines: vec![(Text::colored(format!("{seconds:.1}"), color), 0.0)],
            light: game.world.light(t.position.x.floor() as i32, t.position.y.floor() as i32, t.position.z.floor() as i32),
            discrete: false,
            marks: Default::default(),
        });
    }
    value
}

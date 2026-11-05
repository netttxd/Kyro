// Item physics, in the spirit of CreativeMD's ItemPhysic: dropped items tumble as they fall and
// lie flat where they land, instead of floating and spinning. Display only, so it's safe anywhere.
// ItemEntityRenderer.submit with its pose changed; the rest follows Horizon's own.
fn kyro_feat_item_physics(game: &crate::game::Game<'_>, camera: glam::DVec3, alpha: f64, out: &mut Vec<Vertex>) -> core::ops::ControlFlow<()> {
    use std::f32::consts::{FRAC_PI_2, PI};
    thread_local! {
        static TILT: RefCell<(HashMap<u32, f32>, Option<std::time::Instant>)> = RefCell::new((HashMap::new(), None));
    }
    if !horizon_client::options_ui::kyro_ring::on("item_physics") {
        return core::ops::ControlFlow::Continue(());
    }
    let now = std::time::Instant::now();
    let passed = TILT.with_borrow_mut(|t| {
        let passed = t.1.map_or(0.0, |at| now.duration_since(at).as_secs_f32()).min(0.1);
        t.1 = Some(now);
        if t.0.len() > 1024 {
            t.0.clear();
        }
        passed
    });
    let live = game.dropped.iter().map(|item| {
        let (p, q) = (item.previous, item.position);
        let at = [p.x + (q.x - p.x) * alpha, p.y + (q.y - p.y) * alpha, p.z + (q.z - p.z) * alpha];
        (item, at, alpha as f32, true)
    });
    let taken = game.pickups.live.iter().filter_map(|p| match &p.taken {
        crate::game::pickups::Taken::Item(item) => Some((&**item, crate::game::pickups::position(p, alpha), 1.0, false)),
        _ => None,
    });
    for (item, at, _partial, entity) in live.chain(taken) {
        let entity_start = out.len();
        game.mark_motion(entity_start, (2 << 32) | u64::from(item.id));
        let q = item.position;
        let pos = glam::DVec3::from_array(at) - camera;
        if pos.length_squared() > 64.0 * 64.0 {
            continue;
        }
        if entity && let Some(view) = &game.view_frustum {
            let feet = (glam::DVec3::new(q.x, q.y, q.z) - camera).as_vec3();
            let half = glam::Vec3::new(0.125 + 0.5, 0.5, 0.125 + 0.5);
            if !view.intersects(feet - half, feet + glam::Vec3::new(half.x, 0.25 + 0.5, half.z)) {
                continue;
            }
        }
        let owner = owner_at([q.x, q.y, q.z], 0.0, "minecraft:item");
        let state = game.resolve_item(&item.stack, DisplayContext::Ground, Some(&owner), item.id as i32);
        if state.is_empty() {
            continue;
        }
        let (lo, hi) = state.bounding_box();
        let middle = (lo + hi) / 2.0;
        let depth = hi.z - lo.z;
        let flat = depth <= 0.0625;
        let amount = item_render::rendered_amount(item.stack.count);
        // In the air it tumbles; once down it settles on its nearest side, a flat item on its face.
        let falling = !item.on_ground && (item.position.y - item.previous.y).abs() > 0.001;
        let tilt = TILT.with_borrow_mut(|t| {
            let tilt = t.0.entry(item.id).or_insert(item.bob_offset);
            if falling {
                *tilt += passed * 7.0;
            } else {
                let rest = if flat { ((*tilt - FRAC_PI_2) / PI).round() * PI + FRAC_PI_2 } else { (*tilt / FRAC_PI_2).round() * FRAC_PI_2 };
                *tilt += (rest - *tilt) * (passed * 14.0).min(1.0);
            }
            *tilt
        });
        // Flat stacks pile up, so the lowest rests on the ground, not the middle one.
        let lift = if flat { depth / 2.0 + 0.012 + depth * 1.5 * (amount.max(1) - 1) as f32 / 2.0 } else { (hi.y - lo.y) / 2.0 };
        let pose = glam::Mat4::from_translation(pos.as_vec3())
            * glam::Mat4::from_translation(glam::Vec3::new(0.0, lift, 0.0))
            * glam::Mat4::from_rotation_y(item.bob_offset)
            * glam::Mat4::from_rotation_x(tilt)
            * glam::Mat4::from_translation(-middle);
        let packed = game.world.light(q.x.floor() as i32, (q.y + 0.1).floor() as i32, q.z.floor() as i32);
        let seed = i64::from(game.pack.item_renderer.item_id(&item.stack.item)) + i64::from(item.stack.damage);
        for copy in item_render::cluster_poses(&state, pose, amount, seed) {
            game.submit_item_world(&state, copy, vertex_light(packed), out);
        }
        if game.entity_glowing(item.id) {
            let rgb = crate::game::glowing::team_color(game.mob_team(item.id));
            game.mark_outline(&out[entity_start..], rgb);
        } else {
            game.cull_entity_vertices(out, entity_start);
        }
    }
    core::ops::ControlFlow::Break(())
}

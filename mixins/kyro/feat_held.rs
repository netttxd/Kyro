// Held items, as Flapjacck's Swing (CC0): first-person offset, tilt and scale, after the game's own
// pose. Then Shield's Position: a shield kept low or out to the side, so it hides less.
fn kyro_feat_held(left: bool, inverse_height: f32, attack: f32, item: ItemPose) -> core::ops::ControlFlow<glam::Mat4> {
    use crate::options_ui::kyro_ring as ring;
    use glam::{Mat4, Vec3};
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() {
        return core::ops::ControlFlow::Continue(());
    }
    NESTED.set(true);
    let mut pose = item_pose(left, inverse_height, attack, item);
    NESTED.set(false);
    let side = if left { -1.0 } else { 1.0 };
    if ring::on("held_items") {
        let v = |key: &str| ring::value("held_items", key) as f32;
        pose = pose
            * Mat4::from_translation(Vec3::new(side * v("x") / 100.0, v("y") / 100.0, v("z") / 100.0))
            * Mat4::from_rotation_x(v("pitch").to_radians())
            * Mat4::from_rotation_z((side * v("roll")).to_radians())
            * Mat4::from_scale(Vec3::splat(v("scale") / 100.0));
    }
    if ring::on("shield_status") && ring::shield_in(left) {
        pose = match ring::value("shield_status", "position") {
            1 => pose * Mat4::from_translation(Vec3::new(0.0, -0.35, 0.0)),
            2 => pose * Mat4::from_translation(Vec3::new(side * 0.32, -0.12, 0.0)) * Mat4::from_rotation_y((side * -18.0_f32).to_radians()),
            _ => pose,
        };
    }
    core::ops::ControlFlow::Break(pose)
}

// While a player's layers are raised, the flat ones stay out of the model.
fn kyro_feat_skin_flat(p: &mut Poses, spectator: bool) -> core::ops::ControlFlow<()> {
    if !crate::options_ui::kyro_world::take_raised() {
        return core::ops::ControlFlow::Continue(());
    }
    for part in ["body", "right_arm", "left_arm", "right_leg", "left_leg"] {
        p.visible(part, !spectator);
    }
    for part in ["jacket", "left_sleeve", "right_sleeve", "left_pants", "right_pants", "hat"] {
        p.visible(part, false);
    }
    core::ops::ControlFlow::Break(())
}

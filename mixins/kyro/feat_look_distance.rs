// Freelook: the wheel moves the third-person camera in and out.
fn kyro_feat_look_distance(distance: &mut f32) {
    *distance *= crate::options_ui::kyro_world::look_distance();
}

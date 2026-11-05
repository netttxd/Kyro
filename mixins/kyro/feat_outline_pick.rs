// Block outline: the game's black outline gives way to a rim in your color.
fn kyro_feat_outline_pick(pos: [i32; 3], canceled: &mut bool) {
    if horizon_client::options_ui::kyro_ring::on("block_outline") {
        crate::game::kyro_feat::set_outlined(pos);
        *canceled = true;
    }
}

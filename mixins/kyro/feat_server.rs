// Remembers the address you join, for the Server element.
fn kyro_feat_server(address: &str) {
    crate::game::kyro_feat::world_reset();
    horizon_client::options_ui::kyro_ring::set_server(address);
}

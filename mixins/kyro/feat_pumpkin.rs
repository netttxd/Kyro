// Low obstruction: no carved pumpkin overlay.
fn kyro_feat_pumpkin(mut value: Vec<GuiDraw>) -> Vec<GuiDraw> {
    use crate::options_ui::kyro_ring as ring;
    if ring::on("low_obstruction") && ring::value("low_obstruction", "pumpkin") == 1 {
        value.retain(|d| !matches!(d, GuiDraw::Blit { texture, .. } if texture.contains("pumpkinblur")));
    }
    value
}

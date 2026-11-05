// Food tooltip, as AppleSkin's: room for a row of hunger and a row of saturation.
fn kyro_feat_food_tip(game: &crate::game::Game<'_>, stack: &crate::game::ItemStack, size: &mut Option<[f32; 2]>) {
    use horizon_client::options_ui::kyro_ring as ring;
    if !ring::on("food_saturation") || ring::value("food_saturation", "tooltip") != 1 {
        return;
    }
    if let Some((nutrition, gain)) = game.res.items().get(&stack.item).and_then(|i| i.food) {
        let hunger = nutrition.div_ceil(2);
        let saturation = (gain.abs() / 2.0).ceil() as u32;
        let hunger_w = if hunger > 10 { 9.0 + game.native_text_width(&format!("x{hunger}")) * 0.75 } else { hunger as f32 * 9.0 };
        let saturation_w = if saturation > 10 || saturation == 0 { 7.0 + game.native_text_width(&format!("x{saturation}")) * 0.75 } else { saturation as f32 * 7.0 };
        *size = Some([hunger_w.max(saturation_w) + 2.0, 20.0]);
    }
}

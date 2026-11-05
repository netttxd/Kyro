// Clear fluids: pushes back the thick fog inside water, lava and powder snow.
fn kyro_feat_fog(game: &crate::game::Game<'_>, fog_type: horizon_client::fog::FogType, fog: &mut horizon_client::fog::FogData) {
    use horizon_client::fog::FogType;
    use horizon_client::options_ui::kyro_ring as ring;
    // World look: pushes the open-air fog back; water and lava stay with Clear fluids.
    if matches!(fog_type, FogType::Atmospheric) && ring::on("world_look") && ring::value("world_look", "fog") > 0 {
        let keep = if ring::value("world_look", "fog") == 1 { 0.5 } else { 0.98 };
        fog.render_distance_start += (fog.render_distance_end - fog.render_distance_start) * keep;
        fog.environmental_start = fog.environmental_start.max(fog.render_distance_start);
        fog.environmental_end = fog.environmental_end.max(fog.render_distance_end);
    }
    if !ring::on("clear_fluids") {
        return;
    }
    let key = match fog_type {
        FogType::Water => "water",
        FogType::Lava => "lava",
        FogType::PowderSnow => "powder_snow",
        FogType::Atmospheric => return,
    };
    if ring::value("clear_fluids", key) == 0 {
        return;
    }
    let distance = game.settings.render_distance as f32 * 16.0;
    let end = fog.environmental_end.max(distance * ring::value("clear_fluids", "distance") as f32 / 100.0);
    fog.environmental_start = fog.environmental_start.min(end * 0.25);
    fog.environmental_end = end;
    fog.sky_end = end;
    fog.cloud_end = end;
}

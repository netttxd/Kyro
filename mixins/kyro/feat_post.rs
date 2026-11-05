// The sliders of Color saturation and Motion blur, handed to the post pack.
fn kyro_feat_post(game: &crate::game::Game<'_>, params: &mut [[f32; 4]; 8]) {
    use horizon_client::options_ui::kyro_ring as ring;
    thread_local! {
        static SHUTTER: std::cell::Cell<f32> = const { std::cell::Cell::new(0.0) };
    }
    let v = |key: &str| ring::value("color_saturation", key) as f32;
    params[0] = [1.0, 0.0, 1.0, 1.0];
    params[1] = [0.0; 4];
    if ring::on("color_saturation") {
        params[0] = [v("saturation") / 100.0, v("brightness") / 100.0, v("contrast") / 100.0, v("gamma") / 100.0];
        params[1][0] = v("vibrance") / 100.0;
        params[1][1] = v("hue").to_radians();
    }
    if ring::on("motion_blur") {
        // 50 is 1.0, the strength Natural Motion Blur recommends: a full frame of movement.
        params[1][2] = ring::value("motion_blur", "strength") as f32 / 50.0;
        if ring::value("motion_blur", "match_refresh") == 1 {
            // One monitor refresh, read once: past that FPS a frame moves too little to see.
            if SHUTTER.get() == 0.0 {
                let hz = game.window.as_deref().and_then(horizon_client::telemetry::monitor).and_then(|m| m.refresh_rate_millihertz());
                SHUTTER.set(1000.0 / hz.map_or(60_000.0, |r| r as f32));
            }
            params[1][3] = SHUTTER.get();
        }
    }
}

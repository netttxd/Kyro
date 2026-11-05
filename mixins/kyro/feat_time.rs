// World look: the time the sky is drawn at. Only the frame's sky passes biome colors; the clock keeps running.
fn kyro_feat_time(environment: &Environment, ticks: i64, sky: Option<[f32; 3]>, fog: Option<[f32; 3]>) -> core::ops::ControlFlow<Sky> {
    use crate::options_ui::kyro_ring as ring;
    use std::cell::Cell;
    const TIMES: [i64; 7] = [0, 23500, 1000, 6000, 12500, 14000, 18000];
    thread_local! {
        static NESTED: Cell<bool> = const { Cell::new(false) };
    }
    let index = ring::value("world_look", "time") as usize;
    if sky.is_none() || NESTED.get() || !ring::on("world_look") || index == 0 {
        return core::ops::ControlFlow::Continue(());
    }
    NESTED.set(true);
    let fixed = environment.sample(ticks.div_euclid(24000) * 24000 + TIMES[index], sky, fog);
    NESTED.set(false);
    core::ops::ControlFlow::Break(fixed)
}

// Glint on inventory icons: vanilla scales atlas coordinates by 8, an icon's 1/64 of the atlas, so
// it shows an eighth of the texture. Horizon hands over 0..1 per icon, so all eight repeats land
// on one icon and turn to speckle. Bringing it back to vanilla's share gives the soft bands.
fn kyro_feat_glint_uv(this: &Glint, model: [f32; 2]) -> core::ops::ControlFlow<[f32; 2]> {
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    if NESTED.get() || this.scale != 8.0 {
        return core::ops::ControlFlow::Continue(());
    }
    NESTED.set(true);
    let uv = this.uv([model[0] / 64.0, model[1] / 64.0]);
    NESTED.set(false);
    core::ops::ControlFlow::Break(uv)
}

// Shield Statuses: the shield itself takes the status color while disabled.
fn kyro_feat_shield_tint(
    renderer: &ItemRenderer,
    model: &SpecialModel,
    argument: &Value,
    has_foil: bool,
    pose: Mat4,
    sprites: &HashMap<String, Rect>,
    host: &dyn SpecialHost,
    sink: &mut dyn ItemSink,
) -> core::ops::ControlFlow<()> {
    struct Tinted<'a> {
        sink: &'a mut dyn ItemSink,
        tint: u32,
    }
    impl ItemSink for Tinted<'_> {
        fn quad(&mut self, mut quad: ItemQuad) {
            let [a, r, g, b] = quad.color.to_be_bytes();
            let at = |c: u8, shift: u32| (u32::from(c) * ((self.tint >> shift) & 255) / 255) as u8;
            quad.color = u32::from_be_bytes([a, at(r, 16), at(g, 8), at(b, 0)]);
            self.sink.quad(quad);
        }
    }
    thread_local! {
        static NESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    let tint = crate::options_ui::kyro_ring::shield_tint();
    if NESTED.get() || tint == 0 || !matches!(model.kind, SpecialKind::Shield) {
        return core::ops::ControlFlow::Continue(());
    }
    NESTED.set(true);
    submit(renderer, model, argument, has_foil, pose, sprites, host, &mut Tinted { sink, tint });
    NESTED.set(false);
    core::ops::ControlFlow::Break(())
}

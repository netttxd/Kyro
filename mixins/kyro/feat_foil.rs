// Shiny potions: the shimmer the item renderer draws, from its own check.
fn kyro_feat_foil(this: &Resolve<'_>) -> std::ops::ControlFlow<bool> {
    if horizon_core::kyro_shared::shines(&this.stack.item) {
        return std::ops::ControlFlow::Break(true);
    }
    std::ops::ControlFlow::Continue(())
}

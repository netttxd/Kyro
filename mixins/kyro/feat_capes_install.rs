// The capes ship inside this mod, but the game reads textures from its content packs, so the pack
// is put in the mods folder just before the game looks there.
fn kyro_feat_capes_install(dir: &std::path::Path) {
    const PACK: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../native-data/kyro/kyro_capes.hmod"));
    let path = dir.join("kyro_capes.hmod");
    if dir.file_name().is_some_and(|n| n == "horizon-mods") && std::fs::metadata(&path).map_or(true, |m| m.len() != PACK.len() as u64) {
        let _ = std::fs::write(path, PACK);
    }
}

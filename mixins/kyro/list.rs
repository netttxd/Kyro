// The mod's row in Options > Mods, and its keys (name, label, default key).
fn kyro_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::port_kyro as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: m::DESCRIPTION.into(),
        keys: m::KEYS.iter().map(|[n, l, d]| [n.to_string(), l.to_string(), d.to_string()]).collect(),
    });
}

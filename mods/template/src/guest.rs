use mod_api::bindings::Guest;

struct Mod;

impl Guest for Mod {
    fn init() {}

    fn frame() {}
}

mod_api::bindings::export!(Mod with_types_in mod_api::bindings);

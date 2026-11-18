//! `App::on_modifiers` hears the held modifiers in the runtimes an app runs in. A native window
//! takes the hook in the same place as `App::headless_runtime` (`Host::new` beside `on_focus`), and
//! a test can't open one, so this covers the headless runtime.

use rune::{App, AppSpec, Cx, Env, Mods, PhysicalInput, el};

fn spec() -> AppSpec<bool> {
    AppSpec::new("modifiers", (100.0, 100.0), || false, |_: &bool, _: &mut Cx<bool>| el())
}

#[test]
fn a_headless_runtime_hears_the_held_modifiers() {
    let mut rt = App::new(spec())
        .on_modifiers(|alt: &mut bool, mods, _| *alt = mods.alt)
        .headless_runtime((100.0, 100.0), Env::default());
    rt.frame(0.0);
    rt.physical_input(PhysicalInput::Modifiers(Mods { alt: true, ..Mods::default() }));
    rt.frame(1.0 / 60.0);
    assert!(rt.state, "Alt went down");
    rt.physical_input(PhysicalInput::Modifiers(Mods::default()));
    rt.frame(1.0 / 60.0);
    assert!(!rt.state, "and up");
}

use upwell::{Component as _, Plugin as _};

#[test]
fn plugin_lowers_its_component() {
    let prepared = upwell::App::<()>::builder("plugin-test")
        .register_plugin::<crate::GreetingPlugin>()
        .prepare()
        .expect("plugin contributions prepare");

    assert!(
        prepared
            .plugin_plan()
            .resolution()
            .plugin(crate::GreetingPlugin::ID)
            .is_some()
    );
    assert!(
        prepared
            .registry()
            .components
            .iter()
            .any(|component| component.id == crate::GreetingService::ID)
    );
}

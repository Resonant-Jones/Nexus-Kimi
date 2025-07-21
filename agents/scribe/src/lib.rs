wit_bindgen::generate!({
    world: "loom",
    exports: {
        world: Component,
    },
});

struct Component;

impl exports::loom::Guest for Component {
    fn ping() -> String {
        "pong from loom".into()
    }
}
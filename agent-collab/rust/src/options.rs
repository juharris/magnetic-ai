use optify::provider::{OptionsRegistry, OptionsWatcher};
use std::path::Path;
use std::sync::OnceLock;

static WATCHER: OnceLock<OptionsWatcher> = OnceLock::new();

pub fn get_options_provider() -> &'static impl OptionsRegistry {
    WATCHER.get_or_init(|| {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("options");
        OptionsWatcher::build(&path).expect("Failed to build options watcher")
    })
}

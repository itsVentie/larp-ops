pub use shared_types::OutputEvent;

pub trait LarpPlugin {
    fn filter(&mut self, event: &OutputEvent) -> bool;
}

#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ty) => {
        static mut PLUGIN_INSTANCE: Option<$plugin_type> = None;

        #[no_mangle]
        pub extern "C" fn filter_event(ptr: i32, len: i32) -> i32 {
            unsafe {
                if PLUGIN_INSTANCE.is_none() {
                    PLUGIN_INSTANCE = Some(<$plugin_type>::default());
                }

                let slice = std::slice::from_raw_parts(ptr as *const u8, len as usize);
                if let Ok(event) = serde_json::from_slice::<$crate::OutputEvent>(slice) {
                    if let Some(ref mut plugin) = PLUGIN_INSTANCE {
                        return if plugin.filter(&event) { 1 } else { 0 };
                    }
                }
            }
            0
        }
    };
}



use std::ffi::OsStr;
use libloading::{Library, Symbol};
use log::{debug, trace};
use std::fmt::{self, Formatter, Debug};
use anyhow;

/// This was created with: https://michael-f-bryan.github.io/rust-ffi-guide/dynamic_loading.html
///    It even shows how to interact with C++
/// This one explains the project structure better: https://adventures.michaelfbryan.com/posts/plugins-in-rust/
///    And it has checking for ABI version
/// and https://nullderef.com/blog/plugin-dynload/


/// A plugin which allows you to add extra functionality.
pub trait Plugin:    {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used
    /// for initialization.
    fn on_plugin_load(&mut self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}

    fn work(&self, a: i64, b: i64) -> i64;
}

/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:expr) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}


pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub  fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> anyhow::Result<()> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        unsafe {
            let lib = Library::new(filename.as_ref())?;
            // We need to keep the library around otherwise our plugin's vtable will
            // point to garbage. We do this little dance to make sure the library
            // doesn't end up getting moved.
            self.loaded_libraries.push(lib);

            let lib = self.loaded_libraries.last().unwrap();

            let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")?;
                //.chain_err(|| "The `_plugin_create` symbol wasn't found.")?;
            let boxed_raw = constructor();

            let mut plugin = Box::from_raw(boxed_raw);
            debug!("Loaded plugin: {}", plugin.name());
            plugin.on_plugin_load();
            self.plugins.push(plugin);
        }

        Ok(())
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }


    pub fn get(&self, name: &str) -> anyhow::Result<&Box<dyn Plugin>>
    {
        for p in self.plugins.iter() {
            if p.name() == name {
                return Ok(p);
            }
        }
        Err(anyhow::format_err!("err"))
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}

impl Debug for PluginManager {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let plugins: Vec<_> = self.plugins.iter().map(|p| p.name()).collect();

        f.debug_struct("PluginManager")
            .field("plugins", &plugins)
            .finish()
    }
}
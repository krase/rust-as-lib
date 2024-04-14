use loader::{declare_plugin, Plugin};

use log::info;

declare_plugin!(Plugin1, Plugin1::default);


#[derive(Debug, Default)]
pub struct Plugin1 {
    counter: usize,
}


impl Plugin for Plugin1 {
    fn name(&self) -> &'static str  {
        "Plugin1"
    }

    fn on_plugin_load(&mut self) {
        self.counter = 1;
        info!("Plugin1 loaded");
        println!("Plugin1 loaded");
    }

    fn on_plugin_unload(&self) {
        info!("Plugin1 unloaded");
    }
    
    fn work(&self, a: i64, b: i64) -> i64 {
        a + b
    }

    
/*
    fn pre_send(&self, req: &mut Request) {
        //req.headers.set_raw("some-dodgy-header", "true");
        debug!("Injected header into Request, {:?}", req);
    }
 */
/*     fn post_receive(&self, res: &mut Response) {
        debug!("Received Response");
        debug!("Headers: {:?}", res.headers);
        if res.body.len() < 100 && log_enabled!(log::LogLevel::Debug) {
            if let Ok(body) = str::from_utf8(&res.body) {
                debug!("Body: {:?}", body);
            }
        }
        res.headers.remove_raw("some-dodgy-header");
    }*/
}

#[no_mangle]
pub extern "C" fn add(a: isize, b: isize) -> isize {
    a + b
}

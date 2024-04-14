use std::env;

use anyhow::Result;
use loader::PluginManager;

//use libloading::{Library, Symbol};

//mod loader;

//use crate::loader::PluginManager;



//type AddFunc = fn(isize, isize) -> isize;

fn main() -> Result<()>{

    let library_path = env::args().nth(1).expect("USAGE: loading <LIB>");
    
    println!("Loading add() from {}", library_path);

    let mut manager = PluginManager::new();

    let _ = manager.load_plugin(library_path);
    
    println!("Plugins: {:?}", manager);

    let p = manager.get("Plugin1")?;
    println!("callled {} {}", p.name(), p.work(3,4));

    /*
    unsafe {
        let lib = Library::new(library_path).unwrap();

        let func: Symbol<AddFunc> = lib.get(b"add").unwrap();
        let answer = func(5, 2);
        println!("5 + 2 = {}", answer);
    } */
    Ok(())
}

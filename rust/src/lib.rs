use gdnative::prelude::*;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);

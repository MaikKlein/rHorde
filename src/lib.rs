#[link(name = "horde",
vers = "0.1",
uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
author = "Brendan Zabarauskas",
url = "https://github.com/bjz/glfw3-rs")];

#[comment = "Bindings and wrapper functions for glfw3."];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(macro_rules)];
pub mod ffi;


#[fixed_stack_segment]
fn h3d_init()->bool{
  unsafe { ffi::h3dInit()}
}


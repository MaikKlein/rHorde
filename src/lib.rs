#[link(name = "horde",
vers = "0.1",
uuid = "6199FAD3-6D03-4E29-87E7-7DC1B1B65C2C",
author = "Maik Klein",
url = "https://github.com/MaikKlein/rHorde/")];

#[comment = "Horde3D bindings for Rust."];
#[crate_type = "lib"];
#[feature(globs)];
#[feature(macro_rules)];
pub mod ffi;

#[fixed_stack_segment]
fn h3d_init()->bool{
  unsafe { ffi::h3dInit()}
}

#[fixed_stack_segment]
fn h3d_clear(){
  unsafe{ffi::h3dClear()};
}

#[fixed_stack_segment]
fn h3d_release(){
  unsafe{ ffi::h3dRelease();}
}


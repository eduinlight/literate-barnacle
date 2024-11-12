use libloading::{Library, Symbol};
// use raylib::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  // Load the symbol (function) `my_function` from the library
  unsafe {
    // Load the library
    let lib = Library::new("./libs/game/target/debug/libgame.dylib")?;
    let add: Symbol<unsafe extern "C" fn(usize, usize) -> usize> = lib.get(b"add")?;

    // Call the function
    let result = add(42, 23);
    println!("The result is: {}", result);
  }

  Ok(())
  // let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
  //
  // while !rl.window_should_close() {
  //   let mut d = rl.begin_drawing(&thread);
  //
  //   d.clear_background(Color::WHITE);
  //   d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
  // }
}

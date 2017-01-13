use std::collections::HashMap;
use glium::glutin::VirtualKeyCode;

// TODO: add our own enumeration of keycodes to isolate game layer input
//       handling from the underlying input lib(s)?

// TODO: use statically sized arrays instead of hash tables?
//       would use more memory, but reduce heap allocation.

// TODO: be generic over any hashable/enumerable item? would be easier
//       to port to other OS input libs.

/// Responds to inquiries regarding three sets of keyboard input.
///
///- Pressed keys
///- Released keys
///- Held keys
pub struct Input {
	pressed_keys:   HashMap<u32, bool>,
	released_keys:  HashMap<u32, bool>,
	held_keys:      HashMap<u32, bool>,

    cursor_xy: (i32, i32),
}

impl Input {
	pub fn new() -> Input {
		Input{
			pressed_keys:   HashMap::<u32, bool>::new(),
			released_keys:  HashMap::<u32, bool>::new(),
			held_keys:      HashMap::<u32, bool>::new(),

            cursor_xy: (0,0),
		}
	}

	/// Resets the toggle states of pressed & released keys.
	pub fn begin_new_frame(&mut self) {
		self.pressed_keys.clear();
		self.released_keys.clear();
	}

    /// Handles a mouse movement event
    pub fn move_cursor(&mut self, x: i32, y: i32) { self.cursor_xy = (x,y) }

	/// Handles a key down event
	pub fn key_down_event(&mut self, key: VirtualKeyCode) {
		self.pressed_keys.insert(key as u32, true);
		self.held_keys.insert(key as u32, true);
	}

	/// Handles a key up event
	pub fn key_up_event(&mut self, key: VirtualKeyCode) {
		self.released_keys.insert(key as u32, true);
		self.held_keys.insert(key as u32, false);
	}

    /// Fetches coordinate of mouse cursor in screen space
    pub fn get_cursor(&self) -> (i32, i32) { self.cursor_xy }

	/// Responds true if key was pressed since last call to `beginNewFrame()`.
	/// Responds false otherwise.
	pub fn was_key_pressed(&self, key: VirtualKeyCode) -> bool {
		let key_cap = &(key as u32);
		match self.pressed_keys.get(key_cap) {
			Some(is_pressed) => *is_pressed,
			None             => false,
		}
	}
	
	/// Responds true if key was released since last call to `beginNewFrame()`.
	/// Responds false otherwise.
	pub fn was_key_released(&self, key: VirtualKeyCode) -> bool {
		let key_cap = &(key as u32);
		match self.released_keys.get(key_cap) {
			Some(is_pressed) => *is_pressed,
			None             => false,
		}
	}
	
	/// Responds true if key has been pressed since last call to `beginNewFrame()`
	/// but _has not yet been released._
	///
	/// Responds false otherwise.
	pub fn is_key_held(&self, key: VirtualKeyCode) -> bool {
		let key_cap = &(key as u32);
		match self.held_keys.get(key_cap) {
			Some(is_pressed) => *is_pressed,
			None             => false,
		}
	}
}

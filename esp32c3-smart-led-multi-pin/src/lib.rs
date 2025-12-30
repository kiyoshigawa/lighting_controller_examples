#![no_std]
pub mod office_lights {
    // index for LED strip in logical array
    pub const START_CLOSET_INDEX: usize = 0;
    pub const START_WINDOW_INDEX: usize = NUM_LEDS_CLOSET_WALL;
    pub const START_DOOR_INDEX: usize = START_WINDOW_INDEX + NUM_LEDS_WINDOW_WALL;
    pub const START_NORTH_INDEX: usize = START_DOOR_INDEX + NUM_LEDS_DOOR_WALL;

    pub const NUM_LEDS_CLOSET_WALL: usize = 202;
    pub const NUM_LEDS_WINDOW_WALL: usize = 293;
    pub const NUM_LEDS_DOOR_WALL: usize = 292;
    pub const NUM_LEDS_NORTH_WALL: usize = 202;
}

pub mod test_strip {
    // index for LED strip in logical array
    pub const START_CLOSET_INDEX: usize = 0;
    pub const START_WINDOW_INDEX: usize = NUM_LEDS_CLOSET_WALL;
    pub const START_DOOR_INDEX: usize = START_WINDOW_INDEX + NUM_LEDS_WINDOW_WALL;

    pub const NUM_LEDS_CLOSET_WALL: usize = 55;
    pub const NUM_LEDS_WINDOW_WALL: usize = 55;
    pub const NUM_LEDS_DOOR_WALL: usize = 51;
    pub const NUM_LEDS_NORTH_WALL: usize = 51;
}

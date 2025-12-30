#![no_std]
pub mod default_consts {

    use rgb::RGB8;
    use smart_leds::colors::*;

    pub const BLACK_RAINBOW: &[RGB8] = &[BLACK];
    pub const TYPICAL_RGB_RAINBOW: &[RGB8] = &[RED, YELLOW, GREEN, DARK_BLUE, DARK_MAGENTA];
    pub const KELVIN_2500_RAINBOW: &[RGB8] = &[RGB8 { r: 0xff, g: 0xb5, b: 0x65 }];
    pub const HOMEMADE_OKLCH_RAINBOW: &[RGB8] = &[
        RGB8 { r: 252, g: 059, b: 041 },
        RGB8 { r: 252, g: 059, b: 041 },
        RGB8 { r: 244, g: 075, b: 028 },
        RGB8 { r: 236, g: 087, b: 019 },
        RGB8 { r: 228, g: 097, b: 017 },
        RGB8 { r: 219, g: 107, b: 000 },
        RGB8 { r: 208, g: 116, b: 000 },
        RGB8 { r: 197, g: 125, b: 000 },
        RGB8 { r: 185, g: 132, b: 000 },
        RGB8 { r: 172, g: 139, b: 013 },
        RGB8 { r: 163, g: 144, b: 000 },
        RGB8 { r: 149, g: 150, b: 000 },
        RGB8 { r: 129, g: 156, b: 000 },
        RGB8 { r: 103, g: 163, b: 000 },
        RGB8 { r: 059, g: 170, b: 024 },
        RGB8 { r: 000, g: 157, b: 106 },
        RGB8 { r: 000, g: 132, b: 146 },
        RGB8 { r: 000, g: 099, b: 163 },
        RGB8 { r: 002, g: 063, b: 155 },
        RGB8 { r: 074, g: 053, b: 165 },
        RGB8 { r: 115, g: 042, b: 157 },
        RGB8 { r: 150, g: 028, b: 133 },
        RGB8 { r: 179, g: 005, b: 094 },
        RGB8 { r: 205, g: 022, b: 084 },
        RGB8 { r: 229, g: 040, b: 069 },
    ];

    pub const TWELVE_BIT_OKLCH_RAINBOW: &[RGB8] = &[
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 147, g: 026, b: 111 },
        RGB8 { r: 156, g: 034, b: 102 },
        RGB8 { r: 164, g: 043, b: 094 },
        RGB8 { r: 170, g: 051, b: 085 },
        RGB8 { r: 179, g: 064, b: 088 },
        RGB8 { r: 187, g: 077, b: 092 },
        RGB8 { r: 196, g: 089, b: 096 },
        RGB8 { r: 203, g: 101, b: 101 },
        RGB8 { r: 214, g: 112, b: 093 },
        RGB8 { r: 224, g: 124, b: 084 },
        RGB8 { r: 231, g: 137, b: 075 },
        RGB8 { r: 236, g: 151, b: 066 },
        RGB8 { r: 242, g: 166, b: 047 },
        RGB8 { r: 245, g: 182, b: 022 },
        RGB8 { r: 243, g: 200, b: 000 },
        RGB8 { r: 236, g: 219, b: 000 },
        RGB8 { r: 217, g: 220, b: 028 },
        RGB8 { r: 196, g: 220, b: 049 },
        RGB8 { r: 174, g: 220, b: 067 },
        RGB8 { r: 151, g: 219, b: 083 },
        RGB8 { r: 133, g: 220, b: 097 },
        RGB8 { r: 115, g: 221, b: 111 },
        RGB8 { r: 094, g: 221, b: 124 },
        RGB8 { r: 068, g: 221, b: 136 },
        RGB8 { r: 044, g: 218, b: 153 },
        RGB8 { r: 023, g: 213, b: 166 },
        RGB8 { r: 017, g: 208, b: 177 },
        RGB8 { r: 032, g: 203, b: 186 },
        RGB8 { r: 013, g: 199, b: 191 },
        RGB8 { r: 000, g: 195, b: 195 },
        RGB8 { r: 000, g: 190, b: 199 },
        RGB8 { r: 000, g: 185, b: 202 },
        RGB8 { r: 000, g: 177, b: 203 },
        RGB8 { r: 000, g: 169, b: 204 },
        RGB8 { r: 000, g: 161, b: 204 },
        RGB8 { r: 000, g: 152, b: 203 },
        RGB8 { r: 013, g: 140, b: 201 },
        RGB8 { r: 028, g: 127, b: 197 },
        RGB8 { r: 040, g: 115, b: 192 },
        RGB8 { r: 050, g: 101, b: 186 },
        RGB8 { r: 069, g: 089, b: 182 },
        RGB8 { r: 082, g: 076, b: 175 },
        RGB8 { r: 093, g: 064, b: 165 },
        RGB8 { r: 101, g: 050, b: 152 },
        RGB8 { r: 111, g: 044, b: 147 },
        RGB8 { r: 120, g: 037, b: 139 },
        RGB8 { r: 129, g: 029, b: 130 },
    ];

    pub const TWELVE_BIT_OKLCH_RAINBOW_WEIGHTED: &[RGB8] = &[
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 137, g: 019, b: 120 },
        RGB8 { r: 147, g: 026, b: 111 },
        RGB8 { r: 156, g: 034, b: 102 },
        RGB8 { r: 164, g: 043, b: 094 },
        RGB8 { r: 170, g: 051, b: 085 },
        RGB8 { r: 179, g: 064, b: 088 },
        RGB8 { r: 187, g: 077, b: 092 },
        RGB8 { r: 196, g: 089, b: 096 },
        RGB8 { r: 203, g: 101, b: 101 },
        RGB8 { r: 212, g: 110, b: 095 },
        RGB8 { r: 220, g: 119, b: 088 },
        RGB8 { r: 227, g: 129, b: 080 },
        RGB8 { r: 232, g: 140, b: 073 },
        RGB8 { r: 236, g: 151, b: 066 },
        RGB8 { r: 241, g: 161, b: 054 },
        RGB8 { r: 243, g: 171, b: 040 },
        RGB8 { r: 245, g: 182, b: 022 },
        RGB8 { r: 244, g: 194, b: 000 },
        RGB8 { r: 241, g: 207, b: 000 },
        RGB8 { r: 236, g: 219, b: 000 },
        RGB8 { r: 223, g: 220, b: 019 },
        RGB8 { r: 210, g: 220, b: 036 },
        RGB8 { r: 196, g: 220, b: 049 },
        RGB8 { r: 181, g: 220, b: 061 },
        RGB8 { r: 167, g: 220, b: 072 },
        RGB8 { r: 151, g: 219, b: 083 },
        RGB8 { r: 137, g: 220, b: 094 },
        RGB8 { r: 122, g: 221, b: 106 },
        RGB8 { r: 106, g: 221, b: 116 },
        RGB8 { r: 089, g: 221, b: 126 },
        RGB8 { r: 068, g: 221, b: 136 },
        RGB8 { r: 036, g: 216, b: 158 },
        RGB8 { r: 016, g: 210, b: 174 },
        RGB8 { r: 032, g: 203, b: 186 },
        RGB8 { r: 000, g: 195, b: 195 },
        RGB8 { r: 000, g: 185, b: 202 },
        RGB8 { r: 000, g: 152, b: 203 },
        RGB8 { r: 050, g: 101, b: 186 },
        RGB8 { r: 082, g: 076, b: 175 },
        RGB8 { r: 101, g: 050, b: 152 },
        RGB8 { r: 114, g: 042, b: 145 },
        RGB8 { r: 126, g: 032, b: 134 },
    ];
}

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

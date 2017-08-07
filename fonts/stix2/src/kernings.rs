use font_types::{KernRecord, KernTable};

pub static KERNING_TABLE: [(u32, KernRecord); 166] = [
    (0x41, KernRecord { // A
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252), fontunit!(352) ],
            kern_values: &[ fontunit!(0), fontunit!(-18), fontunit!(-66) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x45, KernRecord { // E
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(32), fontunit!(31) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126) ],
            kern_values: &[ fontunit!(24), fontunit!(32) ],
        }),
        bottom_left: None,
    }),
    (0x46, KernRecord { // F
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(44) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(289) ],
            kern_values: &[ fontunit!(-200), fontunit!(-44), fontunit!(44) ],
        }),
        bottom_left: None,
    }),
    (0x4A, KernRecord { // J
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(64), fontunit!(-28) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0x4B, KernRecord { // K
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(0), fontunit!(-16) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x4C, KernRecord { // L
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(0), fontunit!(-150) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x4D, KernRecord { // M
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(36) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(36) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(40) ],
        }),
    }),
    (0x4E, KernRecord { // N
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-39), fontunit!(40) ],
        }),
        bottom_left: None,
    }),
    (0x50, KernRecord { // P
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(287) ],
            kern_values: &[ fontunit!(-141), fontunit!(32) ],
        }),
        bottom_left: None,
    }),
    (0x52, KernRecord { // R
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(10), fontunit!(-37) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x54, KernRecord { // T
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(285) ],
            kern_values: &[ fontunit!(-90), fontunit!(40) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[ fontunit!(281) ],
            kern_values: &[ fontunit!(-90), fontunit!(48) ],
        }),
    }),
    (0x55, KernRecord { // U
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(280) ],
            kern_values: &[ fontunit!(-74), fontunit!(-43), fontunit!(28) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(256) ],
            kern_values: &[ fontunit!(-80), fontunit!(-60), fontunit!(36) ],
        }),
    }),
    (0x56, KernRecord { // V
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(280) ],
            kern_values: &[ fontunit!(-193), fontunit!(-119), fontunit!(56) ],
        }),
        bottom_left: None,
    }),
    (0x57, KernRecord { // W
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(280) ],
            kern_values: &[ fontunit!(-176), fontunit!(-96), fontunit!(60) ],
        }),
        bottom_left: None,
    }),
    (0x59, KernRecord { // Y
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(277) ],
            kern_values: &[ fontunit!(-113), fontunit!(56) ],
        }),
        bottom_left: None,
    }),
    (0x61, KernRecord { // a
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(32) ],
        }),
        bottom_left: None,
    }),
    (0x62, KernRecord { // b
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(360) ],
            kern_values: &[ fontunit!(4), fontunit!(-8) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x63, KernRecord { // c
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(23), fontunit!(24) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(20) ],
        }),
        bottom_left: None,
    }),
    (0x64, KernRecord { // d
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x65, KernRecord { // e
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(20) ],
        }),
        bottom_left: None,
    }),
    (0x66, KernRecord { // f
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(82) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x67, KernRecord { // g
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(30) ],
        }),
        bottom_left: None,
    }),
    (0x68, KernRecord { // h
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x69, KernRecord { // i
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0xE3C9, KernRecord { // i.TRK
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x6A, KernRecord { // j
        top_right: None,
        top_left: None,
        bottom_right: None,
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(107) ],
        }),
    }),
    (0x6B, KernRecord { // k
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x6C, KernRecord { // l
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x6D, KernRecord { // m
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x6E, KernRecord { // n
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x71, KernRecord { // q
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(30) ],
        }),
        bottom_left: None,
    }),
    (0x72, KernRecord { // r
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(287) ],
            kern_values: &[ fontunit!(-18), fontunit!(51) ],
        }),
        bottom_left: None,
    }),
    (0x74, KernRecord { // t
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x75, KernRecord { // u
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x76, KernRecord { // v
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(160), fontunit!(256) ],
            kern_values: &[ fontunit!(-98), fontunit!(-53), fontunit!(48) ],
        }),
        bottom_left: None,
    }),
    (0x77, KernRecord { // w
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(135), fontunit!(256) ],
            kern_values: &[ fontunit!(-90), fontunit!(-29), fontunit!(36) ],
        }),
        bottom_left: None,
    }),
    (0x78, KernRecord { // x
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(35) ],
        }),
        bottom_left: None,
    }),
    (0x79, KernRecord { // y
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(137), fontunit!(256) ],
            kern_values: &[ fontunit!(-90), fontunit!(-18), fontunit!(36) ],
        }),
        bottom_left: None,
    }),
    (0x391, KernRecord { // Alpha
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(154), fontunit!(348) ],
            kern_values: &[ fontunit!(77), fontunit!(-45), fontunit!(-102) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x393, KernRecord { // Gamma
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(16) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(316) ],
            kern_values: &[ fontunit!(-165), fontunit!(16) ],
        }),
        bottom_left: None,
    }),
    (0x394, KernRecord { // Delta
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252), fontunit!(348) ],
            kern_values: &[ fontunit!(25), fontunit!(-25), fontunit!(-65) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x39B, KernRecord { // Lambda
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(209), fontunit!(334) ],
            kern_values: &[ fontunit!(8), fontunit!(-25), fontunit!(-78) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x3A1, KernRecord { // Rho
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(246), fontunit!(283) ],
            kern_values: &[ fontunit!(-159), fontunit!(-57), fontunit!(21) ],
        }),
        bottom_left: None,
    }),
    (0x3A4, KernRecord { // Tau
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(404) ],
            kern_values: &[ fontunit!(-71), fontunit!(65) ],
        }),
        bottom_left: None,
    }),
    (0x3A5, KernRecord { // Upsilon
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(465) ],
            kern_values: &[ fontunit!(-86), fontunit!(81) ],
        }),
        bottom_left: None,
    }),
    (0x31, KernRecord { // one
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(0), fontunit!(-50) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x33, KernRecord { // three
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252), fontunit!(344) ],
            kern_values: &[ fontunit!(0), fontunit!(0), fontunit!(-20) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(145) ],
            kern_values: &[ fontunit!(-15), fontunit!(20) ],
        }),
        bottom_left: None,
    }),
    (0x34, KernRecord { // four
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(23), fontunit!(-20) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-20), fontunit!(29) ],
        }),
        bottom_left: None,
    }),
    (0x35, KernRecord { // five
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(152) ],
            kern_values: &[ fontunit!(-15), fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x36, KernRecord { // six
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(340) ],
            kern_values: &[ fontunit!(9), fontunit!(-15) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-10), fontunit!(20) ],
        }),
        bottom_left: None,
    }),
    (0x37, KernRecord { // seven
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(289) ],
            kern_values: &[ fontunit!(-33), fontunit!(20) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(145), fontunit!(297) ],
            kern_values: &[ fontunit!(-123), fontunit!(-65), fontunit!(45) ],
        }),
        bottom_left: None,
    }),
    (0x38, KernRecord { // eight
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(160) ],
            kern_values: &[ fontunit!(-10), fontunit!(33) ],
        }),
        bottom_left: None,
    }),
    (0x39, KernRecord { // nine
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(15) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(145), fontunit!(283) ],
            kern_values: &[ fontunit!(-30), fontunit!(17), fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x2131, KernRecord { // uni2131
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(148), fontunit!(271) ],
            kern_values: &[ fontunit!(-221), fontunit!(-84), fontunit!(100) ],
        }),
        bottom_left: None,
    }),
    (0x1D400, KernRecord { // u1D400
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(180), fontunit!(346) ],
            kern_values: &[ fontunit!(31), fontunit!(-32), fontunit!(-76) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[ fontunit!(141), fontunit!(340) ],
            kern_values: &[ fontunit!(124), fontunit!(-24), fontunit!(-98) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D402, KernRecord { // u1D402
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D404, KernRecord { // u1D404
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(30) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D405, KernRecord { // u1D405
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(40) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(139), fontunit!(280) ],
            kern_values: &[ fontunit!(-92), fontunit!(-18), fontunit!(100) ],
        }),
        bottom_left: None,
    }),
    (0x1D409, KernRecord { // u1D409
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-20) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D40B, KernRecord { // u1D40B
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(12), fontunit!(-144) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D40E, KernRecord { // u1D40E
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(221), fontunit!(287) ],
            kern_values: &[ fontunit!(-20), fontunit!(18), fontunit!(72) ],
        }),
        bottom_left: None,
    }),
    (0x1D40F, KernRecord { // u1D40F
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(280) ],
            kern_values: &[ fontunit!(-96), fontunit!(104) ],
        }),
        bottom_left: None,
    }),
    (0x1D411, KernRecord { // u1D411
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(98) ],
            kern_values: &[ fontunit!(10), fontunit!(-33) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D413, KernRecord { // u1D413
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(280) ],
            kern_values: &[ fontunit!(-40), fontunit!(100) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[ fontunit!(280) ],
            kern_values: &[ fontunit!(-32), fontunit!(108) ],
        }),
    }),
    (0x1D415, KernRecord { // u1D415
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(35) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(45) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(135), fontunit!(280) ],
            kern_values: &[ fontunit!(-152), fontunit!(-96), fontunit!(59) ],
        }),
        bottom_left: None,
    }),
    (0x1D416, KernRecord { // u1D416
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(35) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(45) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(145), fontunit!(280) ],
            kern_values: &[ fontunit!(-98), fontunit!(-64), fontunit!(35) ],
        }),
        bottom_left: None,
    }),
    (0x1D418, KernRecord { // u1D418
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(35) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(45) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(280) ],
            kern_values: &[ fontunit!(-70), fontunit!(120) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[ fontunit!(280) ],
            kern_values: &[ fontunit!(-52), fontunit!(124) ],
        }),
    }),
    (0x1D434, KernRecord { // u1D434
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(213), fontunit!(350) ],
            kern_values: &[ fontunit!(58), fontunit!(-58), fontunit!(-70) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(86) ],
        }),
    }),
    (0x1D435, KernRecord { // u1D435
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[ fontunit!(176) ],
            kern_values: &[ fontunit!(81), fontunit!(-50) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(160), fontunit!(283) ],
            kern_values: &[ fontunit!(-50), fontunit!(-12), fontunit!(20) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(76) ],
        }),
    }),
    (0x1D436, KernRecord { // u1D436
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(316) ],
            kern_values: &[ fontunit!(-32), fontunit!(56) ],
        }),
        bottom_left: None,
    }),
    (0x1D437, KernRecord { // u1D437
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(273), fontunit!(305) ],
            kern_values: &[ fontunit!(-43), fontunit!(8), fontunit!(20) ],
        }),
        bottom_left: None,
    }),
    (0x1D438, KernRecord { // u1D438
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(166), fontunit!(309) ],
            kern_values: &[ fontunit!(-36), fontunit!(-24), fontunit!(44) ],
        }),
        bottom_left: None,
    }),
    (0x1D439, KernRecord { // u1D439
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(162), fontunit!(309) ],
            kern_values: &[ fontunit!(-160), fontunit!(-56), fontunit!(104) ],
        }),
        bottom_left: None,
    }),
    (0x1D43A, KernRecord { // u1D43A
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(170), fontunit!(283) ],
            kern_values: &[ fontunit!(-45), fontunit!(36), fontunit!(40) ],
        }),
        bottom_left: None,
    }),
    (0x1D43D, KernRecord { // u1D43D
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(178), fontunit!(332) ],
            kern_values: &[ fontunit!(-84), fontunit!(-27), fontunit!(104) ],
        }),
        bottom_left: None,
    }),
    (0x1D43E, KernRecord { // u1D43E
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x1D43F, KernRecord { // u1D43F
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(219) ],
            kern_values: &[ fontunit!(-2), fontunit!(-109) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D441, KernRecord { // u1D441
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(180), fontunit!(283) ],
            kern_values: &[ fontunit!(-70), fontunit!(-45), fontunit!(108) ],
        }),
        bottom_left: None,
    }),
    (0x1D442, KernRecord { // u1D442
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(160), fontunit!(283) ],
            kern_values: &[ fontunit!(-30), fontunit!(20), fontunit!(36) ],
        }),
        bottom_left: None,
    }),
    (0x1D443, KernRecord { // u1D443
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(277) ],
            kern_values: &[ fontunit!(-219), fontunit!(-70) ],
        }),
        bottom_left: None,
    }),
    (0x1D444, KernRecord { // u1D444
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(139), fontunit!(291) ],
            kern_values: &[ fontunit!(-43), fontunit!(23), fontunit!(56) ],
        }),
        bottom_left: None,
    }),
    (0x1D445, KernRecord { // u1D445
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x1D446, KernRecord { // u1D446
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(164), fontunit!(283) ],
            kern_values: &[ fontunit!(-10), fontunit!(35), fontunit!(76) ],
        }),
        bottom_left: None,
    }),
    (0x1D447, KernRecord { // u1D447
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(30) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(313) ],
            kern_values: &[ fontunit!(-100), fontunit!(114) ],
        }),
        bottom_left: None,
    }),
    (0x1D448, KernRecord { // u1D448
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(162), fontunit!(283) ],
            kern_values: &[ fontunit!(-96), fontunit!(-52), fontunit!(116) ],
        }),
        bottom_left: None,
    }),
    (0x1D449, KernRecord { // u1D449
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(156), fontunit!(280) ],
            kern_values: &[ fontunit!(-222), fontunit!(-118), fontunit!(202) ],
        }),
        bottom_left: None,
    }),
    (0x1D44A, KernRecord { // u1D44A
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(70) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-11) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(160), fontunit!(280) ],
            kern_values: &[ fontunit!(-180), fontunit!(-66), fontunit!(242) ],
        }),
        bottom_left: None,
    }),
    (0x1D44B, KernRecord { // u1D44B
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D44C, KernRecord { // u1D44C
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(280) ],
            kern_values: &[ fontunit!(-107), fontunit!(234) ],
        }),
        bottom_left: None,
    }),
    (0x1D44D, KernRecord { // u1D44D
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D44F, KernRecord { // u1D44F
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(154) ],
            kern_values: &[ fontunit!(-15), fontunit!(20) ],
        }),
        bottom_left: None,
    }),
    (0x1D450, KernRecord { // u1D450
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(154) ],
            kern_values: &[ fontunit!(-10), fontunit!(44) ],
        }),
        bottom_left: None,
    }),
    (0x1D451, KernRecord { // u1D451
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(299) ],
            kern_values: &[ fontunit!(2), fontunit!(33) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D452, KernRecord { // u1D452
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(0) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-10), fontunit!(36) ],
        }),
        bottom_left: None,
    }),
    (0x1D453, KernRecord { // u1D453
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(156), fontunit!(283) ],
            kern_values: &[ fontunit!(-199), fontunit!(-148), fontunit!(16) ],
        }),
        bottom_left: None,
    }),
    (0xE651, KernRecord { // u1D454.alt
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(148) ],
            kern_values: &[ fontunit!(-12), fontunit!(0) ],
        }),
        bottom_left: None,
    }),
    (0x1D456, KernRecord { // u1D456
        top_right: None,
        top_left: None,
        bottom_right: None,
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(12) ],
        }),
    }),
    (0x1D457, KernRecord { // u1D457
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-143) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(293) ],
            kern_values: &[ fontunit!(-16), fontunit!(48) ],
        }),
        bottom_left: None,
    }),
    (0x1D45C, KernRecord { // u1D45C
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(287) ],
            kern_values: &[ fontunit!(-12), fontunit!(41) ],
        }),
        bottom_left: None,
    }),
    (0x1D45D, KernRecord { // u1D45D
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(287) ],
            kern_values: &[ fontunit!(-8), fontunit!(32) ],
        }),
        bottom_left: None,
    }),
    (0x1D45F, KernRecord { // u1D45F
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(162), fontunit!(283) ],
            kern_values: &[ fontunit!(-166), fontunit!(-98), fontunit!(64) ],
        }),
        bottom_left: None,
    }),
    (0x1D461, KernRecord { // u1D461
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(268) ],
            kern_values: &[ fontunit!(-27), fontunit!(32) ],
        }),
        bottom_left: None,
    }),
    (0x1D463, KernRecord { // u1D463
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-25), fontunit!(31), fontunit!(40) ],
        }),
        bottom_left: None,
    }),
    (0x1D464, KernRecord { // u1D464
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-25), fontunit!(31), fontunit!(32) ],
        }),
        bottom_left: None,
    }),
    (0x1D466, KernRecord { // u1D466
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-45), fontunit!(24), fontunit!(60) ],
        }),
        bottom_left: None,
    }),
    (0x1D467, KernRecord { // u1D467
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(271) ],
            kern_values: &[ fontunit!(-6), fontunit!(25) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D473, KernRecord { // u1D473
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(23), fontunit!(-74) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D47B, KernRecord { // u1D47B
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(340) ],
            kern_values: &[ fontunit!(-160), fontunit!(92) ],
        }),
        bottom_left: None,
    }),
    (0x1D47D, KernRecord { // u1D47D
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(283) ],
            kern_values: &[ fontunit!(-256), fontunit!(-178), fontunit!(100) ],
        }),
        bottom_left: None,
    }),
    (0x1D47E, KernRecord { // u1D47E
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(287) ],
            kern_values: &[ fontunit!(-205), fontunit!(-139), fontunit!(100) ],
        }),
        bottom_left: None,
    }),
    (0x1D480, KernRecord { // u1D480
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(285) ],
            kern_values: &[ fontunit!(-192), fontunit!(96) ],
        }),
        bottom_left: None,
    }),
    (0x1D49C, KernRecord { // u1D49C
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-50) ],
        }),
        bottom_right: None,
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(47) ],
        }),
    }),
    (0x1D4A5, KernRecord { // u1D4A5
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(256) ],
            kern_values: &[ fontunit!(-83), fontunit!(-51), fontunit!(69) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(43) ],
        }),
    }),
    (0x1D4A9, KernRecord { // u1D4A9
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(256) ],
            kern_values: &[ fontunit!(-217), fontunit!(-193), fontunit!(41) ],
        }),
        bottom_left: None,
    }),
    (0x1D4AB, KernRecord { // u1D4AB
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(25) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-25) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(365) ],
            kern_values: &[ fontunit!(-284), fontunit!(-155), fontunit!(31) ],
        }),
        bottom_left: None,
    }),
    (0x1D4AF, KernRecord { // u1D4AF
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-15) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(316) ],
            kern_values: &[ fontunit!(-320), fontunit!(-246), fontunit!(25) ],
        }),
        bottom_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(15) ],
        }),
    }),
    (0x1D4B1, KernRecord { // u1D4B1
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-20) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(270) ],
            kern_values: &[ fontunit!(-197), fontunit!(-89), fontunit!(111) ],
        }),
        bottom_left: None,
    }),
    (0x1D4B2, KernRecord { // u1D4B2
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-10) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(270) ],
            kern_values: &[ fontunit!(-153), fontunit!(-65), fontunit!(103) ],
        }),
        bottom_left: None,
    }),
    (0x1D4B3, KernRecord { // u1D4B3
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-15) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(270) ],
            kern_values: &[ fontunit!(-62), fontunit!(-17), fontunit!(55) ],
        }),
        bottom_left: None,
    }),
    (0x1D4B4, KernRecord { // u1D4B4
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-15) ],
        }),
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(130), fontunit!(270) ],
            kern_values: &[ fontunit!(-41), fontunit!(-1), fontunit!(75) ],
        }),
        bottom_left: None,
    }),
    (0x1D4C5, KernRecord { // u1D4C5
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-25) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D4D5, KernRecord { // u1D4D5
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(141), fontunit!(268) ],
            kern_values: &[ fontunit!(-270), fontunit!(-145), fontunit!(4) ],
        }),
        bottom_left: None,
    }),
    (0x1D4E3, KernRecord { // u1D4E3
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(166), fontunit!(256) ],
            kern_values: &[ fontunit!(-326), fontunit!(-292), fontunit!(16) ],
        }),
        bottom_left: None,
    }),
    (0x1D4E5, KernRecord { // u1D4E5
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(146), fontunit!(299) ],
            kern_values: &[ fontunit!(-387), fontunit!(-308), fontunit!(16) ],
        }),
        bottom_left: None,
    }),
    (0x1D4E6, KernRecord { // u1D4E6
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(146), fontunit!(313) ],
            kern_values: &[ fontunit!(-377), fontunit!(-304), fontunit!(16) ],
        }),
        bottom_left: None,
    }),
    (0x1D6FD, KernRecord { // u1D6FD
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(158), fontunit!(283) ],
            kern_values: &[ fontunit!(-25), fontunit!(14), fontunit!(45) ],
        }),
        bottom_left: None,
    }),
    (0x1D6FE, KernRecord { // u1D6FE
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-150), fontunit!(-63), fontunit!(57) ],
        }),
        bottom_left: None,
    }),
    (0x1D6FF, KernRecord { // u1D6FF
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-29), fontunit!(-3), fontunit!(53) ],
        }),
        bottom_left: None,
    }),
    (0x1D701, KernRecord { // u1D701
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-14) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(346) ],
            kern_values: &[ fontunit!(-85), fontunit!(41) ],
        }),
        bottom_left: None,
    }),
    (0x1D703, KernRecord { // u1D703
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(154), fontunit!(283) ],
            kern_values: &[ fontunit!(-47), fontunit!(5), fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x1D706, KernRecord { // u1D706
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252) ],
            kern_values: &[ fontunit!(0), fontunit!(-84) ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D708, KernRecord { // u1D708
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(152), fontunit!(283) ],
            kern_values: &[ fontunit!(-85), fontunit!(14), fontunit!(49) ],
        }),
        bottom_left: None,
    }),
    (0x1D709, KernRecord { // u1D709
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-7) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(225) ],
            kern_values: &[ fontunit!(-49), fontunit!(-16) ],
        }),
        bottom_left: None,
    }),
    (0x1D70A, KernRecord { // u1D70A
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(152), fontunit!(283) ],
            kern_values: &[ fontunit!(-11), fontunit!(17), fontunit!(21) ],
        }),
        bottom_left: None,
    }),
    (0x1D70B, KernRecord { // u1D70B
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(10) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-47), fontunit!(0) ],
        }),
        bottom_left: None,
    }),
    (0x1D70C, KernRecord { // u1D70C
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-25), fontunit!(17), fontunit!(21) ],
        }),
        bottom_left: None,
    }),
    (0x1D70D, KernRecord { // u1D70D
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(283) ],
            kern_values: &[ fontunit!(-59), fontunit!(53) ],
        }),
        bottom_left: None,
    }),
    (0x1D70E, KernRecord { // u1D70E
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(13) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(145), fontunit!(283) ],
            kern_values: &[ fontunit!(-95), fontunit!(-61), fontunit!(57) ],
        }),
        bottom_left: None,
    }),
    (0x1D70F, KernRecord { // u1D70F
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(10) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(281) ],
            kern_values: &[ fontunit!(-105), fontunit!(-66), fontunit!(8) ],
        }),
        bottom_left: None,
    }),
    (0x1D710, KernRecord { // u1D710
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-7), fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x1D711, KernRecord { // u1D711
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-27), fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x1D712, KernRecord { // u1D712
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(152) ],
            kern_values: &[ fontunit!(-72), fontunit!(-23) ],
        }),
        bottom_left: None,
    }),
    (0x1D713, KernRecord { // u1D713
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(154) ],
            kern_values: &[ fontunit!(-31), fontunit!(25) ],
        }),
        bottom_left: None,
    }),
    (0x1D714, KernRecord { // u1D714
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(160) ],
            kern_values: &[ fontunit!(-15), fontunit!(29) ],
        }),
        bottom_left: None,
    }),
    (0x1D715, KernRecord { // u1D715
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(156), fontunit!(283) ],
            kern_values: &[ fontunit!(-39), fontunit!(5), fontunit!(21) ],
        }),
        bottom_left: None,
    }),
    (0x1D716, KernRecord { // u1D716
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-16), fontunit!(14) ],
        }),
        bottom_left: None,
    }),
    (0x1D717, KernRecord { // u1D717
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(154), fontunit!(283) ],
            kern_values: &[ fontunit!(-63), fontunit!(-14), fontunit!(41) ],
        }),
        bottom_left: None,
    }),
    (0x1D718, KernRecord { // u1D718
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(152) ],
            kern_values: &[ fontunit!(-27), fontunit!(10) ],
        }),
        bottom_left: None,
    }),
    (0x1D719, KernRecord { // u1D719
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-20), fontunit!(29) ],
        }),
        bottom_left: None,
    }),
    (0x1D71A, KernRecord { // u1D71A
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150) ],
            kern_values: &[ fontunit!(-27), fontunit!(21) ],
        }),
        bottom_left: None,
    }),
    (0x1D71B, KernRecord { // u1D71B
        top_right: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(10) ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(150), fontunit!(283) ],
            kern_values: &[ fontunit!(-87), fontunit!(-51), fontunit!(57) ],
        }),
        bottom_left: None,
    }),
    (0xE6E8, KernRecord { // u1D434.ssty
        top_right: Some(KernTable {
            correction_heights: &[ fontunit!(252), fontunit!(346) ],
            kern_values: &[ fontunit!(0), fontunit!(-27), fontunit!(-61) ],
        }),
        top_left: Some(KernTable {
            correction_heights: &[ fontunit!(348) ],
            kern_values: &[ fontunit!(23), fontunit!(-125) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE6FA, KernRecord { // u1D446.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-23) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE6FC, KernRecord { // u1D448.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-23) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE6FF, KernRecord { // u1D44B.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-23) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE707, KernRecord { // u1D453.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-80) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE70D, KernRecord { // u1D457.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-80) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE714, KernRecord { // u1D45D.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[ fontunit!(203) ],
            kern_values: &[ fontunit!(-23), fontunit!(-74) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE717, KernRecord { // u1D460.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[ fontunit!(234) ],
            kern_values: &[ fontunit!(4), fontunit!(-27) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE71F, KernRecord { // u1D465.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-43) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE720, KernRecord { // u1D466.ssty
        top_right: None,
        top_left: Some(KernTable {
            correction_heights: &[  ],
            kern_values: &[ fontunit!(-23) ],
        }),
        bottom_right: None,
        bottom_left: None,
    }),
    (0xE865, KernRecord { // uni2131.cal
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(268) ],
            kern_values: &[ fontunit!(-144), fontunit!(-80), fontunit!(260) ],
        }),
        bottom_left: None,
    }),
    (0xE86F, KernRecord { // u1D4AB.cal
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(126), fontunit!(277) ],
            kern_values: &[ fontunit!(-208), fontunit!(-156), fontunit!(156) ],
        }),
        bottom_left: None,
    }),
    (0xE873, KernRecord { // u1D4AF.cal
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(141), fontunit!(270) ],
            kern_values: &[ fontunit!(-88), fontunit!(-32), fontunit!(284) ],
        }),
        bottom_left: None,
    }),
    (0xE875, KernRecord { // u1D4B1.cal
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(145), fontunit!(275) ],
            kern_values: &[ fontunit!(-148), fontunit!(-36), fontunit!(164) ],
        }),
        bottom_left: None,
    }),
    (0xE876, KernRecord { // u1D4B2.cal
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: &[ fontunit!(139), fontunit!(279) ],
            kern_values: &[ fontunit!(-100), fontunit!(-8), fontunit!(164) ],
        }),
        bottom_left: None,
    }),

];
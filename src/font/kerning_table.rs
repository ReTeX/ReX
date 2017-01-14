#![allow(dead_code)]
use super::{ KernRecord, KernTable };

pub static KERNING_TABLE: [(u32, KernRecord); 29] = [
    (0x41, KernRecord { // A
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        bottom_left: None,
    }),
    (0x46, KernRecord { // F
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x4C, KernRecord { // L
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        bottom_left: None,
    }),
    (0x50, KernRecord { // P
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x54, KernRecord { // T
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -100, 0 ],
        }),
        bottom_left: None,
    }),
    (0x56, KernRecord { // V
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x57, KernRecord { // W
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x59, KernRecord { // Y
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x391, KernRecord { // uni0391
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        bottom_left: None,
    }),
    (0x393, KernRecord { // uni0393
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x394, KernRecord { // uni0394
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x3A1, KernRecord { // uni03A1
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x3A4, KernRecord { // uni03A4
        top_right: None,
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -110, 0 ],
        }),
        bottom_left: None,
    }),
    (0x3A5, KernRecord { // uni03A5
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -120, 0 ],
        }),
        bottom_left: None,
    }),
    (0x3BB, KernRecord { // uni03BB
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        bottom_left: None,
    }),
    (0x3D2, KernRecord { // uni03D2
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -120, 0 ],
        }),
        bottom_left: None,
    }),
    (0x3DC, KernRecord { // uni03DC
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D400, KernRecord { // u1D400
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 30, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D405, KernRecord { // u1D405
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D40B, KernRecord { // u1D40B
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        top_left: None,
        bottom_right: None,
        bottom_left: None,
    }),
    (0x1D40F, KernRecord { // u1D40F
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D413, KernRecord { // u1D413
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 30, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -120, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D418, KernRecord { // u1D418
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -100, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D6A8, KernRecord { // u1D6A8
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 30, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D6AA, KernRecord { // u1D6AA
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D6AB, KernRecord { // u1D6AB
        top_right: Some(KernTable {
            correction_heights: [ 275 ],
            kern_values: [ -100, -150 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 30, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D6BB, KernRecord { // u1D6BB
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -100, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D6BC, KernRecord { // u1D6BC
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -100, 0 ],
        }),
        bottom_left: None,
    }),
    (0x1D6C1, KernRecord { // u1D6C1
        top_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ 50, 0 ],
        }),
        top_left: None,
        bottom_right: Some(KernTable {
            correction_heights: [ 0 ],
            kern_values: [ -150, 0 ],
        }),
        bottom_left: None,
    }),
];
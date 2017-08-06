macro_rules! fontunit {
    ($n:expr) => (
        ::font_types::FontUnit { bits: $n << 8u8 }
    )
}

macro_rules! fontunit_raw {
    ($n:expr) => (
        ::font_types::FontUnit { bits: $n }
    )
}
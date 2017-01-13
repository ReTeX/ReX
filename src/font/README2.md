// Legal unicode points in XML (cf section 7.2)
// https://www.w3.org/TR/MathML/chapter7.html

#x9 | #xA | #xD | [#x20-#xD7FF] | [#xE000-#xFFFD] | [#x10000-#x10FFFF]


// Replacements.  See section 7.5 in w3 MathML
// https://www.w3.org/TR/MathML/chapter7.html

PLANCK CONSTANT:           U+1D455  => U+210E
SCRIPT CAPITAL B:          U+1D49D  => U+212C
SCRIPT CAPITAL E:          U+1D4A0  => U+2130
SCRIPT CAPITAL F:          U+1D4A1  => U+2131
SCRIPT CAPITAL H:          U+1D4A3  => U+210B
SCRIPT CAPITAL I:          U+1D4A4  => U+2110
SCRIPT CAPITAL L:          U+1D4A7  => U+2112
SCRIPT CAPITAL M:          U+1D4A8  => U+2133
SCRIPT CAPITAL R:          U+1D4AD  => U+211B
SCRIPT SMALL E:            U+1D4BA  => U+212F
SCRIPT SMALL G:            U+1D4BC  => U+210A
SCRIPT SMALL O:            U+1D4C4  => U+2134
BLACK-LETTER CAPITAL C:    U+1D506  => U+212D
BLACK-LETTER CAPITAL H:    U+1D50B  => U+210C
BLACK-LETTER CAPITAL I:    U+1D50C  => U+2111
BLACK-LETTER CAPITAL R:    U+1D515  => U+211C
BLACK-LETTER CAPITAL Z:    U+1D51D  => U+2128
DOUBLE-STRUCK CAPITAL C:   U+1D53A  => U+2102
DOUBLE-STRUCK CAPITAL H:   U+1D53F  => U+210D
DOUBLE-STRUCK CAPITAL N:   U+1D545  => U+2115
DOUBLE-STRUCK CAPITAL P:   U+1D547  => U+2119
DOUBLE-STRUCK CAPITAL Q:   U+1D548  => U+211A
DOUBLE-STRUCK CAPITAL R:   U+1D549  => U+211D
DOUBLE-STRUCK CAPITAL Z:   U+1D551  => U+2124

// See Section 7.6 for discussions on non-marking characters
// which can increase accessibility. (TODO: ??)

// Combining Characters x338 (/), x20D2 (|), x20E5 (\).

// Anomalous characters.

HYPHEN-MINUS (0x2D)
    MATH => 0x2212
    TEXT => 0x2010

APOSTROPHE   (0x27)
    MATH => 0x2032
    TEXT -> 0x27

MULTIPLE-APOSTROPHE (0x27, 0x27, ...)
    1x => 0x2032
    2x => 0x2033
    3x => 0x2034
    4x => 0x2057

VERTICAL-BAR (0x7C)
    MATH => "\vert"
    TEXT => 

// Section 7.7.2  Pseudo-Scripts. Ignore???

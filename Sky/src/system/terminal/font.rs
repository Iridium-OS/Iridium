// Copyright (c) ChefKiss 2021-2024. Licensed under the Thou Shalt Not Profit License version 1.5. See LICENSE for details.

pub const FONT_WIDTH: usize = 10;
pub const FONT_HEIGHT: usize = 17;

pub static FONT_BITMAP: &[[u8; FONT_HEIGHT * 2]] = &[
    //  32 $20 'space'
    //	width 10, bbx 0, bby 0, bbw 1, bbh 1
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  33 $21 'exclam'
    //	width 10, bbx 4, bby 0, bbw 2, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x08, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  34 $22 'quotedbl'
    //	width 10, bbx 2, bby 7, bbw 5, bbh 5
    [
        0x00, 0x00, 0x00, 0x00, 0x32, 0x00, 0x32, 0x00, 0x32, 0x00, 0x32, 0x00, 0x32, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  35 $23 'numbersign'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x11, 0x00, 0x11, 0x00, 0x11, 0x00, 0x7F, 0x80, 0x12,
        0x00, 0x32, 0x00, 0x22, 0x00, 0x7F, 0x00, 0x22, 0x00, 0x24, 0x00, 0x64, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  36 $24 'dollar'
    //	width 10, bbx 1, bby -2, bbw 7, bbh 16
    [
        0x08, 0x00, 0x08, 0x00, 0x1E, 0x00, 0x3B, 0x00, 0x29, 0x00, 0x68, 0x00, 0x28, 0x00, 0x3C,
        0x00, 0x0E, 0x00, 0x0B, 0x00, 0x09, 0x00, 0x69, 0x00, 0x2B, 0x00, 0x3E, 0x00, 0x08, 0x00,
        0x08, 0x00, 0x00, 0x00,
    ],
    //  37 $25 'glyph9'
    //	width 10, bbx 0, bby 0, bbw 9, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x70, 0x80, 0xD9, 0x80, 0x99, 0x00, 0x9A, 0x00, 0xD6, 0x00, 0x24,
        0x00, 0x08, 0x00, 0x17, 0x80, 0x14, 0x80, 0x24, 0x80, 0x44, 0x80, 0xC3, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  38 $26 'glyph10'
    //	width 10, bbx 0, bby 0, bbw 10, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x36, 0x00, 0x22, 0x00, 0x20, 0x00, 0x30, 0x00, 0x38,
        0x00, 0x6C, 0xC0, 0xC5, 0x80, 0xC3, 0x00, 0xC3, 0x00, 0x45, 0x80, 0x78, 0xC0, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  39 $27 'quotesingle'
    //	width 10, bbx 4, bby 7, bbw 2, bbh 5
    [
        0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  40 $28 'parenleft'
    //	width 10, bbx 3, bby -2, bbw 5, bbh 15
    [
        0x00, 0x00, 0x03, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x18, 0x00, 0x18, 0x00, 0x10, 0x00, 0x10,
        0x00, 0x10, 0x00, 0x10, 0x00, 0x10, 0x00, 0x18, 0x00, 0x18, 0x00, 0x0C, 0x00, 0x07, 0x00,
        0x03, 0x00, 0x00, 0x00,
    ],
    //  41 $29 'parenright'
    //	width 10, bbx 2, bby -2, bbw 5, bbh 15
    [
        0x00, 0x00, 0x30, 0x00, 0x18, 0x00, 0x0C, 0x00, 0x04, 0x00, 0x06, 0x00, 0x06, 0x00, 0x06,
        0x00, 0x06, 0x00, 0x06, 0x00, 0x06, 0x00, 0x04, 0x00, 0x04, 0x00, 0x0C, 0x00, 0x38, 0x00,
        0x20, 0x00, 0x00, 0x00,
    ],
    //  42 $2a 'glyph14'
    //	width 10, bbx 1, bby 2, bbw 8, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x48, 0x80, 0x7B,
        0x80, 0x1C, 0x00, 0x1C, 0x00, 0x36, 0x00, 0x23, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  43 $2b 'glyph15'
    //	width 10, bbx 1, bby 1, bbw 8, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x7F, 0x80, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  44 $2c 'glyph16'
    //	width 10, bbx 3, bby -3, bbw 3, bbh 5
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x08, 0x00,
        0x08, 0x00, 0x18, 0x00,
    ],
    //  45 $2d 'hyphen'
    //	width 10, bbx 2, bby 5, bbw 5, bbh 1
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x3E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  46 $2e 'period'
    //	width 10, bbx 3, bby 0, bbw 3, bbh 2
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  47 $2f 'glyph19'
    //	width 10, bbx 1, bby -2, bbw 7, bbh 15
    [
        0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x02, 0x00, 0x02, 0x00, 0x06, 0x00, 0x04, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x08, 0x00, 0x18, 0x00, 0x10, 0x00, 0x10, 0x00, 0x30, 0x00, 0x20, 0x00,
        0x60, 0x00, 0x00, 0x00,
    ],
    //  48 $30 'zero'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x33, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x6D,
        0x00, 0x6D, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x23, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  49 $31 'one'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x34, 0x00, 0x64, 0x00, 0x44, 0x00, 0x04, 0x00, 0x04,
        0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x7F, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  50 $32 'two'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x37, 0x00, 0x61, 0x00, 0x61, 0x00, 0x01, 0x00, 0x03,
        0x00, 0x06, 0x00, 0x0C, 0x00, 0x18, 0x00, 0x30, 0x00, 0x20, 0x00, 0x7F, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  51 $33 'three'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x7F, 0x00, 0x06, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x1E,
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x63, 0x00, 0x3E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  52 $34 'four'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x18, 0x00, 0x10, 0x00, 0x33,
        0x00, 0x63, 0x00, 0x63, 0x00, 0x7F, 0x00, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  53 $35 'five'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x3F, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x3E,
        0x00, 0x03, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x23, 0x00, 0x3E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  54 $36 'six'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x18, 0x00, 0x30, 0x00, 0x3E,
        0x00, 0x63, 0x00, 0x61, 0x00, 0x41, 0x80, 0x61, 0x00, 0x23, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  55 $37 'seven'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7F, 0x80, 0x61, 0x80, 0x61, 0x00, 0x03, 0x00, 0x03, 0x00, 0x02,
        0x00, 0x06, 0x00, 0x04, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  56 $38 'eight'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x37, 0x00, 0x61, 0x00, 0x61, 0x00, 0x23, 0x00, 0x1E,
        0x00, 0x3F, 0x00, 0x61, 0x00, 0x61, 0x80, 0x61, 0x00, 0x23, 0x00, 0x3E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  57 $39 'nine'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x37, 0x00, 0x61, 0x00, 0x41, 0x80, 0x61, 0x80, 0x61,
        0x00, 0x3F, 0x00, 0x0A, 0x00, 0x06, 0x00, 0x04, 0x00, 0x0C, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  58 $3a 'colon'
    //	width 10, bbx 3, bby 0, bbw 3, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x1C, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  59 $3b 'semicolon'
    //	width 10, bbx 3, bby -3, bbw 3, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x1C, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x08, 0x00, 0x08, 0x00,
        0x18, 0x00, 0x18, 0x00,
    ],
    //  60 $3c 'less'
    //	width 10, bbx 1, bby 1, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x0E, 0x00, 0x38,
        0x00, 0x60, 0x00, 0x70, 0x00, 0x1C, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  61 $3d 'glyph33'
    //	width 10, bbx 1, bby 3, bbw 7, bbh 5
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  62 $3e 'greater'
    //	width 10, bbx 1, bby 1, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x60, 0x00, 0x18, 0x00, 0x06,
        0x00, 0x01, 0x00, 0x03, 0x00, 0x0C, 0x00, 0x70, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  63 $3f 'glyph35'
    //	width 10, bbx 2, bby 0, bbw 6, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x38, 0x00, 0x3E, 0x00, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x06,
        0x00, 0x1C, 0x00, 0x18, 0x00, 0x18, 0x00, 0x00, 0x00, 0x18, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  64 $40 'glyph36'
    //	width 10, bbx 1, bby -3, bbw 8, bbh 15
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x33, 0x00, 0x61, 0x80, 0x40, 0x80, 0x46, 0x80, 0x4D,
        0x80, 0x48, 0x80, 0x48, 0x80, 0x48, 0x80, 0x49, 0x80, 0x47, 0x00, 0x40, 0x00, 0x60, 0x00,
        0x30, 0x00, 0x1E, 0x00,
    ],
    //  65 $41 'A'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x1C, 0x00, 0x16, 0x00, 0x16, 0x00, 0x32,
        0x00, 0x32, 0x00, 0x23, 0x00, 0x3F, 0x00, 0x61, 0x00, 0x61, 0x00, 0x41, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  66 $42 'B'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7C, 0x00, 0x7F, 0x00, 0x61, 0x00, 0x61, 0x00, 0x63, 0x00, 0x7E,
        0x00, 0x7F, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x63, 0x00, 0x7E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  67 $43 'C'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x37, 0x00, 0x21, 0x00, 0x61, 0x00, 0x60, 0x00, 0x60,
        0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x21, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  68 $44 'D'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7C, 0x00, 0x7E, 0x00, 0x63, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x63, 0x00, 0x7E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  69 $45 'E'
    //	width 10, bbx 2, bby 0, bbw 6, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x3F,
        0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x3F, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  70 $46 'F'
    //	width 10, bbx 2, bby 0, bbw 6, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20,
        0x00, 0x3F, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  71 $47 'G'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x37, 0x00, 0x21, 0x00, 0x61, 0x00, 0x60, 0x00, 0x60,
        0x00, 0x6F, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  72 $48 'H'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x7F,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  73 $49 'I'
    //	width 10, bbx 2, bby 0, bbw 6, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x3F, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  74 $4a 'J'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x03,
        0x00, 0x03, 0x00, 0x03, 0x00, 0x03, 0x00, 0x43, 0x00, 0x62, 0x00, 0x3E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  75 $4b 'K'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x63, 0x00, 0x62, 0x00, 0x66, 0x00, 0x64, 0x00, 0x7C,
        0x00, 0x64, 0x00, 0x66, 0x00, 0x62, 0x00, 0x63, 0x00, 0x61, 0x00, 0x61, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  76 $4c 'L'
    //	width 10, bbx 2, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30,
        0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x30, 0x00, 0x3F, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  77 $4d 'M'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x63, 0x00, 0x73, 0x00, 0x53, 0x00, 0x75, 0x00, 0x7D, 0x00, 0x6D,
        0x00, 0x69, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  78 $4e 'N'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x61, 0x00, 0x71, 0x00, 0x71, 0x00, 0x71, 0x00, 0x69, 0x00, 0x69,
        0x00, 0x6D, 0x00, 0x65, 0x00, 0x65, 0x00, 0x67, 0x00, 0x63, 0x00, 0x63, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  79 $4f 'O'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x37, 0x00, 0x21, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  80 $50 'P'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7E, 0x00, 0x7F, 0x00, 0x61, 0x80, 0x61, 0x80, 0x61, 0x80, 0x61,
        0x00, 0x7F, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  81 $51 'Q'
    //	width 10, bbx 1, bby -3, bbw 7, bbh 15
    [
        0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x37, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x06, 0x00,
        0x03, 0x00, 0x01, 0x00,
    ],
    //  82 $52 'R'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7C, 0x00, 0x7F, 0x00, 0x61, 0x00, 0x61, 0x80, 0x61, 0x00, 0x63,
        0x00, 0x7E, 0x00, 0x66, 0x00, 0x62, 0x00, 0x63, 0x00, 0x63, 0x00, 0x61, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  83 $53 'S'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x1C, 0x00, 0x37, 0x00, 0x21, 0x00, 0x60, 0x00, 0x20, 0x00, 0x3C,
        0x00, 0x0F, 0x00, 0x01, 0x00, 0x01, 0x00, 0x61, 0x00, 0x23, 0x00, 0x3E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  84 $54 'T'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7F, 0x80, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  85 $55 'U'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  86 $56 'V'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x41, 0x80, 0x61, 0x00, 0x61, 0x00, 0x21, 0x00, 0x23, 0x00, 0x32,
        0x00, 0x32, 0x00, 0x12, 0x00, 0x16, 0x00, 0x1C, 0x00, 0x1C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  87 $57 'W'
    //	width 10, bbx 0, bby 0, bbw 9, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0xCC, 0x80, 0xCC, 0x80, 0x4C, 0x80, 0x4C, 0x80, 0x54, 0x80, 0x55,
        0x80, 0x55, 0x80, 0x55, 0x00, 0x73, 0x00, 0x73, 0x00, 0x33, 0x00, 0x33, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  88 $58 'X'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x61, 0x00, 0x33, 0x00, 0x12, 0x00, 0x1C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x1C, 0x00, 0x16, 0x00, 0x33, 0x00, 0x61, 0x00, 0x61, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  89 $59 'Y'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x41, 0x80, 0x61, 0x00, 0x21, 0x00, 0x33, 0x00, 0x12, 0x00, 0x1E,
        0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  90 $5a 'Z'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x01, 0x00, 0x02, 0x00, 0x06, 0x00, 0x04, 0x00, 0x0C,
        0x00, 0x08, 0x00, 0x18, 0x00, 0x10, 0x00, 0x30, 0x00, 0x60, 0x00, 0x7F, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  91 $5b 'bracketleft'
    //	width 10, bbx 3, bby -2, bbw 4, bbh 15
    [
        0x00, 0x00, 0x1E, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18,
        0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00,
        0x1E, 0x00, 0x00, 0x00,
    ],
    //  92 $5c 'backslash'
    //	width 10, bbx 1, bby -2, bbw 7, bbh 15
    [
        0x00, 0x00, 0x60, 0x00, 0x20, 0x00, 0x30, 0x00, 0x10, 0x00, 0x10, 0x00, 0x18, 0x00, 0x08,
        0x00, 0x0C, 0x00, 0x0C, 0x00, 0x04, 0x00, 0x06, 0x00, 0x02, 0x00, 0x02, 0x00, 0x03, 0x00,
        0x01, 0x00, 0x00, 0x00,
    ],
    //  93 $5d 'bracketright'
    //	width 10, bbx 2, bby -2, bbw 4, bbh 15
    [
        0x00, 0x00, 0x3C, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04,
        0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00,
        0x3C, 0x00, 0x00, 0x00,
    ],
    //  94 $5e 'asciicircum'
    //	width 10, bbx 1, bby 5, bbw 7, bbh 7
    [
        0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x14, 0x00, 0x12, 0x00, 0x32, 0x00, 0x21,
        0x00, 0x61, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  95 $5f 'underscore'
    //	width 10, bbx 1, bby -1, bbw 8, bbh 1
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7F, 0x80,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  96 $60 'grave'
    //	width 10, bbx 3, bby 10, bbw 3, bbh 3
    [
        0x00, 0x00, 0x10, 0x00, 0x18, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  97 $61 'a'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x33, 0x00, 0x41,
        0x00, 0x01, 0x00, 0x3F, 0x00, 0x61, 0x00, 0x61, 0x00, 0x63, 0x00, 0x3D, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  98 $62 'b'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x6E, 0x00, 0x73, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x73, 0x00, 0x7E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    //  99 $63 'c'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x33, 0x00, 0x61,
        0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 100 $64 'd'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x01, 0x00, 0x1D, 0x00, 0x33, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x23, 0x00, 0x3D, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 101 $65 'e'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x33, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x7F, 0x00, 0x60, 0x00, 0x60, 0x00, 0x23, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 102 $66 'f'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x07, 0x00, 0x0F, 0x00, 0x18, 0x00, 0x18, 0x00, 0x7F, 0x00, 0x7F,
        0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 103 $67 'g'
    //	width 10, bbx 1, bby -3, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1D, 0x00, 0x33, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x23, 0x00, 0x3D, 0x00, 0x01, 0x00, 0x01, 0x00,
        0x03, 0x00, 0x1E, 0x00,
    ],
    // 104 $68 'h'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x60, 0x00, 0x60, 0x00, 0x6E, 0x00, 0x73, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 105 $69 'i'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x00, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x7F, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 106 $6a 'j'
    //	width 10, bbx 1, bby -3, bbw 6, bbh 15
    [
        0x00, 0x00, 0x00, 0x00, 0x06, 0x00, 0x06, 0x00, 0x00, 0x00, 0x7E, 0x00, 0x7E, 0x00, 0x02,
        0x00, 0x02, 0x00, 0x02, 0x00, 0x02, 0x00, 0x02, 0x00, 0x02, 0x00, 0x02, 0x00, 0x06, 0x00,
        0x0C, 0x00, 0x78, 0x00,
    ],
    // 107 $6b 'k'
    //	width 10, bbx 2, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x21, 0x80, 0x23, 0x00, 0x22,
        0x00, 0x26, 0x00, 0x3C, 0x00, 0x26, 0x00, 0x22, 0x00, 0x23, 0x00, 0x21, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 108 $6c 'l'
    //	width 10, bbx 0, bby 0, bbw 9, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0xF8, 0x00, 0xF8, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08,
        0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x0F, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 109 $6d 'm'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x5B, 0x00, 0x6D, 0x80, 0x49,
        0x80, 0x49, 0x80, 0x49, 0x80, 0x49, 0x80, 0x49, 0x80, 0x49, 0x80, 0x49, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 110 $6e 'n'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6E, 0x00, 0x73, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 111 $6f 'o'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x33, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 112 $70 'p'
    //	width 10, bbx 1, bby -3, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x6E, 0x00, 0x73, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x73, 0x00, 0x7E, 0x00, 0x60, 0x00,
        0x60, 0x00, 0x60, 0x00,
    ],
    // 113 $71 'q'
    //	width 10, bbx 1, bby -3, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1D, 0x00, 0x33, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x23, 0x00, 0x3D, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x01, 0x00,
    ],
    // 114 $72 'r'
    //	width 10, bbx 2, bby 0, bbw 6, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2E, 0x00, 0x33, 0x00, 0x21,
        0x00, 0x21, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x20, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 115 $73 's'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x23, 0x00, 0x60,
        0x00, 0x20, 0x00, 0x3E, 0x00, 0x03, 0x00, 0x01, 0x00, 0x63, 0x00, 0x3E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 116 $74 't'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x7F, 0x00, 0x18, 0x00, 0x18,
        0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x18, 0x00, 0x0F, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 117 $75 'u'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61,
        0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x61, 0x00, 0x33, 0x00, 0x1E, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 118 $76 'v'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x61, 0x00, 0x23,
        0x00, 0x23, 0x00, 0x32, 0x00, 0x16, 0x00, 0x16, 0x00, 0x1C, 0x00, 0x0C, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 119 $77 'w'
    //	width 10, bbx 0, bby 0, bbw 9, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xCC, 0x80, 0x4C, 0x80, 0x4C,
        0x80, 0x55, 0x80, 0x55, 0x00, 0x75, 0x00, 0x33, 0x00, 0x33, 0x00, 0x33, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 120 $78 'glyph92'
    //	width 10, bbx 1, bby 0, bbw 8, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x33, 0x00, 0x16,
        0x00, 0x1C, 0x00, 0x0C, 0x00, 0x1C, 0x00, 0x16, 0x00, 0x23, 0x00, 0x61, 0x80, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 121 $79 'y'
    //	width 10, bbx 1, bby -3, bbw 8, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x80, 0x61, 0x00, 0x23,
        0x00, 0x32, 0x00, 0x12, 0x00, 0x16, 0x00, 0x1C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x08, 0x00,
        0x18, 0x00, 0x18, 0x00,
    ],
    // 122 $7a 'z'
    //	width 10, bbx 1, bby 0, bbw 7, bbh 9
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x03, 0x00, 0x06,
        0x00, 0x04, 0x00, 0x0C, 0x00, 0x18, 0x00, 0x30, 0x00, 0x20, 0x00, 0x7F, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
    // 123 $7b 'braceleft'
    //	width 10, bbx 1, bby -2, bbw 7, bbh 15
    [
        0x00, 0x00, 0x07, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x0C,
        0x00, 0x78, 0x00, 0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x07, 0x00,
        0x03, 0x00, 0x00, 0x00,
    ],
    // 124 $7c 'bar'
    //	width 10, bbx 4, bby -2, bbw 2, bbh 15
    [
        0x00, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C,
        0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00, 0x0C, 0x00,
        0x0C, 0x00, 0x00, 0x00,
    ],
    // 125 $7d 'braceright'
    //	width 10, bbx 2, bby -2, bbw 6, bbh 15
    [
        0x00, 0x00, 0x38, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x18, 0x00, 0x18, 0x00, 0x0C,
        0x00, 0x0F, 0x00, 0x18, 0x00, 0x18, 0x00, 0x08, 0x00, 0x08, 0x00, 0x08, 0x00, 0x38, 0x00,
        0x30, 0x00, 0x00, 0x00,
    ],
    // 126 $7e 'asciitilde'
    //	width 10, bbx 1, bby 4, bbw 7, bbh 3
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x39,
        0x00, 0x6D, 0x00, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ],
];

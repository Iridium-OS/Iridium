// Copyright (c) ChefKiss Inc 2021-2023. Licensed under the Thou Shalt Not Profit License version 1.0. See LICENSE for details.

pub const FONT_WIDTH: usize = 8;
pub const FONT_HEIGHT: usize = 17;

pub static FONT_BITMAP: &[[u8; FONT_HEIGHT]] = &[
    // 0x20 'space'
    // width 8, bbx 0, bby 0, bbw 0, bbh 0
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x21 'exclam'
    //	width 8, bbx 3, bby 0, bbw 2, bbh 12
    [
        0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x22 'quotedbl'
    //	width 8, bbx 2, bby 7, bbw 4, bbh 5
    [
        0x00, 0x24, 0x24, 0x24, 0x24, 0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x23 'numbersign'
    //	width 8, bbx 0, bby 0, bbw 8, bbh 11
    [
        0x00, 0x00, 0x12, 0x12, 0x7F, 0x24, 0x24, 0x24, 0x24, 0xFE, 0x4C, 0x48, 0x48, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x24 'dollar'
    //	width 8, bbx 1, bby -2, bbw 6, bbh 15
    [
        0x18, 0x18, 0x3C, 0x76, 0x52, 0x70, 0x30, 0x1C, 0x16, 0x12, 0x52, 0x7E, 0x18, 0x18, 0x18,
        0x00, 0x00,
    ],
    //  0x25 'percent'
    //	width 8, bbx 0, bby 0, bbw 8, bbh 12
    [
        0x00, 0x62, 0xD2, 0xD4, 0xD4, 0xD8, 0x68, 0x16, 0x1F, 0x2B, 0x2B, 0x4B, 0x4E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x26 'ampersand'
    //	width 8, bbx 1, bby 0, bbw 7, bbh 12
    [
        0x00, 0x38, 0x2C, 0x64, 0x64, 0x28, 0x38, 0x38, 0x6B, 0x4E, 0x46, 0x66, 0x3B, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x27 'quotesingle'
    //	width 8, bbx 3, bby 7, bbw 2, bbh 5
    [
        0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x28 'parenleft'
    //	width 8, bbx 2, bby -3, bbw 5, bbh 16
    [
        0x06, 0x0c, 0x18, 0x10, 0x10, 0x30, 0x30, 0x20, 0x20, 0x30, 0x10, 0x10, 0x18, 0x0c, 0x06,
        0x04, 0x00,
    ],
    //  0x29 'parenright'
    //	width 8, bbx 1, bby -3, bbw 5, bbh 16
    [
        0x60, 0x30, 0x18, 0x08, 0x08, 0x0c, 0x0c, 0x04, 0x04, 0x0c, 0x08, 0x08, 0x18, 0x30, 0x60,
        0x40, 0x00,
    ],
    //  0x2a 'asterisk'
    //	width 8, bbx 1, bby 4, bbw 6, bbh 7
    [
        0x00, 0x00, 0x18, 0x5A, 0x7E, 0x18, 0x7E, 0x5A, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x2b 'plus'
    //	width 8, bbx 1, bby 2, bbw 6, bbh 7
    [
        0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x2c 'comma'
    //	width 8, bbx 2, bby -3, bbw 3, bbh 5
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x08, 0x18,
        0x30, 0x00,
    ],
    //  0x2d 'hyphen'
    //	width 8, bbx 1, bby 5, bbw 6, bbh 1
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x2e 'period'
    //	width 8, bbx 3, bby 0, bbw 2, bbh 2
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x2f 'slash'
    //	width 8, bbx 1, bby -2, bbw 6, bbh 15
    [
        0x02, 0x06, 0x04, 0x04, 0x0c, 0x08, 0x08, 0x18, 0x10, 0x10, 0x30, 0x20, 0x20, 0x60, 0x40,
        0x00, 0x00,
    ],
    //  0x30 'zero'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x42, 0x42, 0x5A, 0x5A, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x31 'one'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x18, 0x38, 0x58, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x32 'two'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x02, 0x02, 0x04, 0x0c, 0x08, 0x10, 0x30, 0x60, 0x7E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x33 'three'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x02, 0x06, 0x1C, 0x0c, 0x02, 0x02, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x34 'four'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x0c, 0x0c, 0x1C, 0x14, 0x34, 0x24, 0x64, 0x44, 0x7E, 0x04, 0x04, 0x04, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x35 'five'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x7E, 0x40, 0x40, 0x40, 0x58, 0x7C, 0x62, 0x02, 0x02, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x36 'six'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x40, 0x40, 0x7C, 0x66, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x37 'seven'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x7E, 0x02, 0x06, 0x04, 0x04, 0x0c, 0x08, 0x08, 0x18, 0x10, 0x10, 0x30, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x38 'eight'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x24, 0x42, 0x42, 0x66, 0x3C, 0x3C, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x39 'nine'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x42, 0x42, 0x42, 0x3E, 0x02, 0x02, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x3a 'colon'
    //	width 8, bbx 3, bby 0, bbw 2, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x3b 'semicolon'
    //	width 8, bbx 2, bby -3, bbw 3, bbh 11
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x08, 0x18,
        0x30, 0x00,
    ],
    //  0x3c 'less'
    //	width 8, bbx 1, bby 2, bbw 6, bbh 7
    [
        0x00, 0x00, 0x00, 0x00, 0x02, 0x0c, 0x30, 0x40, 0x30, 0x0c, 0x02, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x3d 'equal'
    //	width 8, bbx 1, bby 3, bbw 6, bbh 4
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x00, 0x7E, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x3e 'greater'
    //	width 8, bbx 1, bby 2, bbw 6, bbh 7
    [
        0x00, 0x00, 0x00, 0x00, 0x40, 0x30, 0x0c, 0x02, 0x0c, 0x30, 0x40, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x3f 'question'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x02, 0x06, 0x0c, 0x08, 0x18, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x40 'at'
    //	width 8, bbx 1, bby -1, bbw 6, bbh 13
    [
        0x00, 0x3C, 0x42, 0x42, 0x5A, 0x7E, 0x7E, 0x7E, 0x7E, 0x7E, 0x5E, 0x40, 0x40, 0x3E, 0x00,
        0x00, 0x00,
    ],
    //  0x41 'A'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x24, 0x24, 0x24, 0x7E, 0x42, 0x42, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x42 'B'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x78, 0x44, 0x46, 0x42, 0x46, 0x7C, 0x4C, 0x42, 0x42, 0x42, 0x46, 0x7C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x43 'C'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x40, 0x40, 0x40, 0x40, 0x40, 0x40, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x44 'D'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x78, 0x4C, 0x44, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x46, 0x4C, 0x78, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x45 'E'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x7E, 0x40, 0x40, 0x40, 0x40, 0x7C, 0x40, 0x40, 0x40, 0x40, 0x40, 0x7E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x46 'F'
    //	width 8, bbx 2, bby 0, bbw 5, bbh 12
    [
        0x00, 0x3E, 0x20, 0x20, 0x20, 0x20, 0x3C, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x47 'G'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x40, 0x40, 0x40, 0x4E, 0x42, 0x42, 0x42, 0x62, 0x3E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x48 'H'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x49 'I'
    //	width 8, bbx 2, bby 0, bbw 4, bbh 12
    [
        0x00, 0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x4a 'J'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x46, 0x44, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x4b 'K'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x62, 0x64, 0x6C, 0x68, 0x78, 0x70, 0x70, 0x68, 0x68, 0x64, 0x64, 0x62, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x4c 'L'
    //	width 8, bbx 2, bby 0, bbw 5, bbh 12
    [
        0x00, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x4d 'M'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x66, 0x66, 0x66, 0x66, 0x5A, 0x5A, 0x5A, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x4e 'N'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x62, 0x62, 0x62, 0x52, 0x52, 0x52, 0x4A, 0x4A, 0x4A, 0x46, 0x46, 0x46, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x4f 'O'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x50 'P'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x7C, 0x66, 0x62, 0x62, 0x62, 0x66, 0x7C, 0x60, 0x60, 0x60, 0x60, 0x60, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x51 'Q'
    //	width 8, bbx 1, bby -3, bbw 6, bbh 15
    [
        0x00, 0x3C, 0x66, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x18, 0x0c,
        0x06, 0x00,
    ],
    //  0x52 'R'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x78, 0x46, 0x42, 0x42, 0x42, 0x46, 0x7C, 0x48, 0x4C, 0x44, 0x46, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x53 'S'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x3C, 0x66, 0x42, 0x40, 0x60, 0x30, 0x1C, 0x06, 0x02, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x54 'T'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x55 'U'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x56 'V'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x42, 0x42, 0x66, 0x24, 0x24, 0x24, 0x24, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x57 'W'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x42, 0x42, 0x42, 0x5A, 0x5A, 0x5A, 0x5A, 0x5A, 0x6C, 0x24, 0x24, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x58 'X'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x66, 0x24, 0x3C, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x24, 0x66, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x59 'Y'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x42, 0x66, 0x24, 0x24, 0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x5a 'Z'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x7E, 0x06, 0x04, 0x04, 0x08, 0x08, 0x18, 0x10, 0x30, 0x20, 0x60, 0x7E, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x5b 'bracketleft'
    //	width 8, bbx 2, bby -2, bbw 5, bbh 15
    [
        0x3E, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x3E,
        0x00, 0x00,
    ],
    //  0x5c 'backslash'
    //	width 8, bbx 1, bby -2, bbw 6, bbh 15
    [
        0x40, 0x60, 0x20, 0x20, 0x30, 0x10, 0x10, 0x18, 0x08, 0x08, 0x0c, 0x04, 0x04, 0x06, 0x02,
        0x00, 0x00,
    ],
    //  0x5d 'bracketright'
    //	width 8, bbx 1, bby -2, bbw 5, bbh 15
    [
        0x7C, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x7C,
        0x00, 0x00,
    ],
    //  0x5e 'asciicircum'
    //	width 8, bbx 1, bby 8, bbw 6, bbh 3
    [
        0x00, 0x00, 0x18, 0x3C, 0x66, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x5f 'underscore'
    //	width 8, bbx 1, bby -3, bbw 6, bbh 1
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x7E, 0x00,
    ],
    //  0x60 'grave'
    //	width 8, bbx 2, bby 9, bbw 3, bbh 4
    [
        0x20, 0x20, 0x10, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x61 'a'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42, 0x02, 0x3E, 0x62, 0x42, 0x46, 0x3A, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x62 'b'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x40, 0x40, 0x40, 0x40, 0x5C, 0x62, 0x42, 0x42, 0x42, 0x42, 0x66, 0x5C, 0x00, 0x00,
        0x00, 0x00,
    ],
    //  0x63 'c'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42, 0x42, 0x40, 0x40, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x64 'd'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x02, 0x02, 0x02, 0x02, 0x3A, 0x46, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3A, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x65 'e'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42, 0x42, 0x7E, 0x40, 0x40, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x66 'f'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x0e, 0x18, 0x10, 0x10, 0x7E, 0x7E, 0x10, 0x10, 0x10, 0x10, 0x10, 0x10, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x67 'g'
    //	width 8, bbx 1, bby -4, bbw 6, bbh 12
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x46, 0x42, 0x66, 0x3C, 0x40, 0x78, 0x7E, 0x42, 0x42,
        0x7E, 0x18,
    ],
    // 0x68 'h'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x40, 0x40, 0x40, 0x40, 0x5C, 0x66, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x69 'i'
    //	width 8, bbx 2, bby 0, bbw 3, bbh 12
    [
        0x00, 0x08, 0x18, 0x00, 0x00, 0x38, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x6a 'j'
    //	width 8, bbx 0, bby -4, bbw 6, bbh 16
    [
        0x00, 0x08, 0x0c, 0x00, 0x00, 0x3C, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0x0c, 0xCC,
        0x78, 0x30,
    ],
    // 0x6b 'k'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x60, 0x60, 0x60, 0x60, 0x66, 0x6C, 0x78, 0x70, 0x78, 0x68, 0x64, 0x66, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x6c 'l'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x78, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x1E, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x6d 'm'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x76, 0x5A, 0x5A, 0x5A, 0x5A, 0x5A, 0x5A, 0x5A, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x6e 'n'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x5C, 0x66, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x6f 'o'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x70 'p'
    //	width 8, bbx 1, bby -3, bbw 6, bbh 11
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x5C, 0x62, 0x42, 0x42, 0x42, 0x42, 0x66, 0x5C, 0x40, 0x40,
        0x40, 0x00,
    ],
    // 0x71 'q'
    //	width 8, bbx 1, bby -3, bbw 6, bbh 11
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x3A, 0x46, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3A, 0x02, 0x02,
        0x02, 0x00,
    ],
    // 0x72 'r'
    //	width 8, bbx 2, bby 0, bbw 5, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x2E, 0x32, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x73 's'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x3C, 0x42, 0x60, 0x38, 0x0c, 0x02, 0x66, 0x3C, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x74 't'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 12
    [
        0x00, 0x10, 0x10, 0x10, 0x10, 0x7E, 0x10, 0x10, 0x10, 0x10, 0x12, 0x12, 0x1E, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x75 'u'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x66, 0x3A, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x76 'v'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x66, 0x24, 0x24, 0x24, 0x18, 0x18, 0x18, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x77 'w'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x42, 0x5A, 0x5A, 0x5A, 0x5A, 0x24, 0x24, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x78 'x'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x66, 0x24, 0x18, 0x18, 0x18, 0x3C, 0x24, 0x66, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x79 'y'
    //	width 8, bbx 1, bby -3, bbw 6, bbh 11
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x42, 0x66, 0x24, 0x24, 0x24, 0x18, 0x18, 0x18, 0x18, 0x10,
        0x70, 0x00,
    ],
    // 0x7a 'z'
    //	width 8, bbx 1, bby 0, bbw 6, bbh 8
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x7E, 0x04, 0x0c, 0x08, 0x10, 0x30, 0x60, 0x7E, 0x00, 0x00,
        0x00, 0x00,
    ],
    // 0x7b 'braceleft'
    //	width 8, bbx 1, bby -2, bbw 6, bbh 15
    [
        0x0e, 0x08, 0x18, 0x18, 0x18, 0x18, 0x10, 0x60, 0x10, 0x18, 0x18, 0x18, 0x18, 0x08, 0x0e,
        0x00, 0x00,
    ],
    // 0x7c 'bar'
    //	width 8, bbx 3, bby -2, bbw 2, bbh 15
    [
        0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18,
        0x00, 0x00,
    ],
    // 0x7d 'braceright'
    //	width 8, bbx 1, bby -2, bbw 6, bbh 15
    [
        0x70, 0x10, 0x18, 0x18, 0x18, 0x18, 0x08, 0x06, 0x08, 0x18, 0x18, 0x18, 0x18, 0x10, 0x70,
        0x00, 0x00,
    ],
    // 0x7e 'asciitilde'
    //	width 8, bbx 1, bby 3, bbw 6, bbh 3
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x5E, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00,
    ],
];

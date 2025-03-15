const O: bool = false;
const X: bool = true;

pub const SIZE: usize = 5;

pub type Bitmap = [[bool; SIZE]; SIZE];

pub const BLANK_BITMAP: Bitmap = [
    [O, O, O, O, O],
    [O, O, O, O, O],
    [O, O, O, O, O],
    [O, O, O, O, O],
    [O, O, O, O, O],
];

pub const FALLBACK_BITMAP: Bitmap = [
    [O, X, O, X, O],
    [O, O, O, O, O],
    [X, O, O, O, X],
    [X, O, O, O, X],
    [O, X, X, X, O],
];

pub const LATIN_ALPHABET: [Bitmap; 26] = [
    // A
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
    ],
    // B
    [
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, X, X, X, O],
        [X, O, O, O, X],
        [X, X, X, X, X],
    ],
    // C
    [
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, X, X, X, X],
    ],
    // D
    [
        [X, X, X, X, O],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, O],
    ],
    // E
    [
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, X],
    ],
    // F
    [
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, O, O, O, O],
    ],
    // G
    [
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
    ],
    // H
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
    ],
    // I
    [
        [X, X, X, X, X],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [X, X, X, X, X],
    ],
    // J
    [
        [X, X, X, X, X],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [X, X, X, O, O],
    ],
    // K
    [
        [X, O, O, X, X],
        [X, O, X, O, O],
        [X, X, O, O, O],
        [X, O, X, O, O],
        [X, O, O, X, X],
    ],
    // L
    [
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, X, X, X, X],
    ],
    // M
    [
        [X, X, X, X, X],
        [X, O, X, O, X],
        [X, O, X, O, X],
        [X, O, X, O, X],
        [X, O, X, O, X],
    ],
    // N
    [
        [X, X, O, O, X],
        [X, O, X, O, X],
        [X, O, X, O, X],
        [X, O, O, X, X],
        [X, O, O, O, X],
    ],
    // O
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // P
    [
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, O, O, O, O],
    ],
    // Q
    [
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, X, O],
        [X, X, X, X, X],
    ],
    // R
    [
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [X, O, X, O, O],
        [X, O, O, X, X],
    ],
    // S
    [
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, X],
        [O, O, O, O, X],
        [X, X, X, X, X],
    ],
    // T
    [
        [X, X, X, X, X],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
    ],
    // U
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
    ],
    // V
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [O, X, O, X, O],
        [O, O, X, O, O],
    ],
    // W
    [
        [X, O, X, O, X],
        [X, O, X, O, X],
        [X, O, X, O, X],
        [X, O, X, O, X],
        [O, X, O, X, O],
    ],
    // X
    [
        [X, O, O, O, X],
        [O, X, O, X, O],
        [O, O, X, O, O],
        [O, X, O, X, O],
        [X, O, O, O, X],
    ],
    // Y
    [
        [X, O, O, O, X],
        [O, X, O, X, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
    ],
    // Z
    [
        [X, X, X, X, X],
        [O, O, O, X, O],
        [O, O, X, O, O],
        [O, X, O, O, O],
        [X, X, X, X, X],
    ],
];

pub const NUMBERS: [Bitmap; 10] = [
    // 0
    [
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
    ],
    // 1
    [
        [O, O, X, O, O],
        [O, X, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
    ],
    // 2
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [O, O, X, X, O],
        [O, X, O, O, O],
        [X, X, X, X, X],
    ],
    // 3
    [
        [X, X, X, X, X],
        [O, O, O, X, O],
        [O, O, X, O, O],
        [O, O, O, X, O],
        [X, X, X, X, X],
    ],
    // 4
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [O, O, O, O, X],
        [O, O, O, O, X],
    ],
    // 5
    [
        [X, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, O],
        [O, O, O, O, X],
        [X, X, X, X, O],
    ],
    // 6
    [
        [O, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, O],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // 7
    [
        [X, X, X, X, X],
        [O, O, O, X, O],
        [O, O, X, O, O],
        [O, X, O, O, O],
        [X, O, O, O, O],
    ],
    // 8
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [O, X, X, X, O],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // 9
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [O, X, X, X, O],
        [O, O, O, O, X],
        [X, X, X, X, O],
    ],
];

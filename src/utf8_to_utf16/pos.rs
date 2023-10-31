use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Default)]
pub struct BytePos(pub u32);

#[derive(Debug, Clone, Default)]
pub struct ByteToCharPosState {
    pub pos: BytePos,
    pub total_extra_bytes: u32,
    pub mbc_index: usize,
}

pub trait Pos {
    fn from_usize(n: usize) -> Self;
    fn to_usize(&self) -> usize;
    fn from_u32(n: u32) -> Self;
    fn to_u32(&self) -> u32;
}

impl Pos for BytePos {
    #[inline(always)]
    fn from_usize(n: usize) -> BytePos {
        BytePos(n as u32)
    }

    #[inline(always)]
    fn to_usize(&self) -> usize {
        self.0 as usize
    }

    #[inline(always)]
    fn from_u32(n: u32) -> BytePos {
        BytePos(n)
    }

    #[inline(always)]
    fn to_u32(&self) -> u32 {
        self.0
    }
}

impl Add for BytePos {
    type Output = BytePos;

    #[inline(always)]
    fn add(self, rhs: BytePos) -> BytePos {
        BytePos((self.to_usize() + rhs.to_usize()) as u32)
    }
}

impl Sub for BytePos {
    type Output = BytePos;

    #[inline(always)]
    fn sub(self, rhs: BytePos) -> BytePos {
        BytePos((self.to_usize() - rhs.to_usize()) as u32)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MultiByteChar {
    pub pos: BytePos,
    pub bytes: u8,
}

impl MultiByteChar {
    pub fn byte_to_char_diff(&self) -> u8 {
        if self.bytes == 4 {
            2
        } else {
            self.bytes - 1
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NonNarrowChar {
    /// Represents a zero-width character
    ZeroWidth(BytePos),
    /// Represents a wide (fullwidth) character
    Wide(BytePos),
    /// Represents a tab character, represented visually with a width of 4
    /// characters
    Tab(BytePos),
}

impl NonNarrowChar {
    pub fn new(pos: BytePos, width: usize) -> Self {
        match width {
            0 => NonNarrowChar::ZeroWidth(pos),
            2 => NonNarrowChar::Wide(pos),
            4 => NonNarrowChar::Tab(pos),
            _ => panic!("width {} given for non-narrow character", width),
        }
    }

    /// Returns the absolute offset of the character in the SourceMap
    pub fn pos(self) -> BytePos {
        match self {
            NonNarrowChar::ZeroWidth(p) | NonNarrowChar::Wide(p) | NonNarrowChar::Tab(p) => p,
        }
    }

    /// Returns the width of the character, 0 (zero-width) or 2 (wide)
    pub fn width(self) -> usize {
        match self {
            NonNarrowChar::ZeroWidth(_) => 0,
            NonNarrowChar::Wide(_) => 2,
            NonNarrowChar::Tab(_) => 4,
        }
    }
}

impl Add<BytePos> for NonNarrowChar {
    type Output = Self;

    fn add(self, rhs: BytePos) -> Self {
        match self {
            NonNarrowChar::ZeroWidth(pos) => NonNarrowChar::ZeroWidth(pos + rhs),
            NonNarrowChar::Wide(pos) => NonNarrowChar::Wide(pos + rhs),
            NonNarrowChar::Tab(pos) => NonNarrowChar::Tab(pos + rhs),
        }
    }
}

impl Sub<BytePos> for NonNarrowChar {
    type Output = Self;

    fn sub(self, rhs: BytePos) -> Self {
        match self {
            NonNarrowChar::ZeroWidth(pos) => NonNarrowChar::ZeroWidth(pos - rhs),
            NonNarrowChar::Wide(pos) => NonNarrowChar::Wide(pos - rhs),
            NonNarrowChar::Tab(pos) => NonNarrowChar::Tab(pos - rhs),
        }
    }
}

pub struct Span {
    pub lo: BytePos,
    pub hi: BytePos,
}

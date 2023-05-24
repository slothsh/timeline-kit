#![allow(dead_code, unused_variables)]

use std::{fmt::Display, write, ops::Rem};
use num_traits::{Bounded, ToPrimitive};

use super::formats::FrameRate;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Global Constants --
//
///////////////////////////////////////////////////////////////////////////

/// Configuration constant that defines how many groups
/// there are in the `Timecode` data field
///
/// There are conventionally, in SMPTE timecodes, four
/// groups, but, for finer precision in each field,
/// this implementation makes use of an additional "ticks"
/// field (which can also be referred to as "sub-frames").
///
/// Exmaple:
///
/// SMPTE Timecode:
///     1  2  3  4  5
///     hh:mm:ss:ff:sub
///     00:00:00:00.000
///
/// 1 -> Hours
/// 2 -> Minutes
/// 3 -> Seconds
/// 4 -> Frames
/// 5 -> Sub-frames/Ticks
///
const TC_TOTAL_GROUPS: usize = 5;

/// Defines the width of the string representation of
/// each group, excluding "ticks", in the `Timecode` data field
///
/// Example:
///
/// SMPTE Timecode:
///     2  2  2  2  -
///     hh:mm:ss:ff:sub
///     00:00:00:00.000
///
const TC_STRING_REGULAR_GROUP_SIZE: usize = 2;

/// Defines the width of the string representation of
/// the "ticks" group in the `Timecode` data field
///
/// Example:
///
/// SMPTE Timecode:
///     -  -  -  -  3
///     hh:mm:ss:ff:sub
///     00:00:00:00.000
///
const TC_STRING_TICKS_GROUP_SIZE: usize = 3;

/// Defines the length of the SMPTE string representation, including
/// all seperators and padding
///
/// Example:
/// 
/// SMPTE Timecode:
///     2  + 1  + 2  + 1  + 2  + 1  + 2  = STRING LENGTH
///     hh   :    mm   :    ss   :    ff
///     00   :    00   :    00   :    00
///
const TC_STRING_REGULAR_LENGTH: usize = TC_TOTAL_GROUPS * TC_STRING_REGULAR_GROUP_SIZE + (TC_TOTAL_GROUPS - 1);

// TODO: Doc comments
const TC_TICK_RESOLUTION: usize = 100;
const TC_SCALAR_HOURS_INDEX: usize = 0;
const TC_SCALAR_MINUTES_INDEX: usize = 1;
const TC_SCALAR_SECONDS_INDEX: usize = 2;
const TC_SCALAR_FRAMES_INDEX: usize = 3;
const TC_SCALAR_TICKS_INDEX: usize = 4;
const TC_FLAGS_DEFAULT: TimecodeFlag = 0;
const TC_SCALAR_ORDER_TABLE: [usize; TC_TOTAL_GROUPS] = [
    TC_SCALAR_HOURS_INDEX,
    TC_SCALAR_MINUTES_INDEX,
    TC_SCALAR_SECONDS_INDEX,
    TC_SCALAR_FRAMES_INDEX,
    TC_SCALAR_TICKS_INDEX,
];

const TC_CONFIG_HOURS_INDEX: usize = TC_SCALAR_HOURS_INDEX;
const TC_CONFIG_MINUTES_INDEX: usize = TC_SCALAR_MINUTES_INDEX;
const TC_CONFIG_SECONDS_INDEX: usize = TC_SCALAR_SECONDS_INDEX;
const TC_CONFIG_FRAMES_INDEX: usize = TC_SCALAR_FRAMES_INDEX;
const TC_CONFIG_TICKS_INDEX: usize = TC_SCALAR_TICKS_INDEX;
const TC_CONFIG_GROUP_TICKS_FACTOR_INDEX: usize = 0;
const TC_CONFIG_GROUP_APPLY_FPS_INDEX: usize = 1;
enum TernaryPredicate {
    True,
    False,
    Other,
}
static TC_CONFIG_TABLE: [(usize, TernaryPredicate); TC_TOTAL_GROUPS] = [
    (60 * 60, TernaryPredicate::True),
    (60, TernaryPredicate::True),
    (1, TernaryPredicate::True),
    (1, TernaryPredicate::False),
    (1, TernaryPredicate::Other),
];

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Associated Type Aliases --
//
///////////////////////////////////////////////////////////////////////////

type TimecodeScalar = u8;
type TimecodeData = [TimecodeScalar; TC_TOTAL_GROUPS];
type TimecodeFlag = u8;
type TimecodeFrameRate = FrameRate;
type TimecodeU64 = u64;

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Structure Definition --
//
///////////////////////////////////////////////////////////////////////////

/// The primary structure for encapsulating timecode scalar data
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct Timecode {
    data: TimecodeData,
    fps: TimecodeFrameRate,
    flags: TimecodeFlag,
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Constructor Associated Functions --
//
///////////////////////////////////////////////////////////////////////////

impl Timecode {
    /// Constructs a new `Timecode` with a specified frame rate
    pub fn with_fps(fps: FrameRate) -> Self {
        Self {
            fps,
            ..Timecode::default()
        }
    }

    pub fn new(groups: &[TimecodeScalar; TC_TOTAL_GROUPS], fps: FrameRate) -> Self {
        // TODO: Check bounds of groups
        // TODO: Check flags based on bounds check of groups

        Self {
            data: groups.clone(),
            fps,
            ..Timecode::default()
        }
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Public Methods --
//
///////////////////////////////////////////////////////////////////////////

impl Timecode {
    pub fn hours<T: num_traits::PrimInt>(&self) -> T {
        return T::from(self.data[TC_SCALAR_HOURS_INDEX]).unwrap();
    }

    pub fn minutes<T: num_traits::PrimInt>(&self) -> T {
        return T::from(self.data[TC_SCALAR_MINUTES_INDEX]).unwrap();
    }

    pub fn seconds<T: num_traits::PrimInt>(&self) -> T {
        return T::from(self.data[TC_SCALAR_SECONDS_INDEX]).unwrap();
    }

    pub fn frames<T: num_traits::PrimInt>(&self) -> T {
        return T::from(self.data[TC_SCALAR_FRAMES_INDEX]).unwrap();
    }

    pub fn ticks<T: num_traits::PrimInt>(&self) -> T {
        return T::from(self.data[TC_SCALAR_TICKS_INDEX]).unwrap();
    }

    pub fn to_ticks(&self) -> usize {
        let mut ticks: usize = 0;
        for (scalar, i) in self.data.iter().zip(TC_SCALAR_ORDER_TABLE) {
            match TC_CONFIG_TABLE[i].1 {
                TernaryPredicate::True => ticks += *scalar as usize * TC_CONFIG_TABLE[i].0 * self.fps.as_usize() * TC_TICK_RESOLUTION,
                TernaryPredicate::False => ticks += *scalar as usize * TC_CONFIG_TABLE[i].0 * TC_TICK_RESOLUTION,
                TernaryPredicate::Other => ticks += *scalar as usize,
            }
        }

        ticks
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Trait Implemenations --
//
///////////////////////////////////////////////////////////////////////////

impl Default for Timecode {
    fn default() -> Self {
        Self {
            data: [TimecodeScalar::default(); TC_TOTAL_GROUPS],
            fps: FrameRate::default(),
            flags: 0,
        }
    }
}

impl Display for Timecode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Handle display of drop-frame delimiter
        // TODO: Handle display of ticks/sub-frames

        let mut tc_string = String::with_capacity(TC_STRING_REGULAR_LENGTH);
        for (i, &scalar) in self.data.iter().take(TC_TOTAL_GROUPS - 1).enumerate() {
            let delimiter = if i < TC_TOTAL_GROUPS - 2 { ":" } else { "" };
            tc_string += format!("{:0>2}{}", scalar, delimiter).as_str();
        }

        write!(f, "{}", tc_string)
    }
}

impl num_traits::PrimInt for Timecode {
    fn signed_shl(self, n: u32) -> Self {
        todo!()
    }

    fn pow(self, exp: u32) -> Self {
        todo!()
    }

    fn to_be(self) -> Self {
        todo!()
    }

    fn to_le(self) -> Self {
        todo!()
    }

    fn count_ones(self) -> u32 {
        todo!()
    }

    fn signed_shr(self, n: u32) -> Self {
        todo!()
    }

    fn swap_bytes(self) -> Self {
        todo!()
    }

    fn count_zeros(self) -> u32 {
        todo!()
    }

    fn rotate_left(self, n: u32) -> Self {
        todo!()
    }

    fn leading_ones(self) -> u32 {
        todo!()
    }

    fn rotate_right(self, n: u32) -> Self {
        todo!()
    }

    fn unsigned_shl(self, n: u32) -> Self {
        todo!()
    }

    fn unsigned_shr(self, n: u32) -> Self {
       todo!()
    }

    fn reverse_bits(self) -> Self {
        todo!()
    }

    fn leading_zeros(self) -> u32 {
        todo!()
    }

    fn trailing_ones(self) -> u32 {
        todo!()
    }

    fn trailing_zeros(self) -> u32 {
        todo!()
    }

    fn from_be(x: Self) -> Self {
        todo!()
    }

    fn from_le(x: Self) -> Self {
        todo!()
    }
}

impl num_traits::Saturating for Timecode {
    fn saturating_add(self, v: Self) -> Self {
        todo!()
    }

    fn saturating_sub(self, v: Self) -> Self {
        todo!()
    }
}

impl num_traits::Num for Timecode {
    // TODO: Change this to a valid error type
    type FromStrRadixErr = String;
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl num_traits::Zero for Timecode {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }

    fn set_zero(&mut self) {
        todo!()
    }
}

impl num_traits::One for Timecode {
    fn one() -> Self {
        todo!()
    }

    fn is_one(&self) -> bool
    where
        Self: PartialEq, 
    {
        todo!()
    }

    fn set_one(&mut self) {
        todo!()
    }
}

impl num_traits::SaturatingAdd for Timecode {
    fn saturating_add(&self, v: &Self) -> Self {
        todo!()
    }
}

impl num_traits::SaturatingSub for Timecode {
    fn saturating_sub(&self, v: &Self) -> Self {
        todo!()
    }
}

impl num_traits::SaturatingMul for Timecode {
    fn saturating_mul(&self, v: &Self) -> Self {
        todo!()
    }
}

impl num_traits::CheckedAdd for Timecode {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl num_traits::CheckedSub for Timecode {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl num_traits::CheckedMul for Timecode {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl num_traits::CheckedDiv for Timecode {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        todo!()
    }
}

impl num_traits::NumCast for Timecode {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        todo!()
    }
}

impl num_traits::ToPrimitive for Timecode {
    fn to_i8(&self) -> Option<i8> {
        todo!()
    }

    fn to_u8(&self) -> Option<u8> {
        todo!()
    }

    fn to_i16(&self) -> Option<i16> {
        todo!()
    }

    fn to_u16(&self) -> Option<u16> {
        todo!()
    }

    fn to_i32(&self) -> Option<i32> {
        todo!()
    }

    fn to_u32(&self) -> Option<u32> {
        todo!()
    }

    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }

    fn to_i128(&self) -> Option<i128> {
        todo!()
    }

    fn to_u128(&self) -> Option<u128> {
        todo!()
    }

    fn to_f32(&self) -> Option<f32> {
        todo!()
    }

    fn to_f64(&self) -> Option<f64> {
        todo!()
    }

    fn to_isize(&self) -> Option<isize> {
        todo!()
    }

    fn to_usize(&self) -> Option<usize> {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Arithmetic Operator Overloads --
//
///////////////////////////////////////////////////////////////////////////

impl std::ops::Add for Timecode {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub for Timecode {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul for Timecode {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for Timecode {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Rem for Timecode {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Bounded for Timecode {
    fn min_value() -> Self {
        todo!()
    }

    fn max_value() -> Self {
        todo!()
    }
}

impl std::ops::Not for Timecode {
    type Output = Self;
    fn not(self) -> Self::Output {
        todo!()
    }
}

impl std::ops::BitAnd for Timecode {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::BitOr for Timecode {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::BitXor for Timecode {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shl for Timecode {
    type Output = Self;
    fn shl(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shr for Timecode {
    type Output = Self;
    fn shr(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shl<usize> for Timecode {
    type Output = Self;
    fn shl(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

impl std::ops::Shr<usize> for Timecode {
    type Output = Self;
    fn shr(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

///////////////////////////////////////////////////////////////////////////
//
//  -- @SECTION `Timecode` Unit Tests --
//
///////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_constructor() {
        let timecode = Timecode::default();
        assert_eq!(timecode.data, [TimecodeScalar::default(); TC_TOTAL_GROUPS]);
        assert_eq!(timecode.fps, TimecodeFrameRate::Fps25);
        assert_eq!(timecode.flags, 0u8);
    }

    #[test]
    fn new_constructor() {
        let timecode = Timecode::new(&[0, 1, 2, 3, 4], FrameRate::Fps25);
        assert_eq!(timecode.data[TC_SCALAR_HOURS_INDEX], 0);
        assert_eq!(timecode.data[TC_SCALAR_MINUTES_INDEX], 1);
        assert_eq!(timecode.data[TC_SCALAR_SECONDS_INDEX], 2);
        assert_eq!(timecode.data[TC_SCALAR_FRAMES_INDEX], 3);
        assert_eq!(timecode.data[TC_SCALAR_TICKS_INDEX], 4);
        assert_eq!(timecode.fps, FrameRate::Fps25);
        assert_eq!(timecode.flags, TC_FLAGS_DEFAULT);
    }

    #[test]
    fn ticks_conversion() {
        let timecode = Timecode::new(&[1, 2, 3, 4, 5], FrameRate::Fps25);
        let exptected_ticks = (timecode.data[TC_SCALAR_HOURS_INDEX] as usize * 3600 * 25 * TC_TICK_RESOLUTION)
                              + (timecode.data[TC_SCALAR_MINUTES_INDEX] as usize * 60 * 25 * TC_TICK_RESOLUTION)
                              + (timecode.data[TC_SCALAR_SECONDS_INDEX] as usize * 25 * TC_TICK_RESOLUTION)
                              + (timecode.data[TC_SCALAR_FRAMES_INDEX] as usize * TC_TICK_RESOLUTION)
                              + (timecode.data[TC_SCALAR_TICKS_INDEX] as usize);

        assert_eq!(timecode.to_ticks(), exptected_ticks);
    }

    #[test]
    fn getters_defaulted() {
        let timecode = Timecode::default();
        assert_eq!(timecode.hours::<u32>(), 0);
        assert_eq!(timecode.minutes::<u32>(), 0);
        assert_eq!(timecode.seconds::<u32>(), 0);
        assert_eq!(timecode.frames::<u32>(), 0);
        assert_eq!(timecode.ticks::<u32>(), 0);
    }

    #[test]
    fn display_trait_regular_representation() {
        let timecode_defaulted = Timecode::default();
        let timecode_new = Timecode::new(&[13, 12, 32, 42, 100], FrameRate::Fps25);
        assert_eq!("00:00:00:00", format!("{}", timecode_defaulted));
        assert_eq!("13:12:32:42", format!("{}", timecode_new));
    }
}

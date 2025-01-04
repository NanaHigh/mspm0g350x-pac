#[doc = "Register `DIN31_0` reader"]
pub type R = crate::R<Din31_0Spec>;
#[doc = "This bit reads the data input value of DIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio0 {
    #[doc = "0: ZERO"]
    Din31_0Dio0Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio0One = 1,
}
impl From<Din31_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO0` reader - This bit reads the data input value of DIO0."]
pub type Din31_0Dio0R = crate::BitReader<Din31_0Dio0>;
impl Din31_0Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio0 {
        match self.bits {
            false => Din31_0Dio0::Din31_0Dio0Zero,
            true => Din31_0Dio0::Din31_0Dio0One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio0_zero(&self) -> bool {
        *self == Din31_0Dio0::Din31_0Dio0Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio0_one(&self) -> bool {
        *self == Din31_0Dio0::Din31_0Dio0One
    }
}
#[doc = "This bit reads the data input value of DIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio1 {
    #[doc = "0: ZERO"]
    Din31_0Dio1Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio1One = 1,
}
impl From<Din31_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO1` reader - This bit reads the data input value of DIO1."]
pub type Din31_0Dio1R = crate::BitReader<Din31_0Dio1>;
impl Din31_0Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio1 {
        match self.bits {
            false => Din31_0Dio1::Din31_0Dio1Zero,
            true => Din31_0Dio1::Din31_0Dio1One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio1_zero(&self) -> bool {
        *self == Din31_0Dio1::Din31_0Dio1Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio1_one(&self) -> bool {
        *self == Din31_0Dio1::Din31_0Dio1One
    }
}
#[doc = "This bit reads the data input value of DIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio2 {
    #[doc = "0: ZERO"]
    Din31_0Dio2Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio2One = 1,
}
impl From<Din31_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO2` reader - This bit reads the data input value of DIO2."]
pub type Din31_0Dio2R = crate::BitReader<Din31_0Dio2>;
impl Din31_0Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio2 {
        match self.bits {
            false => Din31_0Dio2::Din31_0Dio2Zero,
            true => Din31_0Dio2::Din31_0Dio2One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio2_zero(&self) -> bool {
        *self == Din31_0Dio2::Din31_0Dio2Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio2_one(&self) -> bool {
        *self == Din31_0Dio2::Din31_0Dio2One
    }
}
#[doc = "This bit reads the data input value of DIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio3 {
    #[doc = "0: ZERO"]
    Din31_0Dio3Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio3One = 1,
}
impl From<Din31_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO3` reader - This bit reads the data input value of DIO3."]
pub type Din31_0Dio3R = crate::BitReader<Din31_0Dio3>;
impl Din31_0Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio3 {
        match self.bits {
            false => Din31_0Dio3::Din31_0Dio3Zero,
            true => Din31_0Dio3::Din31_0Dio3One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio3_zero(&self) -> bool {
        *self == Din31_0Dio3::Din31_0Dio3Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio3_one(&self) -> bool {
        *self == Din31_0Dio3::Din31_0Dio3One
    }
}
#[doc = "This bit reads the data input value of DIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio4 {
    #[doc = "0: ZERO"]
    Din31_0Dio4Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio4One = 1,
}
impl From<Din31_0Dio4> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO4` reader - This bit reads the data input value of DIO4."]
pub type Din31_0Dio4R = crate::BitReader<Din31_0Dio4>;
impl Din31_0Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio4 {
        match self.bits {
            false => Din31_0Dio4::Din31_0Dio4Zero,
            true => Din31_0Dio4::Din31_0Dio4One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio4_zero(&self) -> bool {
        *self == Din31_0Dio4::Din31_0Dio4Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio4_one(&self) -> bool {
        *self == Din31_0Dio4::Din31_0Dio4One
    }
}
#[doc = "This bit reads the data input value of DIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio5 {
    #[doc = "0: ZERO"]
    Din31_0Dio5Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio5One = 1,
}
impl From<Din31_0Dio5> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO5` reader - This bit reads the data input value of DIO5."]
pub type Din31_0Dio5R = crate::BitReader<Din31_0Dio5>;
impl Din31_0Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio5 {
        match self.bits {
            false => Din31_0Dio5::Din31_0Dio5Zero,
            true => Din31_0Dio5::Din31_0Dio5One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio5_zero(&self) -> bool {
        *self == Din31_0Dio5::Din31_0Dio5Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio5_one(&self) -> bool {
        *self == Din31_0Dio5::Din31_0Dio5One
    }
}
#[doc = "This bit reads the data input value of DIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio6 {
    #[doc = "0: ZERO"]
    Din31_0Dio6Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio6One = 1,
}
impl From<Din31_0Dio6> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO6` reader - This bit reads the data input value of DIO6."]
pub type Din31_0Dio6R = crate::BitReader<Din31_0Dio6>;
impl Din31_0Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio6 {
        match self.bits {
            false => Din31_0Dio6::Din31_0Dio6Zero,
            true => Din31_0Dio6::Din31_0Dio6One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio6_zero(&self) -> bool {
        *self == Din31_0Dio6::Din31_0Dio6Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio6_one(&self) -> bool {
        *self == Din31_0Dio6::Din31_0Dio6One
    }
}
#[doc = "This bit reads the data input value of DIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio7 {
    #[doc = "0: ZERO"]
    Din31_0Dio7Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio7One = 1,
}
impl From<Din31_0Dio7> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO7` reader - This bit reads the data input value of DIO7."]
pub type Din31_0Dio7R = crate::BitReader<Din31_0Dio7>;
impl Din31_0Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio7 {
        match self.bits {
            false => Din31_0Dio7::Din31_0Dio7Zero,
            true => Din31_0Dio7::Din31_0Dio7One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio7_zero(&self) -> bool {
        *self == Din31_0Dio7::Din31_0Dio7Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio7_one(&self) -> bool {
        *self == Din31_0Dio7::Din31_0Dio7One
    }
}
#[doc = "This bit reads the data input value of DIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio8 {
    #[doc = "0: ZERO"]
    Din31_0Dio8Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio8One = 1,
}
impl From<Din31_0Dio8> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO8` reader - This bit reads the data input value of DIO8."]
pub type Din31_0Dio8R = crate::BitReader<Din31_0Dio8>;
impl Din31_0Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio8 {
        match self.bits {
            false => Din31_0Dio8::Din31_0Dio8Zero,
            true => Din31_0Dio8::Din31_0Dio8One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio8_zero(&self) -> bool {
        *self == Din31_0Dio8::Din31_0Dio8Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio8_one(&self) -> bool {
        *self == Din31_0Dio8::Din31_0Dio8One
    }
}
#[doc = "This bit reads the data input value of DIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio9 {
    #[doc = "0: ZERO"]
    Din31_0Dio9Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio9One = 1,
}
impl From<Din31_0Dio9> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO9` reader - This bit reads the data input value of DIO9."]
pub type Din31_0Dio9R = crate::BitReader<Din31_0Dio9>;
impl Din31_0Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio9 {
        match self.bits {
            false => Din31_0Dio9::Din31_0Dio9Zero,
            true => Din31_0Dio9::Din31_0Dio9One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio9_zero(&self) -> bool {
        *self == Din31_0Dio9::Din31_0Dio9Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio9_one(&self) -> bool {
        *self == Din31_0Dio9::Din31_0Dio9One
    }
}
#[doc = "This bit reads the data input value of DIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio10 {
    #[doc = "0: ZERO"]
    Din31_0Dio10Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio10One = 1,
}
impl From<Din31_0Dio10> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO10` reader - This bit reads the data input value of DIO10."]
pub type Din31_0Dio10R = crate::BitReader<Din31_0Dio10>;
impl Din31_0Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio10 {
        match self.bits {
            false => Din31_0Dio10::Din31_0Dio10Zero,
            true => Din31_0Dio10::Din31_0Dio10One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio10_zero(&self) -> bool {
        *self == Din31_0Dio10::Din31_0Dio10Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio10_one(&self) -> bool {
        *self == Din31_0Dio10::Din31_0Dio10One
    }
}
#[doc = "This bit reads the data input value of DIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio11 {
    #[doc = "0: ZERO"]
    Din31_0Dio11Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio11One = 1,
}
impl From<Din31_0Dio11> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO11` reader - This bit reads the data input value of DIO11."]
pub type Din31_0Dio11R = crate::BitReader<Din31_0Dio11>;
impl Din31_0Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio11 {
        match self.bits {
            false => Din31_0Dio11::Din31_0Dio11Zero,
            true => Din31_0Dio11::Din31_0Dio11One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio11_zero(&self) -> bool {
        *self == Din31_0Dio11::Din31_0Dio11Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio11_one(&self) -> bool {
        *self == Din31_0Dio11::Din31_0Dio11One
    }
}
#[doc = "This bit reads the data input value of DIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio12 {
    #[doc = "0: ZERO"]
    Din31_0Dio12Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio12One = 1,
}
impl From<Din31_0Dio12> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO12` reader - This bit reads the data input value of DIO12."]
pub type Din31_0Dio12R = crate::BitReader<Din31_0Dio12>;
impl Din31_0Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio12 {
        match self.bits {
            false => Din31_0Dio12::Din31_0Dio12Zero,
            true => Din31_0Dio12::Din31_0Dio12One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio12_zero(&self) -> bool {
        *self == Din31_0Dio12::Din31_0Dio12Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio12_one(&self) -> bool {
        *self == Din31_0Dio12::Din31_0Dio12One
    }
}
#[doc = "This bit reads the data input value of DIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio13 {
    #[doc = "0: ZERO"]
    Din31_0Dio13Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio13One = 1,
}
impl From<Din31_0Dio13> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO13` reader - This bit reads the data input value of DIO13."]
pub type Din31_0Dio13R = crate::BitReader<Din31_0Dio13>;
impl Din31_0Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio13 {
        match self.bits {
            false => Din31_0Dio13::Din31_0Dio13Zero,
            true => Din31_0Dio13::Din31_0Dio13One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio13_zero(&self) -> bool {
        *self == Din31_0Dio13::Din31_0Dio13Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio13_one(&self) -> bool {
        *self == Din31_0Dio13::Din31_0Dio13One
    }
}
#[doc = "This bit reads the data input value of DIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio14 {
    #[doc = "0: ZERO"]
    Din31_0Dio14Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio14One = 1,
}
impl From<Din31_0Dio14> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO14` reader - This bit reads the data input value of DIO14."]
pub type Din31_0Dio14R = crate::BitReader<Din31_0Dio14>;
impl Din31_0Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio14 {
        match self.bits {
            false => Din31_0Dio14::Din31_0Dio14Zero,
            true => Din31_0Dio14::Din31_0Dio14One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio14_zero(&self) -> bool {
        *self == Din31_0Dio14::Din31_0Dio14Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio14_one(&self) -> bool {
        *self == Din31_0Dio14::Din31_0Dio14One
    }
}
#[doc = "This bit reads the data input value of DIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio15 {
    #[doc = "0: ZERO"]
    Din31_0Dio15Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio15One = 1,
}
impl From<Din31_0Dio15> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO15` reader - This bit reads the data input value of DIO15."]
pub type Din31_0Dio15R = crate::BitReader<Din31_0Dio15>;
impl Din31_0Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio15 {
        match self.bits {
            false => Din31_0Dio15::Din31_0Dio15Zero,
            true => Din31_0Dio15::Din31_0Dio15One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio15_zero(&self) -> bool {
        *self == Din31_0Dio15::Din31_0Dio15Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio15_one(&self) -> bool {
        *self == Din31_0Dio15::Din31_0Dio15One
    }
}
#[doc = "This bit reads the data input value of DIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio16 {
    #[doc = "0: ZERO"]
    Din31_0Dio16Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio16One = 1,
}
impl From<Din31_0Dio16> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO16` reader - This bit reads the data input value of DIO16."]
pub type Din31_0Dio16R = crate::BitReader<Din31_0Dio16>;
impl Din31_0Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio16 {
        match self.bits {
            false => Din31_0Dio16::Din31_0Dio16Zero,
            true => Din31_0Dio16::Din31_0Dio16One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio16_zero(&self) -> bool {
        *self == Din31_0Dio16::Din31_0Dio16Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio16_one(&self) -> bool {
        *self == Din31_0Dio16::Din31_0Dio16One
    }
}
#[doc = "This bit reads the data input value of DIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio17 {
    #[doc = "0: ZERO"]
    Din31_0Dio17Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio17One = 1,
}
impl From<Din31_0Dio17> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO17` reader - This bit reads the data input value of DIO17."]
pub type Din31_0Dio17R = crate::BitReader<Din31_0Dio17>;
impl Din31_0Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio17 {
        match self.bits {
            false => Din31_0Dio17::Din31_0Dio17Zero,
            true => Din31_0Dio17::Din31_0Dio17One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio17_zero(&self) -> bool {
        *self == Din31_0Dio17::Din31_0Dio17Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio17_one(&self) -> bool {
        *self == Din31_0Dio17::Din31_0Dio17One
    }
}
#[doc = "This bit reads the data input value of DIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio18 {
    #[doc = "0: ZERO"]
    Din31_0Dio18Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio18One = 1,
}
impl From<Din31_0Dio18> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO18` reader - This bit reads the data input value of DIO18."]
pub type Din31_0Dio18R = crate::BitReader<Din31_0Dio18>;
impl Din31_0Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio18 {
        match self.bits {
            false => Din31_0Dio18::Din31_0Dio18Zero,
            true => Din31_0Dio18::Din31_0Dio18One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio18_zero(&self) -> bool {
        *self == Din31_0Dio18::Din31_0Dio18Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio18_one(&self) -> bool {
        *self == Din31_0Dio18::Din31_0Dio18One
    }
}
#[doc = "This bit reads the data input value of DIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio19 {
    #[doc = "0: ZERO"]
    Din31_0Dio19Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio19One = 1,
}
impl From<Din31_0Dio19> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO19` reader - This bit reads the data input value of DIO19."]
pub type Din31_0Dio19R = crate::BitReader<Din31_0Dio19>;
impl Din31_0Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio19 {
        match self.bits {
            false => Din31_0Dio19::Din31_0Dio19Zero,
            true => Din31_0Dio19::Din31_0Dio19One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio19_zero(&self) -> bool {
        *self == Din31_0Dio19::Din31_0Dio19Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio19_one(&self) -> bool {
        *self == Din31_0Dio19::Din31_0Dio19One
    }
}
#[doc = "This bit reads the data input value of DIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio20 {
    #[doc = "0: ZERO"]
    Din31_0Dio20Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio20One = 1,
}
impl From<Din31_0Dio20> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO20` reader - This bit reads the data input value of DIO20."]
pub type Din31_0Dio20R = crate::BitReader<Din31_0Dio20>;
impl Din31_0Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio20 {
        match self.bits {
            false => Din31_0Dio20::Din31_0Dio20Zero,
            true => Din31_0Dio20::Din31_0Dio20One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio20_zero(&self) -> bool {
        *self == Din31_0Dio20::Din31_0Dio20Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio20_one(&self) -> bool {
        *self == Din31_0Dio20::Din31_0Dio20One
    }
}
#[doc = "This bit reads the data input value of DIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio21 {
    #[doc = "0: ZERO"]
    Din31_0Dio21Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio21One = 1,
}
impl From<Din31_0Dio21> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO21` reader - This bit reads the data input value of DIO21."]
pub type Din31_0Dio21R = crate::BitReader<Din31_0Dio21>;
impl Din31_0Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio21 {
        match self.bits {
            false => Din31_0Dio21::Din31_0Dio21Zero,
            true => Din31_0Dio21::Din31_0Dio21One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio21_zero(&self) -> bool {
        *self == Din31_0Dio21::Din31_0Dio21Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio21_one(&self) -> bool {
        *self == Din31_0Dio21::Din31_0Dio21One
    }
}
#[doc = "This bit reads the data input value of DIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio22 {
    #[doc = "0: ZERO"]
    Din31_0Dio22Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio22One = 1,
}
impl From<Din31_0Dio22> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO22` reader - This bit reads the data input value of DIO22."]
pub type Din31_0Dio22R = crate::BitReader<Din31_0Dio22>;
impl Din31_0Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio22 {
        match self.bits {
            false => Din31_0Dio22::Din31_0Dio22Zero,
            true => Din31_0Dio22::Din31_0Dio22One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio22_zero(&self) -> bool {
        *self == Din31_0Dio22::Din31_0Dio22Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio22_one(&self) -> bool {
        *self == Din31_0Dio22::Din31_0Dio22One
    }
}
#[doc = "This bit reads the data input value of DIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio23 {
    #[doc = "0: ZERO"]
    Din31_0Dio23Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio23One = 1,
}
impl From<Din31_0Dio23> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO23` reader - This bit reads the data input value of DIO23."]
pub type Din31_0Dio23R = crate::BitReader<Din31_0Dio23>;
impl Din31_0Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio23 {
        match self.bits {
            false => Din31_0Dio23::Din31_0Dio23Zero,
            true => Din31_0Dio23::Din31_0Dio23One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio23_zero(&self) -> bool {
        *self == Din31_0Dio23::Din31_0Dio23Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio23_one(&self) -> bool {
        *self == Din31_0Dio23::Din31_0Dio23One
    }
}
#[doc = "This bit reads the data input value of DIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio24 {
    #[doc = "0: ZERO"]
    Din31_0Dio24Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio24One = 1,
}
impl From<Din31_0Dio24> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO24` reader - This bit reads the data input value of DIO24."]
pub type Din31_0Dio24R = crate::BitReader<Din31_0Dio24>;
impl Din31_0Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio24 {
        match self.bits {
            false => Din31_0Dio24::Din31_0Dio24Zero,
            true => Din31_0Dio24::Din31_0Dio24One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio24_zero(&self) -> bool {
        *self == Din31_0Dio24::Din31_0Dio24Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio24_one(&self) -> bool {
        *self == Din31_0Dio24::Din31_0Dio24One
    }
}
#[doc = "This bit reads the data input value of DIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio25 {
    #[doc = "0: ZERO"]
    Din31_0Dio25Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio25One = 1,
}
impl From<Din31_0Dio25> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO25` reader - This bit reads the data input value of DIO25."]
pub type Din31_0Dio25R = crate::BitReader<Din31_0Dio25>;
impl Din31_0Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio25 {
        match self.bits {
            false => Din31_0Dio25::Din31_0Dio25Zero,
            true => Din31_0Dio25::Din31_0Dio25One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio25_zero(&self) -> bool {
        *self == Din31_0Dio25::Din31_0Dio25Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio25_one(&self) -> bool {
        *self == Din31_0Dio25::Din31_0Dio25One
    }
}
#[doc = "This bit reads the data input value of DIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio26 {
    #[doc = "0: ZERO"]
    Din31_0Dio26Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio26One = 1,
}
impl From<Din31_0Dio26> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO26` reader - This bit reads the data input value of DIO26."]
pub type Din31_0Dio26R = crate::BitReader<Din31_0Dio26>;
impl Din31_0Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio26 {
        match self.bits {
            false => Din31_0Dio26::Din31_0Dio26Zero,
            true => Din31_0Dio26::Din31_0Dio26One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio26_zero(&self) -> bool {
        *self == Din31_0Dio26::Din31_0Dio26Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio26_one(&self) -> bool {
        *self == Din31_0Dio26::Din31_0Dio26One
    }
}
#[doc = "This bit reads the data input value of DIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio27 {
    #[doc = "0: ZERO"]
    Din31_0Dio27Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio27One = 1,
}
impl From<Din31_0Dio27> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO27` reader - This bit reads the data input value of DIO27."]
pub type Din31_0Dio27R = crate::BitReader<Din31_0Dio27>;
impl Din31_0Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio27 {
        match self.bits {
            false => Din31_0Dio27::Din31_0Dio27Zero,
            true => Din31_0Dio27::Din31_0Dio27One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio27_zero(&self) -> bool {
        *self == Din31_0Dio27::Din31_0Dio27Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio27_one(&self) -> bool {
        *self == Din31_0Dio27::Din31_0Dio27One
    }
}
#[doc = "This bit reads the data input value of DIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio28 {
    #[doc = "0: ZERO"]
    Din31_0Dio28Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio28One = 1,
}
impl From<Din31_0Dio28> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO28` reader - This bit reads the data input value of DIO28."]
pub type Din31_0Dio28R = crate::BitReader<Din31_0Dio28>;
impl Din31_0Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio28 {
        match self.bits {
            false => Din31_0Dio28::Din31_0Dio28Zero,
            true => Din31_0Dio28::Din31_0Dio28One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio28_zero(&self) -> bool {
        *self == Din31_0Dio28::Din31_0Dio28Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio28_one(&self) -> bool {
        *self == Din31_0Dio28::Din31_0Dio28One
    }
}
#[doc = "This bit reads the data input value of DIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio29 {
    #[doc = "0: ZERO"]
    Din31_0Dio29Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio29One = 1,
}
impl From<Din31_0Dio29> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO29` reader - This bit reads the data input value of DIO29."]
pub type Din31_0Dio29R = crate::BitReader<Din31_0Dio29>;
impl Din31_0Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio29 {
        match self.bits {
            false => Din31_0Dio29::Din31_0Dio29Zero,
            true => Din31_0Dio29::Din31_0Dio29One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio29_zero(&self) -> bool {
        *self == Din31_0Dio29::Din31_0Dio29Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio29_one(&self) -> bool {
        *self == Din31_0Dio29::Din31_0Dio29One
    }
}
#[doc = "This bit reads the data input value of DIO30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio30 {
    #[doc = "0: ZERO"]
    Din31_0Dio30Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio30One = 1,
}
impl From<Din31_0Dio30> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO30` reader - This bit reads the data input value of DIO30."]
pub type Din31_0Dio30R = crate::BitReader<Din31_0Dio30>;
impl Din31_0Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio30 {
        match self.bits {
            false => Din31_0Dio30::Din31_0Dio30Zero,
            true => Din31_0Dio30::Din31_0Dio30One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio30_zero(&self) -> bool {
        *self == Din31_0Dio30::Din31_0Dio30Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio30_one(&self) -> bool {
        *self == Din31_0Dio30::Din31_0Dio30One
    }
}
#[doc = "This bit reads the data input value of DIO31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_0Dio31 {
    #[doc = "0: ZERO"]
    Din31_0Dio31Zero = 0,
    #[doc = "1: ONE"]
    Din31_0Dio31One = 1,
}
impl From<Din31_0Dio31> for bool {
    #[inline(always)]
    fn from(variant: Din31_0Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_0_DIO31` reader - This bit reads the data input value of DIO31."]
pub type Din31_0Dio31R = crate::BitReader<Din31_0Dio31>;
impl Din31_0Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_0Dio31 {
        match self.bits {
            false => Din31_0Dio31::Din31_0Dio31Zero,
            true => Din31_0Dio31::Din31_0Dio31One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_0_dio31_zero(&self) -> bool {
        *self == Din31_0Dio31::Din31_0Dio31Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_0_dio31_one(&self) -> bool {
        *self == Din31_0Dio31::Din31_0Dio31One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO0."]
    #[inline(always)]
    pub fn din31_0_dio0(&self) -> Din31_0Dio0R {
        Din31_0Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit reads the data input value of DIO1."]
    #[inline(always)]
    pub fn din31_0_dio1(&self) -> Din31_0Dio1R {
        Din31_0Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit reads the data input value of DIO2."]
    #[inline(always)]
    pub fn din31_0_dio2(&self) -> Din31_0Dio2R {
        Din31_0Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit reads the data input value of DIO3."]
    #[inline(always)]
    pub fn din31_0_dio3(&self) -> Din31_0Dio3R {
        Din31_0Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit reads the data input value of DIO4."]
    #[inline(always)]
    pub fn din31_0_dio4(&self) -> Din31_0Dio4R {
        Din31_0Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit reads the data input value of DIO5."]
    #[inline(always)]
    pub fn din31_0_dio5(&self) -> Din31_0Dio5R {
        Din31_0Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit reads the data input value of DIO6."]
    #[inline(always)]
    pub fn din31_0_dio6(&self) -> Din31_0Dio6R {
        Din31_0Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit reads the data input value of DIO7."]
    #[inline(always)]
    pub fn din31_0_dio7(&self) -> Din31_0Dio7R {
        Din31_0Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO8."]
    #[inline(always)]
    pub fn din31_0_dio8(&self) -> Din31_0Dio8R {
        Din31_0Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit reads the data input value of DIO9."]
    #[inline(always)]
    pub fn din31_0_dio9(&self) -> Din31_0Dio9R {
        Din31_0Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit reads the data input value of DIO10."]
    #[inline(always)]
    pub fn din31_0_dio10(&self) -> Din31_0Dio10R {
        Din31_0Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit reads the data input value of DIO11."]
    #[inline(always)]
    pub fn din31_0_dio11(&self) -> Din31_0Dio11R {
        Din31_0Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit reads the data input value of DIO12."]
    #[inline(always)]
    pub fn din31_0_dio12(&self) -> Din31_0Dio12R {
        Din31_0Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit reads the data input value of DIO13."]
    #[inline(always)]
    pub fn din31_0_dio13(&self) -> Din31_0Dio13R {
        Din31_0Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit reads the data input value of DIO14."]
    #[inline(always)]
    pub fn din31_0_dio14(&self) -> Din31_0Dio14R {
        Din31_0Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit reads the data input value of DIO15."]
    #[inline(always)]
    pub fn din31_0_dio15(&self) -> Din31_0Dio15R {
        Din31_0Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO16."]
    #[inline(always)]
    pub fn din31_0_dio16(&self) -> Din31_0Dio16R {
        Din31_0Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit reads the data input value of DIO17."]
    #[inline(always)]
    pub fn din31_0_dio17(&self) -> Din31_0Dio17R {
        Din31_0Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit reads the data input value of DIO18."]
    #[inline(always)]
    pub fn din31_0_dio18(&self) -> Din31_0Dio18R {
        Din31_0Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit reads the data input value of DIO19."]
    #[inline(always)]
    pub fn din31_0_dio19(&self) -> Din31_0Dio19R {
        Din31_0Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit reads the data input value of DIO20."]
    #[inline(always)]
    pub fn din31_0_dio20(&self) -> Din31_0Dio20R {
        Din31_0Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit reads the data input value of DIO21."]
    #[inline(always)]
    pub fn din31_0_dio21(&self) -> Din31_0Dio21R {
        Din31_0Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit reads the data input value of DIO22."]
    #[inline(always)]
    pub fn din31_0_dio22(&self) -> Din31_0Dio22R {
        Din31_0Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit reads the data input value of DIO23."]
    #[inline(always)]
    pub fn din31_0_dio23(&self) -> Din31_0Dio23R {
        Din31_0Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO24."]
    #[inline(always)]
    pub fn din31_0_dio24(&self) -> Din31_0Dio24R {
        Din31_0Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit reads the data input value of DIO25."]
    #[inline(always)]
    pub fn din31_0_dio25(&self) -> Din31_0Dio25R {
        Din31_0Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit reads the data input value of DIO26."]
    #[inline(always)]
    pub fn din31_0_dio26(&self) -> Din31_0Dio26R {
        Din31_0Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit reads the data input value of DIO27."]
    #[inline(always)]
    pub fn din31_0_dio27(&self) -> Din31_0Dio27R {
        Din31_0Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit reads the data input value of DIO28."]
    #[inline(always)]
    pub fn din31_0_dio28(&self) -> Din31_0Dio28R {
        Din31_0Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit reads the data input value of DIO29."]
    #[inline(always)]
    pub fn din31_0_dio29(&self) -> Din31_0Dio29R {
        Din31_0Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit reads the data input value of DIO30."]
    #[inline(always)]
    pub fn din31_0_dio30(&self) -> Din31_0Dio30R {
        Din31_0Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit reads the data input value of DIO31."]
    #[inline(always)]
    pub fn din31_0_dio31(&self) -> Din31_0Dio31R {
        Din31_0Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Data input 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`din31_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din31_0Spec;
impl crate::RegisterSpec for Din31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din31_0::R`](R) reader structure"]
impl crate::Readable for Din31_0Spec {}
#[doc = "`reset()` method sets DIN31_0 to value 0"]
impl crate::Resettable for Din31_0Spec {
    const RESET_VALUE: u32 = 0;
}

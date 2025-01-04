#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio0 {
    #[doc = "0: CLR"]
    IntEvent0RisDio0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio0Set = 1,
}
impl From<IntEvent0RisDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO0` reader - DIO0 event"]
pub type IntEvent0RisDio0R = crate::BitReader<IntEvent0RisDio0>;
impl IntEvent0RisDio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio0 {
        match self.bits {
            false => IntEvent0RisDio0::IntEvent0RisDio0Clr,
            true => IntEvent0RisDio0::IntEvent0RisDio0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio0_clr(&self) -> bool {
        *self == IntEvent0RisDio0::IntEvent0RisDio0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio0_set(&self) -> bool {
        *self == IntEvent0RisDio0::IntEvent0RisDio0Set
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio1 {
    #[doc = "0: CLR"]
    IntEvent0RisDio1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio1Set = 1,
}
impl From<IntEvent0RisDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO1` reader - DIO1 event"]
pub type IntEvent0RisDio1R = crate::BitReader<IntEvent0RisDio1>;
impl IntEvent0RisDio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio1 {
        match self.bits {
            false => IntEvent0RisDio1::IntEvent0RisDio1Clr,
            true => IntEvent0RisDio1::IntEvent0RisDio1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio1_clr(&self) -> bool {
        *self == IntEvent0RisDio1::IntEvent0RisDio1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio1_set(&self) -> bool {
        *self == IntEvent0RisDio1::IntEvent0RisDio1Set
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio2 {
    #[doc = "0: CLR"]
    IntEvent0RisDio2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio2Set = 1,
}
impl From<IntEvent0RisDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO2` reader - DIO2 event"]
pub type IntEvent0RisDio2R = crate::BitReader<IntEvent0RisDio2>;
impl IntEvent0RisDio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio2 {
        match self.bits {
            false => IntEvent0RisDio2::IntEvent0RisDio2Clr,
            true => IntEvent0RisDio2::IntEvent0RisDio2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio2_clr(&self) -> bool {
        *self == IntEvent0RisDio2::IntEvent0RisDio2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio2_set(&self) -> bool {
        *self == IntEvent0RisDio2::IntEvent0RisDio2Set
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio3 {
    #[doc = "0: CLR"]
    IntEvent0RisDio3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio3Set = 1,
}
impl From<IntEvent0RisDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO3` reader - DIO3 event"]
pub type IntEvent0RisDio3R = crate::BitReader<IntEvent0RisDio3>;
impl IntEvent0RisDio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio3 {
        match self.bits {
            false => IntEvent0RisDio3::IntEvent0RisDio3Clr,
            true => IntEvent0RisDio3::IntEvent0RisDio3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio3_clr(&self) -> bool {
        *self == IntEvent0RisDio3::IntEvent0RisDio3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio3_set(&self) -> bool {
        *self == IntEvent0RisDio3::IntEvent0RisDio3Set
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio4 {
    #[doc = "0: CLR"]
    IntEvent0RisDio4Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio4Set = 1,
}
impl From<IntEvent0RisDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO4` reader - DIO4 event"]
pub type IntEvent0RisDio4R = crate::BitReader<IntEvent0RisDio4>;
impl IntEvent0RisDio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio4 {
        match self.bits {
            false => IntEvent0RisDio4::IntEvent0RisDio4Clr,
            true => IntEvent0RisDio4::IntEvent0RisDio4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio4_clr(&self) -> bool {
        *self == IntEvent0RisDio4::IntEvent0RisDio4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio4_set(&self) -> bool {
        *self == IntEvent0RisDio4::IntEvent0RisDio4Set
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio5 {
    #[doc = "0: CLR"]
    IntEvent0RisDio5Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio5Set = 1,
}
impl From<IntEvent0RisDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO5` reader - DIO5 event"]
pub type IntEvent0RisDio5R = crate::BitReader<IntEvent0RisDio5>;
impl IntEvent0RisDio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio5 {
        match self.bits {
            false => IntEvent0RisDio5::IntEvent0RisDio5Clr,
            true => IntEvent0RisDio5::IntEvent0RisDio5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio5_clr(&self) -> bool {
        *self == IntEvent0RisDio5::IntEvent0RisDio5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio5_set(&self) -> bool {
        *self == IntEvent0RisDio5::IntEvent0RisDio5Set
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio6 {
    #[doc = "0: CLR"]
    IntEvent0RisDio6Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio6Set = 1,
}
impl From<IntEvent0RisDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO6` reader - DIO6 event"]
pub type IntEvent0RisDio6R = crate::BitReader<IntEvent0RisDio6>;
impl IntEvent0RisDio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio6 {
        match self.bits {
            false => IntEvent0RisDio6::IntEvent0RisDio6Clr,
            true => IntEvent0RisDio6::IntEvent0RisDio6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio6_clr(&self) -> bool {
        *self == IntEvent0RisDio6::IntEvent0RisDio6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio6_set(&self) -> bool {
        *self == IntEvent0RisDio6::IntEvent0RisDio6Set
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio7 {
    #[doc = "0: CLR"]
    IntEvent0RisDio7Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio7Set = 1,
}
impl From<IntEvent0RisDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO7` reader - DIO7 event"]
pub type IntEvent0RisDio7R = crate::BitReader<IntEvent0RisDio7>;
impl IntEvent0RisDio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio7 {
        match self.bits {
            false => IntEvent0RisDio7::IntEvent0RisDio7Clr,
            true => IntEvent0RisDio7::IntEvent0RisDio7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio7_clr(&self) -> bool {
        *self == IntEvent0RisDio7::IntEvent0RisDio7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio7_set(&self) -> bool {
        *self == IntEvent0RisDio7::IntEvent0RisDio7Set
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio8 {
    #[doc = "0: CLR"]
    IntEvent0RisDio8Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio8Set = 1,
}
impl From<IntEvent0RisDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO8` reader - DIO8 event"]
pub type IntEvent0RisDio8R = crate::BitReader<IntEvent0RisDio8>;
impl IntEvent0RisDio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio8 {
        match self.bits {
            false => IntEvent0RisDio8::IntEvent0RisDio8Clr,
            true => IntEvent0RisDio8::IntEvent0RisDio8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio8_clr(&self) -> bool {
        *self == IntEvent0RisDio8::IntEvent0RisDio8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio8_set(&self) -> bool {
        *self == IntEvent0RisDio8::IntEvent0RisDio8Set
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio9 {
    #[doc = "0: CLR"]
    IntEvent0RisDio9Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio9Set = 1,
}
impl From<IntEvent0RisDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO9` reader - DIO9 event"]
pub type IntEvent0RisDio9R = crate::BitReader<IntEvent0RisDio9>;
impl IntEvent0RisDio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio9 {
        match self.bits {
            false => IntEvent0RisDio9::IntEvent0RisDio9Clr,
            true => IntEvent0RisDio9::IntEvent0RisDio9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio9_clr(&self) -> bool {
        *self == IntEvent0RisDio9::IntEvent0RisDio9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio9_set(&self) -> bool {
        *self == IntEvent0RisDio9::IntEvent0RisDio9Set
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio10 {
    #[doc = "0: CLR"]
    IntEvent0RisDio10Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio10Set = 1,
}
impl From<IntEvent0RisDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO10` reader - DIO10 event"]
pub type IntEvent0RisDio10R = crate::BitReader<IntEvent0RisDio10>;
impl IntEvent0RisDio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio10 {
        match self.bits {
            false => IntEvent0RisDio10::IntEvent0RisDio10Clr,
            true => IntEvent0RisDio10::IntEvent0RisDio10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio10_clr(&self) -> bool {
        *self == IntEvent0RisDio10::IntEvent0RisDio10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio10_set(&self) -> bool {
        *self == IntEvent0RisDio10::IntEvent0RisDio10Set
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio11 {
    #[doc = "0: CLR"]
    IntEvent0RisDio11Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio11Set = 1,
}
impl From<IntEvent0RisDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO11` reader - DIO11 event"]
pub type IntEvent0RisDio11R = crate::BitReader<IntEvent0RisDio11>;
impl IntEvent0RisDio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio11 {
        match self.bits {
            false => IntEvent0RisDio11::IntEvent0RisDio11Clr,
            true => IntEvent0RisDio11::IntEvent0RisDio11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio11_clr(&self) -> bool {
        *self == IntEvent0RisDio11::IntEvent0RisDio11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio11_set(&self) -> bool {
        *self == IntEvent0RisDio11::IntEvent0RisDio11Set
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio12 {
    #[doc = "0: CLR"]
    IntEvent0RisDio12Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio12Set = 1,
}
impl From<IntEvent0RisDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO12` reader - DIO12 event"]
pub type IntEvent0RisDio12R = crate::BitReader<IntEvent0RisDio12>;
impl IntEvent0RisDio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio12 {
        match self.bits {
            false => IntEvent0RisDio12::IntEvent0RisDio12Clr,
            true => IntEvent0RisDio12::IntEvent0RisDio12Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio12_clr(&self) -> bool {
        *self == IntEvent0RisDio12::IntEvent0RisDio12Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio12_set(&self) -> bool {
        *self == IntEvent0RisDio12::IntEvent0RisDio12Set
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio13 {
    #[doc = "0: CLR"]
    IntEvent0RisDio13Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio13Set = 1,
}
impl From<IntEvent0RisDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO13` reader - DIO13 event"]
pub type IntEvent0RisDio13R = crate::BitReader<IntEvent0RisDio13>;
impl IntEvent0RisDio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio13 {
        match self.bits {
            false => IntEvent0RisDio13::IntEvent0RisDio13Clr,
            true => IntEvent0RisDio13::IntEvent0RisDio13Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio13_clr(&self) -> bool {
        *self == IntEvent0RisDio13::IntEvent0RisDio13Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio13_set(&self) -> bool {
        *self == IntEvent0RisDio13::IntEvent0RisDio13Set
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio14 {
    #[doc = "0: CLR"]
    IntEvent0RisDio14Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio14Set = 1,
}
impl From<IntEvent0RisDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO14` reader - DIO14 event"]
pub type IntEvent0RisDio14R = crate::BitReader<IntEvent0RisDio14>;
impl IntEvent0RisDio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio14 {
        match self.bits {
            false => IntEvent0RisDio14::IntEvent0RisDio14Clr,
            true => IntEvent0RisDio14::IntEvent0RisDio14Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio14_clr(&self) -> bool {
        *self == IntEvent0RisDio14::IntEvent0RisDio14Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio14_set(&self) -> bool {
        *self == IntEvent0RisDio14::IntEvent0RisDio14Set
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio15 {
    #[doc = "0: CLR"]
    IntEvent0RisDio15Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio15Set = 1,
}
impl From<IntEvent0RisDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO15` reader - DIO15 event"]
pub type IntEvent0RisDio15R = crate::BitReader<IntEvent0RisDio15>;
impl IntEvent0RisDio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio15 {
        match self.bits {
            false => IntEvent0RisDio15::IntEvent0RisDio15Clr,
            true => IntEvent0RisDio15::IntEvent0RisDio15Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio15_clr(&self) -> bool {
        *self == IntEvent0RisDio15::IntEvent0RisDio15Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio15_set(&self) -> bool {
        *self == IntEvent0RisDio15::IntEvent0RisDio15Set
    }
}
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio16 {
    #[doc = "0: CLR"]
    IntEvent0RisDio16Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio16Set = 1,
}
impl From<IntEvent0RisDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO16` reader - DIO16 event"]
pub type IntEvent0RisDio16R = crate::BitReader<IntEvent0RisDio16>;
impl IntEvent0RisDio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio16 {
        match self.bits {
            false => IntEvent0RisDio16::IntEvent0RisDio16Clr,
            true => IntEvent0RisDio16::IntEvent0RisDio16Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio16_clr(&self) -> bool {
        *self == IntEvent0RisDio16::IntEvent0RisDio16Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio16_set(&self) -> bool {
        *self == IntEvent0RisDio16::IntEvent0RisDio16Set
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio17 {
    #[doc = "0: CLR"]
    IntEvent0RisDio17Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio17Set = 1,
}
impl From<IntEvent0RisDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO17` reader - DIO17 event"]
pub type IntEvent0RisDio17R = crate::BitReader<IntEvent0RisDio17>;
impl IntEvent0RisDio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio17 {
        match self.bits {
            false => IntEvent0RisDio17::IntEvent0RisDio17Clr,
            true => IntEvent0RisDio17::IntEvent0RisDio17Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio17_clr(&self) -> bool {
        *self == IntEvent0RisDio17::IntEvent0RisDio17Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio17_set(&self) -> bool {
        *self == IntEvent0RisDio17::IntEvent0RisDio17Set
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio18 {
    #[doc = "0: CLR"]
    IntEvent0RisDio18Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio18Set = 1,
}
impl From<IntEvent0RisDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO18` reader - DIO18 event"]
pub type IntEvent0RisDio18R = crate::BitReader<IntEvent0RisDio18>;
impl IntEvent0RisDio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio18 {
        match self.bits {
            false => IntEvent0RisDio18::IntEvent0RisDio18Clr,
            true => IntEvent0RisDio18::IntEvent0RisDio18Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio18_clr(&self) -> bool {
        *self == IntEvent0RisDio18::IntEvent0RisDio18Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio18_set(&self) -> bool {
        *self == IntEvent0RisDio18::IntEvent0RisDio18Set
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio19 {
    #[doc = "0: CLR"]
    IntEvent0RisDio19Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio19Set = 1,
}
impl From<IntEvent0RisDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO19` reader - DIO19 event"]
pub type IntEvent0RisDio19R = crate::BitReader<IntEvent0RisDio19>;
impl IntEvent0RisDio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio19 {
        match self.bits {
            false => IntEvent0RisDio19::IntEvent0RisDio19Clr,
            true => IntEvent0RisDio19::IntEvent0RisDio19Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio19_clr(&self) -> bool {
        *self == IntEvent0RisDio19::IntEvent0RisDio19Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio19_set(&self) -> bool {
        *self == IntEvent0RisDio19::IntEvent0RisDio19Set
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio20 {
    #[doc = "0: CLR"]
    IntEvent0RisDio20Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio20Set = 1,
}
impl From<IntEvent0RisDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO20` reader - DIO20 event"]
pub type IntEvent0RisDio20R = crate::BitReader<IntEvent0RisDio20>;
impl IntEvent0RisDio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio20 {
        match self.bits {
            false => IntEvent0RisDio20::IntEvent0RisDio20Clr,
            true => IntEvent0RisDio20::IntEvent0RisDio20Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio20_clr(&self) -> bool {
        *self == IntEvent0RisDio20::IntEvent0RisDio20Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio20_set(&self) -> bool {
        *self == IntEvent0RisDio20::IntEvent0RisDio20Set
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio21 {
    #[doc = "0: CLR"]
    IntEvent0RisDio21Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio21Set = 1,
}
impl From<IntEvent0RisDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO21` reader - DIO21 event"]
pub type IntEvent0RisDio21R = crate::BitReader<IntEvent0RisDio21>;
impl IntEvent0RisDio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio21 {
        match self.bits {
            false => IntEvent0RisDio21::IntEvent0RisDio21Clr,
            true => IntEvent0RisDio21::IntEvent0RisDio21Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio21_clr(&self) -> bool {
        *self == IntEvent0RisDio21::IntEvent0RisDio21Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio21_set(&self) -> bool {
        *self == IntEvent0RisDio21::IntEvent0RisDio21Set
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio22 {
    #[doc = "0: CLR"]
    IntEvent0RisDio22Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio22Set = 1,
}
impl From<IntEvent0RisDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO22` reader - DIO22 event"]
pub type IntEvent0RisDio22R = crate::BitReader<IntEvent0RisDio22>;
impl IntEvent0RisDio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio22 {
        match self.bits {
            false => IntEvent0RisDio22::IntEvent0RisDio22Clr,
            true => IntEvent0RisDio22::IntEvent0RisDio22Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio22_clr(&self) -> bool {
        *self == IntEvent0RisDio22::IntEvent0RisDio22Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio22_set(&self) -> bool {
        *self == IntEvent0RisDio22::IntEvent0RisDio22Set
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio23 {
    #[doc = "0: CLR"]
    IntEvent0RisDio23Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio23Set = 1,
}
impl From<IntEvent0RisDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO23` reader - DIO23 event"]
pub type IntEvent0RisDio23R = crate::BitReader<IntEvent0RisDio23>;
impl IntEvent0RisDio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio23 {
        match self.bits {
            false => IntEvent0RisDio23::IntEvent0RisDio23Clr,
            true => IntEvent0RisDio23::IntEvent0RisDio23Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio23_clr(&self) -> bool {
        *self == IntEvent0RisDio23::IntEvent0RisDio23Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio23_set(&self) -> bool {
        *self == IntEvent0RisDio23::IntEvent0RisDio23Set
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio24 {
    #[doc = "0: CLR"]
    IntEvent0RisDio24Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio24Set = 1,
}
impl From<IntEvent0RisDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO24` reader - DIO24 event"]
pub type IntEvent0RisDio24R = crate::BitReader<IntEvent0RisDio24>;
impl IntEvent0RisDio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio24 {
        match self.bits {
            false => IntEvent0RisDio24::IntEvent0RisDio24Clr,
            true => IntEvent0RisDio24::IntEvent0RisDio24Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio24_clr(&self) -> bool {
        *self == IntEvent0RisDio24::IntEvent0RisDio24Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio24_set(&self) -> bool {
        *self == IntEvent0RisDio24::IntEvent0RisDio24Set
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio25 {
    #[doc = "0: CLR"]
    IntEvent0RisDio25Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio25Set = 1,
}
impl From<IntEvent0RisDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO25` reader - DIO25 event"]
pub type IntEvent0RisDio25R = crate::BitReader<IntEvent0RisDio25>;
impl IntEvent0RisDio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio25 {
        match self.bits {
            false => IntEvent0RisDio25::IntEvent0RisDio25Clr,
            true => IntEvent0RisDio25::IntEvent0RisDio25Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio25_clr(&self) -> bool {
        *self == IntEvent0RisDio25::IntEvent0RisDio25Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio25_set(&self) -> bool {
        *self == IntEvent0RisDio25::IntEvent0RisDio25Set
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio26 {
    #[doc = "0: CLR"]
    IntEvent0RisDio26Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio26Set = 1,
}
impl From<IntEvent0RisDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO26` reader - DIO26 event"]
pub type IntEvent0RisDio26R = crate::BitReader<IntEvent0RisDio26>;
impl IntEvent0RisDio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio26 {
        match self.bits {
            false => IntEvent0RisDio26::IntEvent0RisDio26Clr,
            true => IntEvent0RisDio26::IntEvent0RisDio26Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio26_clr(&self) -> bool {
        *self == IntEvent0RisDio26::IntEvent0RisDio26Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio26_set(&self) -> bool {
        *self == IntEvent0RisDio26::IntEvent0RisDio26Set
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio27 {
    #[doc = "0: CLR"]
    IntEvent0RisDio27Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio27Set = 1,
}
impl From<IntEvent0RisDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO27` reader - DIO27 event"]
pub type IntEvent0RisDio27R = crate::BitReader<IntEvent0RisDio27>;
impl IntEvent0RisDio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio27 {
        match self.bits {
            false => IntEvent0RisDio27::IntEvent0RisDio27Clr,
            true => IntEvent0RisDio27::IntEvent0RisDio27Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio27_clr(&self) -> bool {
        *self == IntEvent0RisDio27::IntEvent0RisDio27Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio27_set(&self) -> bool {
        *self == IntEvent0RisDio27::IntEvent0RisDio27Set
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio28 {
    #[doc = "0: CLR"]
    IntEvent0RisDio28Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio28Set = 1,
}
impl From<IntEvent0RisDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO28` reader - DIO28 event"]
pub type IntEvent0RisDio28R = crate::BitReader<IntEvent0RisDio28>;
impl IntEvent0RisDio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio28 {
        match self.bits {
            false => IntEvent0RisDio28::IntEvent0RisDio28Clr,
            true => IntEvent0RisDio28::IntEvent0RisDio28Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio28_clr(&self) -> bool {
        *self == IntEvent0RisDio28::IntEvent0RisDio28Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio28_set(&self) -> bool {
        *self == IntEvent0RisDio28::IntEvent0RisDio28Set
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio29 {
    #[doc = "0: CLR"]
    IntEvent0RisDio29Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio29Set = 1,
}
impl From<IntEvent0RisDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO29` reader - DIO29 event"]
pub type IntEvent0RisDio29R = crate::BitReader<IntEvent0RisDio29>;
impl IntEvent0RisDio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio29 {
        match self.bits {
            false => IntEvent0RisDio29::IntEvent0RisDio29Clr,
            true => IntEvent0RisDio29::IntEvent0RisDio29Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio29_clr(&self) -> bool {
        *self == IntEvent0RisDio29::IntEvent0RisDio29Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio29_set(&self) -> bool {
        *self == IntEvent0RisDio29::IntEvent0RisDio29Set
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio30 {
    #[doc = "0: CLR"]
    IntEvent0RisDio30Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio30Set = 1,
}
impl From<IntEvent0RisDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO30` reader - DIO30 event"]
pub type IntEvent0RisDio30R = crate::BitReader<IntEvent0RisDio30>;
impl IntEvent0RisDio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio30 {
        match self.bits {
            false => IntEvent0RisDio30::IntEvent0RisDio30Clr,
            true => IntEvent0RisDio30::IntEvent0RisDio30Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio30_clr(&self) -> bool {
        *self == IntEvent0RisDio30::IntEvent0RisDio30Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio30_set(&self) -> bool {
        *self == IntEvent0RisDio30::IntEvent0RisDio30Set
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDio31 {
    #[doc = "0: CLR"]
    IntEvent0RisDio31Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDio31Set = 1,
}
impl From<IntEvent0RisDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DIO31` reader - DIO31 event"]
pub type IntEvent0RisDio31R = crate::BitReader<IntEvent0RisDio31>;
impl IntEvent0RisDio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDio31 {
        match self.bits {
            false => IntEvent0RisDio31::IntEvent0RisDio31Clr,
            true => IntEvent0RisDio31::IntEvent0RisDio31Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio31_clr(&self) -> bool {
        *self == IntEvent0RisDio31::IntEvent0RisDio31Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dio31_set(&self) -> bool {
        *self == IntEvent0RisDio31::IntEvent0RisDio31Set
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio0(&self) -> IntEvent0RisDio0R {
        IntEvent0RisDio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio1(&self) -> IntEvent0RisDio1R {
        IntEvent0RisDio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio2(&self) -> IntEvent0RisDio2R {
        IntEvent0RisDio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio3(&self) -> IntEvent0RisDio3R {
        IntEvent0RisDio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio4(&self) -> IntEvent0RisDio4R {
        IntEvent0RisDio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio5(&self) -> IntEvent0RisDio5R {
        IntEvent0RisDio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio6(&self) -> IntEvent0RisDio6R {
        IntEvent0RisDio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio7(&self) -> IntEvent0RisDio7R {
        IntEvent0RisDio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio8(&self) -> IntEvent0RisDio8R {
        IntEvent0RisDio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio9(&self) -> IntEvent0RisDio9R {
        IntEvent0RisDio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio10(&self) -> IntEvent0RisDio10R {
        IntEvent0RisDio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio11(&self) -> IntEvent0RisDio11R {
        IntEvent0RisDio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio12(&self) -> IntEvent0RisDio12R {
        IntEvent0RisDio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio13(&self) -> IntEvent0RisDio13R {
        IntEvent0RisDio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio14(&self) -> IntEvent0RisDio14R {
        IntEvent0RisDio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio15(&self) -> IntEvent0RisDio15R {
        IntEvent0RisDio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio16(&self) -> IntEvent0RisDio16R {
        IntEvent0RisDio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio17(&self) -> IntEvent0RisDio17R {
        IntEvent0RisDio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio18(&self) -> IntEvent0RisDio18R {
        IntEvent0RisDio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio19(&self) -> IntEvent0RisDio19R {
        IntEvent0RisDio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio20(&self) -> IntEvent0RisDio20R {
        IntEvent0RisDio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio21(&self) -> IntEvent0RisDio21R {
        IntEvent0RisDio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio22(&self) -> IntEvent0RisDio22R {
        IntEvent0RisDio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio23(&self) -> IntEvent0RisDio23R {
        IntEvent0RisDio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio24(&self) -> IntEvent0RisDio24R {
        IntEvent0RisDio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio25(&self) -> IntEvent0RisDio25R {
        IntEvent0RisDio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio26(&self) -> IntEvent0RisDio26R {
        IntEvent0RisDio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio27(&self) -> IntEvent0RisDio27R {
        IntEvent0RisDio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio28(&self) -> IntEvent0RisDio28R {
        IntEvent0RisDio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio29(&self) -> IntEvent0RisDio29R {
        IntEvent0RisDio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio30(&self) -> IntEvent0RisDio30R {
        IntEvent0RisDio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event0_ris_dio31(&self) -> IntEvent0RisDio31R {
        IntEvent0RisDio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0RisSpec;
impl crate::RegisterSpec for IntEvent0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent0RisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_RIS to value 0"]
impl crate::Resettable for IntEvent0RisSpec {
    const RESET_VALUE: u32 = 0;
}

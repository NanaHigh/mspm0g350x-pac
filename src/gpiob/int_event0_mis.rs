#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio0 {
    #[doc = "0: CLR"]
    IntEvent0MisDio0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio0Set = 1,
}
impl From<IntEvent0MisDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO0` reader - DIO0 event"]
pub type IntEvent0MisDio0R = crate::BitReader<IntEvent0MisDio0>;
impl IntEvent0MisDio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio0 {
        match self.bits {
            false => IntEvent0MisDio0::IntEvent0MisDio0Clr,
            true => IntEvent0MisDio0::IntEvent0MisDio0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio0_clr(&self) -> bool {
        *self == IntEvent0MisDio0::IntEvent0MisDio0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio0_set(&self) -> bool {
        *self == IntEvent0MisDio0::IntEvent0MisDio0Set
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio1 {
    #[doc = "0: CLR"]
    IntEvent0MisDio1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio1Set = 1,
}
impl From<IntEvent0MisDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO1` reader - DIO1 event"]
pub type IntEvent0MisDio1R = crate::BitReader<IntEvent0MisDio1>;
impl IntEvent0MisDio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio1 {
        match self.bits {
            false => IntEvent0MisDio1::IntEvent0MisDio1Clr,
            true => IntEvent0MisDio1::IntEvent0MisDio1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio1_clr(&self) -> bool {
        *self == IntEvent0MisDio1::IntEvent0MisDio1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio1_set(&self) -> bool {
        *self == IntEvent0MisDio1::IntEvent0MisDio1Set
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio2 {
    #[doc = "0: CLR"]
    IntEvent0MisDio2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio2Set = 1,
}
impl From<IntEvent0MisDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO2` reader - DIO2 event"]
pub type IntEvent0MisDio2R = crate::BitReader<IntEvent0MisDio2>;
impl IntEvent0MisDio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio2 {
        match self.bits {
            false => IntEvent0MisDio2::IntEvent0MisDio2Clr,
            true => IntEvent0MisDio2::IntEvent0MisDio2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio2_clr(&self) -> bool {
        *self == IntEvent0MisDio2::IntEvent0MisDio2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio2_set(&self) -> bool {
        *self == IntEvent0MisDio2::IntEvent0MisDio2Set
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio3 {
    #[doc = "0: CLR"]
    IntEvent0MisDio3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio3Set = 1,
}
impl From<IntEvent0MisDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO3` reader - DIO3 event"]
pub type IntEvent0MisDio3R = crate::BitReader<IntEvent0MisDio3>;
impl IntEvent0MisDio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio3 {
        match self.bits {
            false => IntEvent0MisDio3::IntEvent0MisDio3Clr,
            true => IntEvent0MisDio3::IntEvent0MisDio3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio3_clr(&self) -> bool {
        *self == IntEvent0MisDio3::IntEvent0MisDio3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio3_set(&self) -> bool {
        *self == IntEvent0MisDio3::IntEvent0MisDio3Set
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio4 {
    #[doc = "0: CLR"]
    IntEvent0MisDio4Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio4Set = 1,
}
impl From<IntEvent0MisDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO4` reader - DIO4 event"]
pub type IntEvent0MisDio4R = crate::BitReader<IntEvent0MisDio4>;
impl IntEvent0MisDio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio4 {
        match self.bits {
            false => IntEvent0MisDio4::IntEvent0MisDio4Clr,
            true => IntEvent0MisDio4::IntEvent0MisDio4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio4_clr(&self) -> bool {
        *self == IntEvent0MisDio4::IntEvent0MisDio4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio4_set(&self) -> bool {
        *self == IntEvent0MisDio4::IntEvent0MisDio4Set
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio5 {
    #[doc = "0: CLR"]
    IntEvent0MisDio5Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio5Set = 1,
}
impl From<IntEvent0MisDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO5` reader - DIO5 event"]
pub type IntEvent0MisDio5R = crate::BitReader<IntEvent0MisDio5>;
impl IntEvent0MisDio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio5 {
        match self.bits {
            false => IntEvent0MisDio5::IntEvent0MisDio5Clr,
            true => IntEvent0MisDio5::IntEvent0MisDio5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio5_clr(&self) -> bool {
        *self == IntEvent0MisDio5::IntEvent0MisDio5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio5_set(&self) -> bool {
        *self == IntEvent0MisDio5::IntEvent0MisDio5Set
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio6 {
    #[doc = "0: CLR"]
    IntEvent0MisDio6Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio6Set = 1,
}
impl From<IntEvent0MisDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO6` reader - DIO6 event"]
pub type IntEvent0MisDio6R = crate::BitReader<IntEvent0MisDio6>;
impl IntEvent0MisDio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio6 {
        match self.bits {
            false => IntEvent0MisDio6::IntEvent0MisDio6Clr,
            true => IntEvent0MisDio6::IntEvent0MisDio6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio6_clr(&self) -> bool {
        *self == IntEvent0MisDio6::IntEvent0MisDio6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio6_set(&self) -> bool {
        *self == IntEvent0MisDio6::IntEvent0MisDio6Set
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio7 {
    #[doc = "0: CLR"]
    IntEvent0MisDio7Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio7Set = 1,
}
impl From<IntEvent0MisDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO7` reader - DIO7 event"]
pub type IntEvent0MisDio7R = crate::BitReader<IntEvent0MisDio7>;
impl IntEvent0MisDio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio7 {
        match self.bits {
            false => IntEvent0MisDio7::IntEvent0MisDio7Clr,
            true => IntEvent0MisDio7::IntEvent0MisDio7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio7_clr(&self) -> bool {
        *self == IntEvent0MisDio7::IntEvent0MisDio7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio7_set(&self) -> bool {
        *self == IntEvent0MisDio7::IntEvent0MisDio7Set
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio8 {
    #[doc = "0: CLR"]
    IntEvent0MisDio8Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio8Set = 1,
}
impl From<IntEvent0MisDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO8` reader - DIO8 event"]
pub type IntEvent0MisDio8R = crate::BitReader<IntEvent0MisDio8>;
impl IntEvent0MisDio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio8 {
        match self.bits {
            false => IntEvent0MisDio8::IntEvent0MisDio8Clr,
            true => IntEvent0MisDio8::IntEvent0MisDio8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio8_clr(&self) -> bool {
        *self == IntEvent0MisDio8::IntEvent0MisDio8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio8_set(&self) -> bool {
        *self == IntEvent0MisDio8::IntEvent0MisDio8Set
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio9 {
    #[doc = "0: CLR"]
    IntEvent0MisDio9Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio9Set = 1,
}
impl From<IntEvent0MisDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO9` reader - DIO9 event"]
pub type IntEvent0MisDio9R = crate::BitReader<IntEvent0MisDio9>;
impl IntEvent0MisDio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio9 {
        match self.bits {
            false => IntEvent0MisDio9::IntEvent0MisDio9Clr,
            true => IntEvent0MisDio9::IntEvent0MisDio9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio9_clr(&self) -> bool {
        *self == IntEvent0MisDio9::IntEvent0MisDio9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio9_set(&self) -> bool {
        *self == IntEvent0MisDio9::IntEvent0MisDio9Set
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio10 {
    #[doc = "0: CLR"]
    IntEvent0MisDio10Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio10Set = 1,
}
impl From<IntEvent0MisDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO10` reader - DIO10 event"]
pub type IntEvent0MisDio10R = crate::BitReader<IntEvent0MisDio10>;
impl IntEvent0MisDio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio10 {
        match self.bits {
            false => IntEvent0MisDio10::IntEvent0MisDio10Clr,
            true => IntEvent0MisDio10::IntEvent0MisDio10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio10_clr(&self) -> bool {
        *self == IntEvent0MisDio10::IntEvent0MisDio10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio10_set(&self) -> bool {
        *self == IntEvent0MisDio10::IntEvent0MisDio10Set
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio11 {
    #[doc = "0: CLR"]
    IntEvent0MisDio11Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio11Set = 1,
}
impl From<IntEvent0MisDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO11` reader - DIO11 event"]
pub type IntEvent0MisDio11R = crate::BitReader<IntEvent0MisDio11>;
impl IntEvent0MisDio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio11 {
        match self.bits {
            false => IntEvent0MisDio11::IntEvent0MisDio11Clr,
            true => IntEvent0MisDio11::IntEvent0MisDio11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio11_clr(&self) -> bool {
        *self == IntEvent0MisDio11::IntEvent0MisDio11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio11_set(&self) -> bool {
        *self == IntEvent0MisDio11::IntEvent0MisDio11Set
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio12 {
    #[doc = "0: CLR"]
    IntEvent0MisDio12Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio12Set = 1,
}
impl From<IntEvent0MisDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO12` reader - DIO12 event"]
pub type IntEvent0MisDio12R = crate::BitReader<IntEvent0MisDio12>;
impl IntEvent0MisDio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio12 {
        match self.bits {
            false => IntEvent0MisDio12::IntEvent0MisDio12Clr,
            true => IntEvent0MisDio12::IntEvent0MisDio12Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio12_clr(&self) -> bool {
        *self == IntEvent0MisDio12::IntEvent0MisDio12Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio12_set(&self) -> bool {
        *self == IntEvent0MisDio12::IntEvent0MisDio12Set
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio13 {
    #[doc = "0: CLR"]
    IntEvent0MisDio13Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio13Set = 1,
}
impl From<IntEvent0MisDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO13` reader - DIO13 event"]
pub type IntEvent0MisDio13R = crate::BitReader<IntEvent0MisDio13>;
impl IntEvent0MisDio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio13 {
        match self.bits {
            false => IntEvent0MisDio13::IntEvent0MisDio13Clr,
            true => IntEvent0MisDio13::IntEvent0MisDio13Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio13_clr(&self) -> bool {
        *self == IntEvent0MisDio13::IntEvent0MisDio13Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio13_set(&self) -> bool {
        *self == IntEvent0MisDio13::IntEvent0MisDio13Set
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio14 {
    #[doc = "0: CLR"]
    IntEvent0MisDio14Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio14Set = 1,
}
impl From<IntEvent0MisDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO14` reader - DIO14 event"]
pub type IntEvent0MisDio14R = crate::BitReader<IntEvent0MisDio14>;
impl IntEvent0MisDio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio14 {
        match self.bits {
            false => IntEvent0MisDio14::IntEvent0MisDio14Clr,
            true => IntEvent0MisDio14::IntEvent0MisDio14Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio14_clr(&self) -> bool {
        *self == IntEvent0MisDio14::IntEvent0MisDio14Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio14_set(&self) -> bool {
        *self == IntEvent0MisDio14::IntEvent0MisDio14Set
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio15 {
    #[doc = "0: CLR"]
    IntEvent0MisDio15Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio15Set = 1,
}
impl From<IntEvent0MisDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO15` reader - DIO15 event"]
pub type IntEvent0MisDio15R = crate::BitReader<IntEvent0MisDio15>;
impl IntEvent0MisDio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio15 {
        match self.bits {
            false => IntEvent0MisDio15::IntEvent0MisDio15Clr,
            true => IntEvent0MisDio15::IntEvent0MisDio15Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio15_clr(&self) -> bool {
        *self == IntEvent0MisDio15::IntEvent0MisDio15Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio15_set(&self) -> bool {
        *self == IntEvent0MisDio15::IntEvent0MisDio15Set
    }
}
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio16 {
    #[doc = "0: CLR"]
    IntEvent0MisDio16Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio16Set = 1,
}
impl From<IntEvent0MisDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO16` reader - DIO16 event"]
pub type IntEvent0MisDio16R = crate::BitReader<IntEvent0MisDio16>;
impl IntEvent0MisDio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio16 {
        match self.bits {
            false => IntEvent0MisDio16::IntEvent0MisDio16Clr,
            true => IntEvent0MisDio16::IntEvent0MisDio16Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio16_clr(&self) -> bool {
        *self == IntEvent0MisDio16::IntEvent0MisDio16Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio16_set(&self) -> bool {
        *self == IntEvent0MisDio16::IntEvent0MisDio16Set
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio17 {
    #[doc = "0: CLR"]
    IntEvent0MisDio17Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio17Set = 1,
}
impl From<IntEvent0MisDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO17` reader - DIO17 event"]
pub type IntEvent0MisDio17R = crate::BitReader<IntEvent0MisDio17>;
impl IntEvent0MisDio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio17 {
        match self.bits {
            false => IntEvent0MisDio17::IntEvent0MisDio17Clr,
            true => IntEvent0MisDio17::IntEvent0MisDio17Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio17_clr(&self) -> bool {
        *self == IntEvent0MisDio17::IntEvent0MisDio17Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio17_set(&self) -> bool {
        *self == IntEvent0MisDio17::IntEvent0MisDio17Set
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio18 {
    #[doc = "0: CLR"]
    IntEvent0MisDio18Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio18Set = 1,
}
impl From<IntEvent0MisDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO18` reader - DIO18 event"]
pub type IntEvent0MisDio18R = crate::BitReader<IntEvent0MisDio18>;
impl IntEvent0MisDio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio18 {
        match self.bits {
            false => IntEvent0MisDio18::IntEvent0MisDio18Clr,
            true => IntEvent0MisDio18::IntEvent0MisDio18Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio18_clr(&self) -> bool {
        *self == IntEvent0MisDio18::IntEvent0MisDio18Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio18_set(&self) -> bool {
        *self == IntEvent0MisDio18::IntEvent0MisDio18Set
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio19 {
    #[doc = "0: CLR"]
    IntEvent0MisDio19Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio19Set = 1,
}
impl From<IntEvent0MisDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO19` reader - DIO19 event"]
pub type IntEvent0MisDio19R = crate::BitReader<IntEvent0MisDio19>;
impl IntEvent0MisDio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio19 {
        match self.bits {
            false => IntEvent0MisDio19::IntEvent0MisDio19Clr,
            true => IntEvent0MisDio19::IntEvent0MisDio19Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio19_clr(&self) -> bool {
        *self == IntEvent0MisDio19::IntEvent0MisDio19Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio19_set(&self) -> bool {
        *self == IntEvent0MisDio19::IntEvent0MisDio19Set
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio20 {
    #[doc = "0: CLR"]
    IntEvent0MisDio20Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio20Set = 1,
}
impl From<IntEvent0MisDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO20` reader - DIO20 event"]
pub type IntEvent0MisDio20R = crate::BitReader<IntEvent0MisDio20>;
impl IntEvent0MisDio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio20 {
        match self.bits {
            false => IntEvent0MisDio20::IntEvent0MisDio20Clr,
            true => IntEvent0MisDio20::IntEvent0MisDio20Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio20_clr(&self) -> bool {
        *self == IntEvent0MisDio20::IntEvent0MisDio20Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio20_set(&self) -> bool {
        *self == IntEvent0MisDio20::IntEvent0MisDio20Set
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio21 {
    #[doc = "0: CLR"]
    IntEvent0MisDio21Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio21Set = 1,
}
impl From<IntEvent0MisDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO21` reader - DIO21 event"]
pub type IntEvent0MisDio21R = crate::BitReader<IntEvent0MisDio21>;
impl IntEvent0MisDio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio21 {
        match self.bits {
            false => IntEvent0MisDio21::IntEvent0MisDio21Clr,
            true => IntEvent0MisDio21::IntEvent0MisDio21Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio21_clr(&self) -> bool {
        *self == IntEvent0MisDio21::IntEvent0MisDio21Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio21_set(&self) -> bool {
        *self == IntEvent0MisDio21::IntEvent0MisDio21Set
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio22 {
    #[doc = "0: CLR"]
    IntEvent0MisDio22Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio22Set = 1,
}
impl From<IntEvent0MisDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO22` reader - DIO22 event"]
pub type IntEvent0MisDio22R = crate::BitReader<IntEvent0MisDio22>;
impl IntEvent0MisDio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio22 {
        match self.bits {
            false => IntEvent0MisDio22::IntEvent0MisDio22Clr,
            true => IntEvent0MisDio22::IntEvent0MisDio22Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio22_clr(&self) -> bool {
        *self == IntEvent0MisDio22::IntEvent0MisDio22Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio22_set(&self) -> bool {
        *self == IntEvent0MisDio22::IntEvent0MisDio22Set
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio23 {
    #[doc = "0: CLR"]
    IntEvent0MisDio23Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio23Set = 1,
}
impl From<IntEvent0MisDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO23` reader - DIO23 event"]
pub type IntEvent0MisDio23R = crate::BitReader<IntEvent0MisDio23>;
impl IntEvent0MisDio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio23 {
        match self.bits {
            false => IntEvent0MisDio23::IntEvent0MisDio23Clr,
            true => IntEvent0MisDio23::IntEvent0MisDio23Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio23_clr(&self) -> bool {
        *self == IntEvent0MisDio23::IntEvent0MisDio23Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio23_set(&self) -> bool {
        *self == IntEvent0MisDio23::IntEvent0MisDio23Set
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio24 {
    #[doc = "0: CLR"]
    IntEvent0MisDio24Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio24Set = 1,
}
impl From<IntEvent0MisDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO24` reader - DIO24 event"]
pub type IntEvent0MisDio24R = crate::BitReader<IntEvent0MisDio24>;
impl IntEvent0MisDio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio24 {
        match self.bits {
            false => IntEvent0MisDio24::IntEvent0MisDio24Clr,
            true => IntEvent0MisDio24::IntEvent0MisDio24Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio24_clr(&self) -> bool {
        *self == IntEvent0MisDio24::IntEvent0MisDio24Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio24_set(&self) -> bool {
        *self == IntEvent0MisDio24::IntEvent0MisDio24Set
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio25 {
    #[doc = "0: CLR"]
    IntEvent0MisDio25Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio25Set = 1,
}
impl From<IntEvent0MisDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO25` reader - DIO25 event"]
pub type IntEvent0MisDio25R = crate::BitReader<IntEvent0MisDio25>;
impl IntEvent0MisDio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio25 {
        match self.bits {
            false => IntEvent0MisDio25::IntEvent0MisDio25Clr,
            true => IntEvent0MisDio25::IntEvent0MisDio25Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio25_clr(&self) -> bool {
        *self == IntEvent0MisDio25::IntEvent0MisDio25Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio25_set(&self) -> bool {
        *self == IntEvent0MisDio25::IntEvent0MisDio25Set
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio26 {
    #[doc = "0: CLR"]
    IntEvent0MisDio26Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio26Set = 1,
}
impl From<IntEvent0MisDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO26` reader - DIO26 event"]
pub type IntEvent0MisDio26R = crate::BitReader<IntEvent0MisDio26>;
impl IntEvent0MisDio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio26 {
        match self.bits {
            false => IntEvent0MisDio26::IntEvent0MisDio26Clr,
            true => IntEvent0MisDio26::IntEvent0MisDio26Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio26_clr(&self) -> bool {
        *self == IntEvent0MisDio26::IntEvent0MisDio26Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio26_set(&self) -> bool {
        *self == IntEvent0MisDio26::IntEvent0MisDio26Set
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio27 {
    #[doc = "0: CLR"]
    IntEvent0MisDio27Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio27Set = 1,
}
impl From<IntEvent0MisDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO27` reader - DIO27 event"]
pub type IntEvent0MisDio27R = crate::BitReader<IntEvent0MisDio27>;
impl IntEvent0MisDio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio27 {
        match self.bits {
            false => IntEvent0MisDio27::IntEvent0MisDio27Clr,
            true => IntEvent0MisDio27::IntEvent0MisDio27Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio27_clr(&self) -> bool {
        *self == IntEvent0MisDio27::IntEvent0MisDio27Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio27_set(&self) -> bool {
        *self == IntEvent0MisDio27::IntEvent0MisDio27Set
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio28 {
    #[doc = "0: CLR"]
    IntEvent0MisDio28Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio28Set = 1,
}
impl From<IntEvent0MisDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO28` reader - DIO28 event"]
pub type IntEvent0MisDio28R = crate::BitReader<IntEvent0MisDio28>;
impl IntEvent0MisDio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio28 {
        match self.bits {
            false => IntEvent0MisDio28::IntEvent0MisDio28Clr,
            true => IntEvent0MisDio28::IntEvent0MisDio28Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio28_clr(&self) -> bool {
        *self == IntEvent0MisDio28::IntEvent0MisDio28Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio28_set(&self) -> bool {
        *self == IntEvent0MisDio28::IntEvent0MisDio28Set
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio29 {
    #[doc = "0: CLR"]
    IntEvent0MisDio29Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio29Set = 1,
}
impl From<IntEvent0MisDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO29` reader - DIO29 event"]
pub type IntEvent0MisDio29R = crate::BitReader<IntEvent0MisDio29>;
impl IntEvent0MisDio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio29 {
        match self.bits {
            false => IntEvent0MisDio29::IntEvent0MisDio29Clr,
            true => IntEvent0MisDio29::IntEvent0MisDio29Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio29_clr(&self) -> bool {
        *self == IntEvent0MisDio29::IntEvent0MisDio29Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio29_set(&self) -> bool {
        *self == IntEvent0MisDio29::IntEvent0MisDio29Set
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio30 {
    #[doc = "0: CLR"]
    IntEvent0MisDio30Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio30Set = 1,
}
impl From<IntEvent0MisDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO30` reader - DIO30 event"]
pub type IntEvent0MisDio30R = crate::BitReader<IntEvent0MisDio30>;
impl IntEvent0MisDio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio30 {
        match self.bits {
            false => IntEvent0MisDio30::IntEvent0MisDio30Clr,
            true => IntEvent0MisDio30::IntEvent0MisDio30Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio30_clr(&self) -> bool {
        *self == IntEvent0MisDio30::IntEvent0MisDio30Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio30_set(&self) -> bool {
        *self == IntEvent0MisDio30::IntEvent0MisDio30Set
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDio31 {
    #[doc = "0: CLR"]
    IntEvent0MisDio31Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDio31Set = 1,
}
impl From<IntEvent0MisDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DIO31` reader - DIO31 event"]
pub type IntEvent0MisDio31R = crate::BitReader<IntEvent0MisDio31>;
impl IntEvent0MisDio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDio31 {
        match self.bits {
            false => IntEvent0MisDio31::IntEvent0MisDio31Clr,
            true => IntEvent0MisDio31::IntEvent0MisDio31Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio31_clr(&self) -> bool {
        *self == IntEvent0MisDio31::IntEvent0MisDio31Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dio31_set(&self) -> bool {
        *self == IntEvent0MisDio31::IntEvent0MisDio31Set
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio0(&self) -> IntEvent0MisDio0R {
        IntEvent0MisDio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio1(&self) -> IntEvent0MisDio1R {
        IntEvent0MisDio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio2(&self) -> IntEvent0MisDio2R {
        IntEvent0MisDio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio3(&self) -> IntEvent0MisDio3R {
        IntEvent0MisDio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio4(&self) -> IntEvent0MisDio4R {
        IntEvent0MisDio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio5(&self) -> IntEvent0MisDio5R {
        IntEvent0MisDio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio6(&self) -> IntEvent0MisDio6R {
        IntEvent0MisDio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio7(&self) -> IntEvent0MisDio7R {
        IntEvent0MisDio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio8(&self) -> IntEvent0MisDio8R {
        IntEvent0MisDio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio9(&self) -> IntEvent0MisDio9R {
        IntEvent0MisDio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio10(&self) -> IntEvent0MisDio10R {
        IntEvent0MisDio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio11(&self) -> IntEvent0MisDio11R {
        IntEvent0MisDio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio12(&self) -> IntEvent0MisDio12R {
        IntEvent0MisDio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio13(&self) -> IntEvent0MisDio13R {
        IntEvent0MisDio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio14(&self) -> IntEvent0MisDio14R {
        IntEvent0MisDio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio15(&self) -> IntEvent0MisDio15R {
        IntEvent0MisDio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio16(&self) -> IntEvent0MisDio16R {
        IntEvent0MisDio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio17(&self) -> IntEvent0MisDio17R {
        IntEvent0MisDio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio18(&self) -> IntEvent0MisDio18R {
        IntEvent0MisDio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio19(&self) -> IntEvent0MisDio19R {
        IntEvent0MisDio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio20(&self) -> IntEvent0MisDio20R {
        IntEvent0MisDio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio21(&self) -> IntEvent0MisDio21R {
        IntEvent0MisDio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio22(&self) -> IntEvent0MisDio22R {
        IntEvent0MisDio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio23(&self) -> IntEvent0MisDio23R {
        IntEvent0MisDio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio24(&self) -> IntEvent0MisDio24R {
        IntEvent0MisDio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio25(&self) -> IntEvent0MisDio25R {
        IntEvent0MisDio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio26(&self) -> IntEvent0MisDio26R {
        IntEvent0MisDio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio27(&self) -> IntEvent0MisDio27R {
        IntEvent0MisDio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio28(&self) -> IntEvent0MisDio28R {
        IntEvent0MisDio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio29(&self) -> IntEvent0MisDio29R {
        IntEvent0MisDio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio30(&self) -> IntEvent0MisDio30R {
        IntEvent0MisDio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event0_mis_dio31(&self) -> IntEvent0MisDio31R {
        IntEvent0MisDio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0MisSpec;
impl crate::RegisterSpec for IntEvent0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent0MisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_MIS to value 0"]
impl crate::Resettable for IntEvent0MisSpec {
    const RESET_VALUE: u32 = 0;
}

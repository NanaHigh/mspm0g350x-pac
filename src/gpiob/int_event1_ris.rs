#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio0 {
    #[doc = "0: CLR"]
    IntEvent1RisDio0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio0Set = 1,
}
impl From<IntEvent1RisDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO0` reader - DIO0 event"]
pub type IntEvent1RisDio0R = crate::BitReader<IntEvent1RisDio0>;
impl IntEvent1RisDio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio0 {
        match self.bits {
            false => IntEvent1RisDio0::IntEvent1RisDio0Clr,
            true => IntEvent1RisDio0::IntEvent1RisDio0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio0_clr(&self) -> bool {
        *self == IntEvent1RisDio0::IntEvent1RisDio0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio0_set(&self) -> bool {
        *self == IntEvent1RisDio0::IntEvent1RisDio0Set
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio1 {
    #[doc = "0: CLR"]
    IntEvent1RisDio1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio1Set = 1,
}
impl From<IntEvent1RisDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO1` reader - DIO1 event"]
pub type IntEvent1RisDio1R = crate::BitReader<IntEvent1RisDio1>;
impl IntEvent1RisDio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio1 {
        match self.bits {
            false => IntEvent1RisDio1::IntEvent1RisDio1Clr,
            true => IntEvent1RisDio1::IntEvent1RisDio1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio1_clr(&self) -> bool {
        *self == IntEvent1RisDio1::IntEvent1RisDio1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio1_set(&self) -> bool {
        *self == IntEvent1RisDio1::IntEvent1RisDio1Set
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio2 {
    #[doc = "0: CLR"]
    IntEvent1RisDio2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio2Set = 1,
}
impl From<IntEvent1RisDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO2` reader - DIO2 event"]
pub type IntEvent1RisDio2R = crate::BitReader<IntEvent1RisDio2>;
impl IntEvent1RisDio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio2 {
        match self.bits {
            false => IntEvent1RisDio2::IntEvent1RisDio2Clr,
            true => IntEvent1RisDio2::IntEvent1RisDio2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio2_clr(&self) -> bool {
        *self == IntEvent1RisDio2::IntEvent1RisDio2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio2_set(&self) -> bool {
        *self == IntEvent1RisDio2::IntEvent1RisDio2Set
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio3 {
    #[doc = "0: CLR"]
    IntEvent1RisDio3Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio3Set = 1,
}
impl From<IntEvent1RisDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO3` reader - DIO3 event"]
pub type IntEvent1RisDio3R = crate::BitReader<IntEvent1RisDio3>;
impl IntEvent1RisDio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio3 {
        match self.bits {
            false => IntEvent1RisDio3::IntEvent1RisDio3Clr,
            true => IntEvent1RisDio3::IntEvent1RisDio3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio3_clr(&self) -> bool {
        *self == IntEvent1RisDio3::IntEvent1RisDio3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio3_set(&self) -> bool {
        *self == IntEvent1RisDio3::IntEvent1RisDio3Set
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio4 {
    #[doc = "0: CLR"]
    IntEvent1RisDio4Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio4Set = 1,
}
impl From<IntEvent1RisDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO4` reader - DIO4 event"]
pub type IntEvent1RisDio4R = crate::BitReader<IntEvent1RisDio4>;
impl IntEvent1RisDio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio4 {
        match self.bits {
            false => IntEvent1RisDio4::IntEvent1RisDio4Clr,
            true => IntEvent1RisDio4::IntEvent1RisDio4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio4_clr(&self) -> bool {
        *self == IntEvent1RisDio4::IntEvent1RisDio4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio4_set(&self) -> bool {
        *self == IntEvent1RisDio4::IntEvent1RisDio4Set
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio5 {
    #[doc = "0: CLR"]
    IntEvent1RisDio5Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio5Set = 1,
}
impl From<IntEvent1RisDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO5` reader - DIO5 event"]
pub type IntEvent1RisDio5R = crate::BitReader<IntEvent1RisDio5>;
impl IntEvent1RisDio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio5 {
        match self.bits {
            false => IntEvent1RisDio5::IntEvent1RisDio5Clr,
            true => IntEvent1RisDio5::IntEvent1RisDio5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio5_clr(&self) -> bool {
        *self == IntEvent1RisDio5::IntEvent1RisDio5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio5_set(&self) -> bool {
        *self == IntEvent1RisDio5::IntEvent1RisDio5Set
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio6 {
    #[doc = "0: CLR"]
    IntEvent1RisDio6Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio6Set = 1,
}
impl From<IntEvent1RisDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO6` reader - DIO6 event"]
pub type IntEvent1RisDio6R = crate::BitReader<IntEvent1RisDio6>;
impl IntEvent1RisDio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio6 {
        match self.bits {
            false => IntEvent1RisDio6::IntEvent1RisDio6Clr,
            true => IntEvent1RisDio6::IntEvent1RisDio6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio6_clr(&self) -> bool {
        *self == IntEvent1RisDio6::IntEvent1RisDio6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio6_set(&self) -> bool {
        *self == IntEvent1RisDio6::IntEvent1RisDio6Set
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio7 {
    #[doc = "0: CLR"]
    IntEvent1RisDio7Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio7Set = 1,
}
impl From<IntEvent1RisDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO7` reader - DIO7 event"]
pub type IntEvent1RisDio7R = crate::BitReader<IntEvent1RisDio7>;
impl IntEvent1RisDio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio7 {
        match self.bits {
            false => IntEvent1RisDio7::IntEvent1RisDio7Clr,
            true => IntEvent1RisDio7::IntEvent1RisDio7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio7_clr(&self) -> bool {
        *self == IntEvent1RisDio7::IntEvent1RisDio7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio7_set(&self) -> bool {
        *self == IntEvent1RisDio7::IntEvent1RisDio7Set
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio8 {
    #[doc = "0: CLR"]
    IntEvent1RisDio8Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio8Set = 1,
}
impl From<IntEvent1RisDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO8` reader - DIO8 event"]
pub type IntEvent1RisDio8R = crate::BitReader<IntEvent1RisDio8>;
impl IntEvent1RisDio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio8 {
        match self.bits {
            false => IntEvent1RisDio8::IntEvent1RisDio8Clr,
            true => IntEvent1RisDio8::IntEvent1RisDio8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio8_clr(&self) -> bool {
        *self == IntEvent1RisDio8::IntEvent1RisDio8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio8_set(&self) -> bool {
        *self == IntEvent1RisDio8::IntEvent1RisDio8Set
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio9 {
    #[doc = "0: CLR"]
    IntEvent1RisDio9Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio9Set = 1,
}
impl From<IntEvent1RisDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO9` reader - DIO9 event"]
pub type IntEvent1RisDio9R = crate::BitReader<IntEvent1RisDio9>;
impl IntEvent1RisDio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio9 {
        match self.bits {
            false => IntEvent1RisDio9::IntEvent1RisDio9Clr,
            true => IntEvent1RisDio9::IntEvent1RisDio9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio9_clr(&self) -> bool {
        *self == IntEvent1RisDio9::IntEvent1RisDio9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio9_set(&self) -> bool {
        *self == IntEvent1RisDio9::IntEvent1RisDio9Set
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio10 {
    #[doc = "0: CLR"]
    IntEvent1RisDio10Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio10Set = 1,
}
impl From<IntEvent1RisDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO10` reader - DIO10 event"]
pub type IntEvent1RisDio10R = crate::BitReader<IntEvent1RisDio10>;
impl IntEvent1RisDio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio10 {
        match self.bits {
            false => IntEvent1RisDio10::IntEvent1RisDio10Clr,
            true => IntEvent1RisDio10::IntEvent1RisDio10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio10_clr(&self) -> bool {
        *self == IntEvent1RisDio10::IntEvent1RisDio10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio10_set(&self) -> bool {
        *self == IntEvent1RisDio10::IntEvent1RisDio10Set
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio11 {
    #[doc = "0: CLR"]
    IntEvent1RisDio11Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio11Set = 1,
}
impl From<IntEvent1RisDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO11` reader - DIO11 event"]
pub type IntEvent1RisDio11R = crate::BitReader<IntEvent1RisDio11>;
impl IntEvent1RisDio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio11 {
        match self.bits {
            false => IntEvent1RisDio11::IntEvent1RisDio11Clr,
            true => IntEvent1RisDio11::IntEvent1RisDio11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio11_clr(&self) -> bool {
        *self == IntEvent1RisDio11::IntEvent1RisDio11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio11_set(&self) -> bool {
        *self == IntEvent1RisDio11::IntEvent1RisDio11Set
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio12 {
    #[doc = "0: CLR"]
    IntEvent1RisDio12Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio12Set = 1,
}
impl From<IntEvent1RisDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO12` reader - DIO12 event"]
pub type IntEvent1RisDio12R = crate::BitReader<IntEvent1RisDio12>;
impl IntEvent1RisDio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio12 {
        match self.bits {
            false => IntEvent1RisDio12::IntEvent1RisDio12Clr,
            true => IntEvent1RisDio12::IntEvent1RisDio12Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio12_clr(&self) -> bool {
        *self == IntEvent1RisDio12::IntEvent1RisDio12Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio12_set(&self) -> bool {
        *self == IntEvent1RisDio12::IntEvent1RisDio12Set
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio13 {
    #[doc = "0: CLR"]
    IntEvent1RisDio13Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio13Set = 1,
}
impl From<IntEvent1RisDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO13` reader - DIO13 event"]
pub type IntEvent1RisDio13R = crate::BitReader<IntEvent1RisDio13>;
impl IntEvent1RisDio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio13 {
        match self.bits {
            false => IntEvent1RisDio13::IntEvent1RisDio13Clr,
            true => IntEvent1RisDio13::IntEvent1RisDio13Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio13_clr(&self) -> bool {
        *self == IntEvent1RisDio13::IntEvent1RisDio13Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio13_set(&self) -> bool {
        *self == IntEvent1RisDio13::IntEvent1RisDio13Set
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio14 {
    #[doc = "0: CLR"]
    IntEvent1RisDio14Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio14Set = 1,
}
impl From<IntEvent1RisDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO14` reader - DIO14 event"]
pub type IntEvent1RisDio14R = crate::BitReader<IntEvent1RisDio14>;
impl IntEvent1RisDio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio14 {
        match self.bits {
            false => IntEvent1RisDio14::IntEvent1RisDio14Clr,
            true => IntEvent1RisDio14::IntEvent1RisDio14Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio14_clr(&self) -> bool {
        *self == IntEvent1RisDio14::IntEvent1RisDio14Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio14_set(&self) -> bool {
        *self == IntEvent1RisDio14::IntEvent1RisDio14Set
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDio15 {
    #[doc = "0: CLR"]
    IntEvent1RisDio15Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDio15Set = 1,
}
impl From<IntEvent1RisDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DIO15` reader - DIO15 event"]
pub type IntEvent1RisDio15R = crate::BitReader<IntEvent1RisDio15>;
impl IntEvent1RisDio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDio15 {
        match self.bits {
            false => IntEvent1RisDio15::IntEvent1RisDio15Clr,
            true => IntEvent1RisDio15::IntEvent1RisDio15Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio15_clr(&self) -> bool {
        *self == IntEvent1RisDio15::IntEvent1RisDio15Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dio15_set(&self) -> bool {
        *self == IntEvent1RisDio15::IntEvent1RisDio15Set
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio0(&self) -> IntEvent1RisDio0R {
        IntEvent1RisDio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio1(&self) -> IntEvent1RisDio1R {
        IntEvent1RisDio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio2(&self) -> IntEvent1RisDio2R {
        IntEvent1RisDio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio3(&self) -> IntEvent1RisDio3R {
        IntEvent1RisDio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio4(&self) -> IntEvent1RisDio4R {
        IntEvent1RisDio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio5(&self) -> IntEvent1RisDio5R {
        IntEvent1RisDio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio6(&self) -> IntEvent1RisDio6R {
        IntEvent1RisDio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio7(&self) -> IntEvent1RisDio7R {
        IntEvent1RisDio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio8(&self) -> IntEvent1RisDio8R {
        IntEvent1RisDio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio9(&self) -> IntEvent1RisDio9R {
        IntEvent1RisDio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio10(&self) -> IntEvent1RisDio10R {
        IntEvent1RisDio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio11(&self) -> IntEvent1RisDio11R {
        IntEvent1RisDio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio12(&self) -> IntEvent1RisDio12R {
        IntEvent1RisDio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio13(&self) -> IntEvent1RisDio13R {
        IntEvent1RisDio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio14(&self) -> IntEvent1RisDio14R {
        IntEvent1RisDio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event1_ris_dio15(&self) -> IntEvent1RisDio15R {
        IntEvent1RisDio15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1RisSpec;
impl crate::RegisterSpec for IntEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent1RisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_RIS to value 0"]
impl crate::Resettable for IntEvent1RisSpec {
    const RESET_VALUE: u32 = 0;
}

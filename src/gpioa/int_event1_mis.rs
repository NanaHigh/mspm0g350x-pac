#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio0 {
    #[doc = "0: CLR"]
    IntEvent1MisDio0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio0Set = 1,
}
impl From<IntEvent1MisDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO0` reader - DIO0 event"]
pub type IntEvent1MisDio0R = crate::BitReader<IntEvent1MisDio0>;
impl IntEvent1MisDio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio0 {
        match self.bits {
            false => IntEvent1MisDio0::IntEvent1MisDio0Clr,
            true => IntEvent1MisDio0::IntEvent1MisDio0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio0_clr(&self) -> bool {
        *self == IntEvent1MisDio0::IntEvent1MisDio0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio0_set(&self) -> bool {
        *self == IntEvent1MisDio0::IntEvent1MisDio0Set
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio1 {
    #[doc = "0: CLR"]
    IntEvent1MisDio1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio1Set = 1,
}
impl From<IntEvent1MisDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO1` reader - DIO1 event"]
pub type IntEvent1MisDio1R = crate::BitReader<IntEvent1MisDio1>;
impl IntEvent1MisDio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio1 {
        match self.bits {
            false => IntEvent1MisDio1::IntEvent1MisDio1Clr,
            true => IntEvent1MisDio1::IntEvent1MisDio1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio1_clr(&self) -> bool {
        *self == IntEvent1MisDio1::IntEvent1MisDio1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio1_set(&self) -> bool {
        *self == IntEvent1MisDio1::IntEvent1MisDio1Set
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio2 {
    #[doc = "0: CLR"]
    IntEvent1MisDio2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio2Set = 1,
}
impl From<IntEvent1MisDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO2` reader - DIO2 event"]
pub type IntEvent1MisDio2R = crate::BitReader<IntEvent1MisDio2>;
impl IntEvent1MisDio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio2 {
        match self.bits {
            false => IntEvent1MisDio2::IntEvent1MisDio2Clr,
            true => IntEvent1MisDio2::IntEvent1MisDio2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio2_clr(&self) -> bool {
        *self == IntEvent1MisDio2::IntEvent1MisDio2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio2_set(&self) -> bool {
        *self == IntEvent1MisDio2::IntEvent1MisDio2Set
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio3 {
    #[doc = "0: CLR"]
    IntEvent1MisDio3Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio3Set = 1,
}
impl From<IntEvent1MisDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO3` reader - DIO3 event"]
pub type IntEvent1MisDio3R = crate::BitReader<IntEvent1MisDio3>;
impl IntEvent1MisDio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio3 {
        match self.bits {
            false => IntEvent1MisDio3::IntEvent1MisDio3Clr,
            true => IntEvent1MisDio3::IntEvent1MisDio3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio3_clr(&self) -> bool {
        *self == IntEvent1MisDio3::IntEvent1MisDio3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio3_set(&self) -> bool {
        *self == IntEvent1MisDio3::IntEvent1MisDio3Set
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio4 {
    #[doc = "0: CLR"]
    IntEvent1MisDio4Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio4Set = 1,
}
impl From<IntEvent1MisDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO4` reader - DIO4 event"]
pub type IntEvent1MisDio4R = crate::BitReader<IntEvent1MisDio4>;
impl IntEvent1MisDio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio4 {
        match self.bits {
            false => IntEvent1MisDio4::IntEvent1MisDio4Clr,
            true => IntEvent1MisDio4::IntEvent1MisDio4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio4_clr(&self) -> bool {
        *self == IntEvent1MisDio4::IntEvent1MisDio4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio4_set(&self) -> bool {
        *self == IntEvent1MisDio4::IntEvent1MisDio4Set
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio5 {
    #[doc = "0: CLR"]
    IntEvent1MisDio5Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio5Set = 1,
}
impl From<IntEvent1MisDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO5` reader - DIO5 event"]
pub type IntEvent1MisDio5R = crate::BitReader<IntEvent1MisDio5>;
impl IntEvent1MisDio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio5 {
        match self.bits {
            false => IntEvent1MisDio5::IntEvent1MisDio5Clr,
            true => IntEvent1MisDio5::IntEvent1MisDio5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio5_clr(&self) -> bool {
        *self == IntEvent1MisDio5::IntEvent1MisDio5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio5_set(&self) -> bool {
        *self == IntEvent1MisDio5::IntEvent1MisDio5Set
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio6 {
    #[doc = "0: CLR"]
    IntEvent1MisDio6Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio6Set = 1,
}
impl From<IntEvent1MisDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO6` reader - DIO6 event"]
pub type IntEvent1MisDio6R = crate::BitReader<IntEvent1MisDio6>;
impl IntEvent1MisDio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio6 {
        match self.bits {
            false => IntEvent1MisDio6::IntEvent1MisDio6Clr,
            true => IntEvent1MisDio6::IntEvent1MisDio6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio6_clr(&self) -> bool {
        *self == IntEvent1MisDio6::IntEvent1MisDio6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio6_set(&self) -> bool {
        *self == IntEvent1MisDio6::IntEvent1MisDio6Set
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio7 {
    #[doc = "0: CLR"]
    IntEvent1MisDio7Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio7Set = 1,
}
impl From<IntEvent1MisDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO7` reader - DIO7 event"]
pub type IntEvent1MisDio7R = crate::BitReader<IntEvent1MisDio7>;
impl IntEvent1MisDio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio7 {
        match self.bits {
            false => IntEvent1MisDio7::IntEvent1MisDio7Clr,
            true => IntEvent1MisDio7::IntEvent1MisDio7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio7_clr(&self) -> bool {
        *self == IntEvent1MisDio7::IntEvent1MisDio7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio7_set(&self) -> bool {
        *self == IntEvent1MisDio7::IntEvent1MisDio7Set
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio8 {
    #[doc = "0: CLR"]
    IntEvent1MisDio8Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio8Set = 1,
}
impl From<IntEvent1MisDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO8` reader - DIO8 event"]
pub type IntEvent1MisDio8R = crate::BitReader<IntEvent1MisDio8>;
impl IntEvent1MisDio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio8 {
        match self.bits {
            false => IntEvent1MisDio8::IntEvent1MisDio8Clr,
            true => IntEvent1MisDio8::IntEvent1MisDio8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio8_clr(&self) -> bool {
        *self == IntEvent1MisDio8::IntEvent1MisDio8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio8_set(&self) -> bool {
        *self == IntEvent1MisDio8::IntEvent1MisDio8Set
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio9 {
    #[doc = "0: CLR"]
    IntEvent1MisDio9Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio9Set = 1,
}
impl From<IntEvent1MisDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO9` reader - DIO9 event"]
pub type IntEvent1MisDio9R = crate::BitReader<IntEvent1MisDio9>;
impl IntEvent1MisDio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio9 {
        match self.bits {
            false => IntEvent1MisDio9::IntEvent1MisDio9Clr,
            true => IntEvent1MisDio9::IntEvent1MisDio9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio9_clr(&self) -> bool {
        *self == IntEvent1MisDio9::IntEvent1MisDio9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio9_set(&self) -> bool {
        *self == IntEvent1MisDio9::IntEvent1MisDio9Set
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio10 {
    #[doc = "0: CLR"]
    IntEvent1MisDio10Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio10Set = 1,
}
impl From<IntEvent1MisDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO10` reader - DIO10 event"]
pub type IntEvent1MisDio10R = crate::BitReader<IntEvent1MisDio10>;
impl IntEvent1MisDio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio10 {
        match self.bits {
            false => IntEvent1MisDio10::IntEvent1MisDio10Clr,
            true => IntEvent1MisDio10::IntEvent1MisDio10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio10_clr(&self) -> bool {
        *self == IntEvent1MisDio10::IntEvent1MisDio10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio10_set(&self) -> bool {
        *self == IntEvent1MisDio10::IntEvent1MisDio10Set
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio11 {
    #[doc = "0: CLR"]
    IntEvent1MisDio11Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio11Set = 1,
}
impl From<IntEvent1MisDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO11` reader - DIO11 event"]
pub type IntEvent1MisDio11R = crate::BitReader<IntEvent1MisDio11>;
impl IntEvent1MisDio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio11 {
        match self.bits {
            false => IntEvent1MisDio11::IntEvent1MisDio11Clr,
            true => IntEvent1MisDio11::IntEvent1MisDio11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio11_clr(&self) -> bool {
        *self == IntEvent1MisDio11::IntEvent1MisDio11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio11_set(&self) -> bool {
        *self == IntEvent1MisDio11::IntEvent1MisDio11Set
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio12 {
    #[doc = "0: CLR"]
    IntEvent1MisDio12Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio12Set = 1,
}
impl From<IntEvent1MisDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO12` reader - DIO12 event"]
pub type IntEvent1MisDio12R = crate::BitReader<IntEvent1MisDio12>;
impl IntEvent1MisDio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio12 {
        match self.bits {
            false => IntEvent1MisDio12::IntEvent1MisDio12Clr,
            true => IntEvent1MisDio12::IntEvent1MisDio12Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio12_clr(&self) -> bool {
        *self == IntEvent1MisDio12::IntEvent1MisDio12Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio12_set(&self) -> bool {
        *self == IntEvent1MisDio12::IntEvent1MisDio12Set
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio13 {
    #[doc = "0: CLR"]
    IntEvent1MisDio13Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio13Set = 1,
}
impl From<IntEvent1MisDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO13` reader - DIO13 event"]
pub type IntEvent1MisDio13R = crate::BitReader<IntEvent1MisDio13>;
impl IntEvent1MisDio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio13 {
        match self.bits {
            false => IntEvent1MisDio13::IntEvent1MisDio13Clr,
            true => IntEvent1MisDio13::IntEvent1MisDio13Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio13_clr(&self) -> bool {
        *self == IntEvent1MisDio13::IntEvent1MisDio13Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio13_set(&self) -> bool {
        *self == IntEvent1MisDio13::IntEvent1MisDio13Set
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio14 {
    #[doc = "0: CLR"]
    IntEvent1MisDio14Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio14Set = 1,
}
impl From<IntEvent1MisDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO14` reader - DIO14 event"]
pub type IntEvent1MisDio14R = crate::BitReader<IntEvent1MisDio14>;
impl IntEvent1MisDio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio14 {
        match self.bits {
            false => IntEvent1MisDio14::IntEvent1MisDio14Clr,
            true => IntEvent1MisDio14::IntEvent1MisDio14Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio14_clr(&self) -> bool {
        *self == IntEvent1MisDio14::IntEvent1MisDio14Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio14_set(&self) -> bool {
        *self == IntEvent1MisDio14::IntEvent1MisDio14Set
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDio15 {
    #[doc = "0: CLR"]
    IntEvent1MisDio15Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDio15Set = 1,
}
impl From<IntEvent1MisDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DIO15` reader - DIO15 event"]
pub type IntEvent1MisDio15R = crate::BitReader<IntEvent1MisDio15>;
impl IntEvent1MisDio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDio15 {
        match self.bits {
            false => IntEvent1MisDio15::IntEvent1MisDio15Clr,
            true => IntEvent1MisDio15::IntEvent1MisDio15Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio15_clr(&self) -> bool {
        *self == IntEvent1MisDio15::IntEvent1MisDio15Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dio15_set(&self) -> bool {
        *self == IntEvent1MisDio15::IntEvent1MisDio15Set
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio0(&self) -> IntEvent1MisDio0R {
        IntEvent1MisDio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio1(&self) -> IntEvent1MisDio1R {
        IntEvent1MisDio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio2(&self) -> IntEvent1MisDio2R {
        IntEvent1MisDio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio3(&self) -> IntEvent1MisDio3R {
        IntEvent1MisDio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio4(&self) -> IntEvent1MisDio4R {
        IntEvent1MisDio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio5(&self) -> IntEvent1MisDio5R {
        IntEvent1MisDio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio6(&self) -> IntEvent1MisDio6R {
        IntEvent1MisDio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio7(&self) -> IntEvent1MisDio7R {
        IntEvent1MisDio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio8(&self) -> IntEvent1MisDio8R {
        IntEvent1MisDio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio9(&self) -> IntEvent1MisDio9R {
        IntEvent1MisDio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio10(&self) -> IntEvent1MisDio10R {
        IntEvent1MisDio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio11(&self) -> IntEvent1MisDio11R {
        IntEvent1MisDio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio12(&self) -> IntEvent1MisDio12R {
        IntEvent1MisDio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio13(&self) -> IntEvent1MisDio13R {
        IntEvent1MisDio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio14(&self) -> IntEvent1MisDio14R {
        IntEvent1MisDio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event1_mis_dio15(&self) -> IntEvent1MisDio15R {
        IntEvent1MisDio15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1MisSpec;
impl crate::RegisterSpec for IntEvent1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent1MisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_MIS to value 0"]
impl crate::Resettable for IntEvent1MisSpec {
    const RESET_VALUE: u32 = 0;
}

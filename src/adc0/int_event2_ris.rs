#[doc = "Register `INT_EVENT2_RIS` reader"]
pub type R = crate::R<IntEvent2RisSpec>;
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg0Set = 1,
}
impl From<IntEvent2RisMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg0R = crate::BitReader<IntEvent2RisMemresifg0>;
impl IntEvent2RisMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg0 {
        match self.bits {
            false => IntEvent2RisMemresifg0::IntEvent2RisMemresifg0Clr,
            true => IntEvent2RisMemresifg0::IntEvent2RisMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg0_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg0::IntEvent2RisMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg0_set(&self) -> bool {
        *self == IntEvent2RisMemresifg0::IntEvent2RisMemresifg0Set
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg1 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg1Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg1Set = 1,
}
impl From<IntEvent2RisMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg1R = crate::BitReader<IntEvent2RisMemresifg1>;
impl IntEvent2RisMemresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg1 {
        match self.bits {
            false => IntEvent2RisMemresifg1::IntEvent2RisMemresifg1Clr,
            true => IntEvent2RisMemresifg1::IntEvent2RisMemresifg1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg1_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg1::IntEvent2RisMemresifg1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg1_set(&self) -> bool {
        *self == IntEvent2RisMemresifg1::IntEvent2RisMemresifg1Set
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg2 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg2Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg2Set = 1,
}
impl From<IntEvent2RisMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg2R = crate::BitReader<IntEvent2RisMemresifg2>;
impl IntEvent2RisMemresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg2 {
        match self.bits {
            false => IntEvent2RisMemresifg2::IntEvent2RisMemresifg2Clr,
            true => IntEvent2RisMemresifg2::IntEvent2RisMemresifg2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg2_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg2::IntEvent2RisMemresifg2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg2_set(&self) -> bool {
        *self == IntEvent2RisMemresifg2::IntEvent2RisMemresifg2Set
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg3 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg3Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg3Set = 1,
}
impl From<IntEvent2RisMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg3R = crate::BitReader<IntEvent2RisMemresifg3>;
impl IntEvent2RisMemresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg3 {
        match self.bits {
            false => IntEvent2RisMemresifg3::IntEvent2RisMemresifg3Clr,
            true => IntEvent2RisMemresifg3::IntEvent2RisMemresifg3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg3_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg3::IntEvent2RisMemresifg3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg3_set(&self) -> bool {
        *self == IntEvent2RisMemresifg3::IntEvent2RisMemresifg3Set
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg4 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg4Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg4Set = 1,
}
impl From<IntEvent2RisMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg4R = crate::BitReader<IntEvent2RisMemresifg4>;
impl IntEvent2RisMemresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg4 {
        match self.bits {
            false => IntEvent2RisMemresifg4::IntEvent2RisMemresifg4Clr,
            true => IntEvent2RisMemresifg4::IntEvent2RisMemresifg4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg4_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg4::IntEvent2RisMemresifg4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg4_set(&self) -> bool {
        *self == IntEvent2RisMemresifg4::IntEvent2RisMemresifg4Set
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg5 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg5Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg5Set = 1,
}
impl From<IntEvent2RisMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg5R = crate::BitReader<IntEvent2RisMemresifg5>;
impl IntEvent2RisMemresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg5 {
        match self.bits {
            false => IntEvent2RisMemresifg5::IntEvent2RisMemresifg5Clr,
            true => IntEvent2RisMemresifg5::IntEvent2RisMemresifg5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg5_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg5::IntEvent2RisMemresifg5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg5_set(&self) -> bool {
        *self == IntEvent2RisMemresifg5::IntEvent2RisMemresifg5Set
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg6 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg6Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg6Set = 1,
}
impl From<IntEvent2RisMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg6R = crate::BitReader<IntEvent2RisMemresifg6>;
impl IntEvent2RisMemresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg6 {
        match self.bits {
            false => IntEvent2RisMemresifg6::IntEvent2RisMemresifg6Clr,
            true => IntEvent2RisMemresifg6::IntEvent2RisMemresifg6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg6_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg6::IntEvent2RisMemresifg6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg6_set(&self) -> bool {
        *self == IntEvent2RisMemresifg6::IntEvent2RisMemresifg6Set
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg7 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg7Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg7Set = 1,
}
impl From<IntEvent2RisMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg7R = crate::BitReader<IntEvent2RisMemresifg7>;
impl IntEvent2RisMemresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg7 {
        match self.bits {
            false => IntEvent2RisMemresifg7::IntEvent2RisMemresifg7Clr,
            true => IntEvent2RisMemresifg7::IntEvent2RisMemresifg7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg7_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg7::IntEvent2RisMemresifg7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg7_set(&self) -> bool {
        *self == IntEvent2RisMemresifg7::IntEvent2RisMemresifg7Set
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg8 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg8Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg8Set = 1,
}
impl From<IntEvent2RisMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg8R = crate::BitReader<IntEvent2RisMemresifg8>;
impl IntEvent2RisMemresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg8 {
        match self.bits {
            false => IntEvent2RisMemresifg8::IntEvent2RisMemresifg8Clr,
            true => IntEvent2RisMemresifg8::IntEvent2RisMemresifg8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg8_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg8::IntEvent2RisMemresifg8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg8_set(&self) -> bool {
        *self == IntEvent2RisMemresifg8::IntEvent2RisMemresifg8Set
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg9 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg9Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg9Set = 1,
}
impl From<IntEvent2RisMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg9R = crate::BitReader<IntEvent2RisMemresifg9>;
impl IntEvent2RisMemresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg9 {
        match self.bits {
            false => IntEvent2RisMemresifg9::IntEvent2RisMemresifg9Clr,
            true => IntEvent2RisMemresifg9::IntEvent2RisMemresifg9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg9_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg9::IntEvent2RisMemresifg9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg9_set(&self) -> bool {
        *self == IntEvent2RisMemresifg9::IntEvent2RisMemresifg9Set
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg10 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg10Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg10Set = 1,
}
impl From<IntEvent2RisMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg10R = crate::BitReader<IntEvent2RisMemresifg10>;
impl IntEvent2RisMemresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg10 {
        match self.bits {
            false => IntEvent2RisMemresifg10::IntEvent2RisMemresifg10Clr,
            true => IntEvent2RisMemresifg10::IntEvent2RisMemresifg10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg10_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg10::IntEvent2RisMemresifg10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg10_set(&self) -> bool {
        *self == IntEvent2RisMemresifg10::IntEvent2RisMemresifg10Set
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMemresifg11 {
    #[doc = "0: CLR"]
    IntEvent2RisMemresifg11Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMemresifg11Set = 1,
}
impl From<IntEvent2RisMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2RisMemresifg11R = crate::BitReader<IntEvent2RisMemresifg11>;
impl IntEvent2RisMemresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMemresifg11 {
        match self.bits {
            false => IntEvent2RisMemresifg11::IntEvent2RisMemresifg11Clr,
            true => IntEvent2RisMemresifg11::IntEvent2RisMemresifg11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg11_clr(&self) -> bool {
        *self == IntEvent2RisMemresifg11::IntEvent2RisMemresifg11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_memresifg11_set(&self) -> bool {
        *self == IntEvent2RisMemresifg11::IntEvent2RisMemresifg11Set
    }
}
impl R {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg0(&self) -> IntEvent2RisMemresifg0R {
        IntEvent2RisMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg1(&self) -> IntEvent2RisMemresifg1R {
        IntEvent2RisMemresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg2(&self) -> IntEvent2RisMemresifg2R {
        IntEvent2RisMemresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg3(&self) -> IntEvent2RisMemresifg3R {
        IntEvent2RisMemresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg4(&self) -> IntEvent2RisMemresifg4R {
        IntEvent2RisMemresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg5(&self) -> IntEvent2RisMemresifg5R {
        IntEvent2RisMemresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg6(&self) -> IntEvent2RisMemresifg6R {
        IntEvent2RisMemresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg7(&self) -> IntEvent2RisMemresifg7R {
        IntEvent2RisMemresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg8(&self) -> IntEvent2RisMemresifg8R {
        IntEvent2RisMemresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg9(&self) -> IntEvent2RisMemresifg9R {
        IntEvent2RisMemresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg10(&self) -> IntEvent2RisMemresifg10R {
        IntEvent2RisMemresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_ris_memresifg11(&self) -> IntEvent2RisMemresifg11R {
        IntEvent2RisMemresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Raw interrupt status extension\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2RisSpec;
impl crate::RegisterSpec for IntEvent2RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent2RisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_RIS to value 0"]
impl crate::Resettable for IntEvent2RisSpec {
    const RESET_VALUE: u32 = 0;
}

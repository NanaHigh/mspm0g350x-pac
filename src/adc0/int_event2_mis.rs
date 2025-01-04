#[doc = "Register `INT_EVENT2_MIS` reader"]
pub type R = crate::R<IntEvent2MisSpec>;
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg0Set = 1,
}
impl From<IntEvent2MisMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg0R = crate::BitReader<IntEvent2MisMemresifg0>;
impl IntEvent2MisMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg0 {
        match self.bits {
            false => IntEvent2MisMemresifg0::IntEvent2MisMemresifg0Clr,
            true => IntEvent2MisMemresifg0::IntEvent2MisMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg0_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg0::IntEvent2MisMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg0_set(&self) -> bool {
        *self == IntEvent2MisMemresifg0::IntEvent2MisMemresifg0Set
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg1 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg1Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg1Set = 1,
}
impl From<IntEvent2MisMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg1R = crate::BitReader<IntEvent2MisMemresifg1>;
impl IntEvent2MisMemresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg1 {
        match self.bits {
            false => IntEvent2MisMemresifg1::IntEvent2MisMemresifg1Clr,
            true => IntEvent2MisMemresifg1::IntEvent2MisMemresifg1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg1_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg1::IntEvent2MisMemresifg1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg1_set(&self) -> bool {
        *self == IntEvent2MisMemresifg1::IntEvent2MisMemresifg1Set
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg2 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg2Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg2Set = 1,
}
impl From<IntEvent2MisMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg2R = crate::BitReader<IntEvent2MisMemresifg2>;
impl IntEvent2MisMemresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg2 {
        match self.bits {
            false => IntEvent2MisMemresifg2::IntEvent2MisMemresifg2Clr,
            true => IntEvent2MisMemresifg2::IntEvent2MisMemresifg2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg2_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg2::IntEvent2MisMemresifg2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg2_set(&self) -> bool {
        *self == IntEvent2MisMemresifg2::IntEvent2MisMemresifg2Set
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg3 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg3Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg3Set = 1,
}
impl From<IntEvent2MisMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg3R = crate::BitReader<IntEvent2MisMemresifg3>;
impl IntEvent2MisMemresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg3 {
        match self.bits {
            false => IntEvent2MisMemresifg3::IntEvent2MisMemresifg3Clr,
            true => IntEvent2MisMemresifg3::IntEvent2MisMemresifg3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg3_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg3::IntEvent2MisMemresifg3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg3_set(&self) -> bool {
        *self == IntEvent2MisMemresifg3::IntEvent2MisMemresifg3Set
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg4 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg4Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg4Set = 1,
}
impl From<IntEvent2MisMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg4R = crate::BitReader<IntEvent2MisMemresifg4>;
impl IntEvent2MisMemresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg4 {
        match self.bits {
            false => IntEvent2MisMemresifg4::IntEvent2MisMemresifg4Clr,
            true => IntEvent2MisMemresifg4::IntEvent2MisMemresifg4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg4_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg4::IntEvent2MisMemresifg4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg4_set(&self) -> bool {
        *self == IntEvent2MisMemresifg4::IntEvent2MisMemresifg4Set
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg5 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg5Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg5Set = 1,
}
impl From<IntEvent2MisMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg5R = crate::BitReader<IntEvent2MisMemresifg5>;
impl IntEvent2MisMemresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg5 {
        match self.bits {
            false => IntEvent2MisMemresifg5::IntEvent2MisMemresifg5Clr,
            true => IntEvent2MisMemresifg5::IntEvent2MisMemresifg5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg5_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg5::IntEvent2MisMemresifg5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg5_set(&self) -> bool {
        *self == IntEvent2MisMemresifg5::IntEvent2MisMemresifg5Set
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg6 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg6Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg6Set = 1,
}
impl From<IntEvent2MisMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg6R = crate::BitReader<IntEvent2MisMemresifg6>;
impl IntEvent2MisMemresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg6 {
        match self.bits {
            false => IntEvent2MisMemresifg6::IntEvent2MisMemresifg6Clr,
            true => IntEvent2MisMemresifg6::IntEvent2MisMemresifg6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg6_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg6::IntEvent2MisMemresifg6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg6_set(&self) -> bool {
        *self == IntEvent2MisMemresifg6::IntEvent2MisMemresifg6Set
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg7 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg7Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg7Set = 1,
}
impl From<IntEvent2MisMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg7R = crate::BitReader<IntEvent2MisMemresifg7>;
impl IntEvent2MisMemresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg7 {
        match self.bits {
            false => IntEvent2MisMemresifg7::IntEvent2MisMemresifg7Clr,
            true => IntEvent2MisMemresifg7::IntEvent2MisMemresifg7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg7_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg7::IntEvent2MisMemresifg7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg7_set(&self) -> bool {
        *self == IntEvent2MisMemresifg7::IntEvent2MisMemresifg7Set
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg8 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg8Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg8Set = 1,
}
impl From<IntEvent2MisMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg8R = crate::BitReader<IntEvent2MisMemresifg8>;
impl IntEvent2MisMemresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg8 {
        match self.bits {
            false => IntEvent2MisMemresifg8::IntEvent2MisMemresifg8Clr,
            true => IntEvent2MisMemresifg8::IntEvent2MisMemresifg8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg8_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg8::IntEvent2MisMemresifg8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg8_set(&self) -> bool {
        *self == IntEvent2MisMemresifg8::IntEvent2MisMemresifg8Set
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg9 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg9Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg9Set = 1,
}
impl From<IntEvent2MisMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg9R = crate::BitReader<IntEvent2MisMemresifg9>;
impl IntEvent2MisMemresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg9 {
        match self.bits {
            false => IntEvent2MisMemresifg9::IntEvent2MisMemresifg9Clr,
            true => IntEvent2MisMemresifg9::IntEvent2MisMemresifg9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg9_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg9::IntEvent2MisMemresifg9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg9_set(&self) -> bool {
        *self == IntEvent2MisMemresifg9::IntEvent2MisMemresifg9Set
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg10 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg10Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg10Set = 1,
}
impl From<IntEvent2MisMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg10R = crate::BitReader<IntEvent2MisMemresifg10>;
impl IntEvent2MisMemresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg10 {
        match self.bits {
            false => IntEvent2MisMemresifg10::IntEvent2MisMemresifg10Clr,
            true => IntEvent2MisMemresifg10::IntEvent2MisMemresifg10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg10_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg10::IntEvent2MisMemresifg10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg10_set(&self) -> bool {
        *self == IntEvent2MisMemresifg10::IntEvent2MisMemresifg10Set
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMemresifg11 {
    #[doc = "0: CLR"]
    IntEvent2MisMemresifg11Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMemresifg11Set = 1,
}
impl From<IntEvent2MisMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2MisMemresifg11R = crate::BitReader<IntEvent2MisMemresifg11>;
impl IntEvent2MisMemresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMemresifg11 {
        match self.bits {
            false => IntEvent2MisMemresifg11::IntEvent2MisMemresifg11Clr,
            true => IntEvent2MisMemresifg11::IntEvent2MisMemresifg11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg11_clr(&self) -> bool {
        *self == IntEvent2MisMemresifg11::IntEvent2MisMemresifg11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_memresifg11_set(&self) -> bool {
        *self == IntEvent2MisMemresifg11::IntEvent2MisMemresifg11Set
    }
}
impl R {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg0(&self) -> IntEvent2MisMemresifg0R {
        IntEvent2MisMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg1(&self) -> IntEvent2MisMemresifg1R {
        IntEvent2MisMemresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg2(&self) -> IntEvent2MisMemresifg2R {
        IntEvent2MisMemresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg3(&self) -> IntEvent2MisMemresifg3R {
        IntEvent2MisMemresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg4(&self) -> IntEvent2MisMemresifg4R {
        IntEvent2MisMemresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg5(&self) -> IntEvent2MisMemresifg5R {
        IntEvent2MisMemresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg6(&self) -> IntEvent2MisMemresifg6R {
        IntEvent2MisMemresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg7(&self) -> IntEvent2MisMemresifg7R {
        IntEvent2MisMemresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg8(&self) -> IntEvent2MisMemresifg8R {
        IntEvent2MisMemresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg9(&self) -> IntEvent2MisMemresifg9R {
        IntEvent2MisMemresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg10(&self) -> IntEvent2MisMemresifg10R {
        IntEvent2MisMemresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_mis_memresifg11(&self) -> IntEvent2MisMemresifg11R {
        IntEvent2MisMemresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Masked interrupt status extension\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2MisSpec;
impl crate::RegisterSpec for IntEvent2MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent2MisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_MIS to value 0"]
impl crate::Resettable for IntEvent2MisSpec {
    const RESET_VALUE: u32 = 0;
}

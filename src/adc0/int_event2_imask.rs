#[doc = "Register `INT_EVENT2_IMASK` reader"]
pub type R = crate::R<IntEvent2ImaskSpec>;
#[doc = "Register `INT_EVENT2_IMASK` writer"]
pub type W = crate::W<IntEvent2ImaskSpec>;
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg0Set = 1,
}
impl From<IntEvent2ImaskMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg0R = crate::BitReader<IntEvent2ImaskMemresifg0>;
impl IntEvent2ImaskMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg0 {
        match self.bits {
            false => IntEvent2ImaskMemresifg0::IntEvent2ImaskMemresifg0Clr,
            true => IntEvent2ImaskMemresifg0::IntEvent2ImaskMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg0_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg0::IntEvent2ImaskMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg0_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg0::IntEvent2ImaskMemresifg0Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg0>;
impl<'a, REG> IntEvent2ImaskMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg0::IntEvent2ImaskMemresifg0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg0::IntEvent2ImaskMemresifg0Set)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg1 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg1Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg1Set = 1,
}
impl From<IntEvent2ImaskMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg1R = crate::BitReader<IntEvent2ImaskMemresifg1>;
impl IntEvent2ImaskMemresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg1 {
        match self.bits {
            false => IntEvent2ImaskMemresifg1::IntEvent2ImaskMemresifg1Clr,
            true => IntEvent2ImaskMemresifg1::IntEvent2ImaskMemresifg1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg1_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg1::IntEvent2ImaskMemresifg1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg1_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg1::IntEvent2ImaskMemresifg1Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg1>;
impl<'a, REG> IntEvent2ImaskMemresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg1::IntEvent2ImaskMemresifg1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg1::IntEvent2ImaskMemresifg1Set)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg2 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg2Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg2Set = 1,
}
impl From<IntEvent2ImaskMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg2R = crate::BitReader<IntEvent2ImaskMemresifg2>;
impl IntEvent2ImaskMemresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg2 {
        match self.bits {
            false => IntEvent2ImaskMemresifg2::IntEvent2ImaskMemresifg2Clr,
            true => IntEvent2ImaskMemresifg2::IntEvent2ImaskMemresifg2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg2_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg2::IntEvent2ImaskMemresifg2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg2_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg2::IntEvent2ImaskMemresifg2Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg2>;
impl<'a, REG> IntEvent2ImaskMemresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg2::IntEvent2ImaskMemresifg2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg2::IntEvent2ImaskMemresifg2Set)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg3 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg3Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg3Set = 1,
}
impl From<IntEvent2ImaskMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg3R = crate::BitReader<IntEvent2ImaskMemresifg3>;
impl IntEvent2ImaskMemresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg3 {
        match self.bits {
            false => IntEvent2ImaskMemresifg3::IntEvent2ImaskMemresifg3Clr,
            true => IntEvent2ImaskMemresifg3::IntEvent2ImaskMemresifg3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg3_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg3::IntEvent2ImaskMemresifg3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg3_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg3::IntEvent2ImaskMemresifg3Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg3>;
impl<'a, REG> IntEvent2ImaskMemresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg3::IntEvent2ImaskMemresifg3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg3::IntEvent2ImaskMemresifg3Set)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg4 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg4Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg4Set = 1,
}
impl From<IntEvent2ImaskMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg4R = crate::BitReader<IntEvent2ImaskMemresifg4>;
impl IntEvent2ImaskMemresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg4 {
        match self.bits {
            false => IntEvent2ImaskMemresifg4::IntEvent2ImaskMemresifg4Clr,
            true => IntEvent2ImaskMemresifg4::IntEvent2ImaskMemresifg4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg4_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg4::IntEvent2ImaskMemresifg4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg4_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg4::IntEvent2ImaskMemresifg4Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg4>;
impl<'a, REG> IntEvent2ImaskMemresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg4::IntEvent2ImaskMemresifg4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg4::IntEvent2ImaskMemresifg4Set)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg5 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg5Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg5Set = 1,
}
impl From<IntEvent2ImaskMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg5R = crate::BitReader<IntEvent2ImaskMemresifg5>;
impl IntEvent2ImaskMemresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg5 {
        match self.bits {
            false => IntEvent2ImaskMemresifg5::IntEvent2ImaskMemresifg5Clr,
            true => IntEvent2ImaskMemresifg5::IntEvent2ImaskMemresifg5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg5_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg5::IntEvent2ImaskMemresifg5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg5_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg5::IntEvent2ImaskMemresifg5Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg5>;
impl<'a, REG> IntEvent2ImaskMemresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg5::IntEvent2ImaskMemresifg5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg5::IntEvent2ImaskMemresifg5Set)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg6 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg6Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg6Set = 1,
}
impl From<IntEvent2ImaskMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg6R = crate::BitReader<IntEvent2ImaskMemresifg6>;
impl IntEvent2ImaskMemresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg6 {
        match self.bits {
            false => IntEvent2ImaskMemresifg6::IntEvent2ImaskMemresifg6Clr,
            true => IntEvent2ImaskMemresifg6::IntEvent2ImaskMemresifg6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg6_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg6::IntEvent2ImaskMemresifg6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg6_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg6::IntEvent2ImaskMemresifg6Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg6>;
impl<'a, REG> IntEvent2ImaskMemresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg6::IntEvent2ImaskMemresifg6Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg6::IntEvent2ImaskMemresifg6Set)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg7 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg7Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg7Set = 1,
}
impl From<IntEvent2ImaskMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg7R = crate::BitReader<IntEvent2ImaskMemresifg7>;
impl IntEvent2ImaskMemresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg7 {
        match self.bits {
            false => IntEvent2ImaskMemresifg7::IntEvent2ImaskMemresifg7Clr,
            true => IntEvent2ImaskMemresifg7::IntEvent2ImaskMemresifg7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg7_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg7::IntEvent2ImaskMemresifg7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg7_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg7::IntEvent2ImaskMemresifg7Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg7>;
impl<'a, REG> IntEvent2ImaskMemresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg7::IntEvent2ImaskMemresifg7Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg7::IntEvent2ImaskMemresifg7Set)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg8 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg8Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg8Set = 1,
}
impl From<IntEvent2ImaskMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg8R = crate::BitReader<IntEvent2ImaskMemresifg8>;
impl IntEvent2ImaskMemresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg8 {
        match self.bits {
            false => IntEvent2ImaskMemresifg8::IntEvent2ImaskMemresifg8Clr,
            true => IntEvent2ImaskMemresifg8::IntEvent2ImaskMemresifg8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg8_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg8::IntEvent2ImaskMemresifg8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg8_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg8::IntEvent2ImaskMemresifg8Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg8>;
impl<'a, REG> IntEvent2ImaskMemresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg8::IntEvent2ImaskMemresifg8Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg8::IntEvent2ImaskMemresifg8Set)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg9 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg9Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg9Set = 1,
}
impl From<IntEvent2ImaskMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg9R = crate::BitReader<IntEvent2ImaskMemresifg9>;
impl IntEvent2ImaskMemresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg9 {
        match self.bits {
            false => IntEvent2ImaskMemresifg9::IntEvent2ImaskMemresifg9Clr,
            true => IntEvent2ImaskMemresifg9::IntEvent2ImaskMemresifg9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg9_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg9::IntEvent2ImaskMemresifg9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg9_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg9::IntEvent2ImaskMemresifg9Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg9>;
impl<'a, REG> IntEvent2ImaskMemresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg9::IntEvent2ImaskMemresifg9Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg9::IntEvent2ImaskMemresifg9Set)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg10 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg10Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg10Set = 1,
}
impl From<IntEvent2ImaskMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg10R = crate::BitReader<IntEvent2ImaskMemresifg10>;
impl IntEvent2ImaskMemresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg10 {
        match self.bits {
            false => IntEvent2ImaskMemresifg10::IntEvent2ImaskMemresifg10Clr,
            true => IntEvent2ImaskMemresifg10::IntEvent2ImaskMemresifg10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg10_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg10::IntEvent2ImaskMemresifg10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg10_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg10::IntEvent2ImaskMemresifg10Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg10>;
impl<'a, REG> IntEvent2ImaskMemresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg10::IntEvent2ImaskMemresifg10Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg10::IntEvent2ImaskMemresifg10Set)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskMemresifg11 {
    #[doc = "0: CLR"]
    IntEvent2ImaskMemresifg11Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskMemresifg11Set = 1,
}
impl From<IntEvent2ImaskMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg11R = crate::BitReader<IntEvent2ImaskMemresifg11>;
impl IntEvent2ImaskMemresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskMemresifg11 {
        match self.bits {
            false => IntEvent2ImaskMemresifg11::IntEvent2ImaskMemresifg11Clr,
            true => IntEvent2ImaskMemresifg11::IntEvent2ImaskMemresifg11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg11_clr(&self) -> bool {
        *self == IntEvent2ImaskMemresifg11::IntEvent2ImaskMemresifg11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_memresifg11_set(&self) -> bool {
        *self == IntEvent2ImaskMemresifg11::IntEvent2ImaskMemresifg11Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2ImaskMemresifg11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskMemresifg11>;
impl<'a, REG> IntEvent2ImaskMemresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg11::IntEvent2ImaskMemresifg11Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskMemresifg11::IntEvent2ImaskMemresifg11Set)
    }
}
impl R {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg0(&self) -> IntEvent2ImaskMemresifg0R {
        IntEvent2ImaskMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg1(&self) -> IntEvent2ImaskMemresifg1R {
        IntEvent2ImaskMemresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg2(&self) -> IntEvent2ImaskMemresifg2R {
        IntEvent2ImaskMemresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg3(&self) -> IntEvent2ImaskMemresifg3R {
        IntEvent2ImaskMemresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg4(&self) -> IntEvent2ImaskMemresifg4R {
        IntEvent2ImaskMemresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg5(&self) -> IntEvent2ImaskMemresifg5R {
        IntEvent2ImaskMemresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg6(&self) -> IntEvent2ImaskMemresifg6R {
        IntEvent2ImaskMemresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg7(&self) -> IntEvent2ImaskMemresifg7R {
        IntEvent2ImaskMemresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg8(&self) -> IntEvent2ImaskMemresifg8R {
        IntEvent2ImaskMemresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg9(&self) -> IntEvent2ImaskMemresifg9R {
        IntEvent2ImaskMemresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg10(&self) -> IntEvent2ImaskMemresifg10R {
        IntEvent2ImaskMemresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg11(&self) -> IntEvent2ImaskMemresifg11R {
        IntEvent2ImaskMemresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg0(&mut self) -> IntEvent2ImaskMemresifg0W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg1(&mut self) -> IntEvent2ImaskMemresifg1W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg2(&mut self) -> IntEvent2ImaskMemresifg2W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg3(&mut self) -> IntEvent2ImaskMemresifg3W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg4(&mut self) -> IntEvent2ImaskMemresifg4W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg5(&mut self) -> IntEvent2ImaskMemresifg5W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg6(&mut self) -> IntEvent2ImaskMemresifg6W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg7(&mut self) -> IntEvent2ImaskMemresifg7W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg8(&mut self) -> IntEvent2ImaskMemresifg8W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg9(&mut self) -> IntEvent2ImaskMemresifg9W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg10(
        &mut self,
    ) -> IntEvent2ImaskMemresifg10W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_imask_memresifg11(
        &mut self,
    ) -> IntEvent2ImaskMemresifg11W<IntEvent2ImaskSpec> {
        IntEvent2ImaskMemresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt mask extension\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2ImaskSpec;
impl crate::RegisterSpec for IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for IntEvent2ImaskSpec {
    const RESET_VALUE: u32 = 0;
}

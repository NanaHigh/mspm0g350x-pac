#[doc = "Register `INT_EVENT2_ICLR` writer"]
pub type W = crate::W<IntEvent2IclrSpec>;
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg0Clr = 1,
}
impl From<IntEvent2IclrMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg0>;
impl<'a, REG> IntEvent2IclrMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg0::IntEvent2IclrMemresifg0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg0::IntEvent2IclrMemresifg0Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg1NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg1Clr = 1,
}
impl From<IntEvent2IclrMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg1>;
impl<'a, REG> IntEvent2IclrMemresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg1::IntEvent2IclrMemresifg1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg1::IntEvent2IclrMemresifg1Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg2Clr = 1,
}
impl From<IntEvent2IclrMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg2>;
impl<'a, REG> IntEvent2IclrMemresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg2::IntEvent2IclrMemresifg2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg2::IntEvent2IclrMemresifg2Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg3NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg3Clr = 1,
}
impl From<IntEvent2IclrMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg3>;
impl<'a, REG> IntEvent2IclrMemresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg3::IntEvent2IclrMemresifg3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg3::IntEvent2IclrMemresifg3Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg4NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg4Clr = 1,
}
impl From<IntEvent2IclrMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg4>;
impl<'a, REG> IntEvent2IclrMemresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg4::IntEvent2IclrMemresifg4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg4::IntEvent2IclrMemresifg4Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg5NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg5Clr = 1,
}
impl From<IntEvent2IclrMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg5>;
impl<'a, REG> IntEvent2IclrMemresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg5::IntEvent2IclrMemresifg5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg5::IntEvent2IclrMemresifg5Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg6NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg6Clr = 1,
}
impl From<IntEvent2IclrMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg6>;
impl<'a, REG> IntEvent2IclrMemresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg6::IntEvent2IclrMemresifg6NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg6::IntEvent2IclrMemresifg6Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg7NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg7Clr = 1,
}
impl From<IntEvent2IclrMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg7>;
impl<'a, REG> IntEvent2IclrMemresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg7::IntEvent2IclrMemresifg7NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg7::IntEvent2IclrMemresifg7Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg8NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg8Clr = 1,
}
impl From<IntEvent2IclrMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg8>;
impl<'a, REG> IntEvent2IclrMemresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg8::IntEvent2IclrMemresifg8NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg8::IntEvent2IclrMemresifg8Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg9NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg9Clr = 1,
}
impl From<IntEvent2IclrMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg9>;
impl<'a, REG> IntEvent2IclrMemresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg9::IntEvent2IclrMemresifg9NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg9::IntEvent2IclrMemresifg9Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg10NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg10Clr = 1,
}
impl From<IntEvent2IclrMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg10>;
impl<'a, REG> IntEvent2IclrMemresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg10::IntEvent2IclrMemresifg10NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg10::IntEvent2IclrMemresifg10Clr)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrMemresifg11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrMemresifg11NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrMemresifg11Clr = 1,
}
impl From<IntEvent2IclrMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IclrMemresifg11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrMemresifg11>;
impl<'a, REG> IntEvent2IclrMemresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg11::IntEvent2IclrMemresifg11NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrMemresifg11::IntEvent2IclrMemresifg11Clr)
    }
}
impl W {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg0(&mut self) -> IntEvent2IclrMemresifg0W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg1(&mut self) -> IntEvent2IclrMemresifg1W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg2(&mut self) -> IntEvent2IclrMemresifg2W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg3(&mut self) -> IntEvent2IclrMemresifg3W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg4(&mut self) -> IntEvent2IclrMemresifg4W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg5(&mut self) -> IntEvent2IclrMemresifg5W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg6(&mut self) -> IntEvent2IclrMemresifg6W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg7(&mut self) -> IntEvent2IclrMemresifg7W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg8(&mut self) -> IntEvent2IclrMemresifg8W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg9(&mut self) -> IntEvent2IclrMemresifg9W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg10(&mut self) -> IntEvent2IclrMemresifg10W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iclr_memresifg11(&mut self) -> IntEvent2IclrMemresifg11W<IntEvent2IclrSpec> {
        IntEvent2IclrMemresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt clear extension\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IclrSpec;
impl crate::RegisterSpec for IntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent2IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for IntEvent2IclrSpec {
    const RESET_VALUE: u32 = 0;
}

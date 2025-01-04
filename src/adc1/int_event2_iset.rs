#[doc = "Register `INT_EVENT2_ISET` writer"]
pub type W = crate::W<IntEvent2IsetSpec>;
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg0Set = 1,
}
impl From<IntEvent2IsetMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg0>;
impl<'a, REG> IntEvent2IsetMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg0::IntEvent2IsetMemresifg0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg0::IntEvent2IsetMemresifg0Set)
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg1NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg1Set = 1,
}
impl From<IntEvent2IsetMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG1` writer - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg1>;
impl<'a, REG> IntEvent2IsetMemresifg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg1::IntEvent2IsetMemresifg1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg1::IntEvent2IsetMemresifg1Set)
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg2Set = 1,
}
impl From<IntEvent2IsetMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG2` writer - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg2>;
impl<'a, REG> IntEvent2IsetMemresifg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg2::IntEvent2IsetMemresifg2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg2::IntEvent2IsetMemresifg2Set)
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg3NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg3Set = 1,
}
impl From<IntEvent2IsetMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG3` writer - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg3>;
impl<'a, REG> IntEvent2IsetMemresifg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg3::IntEvent2IsetMemresifg3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg3::IntEvent2IsetMemresifg3Set)
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg4NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg4Set = 1,
}
impl From<IntEvent2IsetMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG4` writer - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg4>;
impl<'a, REG> IntEvent2IsetMemresifg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg4::IntEvent2IsetMemresifg4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg4::IntEvent2IsetMemresifg4Set)
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg5NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg5Set = 1,
}
impl From<IntEvent2IsetMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG5` writer - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg5>;
impl<'a, REG> IntEvent2IsetMemresifg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg5::IntEvent2IsetMemresifg5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg5::IntEvent2IsetMemresifg5Set)
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg6NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg6Set = 1,
}
impl From<IntEvent2IsetMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG6` writer - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg6>;
impl<'a, REG> IntEvent2IsetMemresifg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg6::IntEvent2IsetMemresifg6NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg6::IntEvent2IsetMemresifg6Set)
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg7NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg7Set = 1,
}
impl From<IntEvent2IsetMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG7` writer - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg7>;
impl<'a, REG> IntEvent2IsetMemresifg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg7::IntEvent2IsetMemresifg7NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg7::IntEvent2IsetMemresifg7Set)
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg8NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg8Set = 1,
}
impl From<IntEvent2IsetMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG8` writer - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg8>;
impl<'a, REG> IntEvent2IsetMemresifg8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg8::IntEvent2IsetMemresifg8NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg8::IntEvent2IsetMemresifg8Set)
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg9NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg9Set = 1,
}
impl From<IntEvent2IsetMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG9` writer - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg9>;
impl<'a, REG> IntEvent2IsetMemresifg9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg9::IntEvent2IsetMemresifg9NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg9::IntEvent2IsetMemresifg9Set)
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg10NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg10Set = 1,
}
impl From<IntEvent2IsetMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG10` writer - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg10>;
impl<'a, REG> IntEvent2IsetMemresifg10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg10::IntEvent2IsetMemresifg10NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg10::IntEvent2IsetMemresifg10Set)
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetMemresifg11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetMemresifg11NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetMemresifg11Set = 1,
}
impl From<IntEvent2IsetMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_MEMRESIFG11` writer - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent2IsetMemresifg11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetMemresifg11>;
impl<'a, REG> IntEvent2IsetMemresifg11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg11::IntEvent2IsetMemresifg11NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetMemresifg11::IntEvent2IsetMemresifg11Set)
    }
}
impl W {
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg0(&mut self) -> IntEvent2IsetMemresifg0W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg0W::new(self, 8)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg1(&mut self) -> IntEvent2IsetMemresifg1W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg1W::new(self, 9)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg2(&mut self) -> IntEvent2IsetMemresifg2W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg2W::new(self, 10)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg3(&mut self) -> IntEvent2IsetMemresifg3W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg3W::new(self, 11)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg4(&mut self) -> IntEvent2IsetMemresifg4W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg4W::new(self, 12)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg5(&mut self) -> IntEvent2IsetMemresifg5W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg5W::new(self, 13)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg6(&mut self) -> IntEvent2IsetMemresifg6W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg6W::new(self, 14)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg7(&mut self) -> IntEvent2IsetMemresifg7W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg7W::new(self, 15)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg8(&mut self) -> IntEvent2IsetMemresifg8W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg8W::new(self, 16)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg9(&mut self) -> IntEvent2IsetMemresifg9W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg9W::new(self, 17)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg10(&mut self) -> IntEvent2IsetMemresifg10W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg10W::new(self, 18)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event2_iset_memresifg11(&mut self) -> IntEvent2IsetMemresifg11W<IntEvent2IsetSpec> {
        IntEvent2IsetMemresifg11W::new(self, 19)
    }
}
#[doc = "Interrupt set extension\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IsetSpec;
impl crate::RegisterSpec for IntEvent2IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent2IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ISET to value 0"]
impl crate::Resettable for IntEvent2IsetSpec {
    const RESET_VALUE: u32 = 0;
}

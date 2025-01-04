#[doc = "Register `INT_EVENT1_ISET` writer"]
pub type W = crate::W<IntEvent1IsetSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetHighifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetHighifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetHighifgSet = 1,
}
impl From<IntEvent1IsetHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1IsetHighifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetHighifg>;
impl<'a, REG> IntEvent1IsetHighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_highifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetHighifg::IntEvent1IsetHighifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_highifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetHighifg::IntEvent1IsetHighifgSet)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetLowifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetLowifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetLowifgSet = 1,
}
impl From<IntEvent1IsetLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1IsetLowifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetLowifg>;
impl<'a, REG> IntEvent1IsetLowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_lowifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetLowifg::IntEvent1IsetLowifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_lowifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetLowifg::IntEvent1IsetLowifgSet)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetInifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetInifgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetInifgSet = 1,
}
impl From<IntEvent1IsetInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_INIFG` writer - Mask INIFG in MIS_EX register."]
pub type IntEvent1IsetInifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetInifg>;
impl<'a, REG> IntEvent1IsetInifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_inifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetInifg::IntEvent1IsetInifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_inifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetInifg::IntEvent1IsetInifgSet)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetMemresifg0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetMemresifg0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetMemresifg0Set = 1,
}
impl From<IntEvent1IsetMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent1IsetMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetMemresifg0>;
impl<'a, REG> IntEvent1IsetMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_memresifg0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetMemresifg0::IntEvent1IsetMemresifg0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_memresifg0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetMemresifg0::IntEvent1IsetMemresifg0Set)
    }
}
impl W {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_iset_highifg(&mut self) -> IntEvent1IsetHighifgW<IntEvent1IsetSpec> {
        IntEvent1IsetHighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_iset_lowifg(&mut self) -> IntEvent1IsetLowifgW<IntEvent1IsetSpec> {
        IntEvent1IsetLowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event1_iset_inifg(&mut self) -> IntEvent1IsetInifgW<IntEvent1IsetSpec> {
        IntEvent1IsetInifgW::new(self, 4)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event1_iset_memresifg0(&mut self) -> IntEvent1IsetMemresifg0W<IntEvent1IsetSpec> {
        IntEvent1IsetMemresifg0W::new(self, 8)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IsetSpec;
impl crate::RegisterSpec for IntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent1IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ISET to value 0"]
impl crate::Resettable for IntEvent1IsetSpec {
    const RESET_VALUE: u32 = 0;
}

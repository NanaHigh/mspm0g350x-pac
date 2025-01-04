#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrHighifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrHighifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrHighifgClr = 1,
}
impl From<IntEvent1IclrHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_HIGHIFG` writer - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1IclrHighifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrHighifg>;
impl<'a, REG> IntEvent1IclrHighifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_highifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrHighifg::IntEvent1IclrHighifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_highifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrHighifg::IntEvent1IclrHighifgClr)
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrLowifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrLowifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrLowifgClr = 1,
}
impl From<IntEvent1IclrLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_LOWIFG` writer - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1IclrLowifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrLowifg>;
impl<'a, REG> IntEvent1IclrLowifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_lowifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrLowifg::IntEvent1IclrLowifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_lowifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrLowifg::IntEvent1IclrLowifgClr)
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrInifg {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrInifgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrInifgClr = 1,
}
impl From<IntEvent1IclrInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_INIFG` writer - Mask INIFG in MIS_EX register."]
pub type IntEvent1IclrInifgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrInifg>;
impl<'a, REG> IntEvent1IclrInifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_inifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrInifg::IntEvent1IclrInifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_inifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrInifg::IntEvent1IclrInifgClr)
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrMemresifg0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrMemresifg0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrMemresifg0Clr = 1,
}
impl From<IntEvent1IclrMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_MEMRESIFG0` writer - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent1IclrMemresifg0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrMemresifg0>;
impl<'a, REG> IntEvent1IclrMemresifg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_memresifg0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrMemresifg0::IntEvent1IclrMemresifg0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_memresifg0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrMemresifg0::IntEvent1IclrMemresifg0Clr)
    }
}
impl W {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_iclr_highifg(&mut self) -> IntEvent1IclrHighifgW<IntEvent1IclrSpec> {
        IntEvent1IclrHighifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_iclr_lowifg(&mut self) -> IntEvent1IclrLowifgW<IntEvent1IclrSpec> {
        IntEvent1IclrLowifgW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event1_iclr_inifg(&mut self) -> IntEvent1IclrInifgW<IntEvent1IclrSpec> {
        IntEvent1IclrInifgW::new(self, 4)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event1_iclr_memresifg0(&mut self) -> IntEvent1IclrMemresifg0W<IntEvent1IclrSpec> {
        IntEvent1IclrMemresifg0W::new(self, 8)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IclrSpec;
impl crate::RegisterSpec for IntEvent1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent1IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ICLR to value 0"]
impl crate::Resettable for IntEvent1IclrSpec {
    const RESET_VALUE: u32 = 0;
}

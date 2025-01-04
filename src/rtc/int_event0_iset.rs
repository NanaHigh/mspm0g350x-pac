#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "Set RTC-Ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRtcrdy {
    #[doc = "0: CLR"]
    IntEvent0IsetRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRtcrdySet = 1,
}
impl From<IntEvent0IsetRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RTCRDY` writer - Set RTC-Ready interrupt"]
pub type IntEvent0IsetRtcrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRtcrdy>;
impl<'a, REG> IntEvent0IsetRtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iset_rtcrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtcrdy::IntEvent0IsetRtcrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rtcrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtcrdy::IntEvent0IsetRtcrdySet)
    }
}
#[doc = "Set Time-Event interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRtctev {
    #[doc = "0: CLR"]
    IntEvent0IsetRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRtctevSet = 1,
}
impl From<IntEvent0IsetRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RTCTEV` writer - Set Time-Event interrupt"]
pub type IntEvent0IsetRtctevW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRtctev>;
impl<'a, REG> IntEvent0IsetRtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iset_rtctev_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtctev::IntEvent0IsetRtctevClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rtctev_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtctev::IntEvent0IsetRtctevSet)
    }
}
#[doc = "Set Alarm-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRtca1 {
    #[doc = "0: CLR"]
    IntEvent0IsetRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRtca1Set = 1,
}
impl From<IntEvent0IsetRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RTCA1` writer - Set Alarm-1 interrupt"]
pub type IntEvent0IsetRtca1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRtca1>;
impl<'a, REG> IntEvent0IsetRtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iset_rtca1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtca1::IntEvent0IsetRtca1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rtca1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtca1::IntEvent0IsetRtca1Set)
    }
}
#[doc = "Set Alarm-2 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRtca2 {
    #[doc = "0: CLR"]
    IntEvent0IsetRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRtca2Set = 1,
}
impl From<IntEvent0IsetRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RTCA2` writer - Set Alarm-2 interrupt"]
pub type IntEvent0IsetRtca2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRtca2>;
impl<'a, REG> IntEvent0IsetRtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iset_rtca2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtca2::IntEvent0IsetRtca2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rtca2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtca2::IntEvent0IsetRtca2Set)
    }
}
#[doc = "Set Prescaler-0 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRt0ps {
    #[doc = "0: CLR"]
    IntEvent0IsetRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRt0psSet = 1,
}
impl From<IntEvent0IsetRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RT0PS` writer - Set Prescaler-0 interrupt"]
pub type IntEvent0IsetRt0psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRt0ps>;
impl<'a, REG> IntEvent0IsetRt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iset_rt0ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRt0ps::IntEvent0IsetRt0psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rt0ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRt0ps::IntEvent0IsetRt0psSet)
    }
}
#[doc = "Set Prescaler-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRt1ps {
    #[doc = "0: CLR"]
    IntEvent0IsetRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRt1psSet = 1,
}
impl From<IntEvent0IsetRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RT1PS` writer - Set Prescaler-1 interrupt"]
pub type IntEvent0IsetRt1psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRt1ps>;
impl<'a, REG> IntEvent0IsetRt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iset_rt1ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRt1ps::IntEvent0IsetRt1psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rt1ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRt1ps::IntEvent0IsetRt1psSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_rtcrdy(&mut self) -> IntEvent0IsetRtcrdyW<IntEvent0IsetSpec> {
        IntEvent0IsetRtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Set Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_rtctev(&mut self) -> IntEvent0IsetRtctevW<IntEvent0IsetSpec> {
        IntEvent0IsetRtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - Set Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_rtca1(&mut self) -> IntEvent0IsetRtca1W<IntEvent0IsetSpec> {
        IntEvent0IsetRtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - Set Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_rtca2(&mut self) -> IntEvent0IsetRtca2W<IntEvent0IsetSpec> {
        IntEvent0IsetRtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - Set Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_rt0ps(&mut self) -> IntEvent0IsetRt0psW<IntEvent0IsetSpec> {
        IntEvent0IsetRt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_rt1ps(&mut self) -> IntEvent0IsetRt1psW<IntEvent0IsetSpec> {
        IntEvent0IsetRt1psW::new(self, 5)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IsetSpec;
impl crate::RegisterSpec for IntEvent0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent0IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ISET to value 0"]
impl crate::Resettable for IntEvent0IsetSpec {
    const RESET_VALUE: u32 = 0;
}

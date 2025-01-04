#[doc = "Register `INT_EVENT1_ISET` writer"]
pub type W = crate::W<IntEvent1IsetSpec>;
#[doc = "Set RTC-Ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRtcrdy {
    #[doc = "0: CLR"]
    IntEvent1IsetRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRtcrdySet = 1,
}
impl From<IntEvent1IsetRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RTCRDY` writer - Set RTC-Ready interrupt"]
pub type IntEvent1IsetRtcrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRtcrdy>;
impl<'a, REG> IntEvent1IsetRtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iset_rtcrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtcrdy::IntEvent1IsetRtcrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rtcrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtcrdy::IntEvent1IsetRtcrdySet)
    }
}
#[doc = "Set Time-Event interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRtctev {
    #[doc = "0: CLR"]
    IntEvent1IsetRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRtctevSet = 1,
}
impl From<IntEvent1IsetRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RTCTEV` writer - Set Time-Event interrupt"]
pub type IntEvent1IsetRtctevW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRtctev>;
impl<'a, REG> IntEvent1IsetRtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iset_rtctev_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtctev::IntEvent1IsetRtctevClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rtctev_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtctev::IntEvent1IsetRtctevSet)
    }
}
#[doc = "Set Alarm-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRtca1 {
    #[doc = "0: CLR"]
    IntEvent1IsetRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRtca1Set = 1,
}
impl From<IntEvent1IsetRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RTCA1` writer - Set Alarm-1 interrupt"]
pub type IntEvent1IsetRtca1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRtca1>;
impl<'a, REG> IntEvent1IsetRtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iset_rtca1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtca1::IntEvent1IsetRtca1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rtca1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtca1::IntEvent1IsetRtca1Set)
    }
}
#[doc = "Set Alarm-2 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRtca2 {
    #[doc = "0: CLR"]
    IntEvent1IsetRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRtca2Set = 1,
}
impl From<IntEvent1IsetRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RTCA2` writer - Set Alarm-2 interrupt"]
pub type IntEvent1IsetRtca2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRtca2>;
impl<'a, REG> IntEvent1IsetRtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iset_rtca2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtca2::IntEvent1IsetRtca2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rtca2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRtca2::IntEvent1IsetRtca2Set)
    }
}
#[doc = "Set Prescaler-0 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRt0ps {
    #[doc = "0: CLR"]
    IntEvent1IsetRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRt0psSet = 1,
}
impl From<IntEvent1IsetRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RT0PS` writer - Set Prescaler-0 interrupt"]
pub type IntEvent1IsetRt0psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRt0ps>;
impl<'a, REG> IntEvent1IsetRt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iset_rt0ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRt0ps::IntEvent1IsetRt0psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rt0ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRt0ps::IntEvent1IsetRt0psSet)
    }
}
#[doc = "Set Prescaler-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetRt1ps {
    #[doc = "0: CLR"]
    IntEvent1IsetRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent1IsetRt1psSet = 1,
}
impl From<IntEvent1IsetRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_RT1PS` writer - Set Prescaler-1 interrupt"]
pub type IntEvent1IsetRt1psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetRt1ps>;
impl<'a, REG> IntEvent1IsetRt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iset_rt1ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRt1ps::IntEvent1IsetRt1psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_rt1ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetRt1ps::IntEvent1IsetRt1psSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event1_iset_rtcrdy(&mut self) -> IntEvent1IsetRtcrdyW<IntEvent1IsetSpec> {
        IntEvent1IsetRtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Set Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event1_iset_rtctev(&mut self) -> IntEvent1IsetRtctevW<IntEvent1IsetSpec> {
        IntEvent1IsetRtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - Set Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_iset_rtca1(&mut self) -> IntEvent1IsetRtca1W<IntEvent1IsetSpec> {
        IntEvent1IsetRtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - Set Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event1_iset_rtca2(&mut self) -> IntEvent1IsetRtca2W<IntEvent1IsetSpec> {
        IntEvent1IsetRtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - Set Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event1_iset_rt0ps(&mut self) -> IntEvent1IsetRt0psW<IntEvent1IsetSpec> {
        IntEvent1IsetRt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_iset_rt1ps(&mut self) -> IntEvent1IsetRt1psW<IntEvent1IsetSpec> {
        IntEvent1IsetRt1psW::new(self, 5)
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

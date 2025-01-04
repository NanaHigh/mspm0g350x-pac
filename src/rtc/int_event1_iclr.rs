#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "Clear RTC-Ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRtcrdy {
    #[doc = "0: CLR"]
    IntEvent1IclrRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent1IclrRtcrdySet = 1,
}
impl From<IntEvent1IclrRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RTCRDY` writer - Clear RTC-Ready interrupt"]
pub type IntEvent1IclrRtcrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRtcrdy>;
impl<'a, REG> IntEvent1IclrRtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rtcrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtcrdy::IntEvent1IclrRtcrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iclr_rtcrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtcrdy::IntEvent1IclrRtcrdySet)
    }
}
#[doc = "Clear Time-Event interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRtctev {
    #[doc = "0: CLR"]
    IntEvent1IclrRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent1IclrRtctevSet = 1,
}
impl From<IntEvent1IclrRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RTCTEV` writer - Clear Time-Event interrupt"]
pub type IntEvent1IclrRtctevW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRtctev>;
impl<'a, REG> IntEvent1IclrRtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rtctev_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtctev::IntEvent1IclrRtctevClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iclr_rtctev_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtctev::IntEvent1IclrRtctevSet)
    }
}
#[doc = "Clear Alarm-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRtca1 {
    #[doc = "0: CLR"]
    IntEvent1IclrRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1IclrRtca1Set = 1,
}
impl From<IntEvent1IclrRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RTCA1` writer - Clear Alarm-1 interrupt"]
pub type IntEvent1IclrRtca1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRtca1>;
impl<'a, REG> IntEvent1IclrRtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rtca1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtca1::IntEvent1IclrRtca1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iclr_rtca1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtca1::IntEvent1IclrRtca1Set)
    }
}
#[doc = "Clear Alarm-2 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRtca2 {
    #[doc = "0: CLR"]
    IntEvent1IclrRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1IclrRtca2Set = 1,
}
impl From<IntEvent1IclrRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RTCA2` writer - Clear Alarm-2 interrupt"]
pub type IntEvent1IclrRtca2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRtca2>;
impl<'a, REG> IntEvent1IclrRtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rtca2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtca2::IntEvent1IclrRtca2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iclr_rtca2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRtca2::IntEvent1IclrRtca2Set)
    }
}
#[doc = "Clear Prescaler-0 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRt0ps {
    #[doc = "0: CLR"]
    IntEvent1IclrRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent1IclrRt0psSet = 1,
}
impl From<IntEvent1IclrRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RT0PS` writer - Clear Prescaler-0 interrupt"]
pub type IntEvent1IclrRt0psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRt0ps>;
impl<'a, REG> IntEvent1IclrRt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rt0ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRt0ps::IntEvent1IclrRt0psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iclr_rt0ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRt0ps::IntEvent1IclrRt0psSet)
    }
}
#[doc = "Clear Prescaler-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrRt1ps {
    #[doc = "0: CLR"]
    IntEvent1IclrRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent1IclrRt1psSet = 1,
}
impl From<IntEvent1IclrRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_RT1PS` writer - Clear Prescaler-1 interrupt"]
pub type IntEvent1IclrRt1psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrRt1ps>;
impl<'a, REG> IntEvent1IclrRt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_rt1ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRt1ps::IntEvent1IclrRt1psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iclr_rt1ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrRt1ps::IntEvent1IclrRt1psSet)
    }
}
impl W {
    #[doc = "Bit 0 - Clear RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event1_iclr_rtcrdy(&mut self) -> IntEvent1IclrRtcrdyW<IntEvent1IclrSpec> {
        IntEvent1IclrRtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event1_iclr_rtctev(&mut self) -> IntEvent1IclrRtctevW<IntEvent1IclrSpec> {
        IntEvent1IclrRtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_iclr_rtca1(&mut self) -> IntEvent1IclrRtca1W<IntEvent1IclrSpec> {
        IntEvent1IclrRtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event1_iclr_rtca2(&mut self) -> IntEvent1IclrRtca2W<IntEvent1IclrSpec> {
        IntEvent1IclrRtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event1_iclr_rt0ps(&mut self) -> IntEvent1IclrRt0psW<IntEvent1IclrSpec> {
        IntEvent1IclrRt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_iclr_rt1ps(&mut self) -> IntEvent1IclrRt1psW<IntEvent1IclrSpec> {
        IntEvent1IclrRt1psW::new(self, 5)
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

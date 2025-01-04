#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "Clear RTC-Ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRtcrdy {
    #[doc = "0: CLR"]
    IntEvent0IclrRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0IclrRtcrdySet = 1,
}
impl From<IntEvent0IclrRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RTCRDY` writer - Clear RTC-Ready interrupt"]
pub type IntEvent0IclrRtcrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRtcrdy>;
impl<'a, REG> IntEvent0IclrRtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rtcrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtcrdy::IntEvent0IclrRtcrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iclr_rtcrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtcrdy::IntEvent0IclrRtcrdySet)
    }
}
#[doc = "Clear Time-Event interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRtctev {
    #[doc = "0: CLR"]
    IntEvent0IclrRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent0IclrRtctevSet = 1,
}
impl From<IntEvent0IclrRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RTCTEV` writer - Clear Time-Event interrupt"]
pub type IntEvent0IclrRtctevW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRtctev>;
impl<'a, REG> IntEvent0IclrRtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rtctev_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtctev::IntEvent0IclrRtctevClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iclr_rtctev_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtctev::IntEvent0IclrRtctevSet)
    }
}
#[doc = "Clear Alarm-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRtca1 {
    #[doc = "0: CLR"]
    IntEvent0IclrRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0IclrRtca1Set = 1,
}
impl From<IntEvent0IclrRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RTCA1` writer - Clear Alarm-1 interrupt"]
pub type IntEvent0IclrRtca1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRtca1>;
impl<'a, REG> IntEvent0IclrRtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rtca1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtca1::IntEvent0IclrRtca1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iclr_rtca1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtca1::IntEvent0IclrRtca1Set)
    }
}
#[doc = "Clear Alarm-2 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRtca2 {
    #[doc = "0: CLR"]
    IntEvent0IclrRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0IclrRtca2Set = 1,
}
impl From<IntEvent0IclrRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RTCA2` writer - Clear Alarm-2 interrupt"]
pub type IntEvent0IclrRtca2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRtca2>;
impl<'a, REG> IntEvent0IclrRtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rtca2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtca2::IntEvent0IclrRtca2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iclr_rtca2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtca2::IntEvent0IclrRtca2Set)
    }
}
#[doc = "Clear Prescaler-0 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRt0ps {
    #[doc = "0: CLR"]
    IntEvent0IclrRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent0IclrRt0psSet = 1,
}
impl From<IntEvent0IclrRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RT0PS` writer - Clear Prescaler-0 interrupt"]
pub type IntEvent0IclrRt0psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRt0ps>;
impl<'a, REG> IntEvent0IclrRt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rt0ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRt0ps::IntEvent0IclrRt0psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iclr_rt0ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRt0ps::IntEvent0IclrRt0psSet)
    }
}
#[doc = "Clear Prescaler-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRt1ps {
    #[doc = "0: CLR"]
    IntEvent0IclrRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent0IclrRt1psSet = 1,
}
impl From<IntEvent0IclrRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RT1PS` writer - Clear Prescaler-1 interrupt"]
pub type IntEvent0IclrRt1psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRt1ps>;
impl<'a, REG> IntEvent0IclrRt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rt1ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRt1ps::IntEvent0IclrRt1psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iclr_rt1ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRt1ps::IntEvent0IclrRt1psSet)
    }
}
impl W {
    #[doc = "Bit 0 - Clear RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_rtcrdy(&mut self) -> IntEvent0IclrRtcrdyW<IntEvent0IclrSpec> {
        IntEvent0IclrRtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_rtctev(&mut self) -> IntEvent0IclrRtctevW<IntEvent0IclrSpec> {
        IntEvent0IclrRtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_rtca1(&mut self) -> IntEvent0IclrRtca1W<IntEvent0IclrSpec> {
        IntEvent0IclrRtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_rtca2(&mut self) -> IntEvent0IclrRtca2W<IntEvent0IclrSpec> {
        IntEvent0IclrRtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_rt0ps(&mut self) -> IntEvent0IclrRt0psW<IntEvent0IclrSpec> {
        IntEvent0IclrRt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_rt1ps(&mut self) -> IntEvent0IclrRt1psW<IntEvent0IclrSpec> {
        IntEvent0IclrRt1psW::new(self, 5)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IclrSpec;
impl crate::RegisterSpec for IntEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent0IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ICLR to value 0"]
impl crate::Resettable for IntEvent0IclrSpec {
    const RESET_VALUE: u32 = 0;
}

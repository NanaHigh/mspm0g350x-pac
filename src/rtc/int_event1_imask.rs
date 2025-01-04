#[doc = "Register `INT_EVENT1_IMASK` reader"]
pub type R = crate::R<IntEvent1ImaskSpec>;
#[doc = "Register `INT_EVENT1_IMASK` writer"]
pub type W = crate::W<IntEvent1ImaskSpec>;
#[doc = "Enable RTC-Ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRtcrdy {
    #[doc = "0: CLR"]
    IntEvent1ImaskRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRtcrdySet = 1,
}
impl From<IntEvent1ImaskRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCRDY` reader - Enable RTC-Ready interrupt"]
pub type IntEvent1ImaskRtcrdyR = crate::BitReader<IntEvent1ImaskRtcrdy>;
impl IntEvent1ImaskRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRtcrdy {
        match self.bits {
            false => IntEvent1ImaskRtcrdy::IntEvent1ImaskRtcrdyClr,
            true => IntEvent1ImaskRtcrdy::IntEvent1ImaskRtcrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtcrdy_clr(&self) -> bool {
        *self == IntEvent1ImaskRtcrdy::IntEvent1ImaskRtcrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtcrdy_set(&self) -> bool {
        *self == IntEvent1ImaskRtcrdy::IntEvent1ImaskRtcrdySet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCRDY` writer - Enable RTC-Ready interrupt"]
pub type IntEvent1ImaskRtcrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRtcrdy>;
impl<'a, REG> IntEvent1ImaskRtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rtcrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtcrdy::IntEvent1ImaskRtcrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rtcrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtcrdy::IntEvent1ImaskRtcrdySet)
    }
}
#[doc = "Enable Time-Event interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRtctev {
    #[doc = "0: CLR"]
    IntEvent1ImaskRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRtctevSet = 1,
}
impl From<IntEvent1ImaskRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCTEV` reader - Enable Time-Event interrupt"]
pub type IntEvent1ImaskRtctevR = crate::BitReader<IntEvent1ImaskRtctev>;
impl IntEvent1ImaskRtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRtctev {
        match self.bits {
            false => IntEvent1ImaskRtctev::IntEvent1ImaskRtctevClr,
            true => IntEvent1ImaskRtctev::IntEvent1ImaskRtctevSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtctev_clr(&self) -> bool {
        *self == IntEvent1ImaskRtctev::IntEvent1ImaskRtctevClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtctev_set(&self) -> bool {
        *self == IntEvent1ImaskRtctev::IntEvent1ImaskRtctevSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCTEV` writer - Enable Time-Event interrupt"]
pub type IntEvent1ImaskRtctevW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRtctev>;
impl<'a, REG> IntEvent1ImaskRtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rtctev_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtctev::IntEvent1ImaskRtctevClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rtctev_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtctev::IntEvent1ImaskRtctevSet)
    }
}
#[doc = "Enable Alarm-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRtca1 {
    #[doc = "0: CLR"]
    IntEvent1ImaskRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRtca1Set = 1,
}
impl From<IntEvent1ImaskRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCA1` reader - Enable Alarm-1 interrupt"]
pub type IntEvent1ImaskRtca1R = crate::BitReader<IntEvent1ImaskRtca1>;
impl IntEvent1ImaskRtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRtca1 {
        match self.bits {
            false => IntEvent1ImaskRtca1::IntEvent1ImaskRtca1Clr,
            true => IntEvent1ImaskRtca1::IntEvent1ImaskRtca1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtca1_clr(&self) -> bool {
        *self == IntEvent1ImaskRtca1::IntEvent1ImaskRtca1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtca1_set(&self) -> bool {
        *self == IntEvent1ImaskRtca1::IntEvent1ImaskRtca1Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCA1` writer - Enable Alarm-1 interrupt"]
pub type IntEvent1ImaskRtca1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRtca1>;
impl<'a, REG> IntEvent1ImaskRtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rtca1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtca1::IntEvent1ImaskRtca1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rtca1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtca1::IntEvent1ImaskRtca1Set)
    }
}
#[doc = "Enable Alarm-2 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRtca2 {
    #[doc = "0: CLR"]
    IntEvent1ImaskRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRtca2Set = 1,
}
impl From<IntEvent1ImaskRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCA2` reader - Enable Alarm-2 interrupt"]
pub type IntEvent1ImaskRtca2R = crate::BitReader<IntEvent1ImaskRtca2>;
impl IntEvent1ImaskRtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRtca2 {
        match self.bits {
            false => IntEvent1ImaskRtca2::IntEvent1ImaskRtca2Clr,
            true => IntEvent1ImaskRtca2::IntEvent1ImaskRtca2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtca2_clr(&self) -> bool {
        *self == IntEvent1ImaskRtca2::IntEvent1ImaskRtca2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtca2_set(&self) -> bool {
        *self == IntEvent1ImaskRtca2::IntEvent1ImaskRtca2Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTCA2` writer - Enable Alarm-2 interrupt"]
pub type IntEvent1ImaskRtca2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRtca2>;
impl<'a, REG> IntEvent1ImaskRtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rtca2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtca2::IntEvent1ImaskRtca2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rtca2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtca2::IntEvent1ImaskRtca2Set)
    }
}
#[doc = "Enable Prescaler-0 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRt0ps {
    #[doc = "0: CLR"]
    IntEvent1ImaskRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRt0psSet = 1,
}
impl From<IntEvent1ImaskRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RT0PS` reader - Enable Prescaler-0 interrupt"]
pub type IntEvent1ImaskRt0psR = crate::BitReader<IntEvent1ImaskRt0ps>;
impl IntEvent1ImaskRt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRt0ps {
        match self.bits {
            false => IntEvent1ImaskRt0ps::IntEvent1ImaskRt0psClr,
            true => IntEvent1ImaskRt0ps::IntEvent1ImaskRt0psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rt0ps_clr(&self) -> bool {
        *self == IntEvent1ImaskRt0ps::IntEvent1ImaskRt0psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rt0ps_set(&self) -> bool {
        *self == IntEvent1ImaskRt0ps::IntEvent1ImaskRt0psSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RT0PS` writer - Enable Prescaler-0 interrupt"]
pub type IntEvent1ImaskRt0psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRt0ps>;
impl<'a, REG> IntEvent1ImaskRt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rt0ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRt0ps::IntEvent1ImaskRt0psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rt0ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRt0ps::IntEvent1ImaskRt0psSet)
    }
}
#[doc = "Enable Prescaler-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRt1ps {
    #[doc = "0: CLR"]
    IntEvent1ImaskRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRt1psSet = 1,
}
impl From<IntEvent1ImaskRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RT1PS` reader - Enable Prescaler-1 interrupt"]
pub type IntEvent1ImaskRt1psR = crate::BitReader<IntEvent1ImaskRt1ps>;
impl IntEvent1ImaskRt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRt1ps {
        match self.bits {
            false => IntEvent1ImaskRt1ps::IntEvent1ImaskRt1psClr,
            true => IntEvent1ImaskRt1ps::IntEvent1ImaskRt1psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rt1ps_clr(&self) -> bool {
        *self == IntEvent1ImaskRt1ps::IntEvent1ImaskRt1psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rt1ps_set(&self) -> bool {
        *self == IntEvent1ImaskRt1ps::IntEvent1ImaskRt1psSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RT1PS` writer - Enable Prescaler-1 interrupt"]
pub type IntEvent1ImaskRt1psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRt1ps>;
impl<'a, REG> IntEvent1ImaskRt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rt1ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRt1ps::IntEvent1ImaskRt1psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rt1ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRt1ps::IntEvent1ImaskRt1psSet)
    }
}
impl R {
    #[doc = "Bit 0 - Enable RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtcrdy(&self) -> IntEvent1ImaskRtcrdyR {
        IntEvent1ImaskRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtctev(&self) -> IntEvent1ImaskRtctevR {
        IntEvent1ImaskRtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtca1(&self) -> IntEvent1ImaskRtca1R {
        IntEvent1ImaskRtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtca2(&self) -> IntEvent1ImaskRtca2R {
        IntEvent1ImaskRtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rt0ps(&self) -> IntEvent1ImaskRt0psR {
        IntEvent1ImaskRt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rt1ps(&self) -> IntEvent1ImaskRt1psR {
        IntEvent1ImaskRt1psR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtcrdy(&mut self) -> IntEvent1ImaskRtcrdyW<IntEvent1ImaskSpec> {
        IntEvent1ImaskRtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtctev(&mut self) -> IntEvent1ImaskRtctevW<IntEvent1ImaskSpec> {
        IntEvent1ImaskRtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtca1(&mut self) -> IntEvent1ImaskRtca1W<IntEvent1ImaskSpec> {
        IntEvent1ImaskRtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rtca2(&mut self) -> IntEvent1ImaskRtca2W<IntEvent1ImaskSpec> {
        IntEvent1ImaskRtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rt0ps(&mut self) -> IntEvent1ImaskRt0psW<IntEvent1ImaskSpec> {
        IntEvent1ImaskRt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event1_imask_rt1ps(&mut self) -> IntEvent1ImaskRt1psW<IntEvent1ImaskSpec> {
        IntEvent1ImaskRt1psW::new(self, 5)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1ImaskSpec;
impl crate::RegisterSpec for IntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for IntEvent1ImaskSpec {
    const RESET_VALUE: u32 = 0;
}

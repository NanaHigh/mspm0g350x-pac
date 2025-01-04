#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "Enable RTC-Ready interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRtcrdy {
    #[doc = "0: CLR"]
    IntEvent0ImaskRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRtcrdySet = 1,
}
impl From<IntEvent0ImaskRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCRDY` reader - Enable RTC-Ready interrupt"]
pub type IntEvent0ImaskRtcrdyR = crate::BitReader<IntEvent0ImaskRtcrdy>;
impl IntEvent0ImaskRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRtcrdy {
        match self.bits {
            false => IntEvent0ImaskRtcrdy::IntEvent0ImaskRtcrdyClr,
            true => IntEvent0ImaskRtcrdy::IntEvent0ImaskRtcrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtcrdy_clr(&self) -> bool {
        *self == IntEvent0ImaskRtcrdy::IntEvent0ImaskRtcrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtcrdy_set(&self) -> bool {
        *self == IntEvent0ImaskRtcrdy::IntEvent0ImaskRtcrdySet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCRDY` writer - Enable RTC-Ready interrupt"]
pub type IntEvent0ImaskRtcrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRtcrdy>;
impl<'a, REG> IntEvent0ImaskRtcrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rtcrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtcrdy::IntEvent0ImaskRtcrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rtcrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtcrdy::IntEvent0ImaskRtcrdySet)
    }
}
#[doc = "Enable Time-Event interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRtctev {
    #[doc = "0: CLR"]
    IntEvent0ImaskRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRtctevSet = 1,
}
impl From<IntEvent0ImaskRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCTEV` reader - Enable Time-Event interrupt"]
pub type IntEvent0ImaskRtctevR = crate::BitReader<IntEvent0ImaskRtctev>;
impl IntEvent0ImaskRtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRtctev {
        match self.bits {
            false => IntEvent0ImaskRtctev::IntEvent0ImaskRtctevClr,
            true => IntEvent0ImaskRtctev::IntEvent0ImaskRtctevSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtctev_clr(&self) -> bool {
        *self == IntEvent0ImaskRtctev::IntEvent0ImaskRtctevClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtctev_set(&self) -> bool {
        *self == IntEvent0ImaskRtctev::IntEvent0ImaskRtctevSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCTEV` writer - Enable Time-Event interrupt"]
pub type IntEvent0ImaskRtctevW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRtctev>;
impl<'a, REG> IntEvent0ImaskRtctevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rtctev_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtctev::IntEvent0ImaskRtctevClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rtctev_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtctev::IntEvent0ImaskRtctevSet)
    }
}
#[doc = "Enable Alarm-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRtca1 {
    #[doc = "0: CLR"]
    IntEvent0ImaskRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRtca1Set = 1,
}
impl From<IntEvent0ImaskRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCA1` reader - Enable Alarm-1 interrupt"]
pub type IntEvent0ImaskRtca1R = crate::BitReader<IntEvent0ImaskRtca1>;
impl IntEvent0ImaskRtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRtca1 {
        match self.bits {
            false => IntEvent0ImaskRtca1::IntEvent0ImaskRtca1Clr,
            true => IntEvent0ImaskRtca1::IntEvent0ImaskRtca1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtca1_clr(&self) -> bool {
        *self == IntEvent0ImaskRtca1::IntEvent0ImaskRtca1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtca1_set(&self) -> bool {
        *self == IntEvent0ImaskRtca1::IntEvent0ImaskRtca1Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCA1` writer - Enable Alarm-1 interrupt"]
pub type IntEvent0ImaskRtca1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRtca1>;
impl<'a, REG> IntEvent0ImaskRtca1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rtca1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtca1::IntEvent0ImaskRtca1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rtca1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtca1::IntEvent0ImaskRtca1Set)
    }
}
#[doc = "Enable Alarm-2 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRtca2 {
    #[doc = "0: CLR"]
    IntEvent0ImaskRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRtca2Set = 1,
}
impl From<IntEvent0ImaskRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCA2` reader - Enable Alarm-2 interrupt"]
pub type IntEvent0ImaskRtca2R = crate::BitReader<IntEvent0ImaskRtca2>;
impl IntEvent0ImaskRtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRtca2 {
        match self.bits {
            false => IntEvent0ImaskRtca2::IntEvent0ImaskRtca2Clr,
            true => IntEvent0ImaskRtca2::IntEvent0ImaskRtca2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtca2_clr(&self) -> bool {
        *self == IntEvent0ImaskRtca2::IntEvent0ImaskRtca2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtca2_set(&self) -> bool {
        *self == IntEvent0ImaskRtca2::IntEvent0ImaskRtca2Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTCA2` writer - Enable Alarm-2 interrupt"]
pub type IntEvent0ImaskRtca2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRtca2>;
impl<'a, REG> IntEvent0ImaskRtca2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rtca2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtca2::IntEvent0ImaskRtca2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rtca2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtca2::IntEvent0ImaskRtca2Set)
    }
}
#[doc = "Enable Prescaler-0 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRt0ps {
    #[doc = "0: CLR"]
    IntEvent0ImaskRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRt0psSet = 1,
}
impl From<IntEvent0ImaskRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RT0PS` reader - Enable Prescaler-0 interrupt"]
pub type IntEvent0ImaskRt0psR = crate::BitReader<IntEvent0ImaskRt0ps>;
impl IntEvent0ImaskRt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRt0ps {
        match self.bits {
            false => IntEvent0ImaskRt0ps::IntEvent0ImaskRt0psClr,
            true => IntEvent0ImaskRt0ps::IntEvent0ImaskRt0psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rt0ps_clr(&self) -> bool {
        *self == IntEvent0ImaskRt0ps::IntEvent0ImaskRt0psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rt0ps_set(&self) -> bool {
        *self == IntEvent0ImaskRt0ps::IntEvent0ImaskRt0psSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RT0PS` writer - Enable Prescaler-0 interrupt"]
pub type IntEvent0ImaskRt0psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRt0ps>;
impl<'a, REG> IntEvent0ImaskRt0psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rt0ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRt0ps::IntEvent0ImaskRt0psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rt0ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRt0ps::IntEvent0ImaskRt0psSet)
    }
}
#[doc = "Enable Prescaler-1 interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRt1ps {
    #[doc = "0: CLR"]
    IntEvent0ImaskRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRt1psSet = 1,
}
impl From<IntEvent0ImaskRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RT1PS` reader - Enable Prescaler-1 interrupt"]
pub type IntEvent0ImaskRt1psR = crate::BitReader<IntEvent0ImaskRt1ps>;
impl IntEvent0ImaskRt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRt1ps {
        match self.bits {
            false => IntEvent0ImaskRt1ps::IntEvent0ImaskRt1psClr,
            true => IntEvent0ImaskRt1ps::IntEvent0ImaskRt1psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rt1ps_clr(&self) -> bool {
        *self == IntEvent0ImaskRt1ps::IntEvent0ImaskRt1psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rt1ps_set(&self) -> bool {
        *self == IntEvent0ImaskRt1ps::IntEvent0ImaskRt1psSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RT1PS` writer - Enable Prescaler-1 interrupt"]
pub type IntEvent0ImaskRt1psW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRt1ps>;
impl<'a, REG> IntEvent0ImaskRt1psW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rt1ps_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRt1ps::IntEvent0ImaskRt1psClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rt1ps_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRt1ps::IntEvent0ImaskRt1psSet)
    }
}
impl R {
    #[doc = "Bit 0 - Enable RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtcrdy(&self) -> IntEvent0ImaskRtcrdyR {
        IntEvent0ImaskRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtctev(&self) -> IntEvent0ImaskRtctevR {
        IntEvent0ImaskRtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtca1(&self) -> IntEvent0ImaskRtca1R {
        IntEvent0ImaskRtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtca2(&self) -> IntEvent0ImaskRtca2R {
        IntEvent0ImaskRtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rt0ps(&self) -> IntEvent0ImaskRt0psR {
        IntEvent0ImaskRt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rt1ps(&self) -> IntEvent0ImaskRt1psR {
        IntEvent0ImaskRt1psR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RTC-Ready interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtcrdy(&mut self) -> IntEvent0ImaskRtcrdyW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRtcrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Time-Event interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtctev(&mut self) -> IntEvent0ImaskRtctevW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRtctevW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Alarm-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtca1(&mut self) -> IntEvent0ImaskRtca1W<IntEvent0ImaskSpec> {
        IntEvent0ImaskRtca1W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Alarm-2 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rtca2(&mut self) -> IntEvent0ImaskRtca2W<IntEvent0ImaskSpec> {
        IntEvent0ImaskRtca2W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Prescaler-0 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rt0ps(&mut self) -> IntEvent0ImaskRt0psW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRt0psW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Prescaler-1 interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_rt1ps(&mut self) -> IntEvent0ImaskRt1psW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRt1psW::new(self, 5)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}

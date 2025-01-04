#[doc = "Register `SYSPLLPARAM0` reader"]
pub type R = crate::R<Syspllparam0Spec>;
#[doc = "Register `SYSPLLPARAM0` writer"]
pub type W = crate::W<Syspllparam0Spec>;
#[doc = "Field `SYSPLLPARAM0_STARTTIME` reader - Startup time from enable to locked clock, in 1us resolution"]
pub type Syspllparam0StarttimeR = crate::FieldReader;
#[doc = "Field `SYSPLLPARAM0_STARTTIME` writer - Startup time from enable to locked clock, in 1us resolution"]
pub type Syspllparam0StarttimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYSPLLPARAM0_STARTTIMELP` reader - Startup time from low power mode exit to locked clock, in 1us resolution"]
pub type Syspllparam0StarttimelpR = crate::FieldReader;
#[doc = "Field `SYSPLLPARAM0_STARTTIMELP` writer - Startup time from low power mode exit to locked clock, in 1us resolution"]
pub type Syspllparam0StarttimelpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYSPLLPARAM0_CPCURRENT` reader - Charge pump current"]
pub type Syspllparam0CpcurrentR = crate::FieldReader;
#[doc = "Field `SYSPLLPARAM0_CPCURRENT` writer - Charge pump current"]
pub type Syspllparam0CpcurrentW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYSPLLPARAM0_CAPBVAL` reader - Override value for Cap B"]
pub type Syspllparam0CapbvalR = crate::FieldReader;
#[doc = "Field `SYSPLLPARAM0_CAPBVAL` writer - Override value for Cap B"]
pub type Syspllparam0CapbvalW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "CAPBOVERRIDE controls the override for Cap B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllparam0Capboverride {
    #[doc = "0: DISABLE"]
    Syspllparam0CapboverrideDisable = 0,
    #[doc = "1: ENABLE"]
    Syspllparam0CapboverrideEnable = 1,
}
impl From<Syspllparam0Capboverride> for bool {
    #[inline(always)]
    fn from(variant: Syspllparam0Capboverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLPARAM0_CAPBOVERRIDE` reader - CAPBOVERRIDE controls the override for Cap B"]
pub type Syspllparam0CapboverrideR = crate::BitReader<Syspllparam0Capboverride>;
impl Syspllparam0CapboverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllparam0Capboverride {
        match self.bits {
            false => Syspllparam0Capboverride::Syspllparam0CapboverrideDisable,
            true => Syspllparam0Capboverride::Syspllparam0CapboverrideEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_syspllparam0_capboverride_disable(&self) -> bool {
        *self == Syspllparam0Capboverride::Syspllparam0CapboverrideDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_syspllparam0_capboverride_enable(&self) -> bool {
        *self == Syspllparam0Capboverride::Syspllparam0CapboverrideEnable
    }
}
#[doc = "Field `SYSPLLPARAM0_CAPBOVERRIDE` writer - CAPBOVERRIDE controls the override for Cap B"]
pub type Syspllparam0CapboverrideW<'a, REG> = crate::BitWriter<'a, REG, Syspllparam0Capboverride>;
impl<'a, REG> Syspllparam0CapboverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn syspllparam0_capboverride_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllparam0Capboverride::Syspllparam0CapboverrideDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn syspllparam0_capboverride_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllparam0Capboverride::Syspllparam0CapboverrideEnable)
    }
}
impl R {
    #[doc = "Bits 0:5 - Startup time from enable to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn syspllparam0_starttime(&self) -> Syspllparam0StarttimeR {
        Syspllparam0StarttimeR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Startup time from low power mode exit to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn syspllparam0_starttimelp(&self) -> Syspllparam0StarttimelpR {
        Syspllparam0StarttimelpR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Charge pump current"]
    #[inline(always)]
    pub fn syspllparam0_cpcurrent(&self) -> Syspllparam0CpcurrentR {
        Syspllparam0CpcurrentR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:28 - Override value for Cap B"]
    #[inline(always)]
    pub fn syspllparam0_capbval(&self) -> Syspllparam0CapbvalR {
        Syspllparam0CapbvalR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - CAPBOVERRIDE controls the override for Cap B"]
    #[inline(always)]
    pub fn syspllparam0_capboverride(&self) -> Syspllparam0CapboverrideR {
        Syspllparam0CapboverrideR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Startup time from enable to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn syspllparam0_starttime(&mut self) -> Syspllparam0StarttimeW<Syspllparam0Spec> {
        Syspllparam0StarttimeW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Startup time from low power mode exit to locked clock, in 1us resolution"]
    #[inline(always)]
    pub fn syspllparam0_starttimelp(&mut self) -> Syspllparam0StarttimelpW<Syspllparam0Spec> {
        Syspllparam0StarttimelpW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Charge pump current"]
    #[inline(always)]
    pub fn syspllparam0_cpcurrent(&mut self) -> Syspllparam0CpcurrentW<Syspllparam0Spec> {
        Syspllparam0CpcurrentW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Override value for Cap B"]
    #[inline(always)]
    pub fn syspllparam0_capbval(&mut self) -> Syspllparam0CapbvalW<Syspllparam0Spec> {
        Syspllparam0CapbvalW::new(self, 24)
    }
    #[doc = "Bit 31 - CAPBOVERRIDE controls the override for Cap B"]
    #[inline(always)]
    pub fn syspllparam0_capboverride(&mut self) -> Syspllparam0CapboverrideW<Syspllparam0Spec> {
        Syspllparam0CapboverrideW::new(self, 31)
    }
}
#[doc = "SYSPLL PARAM0 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllparam0Spec;
impl crate::RegisterSpec for Syspllparam0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllparam0::R`](R) reader structure"]
impl crate::Readable for Syspllparam0Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllparam0::W`](W) writer structure"]
impl crate::Writable for Syspllparam0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLLPARAM0 to value 0"]
impl crate::Resettable for Syspllparam0Spec {
    const RESET_VALUE: u32 = 0;
}

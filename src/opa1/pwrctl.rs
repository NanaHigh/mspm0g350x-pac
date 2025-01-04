#[doc = "Register `PWRCTL` reader"]
pub type R = crate::R<PwrctlSpec>;
#[doc = "Register `PWRCTL` writer"]
pub type W = crate::W<PwrctlSpec>;
#[doc = "When set the peripheral will remove its local IP request for enable so that it can be disabled if no other entities in the system are requesting it to be enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrctlAutoOff {
    #[doc = "0: DISABLE"]
    PwrctlAutoOffDisable = 0,
    #[doc = "1: ENABLE"]
    PwrctlAutoOffEnable = 1,
}
impl From<PwrctlAutoOff> for bool {
    #[inline(always)]
    fn from(variant: PwrctlAutoOff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRCTL_AUTO_OFF` reader - When set the peripheral will remove its local IP request for enable so that it can be disabled if no other entities in the system are requesting it to be enabled."]
pub type PwrctlAutoOffR = crate::BitReader<PwrctlAutoOff>;
impl PwrctlAutoOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrctlAutoOff {
        match self.bits {
            false => PwrctlAutoOff::PwrctlAutoOffDisable,
            true => PwrctlAutoOff::PwrctlAutoOffEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pwrctl_auto_off_disable(&self) -> bool {
        *self == PwrctlAutoOff::PwrctlAutoOffDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pwrctl_auto_off_enable(&self) -> bool {
        *self == PwrctlAutoOff::PwrctlAutoOffEnable
    }
}
#[doc = "Field `PWRCTL_AUTO_OFF` writer - When set the peripheral will remove its local IP request for enable so that it can be disabled if no other entities in the system are requesting it to be enabled."]
pub type PwrctlAutoOffW<'a, REG> = crate::BitWriter<'a, REG, PwrctlAutoOff>;
impl<'a, REG> PwrctlAutoOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pwrctl_auto_off_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PwrctlAutoOff::PwrctlAutoOffDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pwrctl_auto_off_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PwrctlAutoOff::PwrctlAutoOffEnable)
    }
}
impl R {
    #[doc = "Bit 0 - When set the peripheral will remove its local IP request for enable so that it can be disabled if no other entities in the system are requesting it to be enabled."]
    #[inline(always)]
    pub fn pwrctl_auto_off(&self) -> PwrctlAutoOffR {
        PwrctlAutoOffR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set the peripheral will remove its local IP request for enable so that it can be disabled if no other entities in the system are requesting it to be enabled."]
    #[inline(always)]
    pub fn pwrctl_auto_off(&mut self) -> PwrctlAutoOffW<PwrctlSpec> {
        PwrctlAutoOffW::new(self, 0)
    }
}
#[doc = "Power Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctlSpec;
impl crate::RegisterSpec for PwrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctl::R`](R) reader structure"]
impl crate::Readable for PwrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctl::W`](W) writer structure"]
impl crate::Writable for PwrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCTL to value 0"]
impl crate::Resettable for PwrctlSpec {
    const RESET_VALUE: u32 = 0;
}

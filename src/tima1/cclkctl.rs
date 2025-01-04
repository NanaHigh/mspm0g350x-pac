#[doc = "Register `CCLKCTL` reader"]
pub type R = crate::R<CclkctlSpec>;
#[doc = "Register `CCLKCTL` writer"]
pub type W = crate::W<CclkctlSpec>;
#[doc = "Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CclkctlClken {
    #[doc = "0: DISABLED"]
    CclkctlClkenDisabled = 0,
    #[doc = "1: ENABLED"]
    CclkctlClkenEnabled = 1,
}
impl From<CclkctlClken> for bool {
    #[inline(always)]
    fn from(variant: CclkctlClken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLKCTL_CLKEN` reader - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
pub type CclkctlClkenR = crate::BitReader<CclkctlClken>;
impl CclkctlClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CclkctlClken {
        match self.bits {
            false => CclkctlClken::CclkctlClkenDisabled,
            true => CclkctlClken::CclkctlClkenEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_cclkctl_clken_disabled(&self) -> bool {
        *self == CclkctlClken::CclkctlClkenDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_cclkctl_clken_enabled(&self) -> bool {
        *self == CclkctlClken::CclkctlClkenEnabled
    }
}
#[doc = "Field `CCLKCTL_CLKEN` writer - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
pub type CclkctlClkenW<'a, REG> = crate::BitWriter<'a, REG, CclkctlClken>;
impl<'a, REG> CclkctlClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn cclkctl_clken_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CclkctlClken::CclkctlClkenDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn cclkctl_clken_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CclkctlClken::CclkctlClkenEnabled)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
    #[inline(always)]
    pub fn cclkctl_clken(&self) -> CclkctlClkenR {
        CclkctlClkenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable Disables the clock gating to the module. SW has to explicitly program the value to 0 to gate the clock."]
    #[inline(always)]
    pub fn cclkctl_clken(&mut self) -> CclkctlClkenW<CclkctlSpec> {
        CclkctlClkenW::new(self, 0)
    }
}
#[doc = "Counter Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CclkctlSpec;
impl crate::RegisterSpec for CclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cclkctl::R`](R) reader structure"]
impl crate::Readable for CclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cclkctl::W`](W) writer structure"]
impl crate::Writable for CclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCLKCTL to value 0"]
impl crate::Resettable for CclkctlSpec {
    const RESET_VALUE: u32 = 0;
}

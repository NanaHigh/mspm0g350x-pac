#[doc = "Register `EXLFCTL` writer"]
pub type W = crate::W<ExlfctlSpec>;
#[doc = "Set SETUSEEXLF to switch LFCLK to the LFCLK_IN digital clock input. Once set, SETUSEEXLF remains set until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExlfctlSetuseexlf {
    #[doc = "1: TRUE"]
    ExlfctlSetuseexlfTrue = 1,
}
impl From<ExlfctlSetuseexlf> for bool {
    #[inline(always)]
    fn from(variant: ExlfctlSetuseexlf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXLFCTL_SETUSEEXLF` writer - Set SETUSEEXLF to switch LFCLK to the LFCLK_IN digital clock input. Once set, SETUSEEXLF remains set until the next BOOTRST."]
pub type ExlfctlSetuseexlfW<'a, REG> = crate::BitWriter<'a, REG, ExlfctlSetuseexlf>;
impl<'a, REG> ExlfctlSetuseexlfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn exlfctl_setuseexlf_true(self) -> &'a mut crate::W<REG> {
        self.variant(ExlfctlSetuseexlf::ExlfctlSetuseexlfTrue)
    }
}
#[doc = "The key value of 36h (54) must be written to KEY together with SETUSEEXLF to set SETUSEEXLF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExlfctlKey {
    #[doc = "54: VALUE"]
    ExlfctlKeyValue = 54,
}
impl From<ExlfctlKey> for u8 {
    #[inline(always)]
    fn from(variant: ExlfctlKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ExlfctlKey {
    type Ux = u8;
}
impl crate::IsEnum for ExlfctlKey {}
#[doc = "Field `EXLFCTL_KEY` writer - The key value of 36h (54) must be written to KEY together with SETUSEEXLF to set SETUSEEXLF."]
pub type ExlfctlKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, ExlfctlKey>;
impl<'a, REG> ExlfctlKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn exlfctl_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(ExlfctlKey::ExlfctlKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set SETUSEEXLF to switch LFCLK to the LFCLK_IN digital clock input. Once set, SETUSEEXLF remains set until the next BOOTRST."]
    #[inline(always)]
    pub fn exlfctl_setuseexlf(&mut self) -> ExlfctlSetuseexlfW<ExlfctlSpec> {
        ExlfctlSetuseexlfW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value of 36h (54) must be written to KEY together with SETUSEEXLF to set SETUSEEXLF."]
    #[inline(always)]
    pub fn exlfctl_key(&mut self) -> ExlfctlKeyW<ExlfctlSpec> {
        ExlfctlKeyW::new(self, 24)
    }
}
#[doc = "LFCLK_IN and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exlfctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExlfctlSpec;
impl crate::RegisterSpec for ExlfctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`exlfctl::W`](W) writer structure"]
impl crate::Writable for ExlfctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXLFCTL to value 0"]
impl crate::Resettable for ExlfctlSpec {
    const RESET_VALUE: u32 = 0;
}

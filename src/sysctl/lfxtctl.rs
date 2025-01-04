#[doc = "Register `LFXTCTL` writer"]
pub type W = crate::W<LfxtctlSpec>;
#[doc = "Set STARTLFXT to start the low frequency crystal oscillator (LFXT). Once set, STARTLFXT remains set until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfxtctlStartlfxt {
    #[doc = "0: FALSE"]
    LfxtctlStartlfxtFalse = 0,
    #[doc = "1: TRUE"]
    LfxtctlStartlfxtTrue = 1,
}
impl From<LfxtctlStartlfxt> for bool {
    #[inline(always)]
    fn from(variant: LfxtctlStartlfxt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTCTL_STARTLFXT` writer - Set STARTLFXT to start the low frequency crystal oscillator (LFXT). Once set, STARTLFXT remains set until the next BOOTRST."]
pub type LfxtctlStartlfxtW<'a, REG> = crate::BitWriter<'a, REG, LfxtctlStartlfxt>;
impl<'a, REG> LfxtctlStartlfxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn lfxtctl_startlfxt_false(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtctlStartlfxt::LfxtctlStartlfxtFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn lfxtctl_startlfxt_true(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtctlStartlfxt::LfxtctlStartlfxtTrue)
    }
}
#[doc = "Set SETUSELFXT to switch LFCLK to LFXT. Once set, SETUSELFXT remains set until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LfxtctlSetuselfxt {
    #[doc = "0: FALSE"]
    LfxtctlSetuselfxtFalse = 0,
    #[doc = "1: TRUE"]
    LfxtctlSetuselfxtTrue = 1,
}
impl From<LfxtctlSetuselfxt> for bool {
    #[inline(always)]
    fn from(variant: LfxtctlSetuselfxt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFXTCTL_SETUSELFXT` writer - Set SETUSELFXT to switch LFCLK to LFXT. Once set, SETUSELFXT remains set until the next BOOTRST."]
pub type LfxtctlSetuselfxtW<'a, REG> = crate::BitWriter<'a, REG, LfxtctlSetuselfxt>;
impl<'a, REG> LfxtctlSetuselfxtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn lfxtctl_setuselfxt_false(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtctlSetuselfxt::LfxtctlSetuselfxtFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn lfxtctl_setuselfxt_true(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtctlSetuselfxt::LfxtctlSetuselfxtTrue)
    }
}
#[doc = "The key value of 91h (145) must be written to KEY together with either STARTLFXT or SETUSELFXT to set the corresponding bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LfxtctlKey {
    #[doc = "145: VALUE"]
    LfxtctlKeyValue = 145,
}
impl From<LfxtctlKey> for u8 {
    #[inline(always)]
    fn from(variant: LfxtctlKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LfxtctlKey {
    type Ux = u8;
}
impl crate::IsEnum for LfxtctlKey {}
#[doc = "Field `LFXTCTL_KEY` writer - The key value of 91h (145) must be written to KEY together with either STARTLFXT or SETUSELFXT to set the corresponding bit."]
pub type LfxtctlKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, LfxtctlKey>;
impl<'a, REG> LfxtctlKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn lfxtctl_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(LfxtctlKey::LfxtctlKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set STARTLFXT to start the low frequency crystal oscillator (LFXT). Once set, STARTLFXT remains set until the next BOOTRST."]
    #[inline(always)]
    pub fn lfxtctl_startlfxt(&mut self) -> LfxtctlStartlfxtW<LfxtctlSpec> {
        LfxtctlStartlfxtW::new(self, 0)
    }
    #[doc = "Bit 1 - Set SETUSELFXT to switch LFCLK to LFXT. Once set, SETUSELFXT remains set until the next BOOTRST."]
    #[inline(always)]
    pub fn lfxtctl_setuselfxt(&mut self) -> LfxtctlSetuselfxtW<LfxtctlSpec> {
        LfxtctlSetuselfxtW::new(self, 1)
    }
    #[doc = "Bits 24:31 - The key value of 91h (145) must be written to KEY together with either STARTLFXT or SETUSELFXT to set the corresponding bit."]
    #[inline(always)]
    pub fn lfxtctl_key(&mut self) -> LfxtctlKeyW<LfxtctlSpec> {
        LfxtctlKeyW::new(self, 24)
    }
}
#[doc = "LFXT and LFCLK control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfxtctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfxtctlSpec;
impl crate::RegisterSpec for LfxtctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lfxtctl::W`](W) writer structure"]
impl crate::Writable for LfxtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFXTCTL to value 0"]
impl crate::Resettable for LfxtctlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `SWDCFG` writer"]
pub type W = crate::W<SwdcfgSpec>;
#[doc = "Set DISABLE to disable the SWD function on SWD pins, allowing the SWD pins to be used as GPIO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwdcfgDisable {
    #[doc = "1: TRUE"]
    SwdcfgDisableTrue = 1,
}
impl From<SwdcfgDisable> for bool {
    #[inline(always)]
    fn from(variant: SwdcfgDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDCFG_DISABLE` writer - Set DISABLE to disable the SWD function on SWD pins, allowing the SWD pins to be used as GPIO."]
pub type SwdcfgDisableW<'a, REG> = crate::BitWriter<'a, REG, SwdcfgDisable>;
impl<'a, REG> SwdcfgDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn swdcfg_disable_true(self) -> &'a mut crate::W<REG> {
        self.variant(SwdcfgDisable::SwdcfgDisableTrue)
    }
}
#[doc = "The key value 62h (98) must be written to KEY together with DISBALE to disable the SWD functions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SwdcfgKey {
    #[doc = "98: VALUE"]
    SwdcfgKeyValue = 98,
}
impl From<SwdcfgKey> for u8 {
    #[inline(always)]
    fn from(variant: SwdcfgKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SwdcfgKey {
    type Ux = u8;
}
impl crate::IsEnum for SwdcfgKey {}
#[doc = "Field `SWDCFG_KEY` writer - The key value 62h (98) must be written to KEY together with DISBALE to disable the SWD functions."]
pub type SwdcfgKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, SwdcfgKey>;
impl<'a, REG> SwdcfgKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn swdcfg_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(SwdcfgKey::SwdcfgKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set DISABLE to disable the SWD function on SWD pins, allowing the SWD pins to be used as GPIO."]
    #[inline(always)]
    pub fn swdcfg_disable(&mut self) -> SwdcfgDisableW<SwdcfgSpec> {
        SwdcfgDisableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value 62h (98) must be written to KEY together with DISBALE to disable the SWD functions."]
    #[inline(always)]
    pub fn swdcfg_key(&mut self) -> SwdcfgKeyW<SwdcfgSpec> {
        SwdcfgKeyW::new(self, 24)
    }
}
#[doc = "Disable the SWD function on the SWD pins\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swdcfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwdcfgSpec;
impl crate::RegisterSpec for SwdcfgSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swdcfg::W`](W) writer structure"]
impl crate::Writable for SwdcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWDCFG to value 0"]
impl crate::Resettable for SwdcfgSpec {
    const RESET_VALUE: u32 = 0;
}

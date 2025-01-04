#[doc = "Register `EXRSTPIN` writer"]
pub type W = crate::W<ExrstpinSpec>;
#[doc = "Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExrstpinDisable {
    #[doc = "0: FALSE"]
    ExrstpinDisableFalse = 0,
    #[doc = "1: TRUE"]
    ExrstpinDisableTrue = 1,
}
impl From<ExrstpinDisable> for bool {
    #[inline(always)]
    fn from(variant: ExrstpinDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXRSTPIN_DISABLE` writer - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."]
pub type ExrstpinDisableW<'a, REG> = crate::BitWriter<'a, REG, ExrstpinDisable>;
impl<'a, REG> ExrstpinDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn exrstpin_disable_false(self) -> &'a mut crate::W<REG> {
        self.variant(ExrstpinDisable::ExrstpinDisableFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn exrstpin_disable_true(self) -> &'a mut crate::W<REG> {
        self.variant(ExrstpinDisable::ExrstpinDisableTrue)
    }
}
#[doc = "The key value 1Eh must be written together with DISABLE to disable the reset function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExrstpinKey {
    #[doc = "30: VALUE"]
    ExrstpinKeyValue = 30,
}
impl From<ExrstpinKey> for u8 {
    #[inline(always)]
    fn from(variant: ExrstpinKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ExrstpinKey {
    type Ux = u8;
}
impl crate::IsEnum for ExrstpinKey {}
#[doc = "Field `EXRSTPIN_KEY` writer - The key value 1Eh must be written together with DISABLE to disable the reset function."]
pub type ExrstpinKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, ExrstpinKey>;
impl<'a, REG> ExrstpinKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn exrstpin_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(ExrstpinKey::ExrstpinKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set DISABLE to disable the reset function of the NRST pin. Once set, this configuration is locked until the next POR."]
    #[inline(always)]
    pub fn exrstpin_disable(&mut self) -> ExrstpinDisableW<ExrstpinSpec> {
        ExrstpinDisableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value 1Eh must be written together with DISABLE to disable the reset function."]
    #[inline(always)]
    pub fn exrstpin_key(&mut self) -> ExrstpinKeyW<ExrstpinSpec> {
        ExrstpinKeyW::new(self, 24)
    }
}
#[doc = "Disable the reset function of the NRST pin\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exrstpin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExrstpinSpec;
impl crate::RegisterSpec for ExrstpinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`exrstpin::W`](W) writer structure"]
impl crate::Writable for ExrstpinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXRSTPIN to value 0"]
impl crate::Resettable for ExrstpinSpec {
    const RESET_VALUE: u32 = 0;
}

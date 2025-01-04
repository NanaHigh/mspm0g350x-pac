#[doc = "Register `PWREN` reader"]
pub type R = crate::R<PwrenSpec>;
#[doc = "Register `PWREN` writer"]
pub type W = crate::W<PwrenSpec>;
#[doc = "Enable the power Note: For safety devices the power cannot be disabled once enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrenEnable {
    #[doc = "0: DISABLE"]
    PwrenEnableDisable = 0,
    #[doc = "1: ENABLE"]
    PwrenEnableEnable = 1,
}
impl From<PwrenEnable> for bool {
    #[inline(always)]
    fn from(variant: PwrenEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWREN_ENABLE` reader - Enable the power Note: For safety devices the power cannot be disabled once enabled."]
pub type PwrenEnableR = crate::BitReader<PwrenEnable>;
impl PwrenEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrenEnable {
        match self.bits {
            false => PwrenEnable::PwrenEnableDisable,
            true => PwrenEnable::PwrenEnableEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_pwren_enable_disable(&self) -> bool {
        *self == PwrenEnable::PwrenEnableDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_pwren_enable_enable(&self) -> bool {
        *self == PwrenEnable::PwrenEnableEnable
    }
}
#[doc = "Field `PWREN_ENABLE` writer - Enable the power Note: For safety devices the power cannot be disabled once enabled."]
pub type PwrenEnableW<'a, REG> = crate::BitWriter<'a, REG, PwrenEnable>;
impl<'a, REG> PwrenEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn pwren_enable_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PwrenEnable::PwrenEnableDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn pwren_enable_enable(self) -> &'a mut crate::W<REG> {
        self.variant(PwrenEnable::PwrenEnableEnable)
    }
}
#[doc = "KEY to allow Power State Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrenKey {
    #[doc = "38: _TO_UNLOCK_W_"]
    PwrenKeyUnlockW = 38,
}
impl From<PwrenKey> for u8 {
    #[inline(always)]
    fn from(variant: PwrenKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrenKey {
    type Ux = u8;
}
impl crate::IsEnum for PwrenKey {}
#[doc = "Field `PWREN_KEY` writer - KEY to allow Power State Change"]
pub type PwrenKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, PwrenKey>;
impl<'a, REG> PwrenKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_TO_UNLOCK_W_"]
    #[inline(always)]
    pub fn pwren_key_unlock_w(self) -> &'a mut crate::W<REG> {
        self.variant(PwrenKey::PwrenKeyUnlockW)
    }
}
impl R {
    #[doc = "Bit 0 - Enable the power Note: For safety devices the power cannot be disabled once enabled."]
    #[inline(always)]
    pub fn pwren_enable(&self) -> PwrenEnableR {
        PwrenEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the power Note: For safety devices the power cannot be disabled once enabled."]
    #[inline(always)]
    pub fn pwren_enable(&mut self) -> PwrenEnableW<PwrenSpec> {
        PwrenEnableW::new(self, 0)
    }
    #[doc = "Bits 24:31 - KEY to allow Power State Change"]
    #[inline(always)]
    pub fn pwren_key(&mut self) -> PwrenKeyW<PwrenSpec> {
        PwrenKeyW::new(self, 24)
    }
}
#[doc = "Power enable\n\nYou can [`read`](crate::Reg::read) this register and get [`pwren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrenSpec;
impl crate::RegisterSpec for PwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwren::R`](R) reader structure"]
impl crate::Readable for PwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwren::W`](W) writer structure"]
impl crate::Writable for PwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWREN to value 0"]
impl crate::Resettable for PwrenSpec {
    const RESET_VALUE: u32 = 0;
}

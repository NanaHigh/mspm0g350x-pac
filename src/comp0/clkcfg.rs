#[doc = "Register `CLKCFG` reader"]
pub type R = crate::R<ClkcfgSpec>;
#[doc = "Register `CLKCFG` writer"]
pub type W = crate::W<ClkcfgSpec>;
#[doc = "Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkcfgBlockasync {
    #[doc = "0: DISABLE"]
    ClkcfgBlockasyncDisable = 0,
    #[doc = "1: ENABLE"]
    ClkcfgBlockasyncEnable = 1,
}
impl From<ClkcfgBlockasync> for bool {
    #[inline(always)]
    fn from(variant: ClkcfgBlockasync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKCFG_BLOCKASYNC` reader - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type ClkcfgBlockasyncR = crate::BitReader<ClkcfgBlockasync>;
impl ClkcfgBlockasyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkcfgBlockasync {
        match self.bits {
            false => ClkcfgBlockasync::ClkcfgBlockasyncDisable,
            true => ClkcfgBlockasync::ClkcfgBlockasyncEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clkcfg_blockasync_disable(&self) -> bool {
        *self == ClkcfgBlockasync::ClkcfgBlockasyncDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clkcfg_blockasync_enable(&self) -> bool {
        *self == ClkcfgBlockasync::ClkcfgBlockasyncEnable
    }
}
#[doc = "Field `CLKCFG_BLOCKASYNC` writer - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
pub type ClkcfgBlockasyncW<'a, REG> = crate::BitWriter<'a, REG, ClkcfgBlockasync>;
impl<'a, REG> ClkcfgBlockasyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clkcfg_blockasync_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgBlockasync::ClkcfgBlockasyncDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clkcfg_blockasync_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgBlockasync::ClkcfgBlockasyncEnable)
    }
}
#[doc = "KEY to Allow State Change -- 0xA9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkcfgKey {
    #[doc = "169: _UNLOCK_W_"]
    ClkcfgKeyUnlock = 169,
}
impl From<ClkcfgKey> for u8 {
    #[inline(always)]
    fn from(variant: ClkcfgKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkcfgKey {
    type Ux = u8;
}
impl crate::IsEnum for ClkcfgKey {}
#[doc = "Field `CLKCFG_KEY` writer - KEY to Allow State Change -- 0xA9"]
pub type ClkcfgKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, ClkcfgKey>;
impl<'a, REG> ClkcfgKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_UNLOCK_W_"]
    #[inline(always)]
    pub fn clkcfg_key_unlock(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgKey::ClkcfgKeyUnlock)
    }
}
impl R {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn clkcfg_blockasync(&self) -> ClkcfgBlockasyncR {
        ClkcfgBlockasyncR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Async Clock Request is blocked from starting SYSOSC or forcing bus clock to 32MHz"]
    #[inline(always)]
    pub fn clkcfg_blockasync(&mut self) -> ClkcfgBlockasyncW<ClkcfgSpec> {
        ClkcfgBlockasyncW::new(self, 8)
    }
    #[doc = "Bits 24:31 - KEY to Allow State Change -- 0xA9"]
    #[inline(always)]
    pub fn clkcfg_key(&mut self) -> ClkcfgKeyW<ClkcfgSpec> {
        ClkcfgKeyW::new(self, 24)
    }
}
#[doc = "Peripheral Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcfgSpec;
impl crate::RegisterSpec for ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg::R`](R) reader structure"]
impl crate::Readable for ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"]
impl crate::Writable for ClkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for ClkcfgSpec {
    const RESET_VALUE: u32 = 0;
}

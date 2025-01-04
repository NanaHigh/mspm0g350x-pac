#[doc = "Register `RSTCTL` writer"]
pub type W = crate::W<RstctlSpec>;
#[doc = "Assert reset to the peripheral\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstctlResetassert {
    #[doc = "0: NOP"]
    RstctlResetassertNop = 0,
    #[doc = "1: ASSERT"]
    RstctlResetassertAssert = 1,
}
impl From<RstctlResetassert> for bool {
    #[inline(always)]
    fn from(variant: RstctlResetassert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCTL_RESETASSERT` writer - Assert reset to the peripheral"]
pub type RstctlResetassertW<'a, REG> = crate::BitWriter<'a, REG, RstctlResetassert>;
impl<'a, REG> RstctlResetassertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOP"]
    #[inline(always)]
    pub fn rstctl_resetassert_nop(self) -> &'a mut crate::W<REG> {
        self.variant(RstctlResetassert::RstctlResetassertNop)
    }
    #[doc = "ASSERT"]
    #[inline(always)]
    pub fn rstctl_resetassert_assert(self) -> &'a mut crate::W<REG> {
        self.variant(RstctlResetassert::RstctlResetassertAssert)
    }
}
#[doc = "Clear the RESETSTKY bit in the STAT register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstctlResetstkyclr {
    #[doc = "0: NOP"]
    RstctlResetstkyclrNop = 0,
    #[doc = "1: CLR"]
    RstctlResetstkyclrClr = 1,
}
impl From<RstctlResetstkyclr> for bool {
    #[inline(always)]
    fn from(variant: RstctlResetstkyclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTCTL_RESETSTKYCLR` writer - Clear the RESETSTKY bit in the STAT register"]
pub type RstctlResetstkyclrW<'a, REG> = crate::BitWriter<'a, REG, RstctlResetstkyclr>;
impl<'a, REG> RstctlResetstkyclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOP"]
    #[inline(always)]
    pub fn rstctl_resetstkyclr_nop(self) -> &'a mut crate::W<REG> {
        self.variant(RstctlResetstkyclr::RstctlResetstkyclrNop)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn rstctl_resetstkyclr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(RstctlResetstkyclr::RstctlResetstkyclrClr)
    }
}
#[doc = "Unlock key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RstctlKey {
    #[doc = "177: _TO_UNLOCK_W_"]
    RstctlKeyUnlockW = 177,
}
impl From<RstctlKey> for u8 {
    #[inline(always)]
    fn from(variant: RstctlKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RstctlKey {
    type Ux = u8;
}
impl crate::IsEnum for RstctlKey {}
#[doc = "Field `RSTCTL_KEY` writer - Unlock key"]
pub type RstctlKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, RstctlKey>;
impl<'a, REG> RstctlKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_TO_UNLOCK_W_"]
    #[inline(always)]
    pub fn rstctl_key_unlock_w(self) -> &'a mut crate::W<REG> {
        self.variant(RstctlKey::RstctlKeyUnlockW)
    }
}
impl W {
    #[doc = "Bit 0 - Assert reset to the peripheral"]
    #[inline(always)]
    pub fn rstctl_resetassert(&mut self) -> RstctlResetassertW<RstctlSpec> {
        RstctlResetassertW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the RESETSTKY bit in the STAT register"]
    #[inline(always)]
    pub fn rstctl_resetstkyclr(&mut self) -> RstctlResetstkyclrW<RstctlSpec> {
        RstctlResetstkyclrW::new(self, 1)
    }
    #[doc = "Bits 24:31 - Unlock key"]
    #[inline(always)]
    pub fn rstctl_key(&mut self) -> RstctlKeyW<RstctlSpec> {
        RstctlKeyW::new(self, 24)
    }
}
#[doc = "Reset Control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlSpec;
impl crate::RegisterSpec for RstctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rstctl::W`](W) writer structure"]
impl crate::Writable for RstctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL to value 0"]
impl crate::Resettable for RstctlSpec {
    const RESET_VALUE: u32 = 0;
}

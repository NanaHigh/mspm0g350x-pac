#[doc = "Register `SHDNIOREL` writer"]
pub type W = crate::W<ShdniorelSpec>;
#[doc = "Set RELEASE to release the IO after a SHUTDOWN mode exit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShdniorelRelease {
    #[doc = "1: TRUE"]
    ShdniorelReleaseTrue = 1,
}
impl From<ShdniorelRelease> for bool {
    #[inline(always)]
    fn from(variant: ShdniorelRelease) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHDNIOREL_RELEASE` writer - Set RELEASE to release the IO after a SHUTDOWN mode exit."]
pub type ShdniorelReleaseW<'a, REG> = crate::BitWriter<'a, REG, ShdniorelRelease>;
impl<'a, REG> ShdniorelReleaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn shdniorel_release_true(self) -> &'a mut crate::W<REG> {
        self.variant(ShdniorelRelease::ShdniorelReleaseTrue)
    }
}
#[doc = "The key value 91h must be written to KEY together with RELEASE to set RELEASE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ShdniorelKey {
    #[doc = "145: VALUE"]
    ShdniorelKeyValue = 145,
}
impl From<ShdniorelKey> for u8 {
    #[inline(always)]
    fn from(variant: ShdniorelKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ShdniorelKey {
    type Ux = u8;
}
impl crate::IsEnum for ShdniorelKey {}
#[doc = "Field `SHDNIOREL_KEY` writer - The key value 91h must be written to KEY together with RELEASE to set RELEASE."]
pub type ShdniorelKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, ShdniorelKey>;
impl<'a, REG> ShdniorelKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn shdniorel_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(ShdniorelKey::ShdniorelKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set RELEASE to release the IO after a SHUTDOWN mode exit."]
    #[inline(always)]
    pub fn shdniorel_release(&mut self) -> ShdniorelReleaseW<ShdniorelSpec> {
        ShdniorelReleaseW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value 91h must be written to KEY together with RELEASE to set RELEASE."]
    #[inline(always)]
    pub fn shdniorel_key(&mut self) -> ShdniorelKeyW<ShdniorelSpec> {
        ShdniorelKeyW::new(self, 24)
    }
}
#[doc = "SHUTDOWN IO release control\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shdniorel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdniorelSpec;
impl crate::RegisterSpec for ShdniorelSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shdniorel::W`](W) writer structure"]
impl crate::Writable for ShdniorelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDNIOREL to value 0"]
impl crate::Resettable for ShdniorelSpec {
    const RESET_VALUE: u32 = 0;
}

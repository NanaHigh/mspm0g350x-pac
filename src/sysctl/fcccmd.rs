#[doc = "Register `FCCCMD` writer"]
pub type W = crate::W<FcccmdSpec>;
#[doc = "Set GO to start a capture with the frequency clock counter (FCC).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FcccmdGo {
    #[doc = "1: TRUE"]
    FcccmdGoTrue = 1,
}
impl From<FcccmdGo> for bool {
    #[inline(always)]
    fn from(variant: FcccmdGo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCCCMD_GO` writer - Set GO to start a capture with the frequency clock counter (FCC)."]
pub type FcccmdGoW<'a, REG> = crate::BitWriter<'a, REG, FcccmdGo>;
impl<'a, REG> FcccmdGoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn fcccmd_go_true(self) -> &'a mut crate::W<REG> {
        self.variant(FcccmdGo::FcccmdGoTrue)
    }
}
#[doc = "The key value 0Eh (14) must be written with GO to start a capture.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FcccmdKey {
    #[doc = "14: VALUE"]
    FcccmdKeyValue = 14,
}
impl From<FcccmdKey> for u8 {
    #[inline(always)]
    fn from(variant: FcccmdKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FcccmdKey {
    type Ux = u8;
}
impl crate::IsEnum for FcccmdKey {}
#[doc = "Field `FCCCMD_KEY` writer - The key value 0Eh (14) must be written with GO to start a capture."]
pub type FcccmdKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, FcccmdKey>;
impl<'a, REG> FcccmdKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn fcccmd_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(FcccmdKey::FcccmdKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set GO to start a capture with the frequency clock counter (FCC)."]
    #[inline(always)]
    pub fn fcccmd_go(&mut self) -> FcccmdGoW<FcccmdSpec> {
        FcccmdGoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value 0Eh (14) must be written with GO to start a capture."]
    #[inline(always)]
    pub fn fcccmd_key(&mut self) -> FcccmdKeyW<FcccmdSpec> {
        FcccmdKeyW::new(self, 24)
    }
}
#[doc = "Frequency clock counter start capture\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcccmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcccmdSpec;
impl crate::RegisterSpec for FcccmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcccmd::W`](W) writer structure"]
impl crate::Writable for FcccmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCCCMD to value 0"]
impl crate::Resettable for FcccmdSpec {
    const RESET_VALUE: u32 = 0;
}

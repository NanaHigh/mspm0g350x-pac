#[doc = "Register `RESETCMD` writer"]
pub type W = crate::W<ResetcmdSpec>;
#[doc = "Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetcmdGo {
    #[doc = "1: TRUE"]
    ResetcmdGoTrue = 1,
}
impl From<ResetcmdGo> for bool {
    #[inline(always)]
    fn from(variant: ResetcmdGo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESETCMD_GO` writer - Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY."]
pub type ResetcmdGoW<'a, REG> = crate::BitWriter<'a, REG, ResetcmdGo>;
impl<'a, REG> ResetcmdGoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn resetcmd_go_true(self) -> &'a mut crate::W<REG> {
        self.variant(ResetcmdGo::ResetcmdGoTrue)
    }
}
#[doc = "The key value of E4h (228) must be written to KEY together with GO to trigger the reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetcmdKey {
    #[doc = "228: VALUE"]
    ResetcmdKeyValue = 228,
}
impl From<ResetcmdKey> for u8 {
    #[inline(always)]
    fn from(variant: ResetcmdKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ResetcmdKey {
    type Ux = u8;
}
impl crate::IsEnum for ResetcmdKey {}
#[doc = "Field `RESETCMD_KEY` writer - The key value of E4h (228) must be written to KEY together with GO to trigger the reset."]
pub type ResetcmdKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, ResetcmdKey>;
impl<'a, REG> ResetcmdKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn resetcmd_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(ResetcmdKey::ResetcmdKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Execute the reset specified in RESETLEVEL.LEVEL. Must be written together with the KEY."]
    #[inline(always)]
    pub fn resetcmd_go(&mut self) -> ResetcmdGoW<ResetcmdSpec> {
        ResetcmdGoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value of E4h (228) must be written to KEY together with GO to trigger the reset."]
    #[inline(always)]
    pub fn resetcmd_key(&mut self) -> ResetcmdKeyW<ResetcmdSpec> {
        ResetcmdKeyW::new(self, 24)
    }
}
#[doc = "Execute an application-triggered reset command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetcmdSpec;
impl crate::RegisterSpec for ResetcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`resetcmd::W`](W) writer structure"]
impl crate::Writable for ResetcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETCMD to value 0"]
impl crate::Resettable for ResetcmdSpec {
    const RESET_VALUE: u32 = 0;
}

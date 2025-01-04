#[doc = "Register `BORCLRCMD` writer"]
pub type W = crate::W<BorclrcmdSpec>;
#[doc = "GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorclrcmdGo {
    #[doc = "1: TRUE"]
    BorclrcmdGoTrue = 1,
}
impl From<BorclrcmdGo> for bool {
    #[inline(always)]
    fn from(variant: BorclrcmdGo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BORCLRCMD_GO` writer - GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register."]
pub type BorclrcmdGoW<'a, REG> = crate::BitWriter<'a, REG, BorclrcmdGo>;
impl<'a, REG> BorclrcmdGoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn borclrcmd_go_true(self) -> &'a mut crate::W<REG> {
        self.variant(BorclrcmdGo::BorclrcmdGoTrue)
    }
}
#[doc = "The key value of C7h (199) must be written to KEY together with GO to trigger the clear and BOR threshold change.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BorclrcmdKey {
    #[doc = "199: VALUE"]
    BorclrcmdKeyValue = 199,
}
impl From<BorclrcmdKey> for u8 {
    #[inline(always)]
    fn from(variant: BorclrcmdKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BorclrcmdKey {
    type Ux = u8;
}
impl crate::IsEnum for BorclrcmdKey {}
#[doc = "Field `BORCLRCMD_KEY` writer - The key value of C7h (199) must be written to KEY together with GO to trigger the clear and BOR threshold change."]
pub type BorclrcmdKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, BorclrcmdKey>;
impl<'a, REG> BorclrcmdKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn borclrcmd_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(BorclrcmdKey::BorclrcmdKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - GO clears any prior BOR violation status indications and attempts to change the active BOR mode to that specified in the LEVEL field of the BORTHRESHOLD register."]
    #[inline(always)]
    pub fn borclrcmd_go(&mut self) -> BorclrcmdGoW<BorclrcmdSpec> {
        BorclrcmdGoW::new(self, 0)
    }
    #[doc = "Bits 24:31 - The key value of C7h (199) must be written to KEY together with GO to trigger the clear and BOR threshold change."]
    #[inline(always)]
    pub fn borclrcmd_key(&mut self) -> BorclrcmdKeyW<BorclrcmdSpec> {
        BorclrcmdKeyW::new(self, 24)
    }
}
#[doc = "Set the BOR threshold\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`borclrcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorclrcmdSpec;
impl crate::RegisterSpec for BorclrcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`borclrcmd::W`](W) writer structure"]
impl crate::Writable for BorclrcmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BORCLRCMD to value 0"]
impl crate::Resettable for BorclrcmdSpec {
    const RESET_VALUE: u32 = 0;
}

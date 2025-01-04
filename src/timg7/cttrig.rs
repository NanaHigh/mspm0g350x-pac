#[doc = "Register `CTTRIG` writer"]
pub type W = crate::W<CttrigSpec>;
#[doc = "Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CttrigTrig {
    #[doc = "0: DISABLED"]
    CttrigTrigDisabled = 0,
    #[doc = "1: GENERATE"]
    CttrigTrigGenerate = 1,
}
impl From<CttrigTrig> for bool {
    #[inline(always)]
    fn from(variant: CttrigTrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTRIG_TRIG` writer - Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance."]
pub type CttrigTrigW<'a, REG> = crate::BitWriter<'a, REG, CttrigTrig>;
impl<'a, REG> CttrigTrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn cttrig_trig_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigTrig::CttrigTrigDisabled)
    }
    #[doc = "GENERATE"]
    #[inline(always)]
    pub fn cttrig_trig_generate(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigTrig::CttrigTrigGenerate)
    }
}
impl W {
    #[doc = "Bit 0 - Generate Cross Trigger This bit when programmed will generate a synchronized trigger condition all the cross trigger enabled Timer instances including current timer instance."]
    #[inline(always)]
    pub fn cttrig_trig(&mut self) -> CttrigTrigW<CttrigSpec> {
        CttrigTrigW::new(self, 0)
    }
}
#[doc = "Timer Cross Trigger Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CttrigSpec;
impl crate::RegisterSpec for CttrigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cttrig::W`](W) writer structure"]
impl crate::Writable for CttrigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTTRIG to value 0"]
impl crate::Resettable for CttrigSpec {
    const RESET_VALUE: u32 = 0;
}

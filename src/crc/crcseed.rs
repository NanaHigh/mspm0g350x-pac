#[doc = "Register `CRCSEED` writer"]
pub type W = crate::W<CrcseedSpec>;
#[doc = "Field `CRCSEED_SEED` writer - Seed Data"]
pub type CrcseedSeedW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Seed Data"]
    #[inline(always)]
    pub fn crcseed_seed(&mut self) -> CrcseedSeedW<CrcseedSpec> {
        CrcseedSeedW::new(self, 0)
    }
}
#[doc = "CRC Seed Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcseed::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcseedSpec;
impl crate::RegisterSpec for CrcseedSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcseed::W`](W) writer structure"]
impl crate::Writable for CrcseedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCSEED to value 0"]
impl crate::Resettable for CrcseedSpec {
    const RESET_VALUE: u32 = 0;
}

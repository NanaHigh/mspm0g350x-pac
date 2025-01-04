#[doc = "Register `CRCIN` writer"]
pub type W = crate::W<CrcinSpec>;
#[doc = "Field `CRCIN_DATA` writer - Input Data"]
pub type CrcinDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn crcin_data(&mut self) -> CrcinDataW<CrcinSpec> {
        CrcinDataW::new(self, 0)
    }
}
#[doc = "CRC Input Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcinSpec;
impl crate::RegisterSpec for CrcinSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcin::W`](W) writer structure"]
impl crate::Writable for CrcinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCIN to value 0"]
impl crate::Resettable for CrcinSpec {
    const RESET_VALUE: u32 = 0;
}

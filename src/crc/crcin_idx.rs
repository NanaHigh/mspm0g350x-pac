#[doc = "Register `CRCIN_IDX[%s]` writer"]
pub type W = crate::W<CrcinIdxSpec>;
#[doc = "Field `CRCIN_IDX_DATA` writer - Input Data"]
pub type CrcinIdxDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn crcin_idx_data(&mut self) -> CrcinIdxDataW<CrcinIdxSpec> {
        CrcinIdxDataW::new(self, 0)
    }
}
#[doc = "CRC Input Data Array Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcin_idx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcinIdxSpec;
impl crate::RegisterSpec for CrcinIdxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crcin_idx::W`](W) writer structure"]
impl crate::Writable for CrcinIdxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRCIN_IDX[%s]
to value 0"]
impl crate::Resettable for CrcinIdxSpec {
    const RESET_VALUE: u32 = 0;
}

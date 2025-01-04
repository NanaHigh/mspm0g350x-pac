#[doc = "Register `MCANERR_ERR_STAT2` reader"]
pub type R = crate::R<McanerrErrStat2Spec>;
#[doc = "Field `MCANERR_ERR_STAT2_ECC_ROW` reader - Indicates the row address where the single or double-bit error occurred. This value is address offset/4."]
pub type McanerrErrStat2EccRowR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Indicates the row address where the single or double-bit error occurred. This value is address offset/4."]
    #[inline(always)]
    pub fn mcanerr_err_stat2_ecc_row(&self) -> McanerrErrStat2EccRowR {
        McanerrErrStat2EccRowR::new(self.bits)
    }
}
#[doc = "MCAN ECC Error Status 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_stat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrErrStat2Spec;
impl crate::RegisterSpec for McanerrErrStat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_err_stat2::R`](R) reader structure"]
impl crate::Readable for McanerrErrStat2Spec {}
#[doc = "`reset()` method sets MCANERR_ERR_STAT2 to value 0"]
impl crate::Resettable for McanerrErrStat2Spec {
    const RESET_VALUE: u32 = 0;
}

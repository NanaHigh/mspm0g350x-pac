#[doc = "Register `MCANERR_ERR_CTRL1` reader"]
pub type R = crate::R<McanerrErrCtrl1Spec>;
#[doc = "Register `MCANERR_ERR_CTRL1` writer"]
pub type W = crate::W<McanerrErrCtrl1Spec>;
#[doc = "Field `MCANERR_ERR_CTRL1_ECC_ROW` reader - Row address where FORCE_SEC or FORCE_DED needs to be applied. This is ignored if FORCE_N_ROW is set."]
pub type McanerrErrCtrl1EccRowR = crate::FieldReader<u32>;
#[doc = "Field `MCANERR_ERR_CTRL1_ECC_ROW` writer - Row address where FORCE_SEC or FORCE_DED needs to be applied. This is ignored if FORCE_N_ROW is set."]
pub type McanerrErrCtrl1EccRowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Row address where FORCE_SEC or FORCE_DED needs to be applied. This is ignored if FORCE_N_ROW is set."]
    #[inline(always)]
    pub fn mcanerr_err_ctrl1_ecc_row(&self) -> McanerrErrCtrl1EccRowR {
        McanerrErrCtrl1EccRowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Row address where FORCE_SEC or FORCE_DED needs to be applied. This is ignored if FORCE_N_ROW is set."]
    #[inline(always)]
    pub fn mcanerr_err_ctrl1_ecc_row(&mut self) -> McanerrErrCtrl1EccRowW<McanerrErrCtrl1Spec> {
        McanerrErrCtrl1EccRowW::new(self, 0)
    }
}
#[doc = "MCAN ECC Error Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrErrCtrl1Spec;
impl crate::RegisterSpec for McanerrErrCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_err_ctrl1::R`](R) reader structure"]
impl crate::Readable for McanerrErrCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_err_ctrl1::W`](W) writer structure"]
impl crate::Writable for McanerrErrCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_ERR_CTRL1 to value 0"]
impl crate::Resettable for McanerrErrCtrl1Spec {
    const RESET_VALUE: u32 = 0;
}

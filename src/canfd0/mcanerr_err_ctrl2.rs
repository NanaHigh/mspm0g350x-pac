#[doc = "Register `MCANERR_ERR_CTRL2` reader"]
pub type R = crate::R<McanerrErrCtrl2Spec>;
#[doc = "Register `MCANERR_ERR_CTRL2` writer"]
pub type W = crate::W<McanerrErrCtrl2Spec>;
#[doc = "Field `MCANERR_ERR_CTRL2_ECC_BIT1` reader - Column/Data bit that needs to be flipped when FORCE_SEC or FORCE_DED is set"]
pub type McanerrErrCtrl2EccBit1R = crate::FieldReader<u16>;
#[doc = "Field `MCANERR_ERR_CTRL2_ECC_BIT1` writer - Column/Data bit that needs to be flipped when FORCE_SEC or FORCE_DED is set"]
pub type McanerrErrCtrl2EccBit1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MCANERR_ERR_CTRL2_ECC_BIT2` reader - Second column/data bit that needs to be flipped when FORCE_DED is set"]
pub type McanerrErrCtrl2EccBit2R = crate::FieldReader<u16>;
#[doc = "Field `MCANERR_ERR_CTRL2_ECC_BIT2` writer - Second column/data bit that needs to be flipped when FORCE_DED is set"]
pub type McanerrErrCtrl2EccBit2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Column/Data bit that needs to be flipped when FORCE_SEC or FORCE_DED is set"]
    #[inline(always)]
    pub fn mcanerr_err_ctrl2_ecc_bit1(&self) -> McanerrErrCtrl2EccBit1R {
        McanerrErrCtrl2EccBit1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Second column/data bit that needs to be flipped when FORCE_DED is set"]
    #[inline(always)]
    pub fn mcanerr_err_ctrl2_ecc_bit2(&self) -> McanerrErrCtrl2EccBit2R {
        McanerrErrCtrl2EccBit2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Column/Data bit that needs to be flipped when FORCE_SEC or FORCE_DED is set"]
    #[inline(always)]
    pub fn mcanerr_err_ctrl2_ecc_bit1(&mut self) -> McanerrErrCtrl2EccBit1W<McanerrErrCtrl2Spec> {
        McanerrErrCtrl2EccBit1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Second column/data bit that needs to be flipped when FORCE_DED is set"]
    #[inline(always)]
    pub fn mcanerr_err_ctrl2_ecc_bit2(&mut self) -> McanerrErrCtrl2EccBit2W<McanerrErrCtrl2Spec> {
        McanerrErrCtrl2EccBit2W::new(self, 16)
    }
}
#[doc = "MCAN ECC Error Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_err_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_err_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrErrCtrl2Spec;
impl crate::RegisterSpec for McanerrErrCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_err_ctrl2::R`](R) reader structure"]
impl crate::Readable for McanerrErrCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_err_ctrl2::W`](W) writer structure"]
impl crate::Writable for McanerrErrCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_ERR_CTRL2 to value 0"]
impl crate::Resettable for McanerrErrCtrl2Spec {
    const RESET_VALUE: u32 = 0;
}

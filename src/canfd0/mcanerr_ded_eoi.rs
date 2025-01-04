#[doc = "Register `MCANERR_DED_EOI` reader"]
pub type R = crate::R<McanerrDedEoiSpec>;
#[doc = "Register `MCANERR_DED_EOI` writer"]
pub type W = crate::W<McanerrDedEoiSpec>;
#[doc = "Field `MCANERR_DED_EOI_EOI_WR` reader - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_DED goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
pub type McanerrDedEoiEoiWrR = crate::BitReader;
#[doc = "Field `MCANERR_DED_EOI_EOI_WR` writer - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_DED goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
pub type McanerrDedEoiEoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_DED goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
    #[inline(always)]
    pub fn mcanerr_ded_eoi_eoi_wr(&self) -> McanerrDedEoiEoiWrR {
        McanerrDedEoiEoiWrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_DED goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
    #[inline(always)]
    pub fn mcanerr_ded_eoi_eoi_wr(&mut self) -> McanerrDedEoiEoiWrW<McanerrDedEoiSpec> {
        McanerrDedEoiEoiWrW::new(self, 0)
    }
}
#[doc = "MCAN Double Error Detected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrDedEoiSpec;
impl crate::RegisterSpec for McanerrDedEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_ded_eoi::R`](R) reader structure"]
impl crate::Readable for McanerrDedEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_ded_eoi::W`](W) writer structure"]
impl crate::Writable for McanerrDedEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_DED_EOI to value 0"]
impl crate::Resettable for McanerrDedEoiSpec {
    const RESET_VALUE: u32 = 0;
}

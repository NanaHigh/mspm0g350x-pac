#[doc = "Register `MCANERR_SEC_EOI` reader"]
pub type R = crate::R<McanerrSecEoiSpec>;
#[doc = "Register `MCANERR_SEC_EOI` writer"]
pub type W = crate::W<McanerrSecEoiSpec>;
#[doc = "Field `MCANERR_SEC_EOI_EOI_WR` reader - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_SEC goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
pub type McanerrSecEoiEoiWrR = crate::BitReader;
#[doc = "Field `MCANERR_SEC_EOI_EOI_WR` writer - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_SEC goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
pub type McanerrSecEoiEoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_SEC goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
    #[inline(always)]
    pub fn mcanerr_sec_eoi_eoi_wr(&self) -> McanerrSecEoiEoiWrR {
        McanerrSecEoiEoiWrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write to this register indicates that software has acknowledged the pending interrupt and the next interrupt can be sent to the host. Note that a write to the MCANERR_ERR_STAT1.CLR_ECC_SEC goes through the SVBUS and has a delayed completion. To avoid an additional interrupt, read the MCANERR_ERR_STAT1 register back prior to writing to this bit field."]
    #[inline(always)]
    pub fn mcanerr_sec_eoi_eoi_wr(&mut self) -> McanerrSecEoiEoiWrW<McanerrSecEoiSpec> {
        McanerrSecEoiEoiWrW::new(self, 0)
    }
}
#[doc = "MCAN Single Error Corrected End of Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_eoi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_eoi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrSecEoiSpec;
impl crate::RegisterSpec for McanerrSecEoiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_sec_eoi::R`](R) reader structure"]
impl crate::Readable for McanerrSecEoiSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_sec_eoi::W`](W) writer structure"]
impl crate::Writable for McanerrSecEoiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_SEC_EOI to value 0"]
impl crate::Resettable for McanerrSecEoiSpec {
    const RESET_VALUE: u32 = 0;
}

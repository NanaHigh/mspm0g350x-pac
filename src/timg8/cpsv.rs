#[doc = "Register `CPSV` reader"]
pub type R = crate::R<CpsvSpec>;
#[doc = "Field `CPSV_CPSVAL` reader - Current Prescale Count Value"]
pub type CpsvCpsvalR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Current Prescale Count Value"]
    #[inline(always)]
    pub fn cpsv_cpsval(&self) -> CpsvCpsvalR {
        CpsvCpsvalR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Clock prescale count status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsvSpec;
impl crate::RegisterSpec for CpsvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsv::R`](R) reader structure"]
impl crate::Readable for CpsvSpec {}
#[doc = "`reset()` method sets CPSV to value 0"]
impl crate::Resettable for CpsvSpec {
    const RESET_VALUE: u32 = 0;
}

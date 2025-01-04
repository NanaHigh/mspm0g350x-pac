#[doc = "Register `FIFODATA` reader"]
pub type R = crate::R<FifodataSpec>;
#[doc = "Field `FIFODATA_DATA` reader - Read from this data field returns the ADC sample from FIFO."]
pub type FifodataDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read from this data field returns the ADC sample from FIFO."]
    #[inline(always)]
    pub fn fifodata_data(&self) -> FifodataDataR {
        FifodataDataR::new(self.bits)
    }
}
#[doc = "FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodataSpec;
impl crate::RegisterSpec for FifodataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodata::R`](R) reader structure"]
impl crate::Readable for FifodataSpec {}
#[doc = "`reset()` method sets FIFODATA to value 0"]
impl crate::Resettable for FifodataSpec {
    const RESET_VALUE: u32 = 0;
}

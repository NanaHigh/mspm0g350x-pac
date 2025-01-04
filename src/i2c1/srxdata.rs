#[doc = "Register `SRXDATA` reader"]
pub type R = crate::R<SrxdataSpec>;
#[doc = "Field `SRXDATA_VALUE` reader - Received Data. This field contains the last received data."]
pub type SrxdataValueR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Data. This field contains the last received data."]
    #[inline(always)]
    pub fn srxdata_value(&self) -> SrxdataValueR {
        SrxdataValueR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Slave RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`srxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrxdataSpec;
impl crate::RegisterSpec for SrxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srxdata::R`](R) reader structure"]
impl crate::Readable for SrxdataSpec {}
#[doc = "`reset()` method sets SRXDATA to value 0"]
impl crate::Resettable for SrxdataSpec {
    const RESET_VALUE: u32 = 0;
}

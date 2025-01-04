#[doc = "Register `MRXDATA` reader"]
pub type R = crate::R<MrxdataSpec>;
#[doc = "Field `MRXDATA_VALUE` reader - Received Data. This field contains the last received data."]
pub type MrxdataValueR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Data. This field contains the last received data."]
    #[inline(always)]
    pub fn mrxdata_value(&self) -> MrxdataValueR {
        MrxdataValueR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Master RXData\n\nYou can [`read`](crate::Reg::read) this register and get [`mrxdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrxdataSpec;
impl crate::RegisterSpec for MrxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrxdata::R`](R) reader structure"]
impl crate::Readable for MrxdataSpec {}
#[doc = "`reset()` method sets MRXDATA to value 0"]
impl crate::Resettable for MrxdataSpec {
    const RESET_VALUE: u32 = 0;
}

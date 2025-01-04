#[doc = "Register `CRCOUT` reader"]
pub type R = crate::R<CrcoutSpec>;
#[doc = "Field `CRCOUT_RESULT` reader - Result"]
pub type CrcoutResultR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Result"]
    #[inline(always)]
    pub fn crcout_result(&self) -> CrcoutResultR {
        CrcoutResultR::new(self.bits)
    }
}
#[doc = "CRC Output Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcoutSpec;
impl crate::RegisterSpec for CrcoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcout::R`](R) reader structure"]
impl crate::Readable for CrcoutSpec {}
#[doc = "`reset()` method sets CRCOUT to value 0"]
impl crate::Resettable for CrcoutSpec {
    const RESET_VALUE: u32 = 0;
}

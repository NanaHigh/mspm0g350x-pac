#[doc = "Register `CALDATA` reader"]
pub type R = crate::R<CaldataSpec>;
#[doc = "Field `CALDATA_DATA` reader - DAC offset error calibration data. The DAC offset error calibration data is represented in twos complement format providing a range of 64 to +63. This is read-only bit, reflecting the calibration data. Writing to this register will have no effect, it will not change the calibration value."]
pub type CaldataDataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - DAC offset error calibration data. The DAC offset error calibration data is represented in twos complement format providing a range of 64 to +63. This is read-only bit, reflecting the calibration data. Writing to this register will have no effect, it will not change the calibration value."]
    #[inline(always)]
    pub fn caldata_data(&self) -> CaldataDataR {
        CaldataDataR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Calibration data\n\nYou can [`read`](crate::Reg::read) this register and get [`caldata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaldataSpec;
impl crate::RegisterSpec for CaldataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caldata::R`](R) reader structure"]
impl crate::Readable for CaldataSpec {}
#[doc = "`reset()` method sets CALDATA to value 0"]
impl crate::Resettable for CaldataSpec {
    const RESET_VALUE: u32 = 0;
}

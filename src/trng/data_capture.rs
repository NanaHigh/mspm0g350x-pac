#[doc = "Register `DATA_CAPTURE` reader"]
pub type R = crate::R<DataCaptureSpec>;
#[doc = "Field `DATA_CAPTURE_BUFFER` reader - Captured Data from the Decimation Block"]
pub type DataCaptureBufferR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Captured Data from the Decimation Block"]
    #[inline(always)]
    pub fn data_capture_buffer(&self) -> DataCaptureBufferR {
        DataCaptureBufferR::new(self.bits)
    }
}
#[doc = "Captured word buffer of RNG data\n\nYou can [`read`](crate::Reg::read) this register and get [`data_capture::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataCaptureSpec;
impl crate::RegisterSpec for DataCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_capture::R`](R) reader structure"]
impl crate::Readable for DataCaptureSpec {}
#[doc = "`reset()` method sets DATA_CAPTURE to value 0"]
impl crate::Resettable for DataCaptureSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FCC` reader"]
pub type R = crate::R<FccSpec>;
#[doc = "Field `FCC_DATA` reader - Frequency clock counter (FCC) count value."]
pub type FccDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:21 - Frequency clock counter (FCC) count value."]
    #[inline(always)]
    pub fn fcc_data(&self) -> FccDataR {
        FccDataR::new(self.bits & 0x003f_ffff)
    }
}
#[doc = "Frequency clock counter (FCC) count\n\nYou can [`read`](crate::Reg::read) this register and get [`fcc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FccSpec;
impl crate::RegisterSpec for FccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcc::R`](R) reader structure"]
impl crate::Readable for FccSpec {}
#[doc = "`reset()` method sets FCC to value 0"]
impl crate::Resettable for FccSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MEMRES[%s]` reader"]
pub type R = crate::R<MemresSpec>;
#[doc = "Field `MEMRES_DATA` reader - MEMRES result register. If DF = 0, unsigned binary: The conversion results are right aligned. In 10 and 8 bit modes, the unused MSB bits are forced to 0. If DF = 1, 2s-complement format: The conversion results are left aligned. In 10 and 8 bit modes, the unused LSB bits are forced to 0. The data is stored in the right-justified format and is converted to the left-justified 2s-complement format during read back."]
pub type MemresDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - MEMRES result register. If DF = 0, unsigned binary: The conversion results are right aligned. In 10 and 8 bit modes, the unused MSB bits are forced to 0. If DF = 1, 2s-complement format: The conversion results are left aligned. In 10 and 8 bit modes, the unused LSB bits are forced to 0. The data is stored in the right-justified format and is converted to the left-justified 2s-complement format during read back."]
    #[inline(always)]
    pub fn memres_data(&self) -> MemresDataR {
        MemresDataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Memory Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memres::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemresSpec;
impl crate::RegisterSpec for MemresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memres::R`](R) reader structure"]
impl crate::Readable for MemresSpec {}
#[doc = "`reset()` method sets MEMRES[%s]
to value 0"]
impl crate::Resettable for MemresSpec {
    const RESET_VALUE: u32 = 0;
}

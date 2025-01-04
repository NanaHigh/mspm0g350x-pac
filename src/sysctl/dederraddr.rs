#[doc = "Register `DEDERRADDR` reader"]
pub type R = crate::R<DederraddrSpec>;
#[doc = "Field `DEDERRADDR_ADDR` reader - Address of MEMORY DED Error."]
pub type DederraddrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of MEMORY DED Error."]
    #[inline(always)]
    pub fn dederraddr_addr(&self) -> DederraddrAddrR {
        DederraddrAddrR::new(self.bits)
    }
}
#[doc = "Memory DED Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dederraddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DederraddrSpec;
impl crate::RegisterSpec for DederraddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dederraddr::R`](R) reader structure"]
impl crate::Readable for DederraddrSpec {}
#[doc = "`reset()` method sets DEDERRADDR to value 0"]
impl crate::Resettable for DederraddrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MCAN_ENDN` reader"]
pub type R = crate::R<McanEndnSpec>;
#[doc = "Field `MCAN_ENDN_ETV` reader - Endianess Test Value. Reading the constant value maintained in this register allows software to determine the endianess of the host CPU."]
pub type McanEndnEtvR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endianess Test Value. Reading the constant value maintained in this register allows software to determine the endianess of the host CPU."]
    #[inline(always)]
    pub fn mcan_endn_etv(&self) -> McanEndnEtvR {
        McanEndnEtvR::new(self.bits)
    }
}
#[doc = "MCAN Endian Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_endn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanEndnSpec;
impl crate::RegisterSpec for McanEndnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_endn::R`](R) reader structure"]
impl crate::Readable for McanEndnSpec {}
#[doc = "`reset()` method sets MCAN_ENDN to value 0x8765_4321"]
impl crate::Resettable for McanEndnSpec {
    const RESET_VALUE: u32 = 0x8765_4321;
}

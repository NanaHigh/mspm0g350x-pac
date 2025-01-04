#[doc = "Register `SRAMBOUNDARY` reader"]
pub type R = crate::R<SramboundarySpec>;
#[doc = "Register `SRAMBOUNDARY` writer"]
pub type W = crate::W<SramboundarySpec>;
#[doc = "Field `SRAMBOUNDARY_ADDR` reader - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
pub type SramboundaryAddrR = crate::FieldReader<u16>;
#[doc = "Field `SRAMBOUNDARY_ADDR` writer - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
pub type SramboundaryAddrW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 5:19 - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
    #[inline(always)]
    pub fn sramboundary_addr(&self) -> SramboundaryAddrR {
        SramboundaryAddrR::new(((self.bits >> 5) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 5:19 - SRAM boundary configuration. The value configured into this acts such that: SRAM accesses to addresses less than or equal value will be RW only. SRAM accesses to addresses greater than value will be RX only. Value of 0 is not valid (system will have no stack). If set to 0, the system acts as if the entire SRAM is RWX. Any non-zero value can be configured, including a value = SRAM size."]
    #[inline(always)]
    pub fn sramboundary_addr(&mut self) -> SramboundaryAddrW<SramboundarySpec> {
        SramboundaryAddrW::new(self, 5)
    }
}
#[doc = "SRAM Write Boundary\n\nYou can [`read`](crate::Reg::read) this register and get [`sramboundary::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramboundary::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramboundarySpec;
impl crate::RegisterSpec for SramboundarySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sramboundary::R`](R) reader structure"]
impl crate::Readable for SramboundarySpec {}
#[doc = "`write(|w| ..)` method takes [`sramboundary::W`](W) writer structure"]
impl crate::Writable for SramboundarySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAMBOUNDARY to value 0"]
impl crate::Resettable for SramboundarySpec {
    const RESET_VALUE: u32 = 0;
}

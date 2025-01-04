#[doc = "Register `DMADA` reader"]
pub type R = crate::R<DmadaSpec>;
#[doc = "Register `DMADA` writer"]
pub type W = crate::W<DmadaSpec>;
#[doc = "Field `DMADA_ADDR` reader - DMA Channel Destination Address"]
pub type DmadaAddrR = crate::FieldReader<u32>;
#[doc = "Field `DMADA_ADDR` writer - DMA Channel Destination Address"]
pub type DmadaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Channel Destination Address"]
    #[inline(always)]
    pub fn dmada_addr(&self) -> DmadaAddrR {
        DmadaAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Channel Destination Address"]
    #[inline(always)]
    pub fn dmada_addr(&mut self) -> DmadaAddrW<DmadaSpec> {
        DmadaAddrW::new(self, 0)
    }
}
#[doc = "DMA Channel Destination Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmada::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmada::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmadaSpec;
impl crate::RegisterSpec for DmadaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmada::R`](R) reader structure"]
impl crate::Readable for DmadaSpec {}
#[doc = "`write(|w| ..)` method takes [`dmada::W`](W) writer structure"]
impl crate::Writable for DmadaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMADA to value 0"]
impl crate::Resettable for DmadaSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `DMASA` reader"]
pub type R = crate::R<DmasaSpec>;
#[doc = "Register `DMASA` writer"]
pub type W = crate::W<DmasaSpec>;
#[doc = "Field `DMASA_ADDR` reader - DMA Channel Source Address"]
pub type DmasaAddrR = crate::FieldReader<u32>;
#[doc = "Field `DMASA_ADDR` writer - DMA Channel Source Address"]
pub type DmasaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Channel Source Address"]
    #[inline(always)]
    pub fn dmasa_addr(&self) -> DmasaAddrR {
        DmasaAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Channel Source Address"]
    #[inline(always)]
    pub fn dmasa_addr(&mut self) -> DmasaAddrW<DmasaSpec> {
        DmasaAddrW::new(self, 0)
    }
}
#[doc = "DMA Channel Source Address\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmasaSpec;
impl crate::RegisterSpec for DmasaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasa::R`](R) reader structure"]
impl crate::Readable for DmasaSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasa::W`](W) writer structure"]
impl crate::Writable for DmasaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASA to value 0"]
impl crate::Resettable for DmasaSpec {
    const RESET_VALUE: u32 = 0;
}

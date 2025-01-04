#[doc = "Register `DMASZ` reader"]
pub type R = crate::R<DmaszSpec>;
#[doc = "Register `DMASZ` writer"]
pub type W = crate::W<DmaszSpec>;
#[doc = "Field `DMASZ_SIZE` reader - DMA Channel Size in number of transfers"]
pub type DmaszSizeR = crate::FieldReader<u16>;
#[doc = "Field `DMASZ_SIZE` writer - DMA Channel Size in number of transfers"]
pub type DmaszSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA Channel Size in number of transfers"]
    #[inline(always)]
    pub fn dmasz_size(&self) -> DmaszSizeR {
        DmaszSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA Channel Size in number of transfers"]
    #[inline(always)]
    pub fn dmasz_size(&mut self) -> DmaszSizeW<DmaszSpec> {
        DmaszSizeW::new(self, 0)
    }
}
#[doc = "DMA Channel Size\n\nYou can [`read`](crate::Reg::read) this register and get [`dmasz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmasz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaszSpec;
impl crate::RegisterSpec for DmaszSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasz::R`](R) reader structure"]
impl crate::Readable for DmaszSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasz::W`](W) writer structure"]
impl crate::Writable for DmaszSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASZ to value 0"]
impl crate::Resettable for DmaszSpec {
    const RESET_VALUE: u32 = 0;
}

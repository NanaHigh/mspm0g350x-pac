#[doc = "Register `FBRD` reader"]
pub type R = crate::R<FbrdSpec>;
#[doc = "Register `FBRD` writer"]
pub type W = crate::W<FbrdSpec>;
#[doc = "Field `FBRD_DIVFRAC` reader - Fractional Baud-Rate Divisor"]
pub type FbrdDivfracR = crate::FieldReader;
#[doc = "Field `FBRD_DIVFRAC` writer - Fractional Baud-Rate Divisor"]
pub type FbrdDivfracW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn fbrd_divfrac(&self) -> FbrdDivfracR {
        FbrdDivfracR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional Baud-Rate Divisor"]
    #[inline(always)]
    pub fn fbrd_divfrac(&mut self) -> FbrdDivfracW<FbrdSpec> {
        FbrdDivfracW::new(self, 0)
    }
}
#[doc = "UART Fractional Baud-Rate Divisor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fbrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbrdSpec;
impl crate::RegisterSpec for FbrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbrd::R`](R) reader structure"]
impl crate::Readable for FbrdSpec {}
#[doc = "`write(|w| ..)` method takes [`fbrd::W`](W) writer structure"]
impl crate::Writable for FbrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FbrdSpec {
    const RESET_VALUE: u32 = 0;
}

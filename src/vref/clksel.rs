#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<ClkselSpec>;
#[doc = "Field `CLKSEL_LFCLK_SEL` reader - Selects LFCLK as clock source if enabled"]
pub type ClkselLfclkSelR = crate::BitReader;
#[doc = "Field `CLKSEL_LFCLK_SEL` writer - Selects LFCLK as clock source if enabled"]
pub type ClkselLfclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL_MFCLK_SEL` reader - Selects MFCLK as clock source if enabled"]
pub type ClkselMfclkSelR = crate::BitReader;
#[doc = "Field `CLKSEL_MFCLK_SEL` writer - Selects MFCLK as clock source if enabled"]
pub type ClkselMfclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL_BUSCLK_SEL` reader - Selects BUSCLK as clock source if enabled"]
pub type ClkselBusclkSelR = crate::BitReader;
#[doc = "Field `CLKSEL_BUSCLK_SEL` writer - Selects BUSCLK as clock source if enabled"]
pub type ClkselBusclkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_lfclk_sel(&self) -> ClkselLfclkSelR {
        ClkselLfclkSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects MFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_mfclk_sel(&self) -> ClkselMfclkSelR {
        ClkselMfclkSelR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects BUSCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_busclk_sel(&self) -> ClkselBusclkSelR {
        ClkselBusclkSelR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_lfclk_sel(&mut self) -> ClkselLfclkSelW<ClkselSpec> {
        ClkselLfclkSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Selects MFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_mfclk_sel(&mut self) -> ClkselMfclkSelW<ClkselSpec> {
        ClkselMfclkSelW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects BUSCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_busclk_sel(&mut self) -> ClkselBusclkSelW<ClkselSpec> {
        ClkselBusclkSelW::new(self, 3)
    }
}
#[doc = "Clock Selection\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselSpec;
impl crate::RegisterSpec for ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for ClkselSpec {}
#[doc = "`write(|w| ..)` method takes [`clksel::W`](W) writer structure"]
impl crate::Writable for ClkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for ClkselSpec {
    const RESET_VALUE: u32 = 0;
}

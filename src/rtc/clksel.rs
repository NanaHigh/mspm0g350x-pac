#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Selects LFCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkselLfclkSel {
    #[doc = "0: DISABLE"]
    ClkselLfclkSelDisable = 0,
    #[doc = "1: ENABLE"]
    ClkselLfclkSelEnable = 1,
}
impl From<ClkselLfclkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkselLfclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL_LFCLK_SEL` reader - Selects LFCLK as clock source if enabled"]
pub type ClkselLfclkSelR = crate::BitReader<ClkselLfclkSel>;
impl ClkselLfclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkselLfclkSel {
        match self.bits {
            false => ClkselLfclkSel::ClkselLfclkSelDisable,
            true => ClkselLfclkSel::ClkselLfclkSelEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clksel_lfclk_sel_disable(&self) -> bool {
        *self == ClkselLfclkSel::ClkselLfclkSelDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clksel_lfclk_sel_enable(&self) -> bool {
        *self == ClkselLfclkSel::ClkselLfclkSelEnable
    }
}
impl R {
    #[doc = "Bit 1 - Selects LFCLK as clock source if enabled"]
    #[inline(always)]
    pub fn clksel_lfclk_sel(&self) -> ClkselLfclkSelR {
        ClkselLfclkSelR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkselSpec;
impl crate::RegisterSpec for ClkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksel::R`](R) reader structure"]
impl crate::Readable for ClkselSpec {}
#[doc = "`reset()` method sets CLKSEL to value 0"]
impl crate::Resettable for ClkselSpec {
    const RESET_VALUE: u32 = 0;
}

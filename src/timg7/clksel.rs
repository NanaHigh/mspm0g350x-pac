#[doc = "Register `CLKSEL` reader"]
pub type R = crate::R<ClkselSpec>;
#[doc = "Register `CLKSEL` writer"]
pub type W = crate::W<ClkselSpec>;
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
#[doc = "Field `CLKSEL_LFCLK_SEL` writer - Selects LFCLK as clock source if enabled"]
pub type ClkselLfclkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkselLfclkSel>;
impl<'a, REG> ClkselLfclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clksel_lfclk_sel_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkselLfclkSel::ClkselLfclkSelDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clksel_lfclk_sel_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkselLfclkSel::ClkselLfclkSelEnable)
    }
}
#[doc = "Selects MFCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkselMfclkSel {
    #[doc = "0: DISABLE"]
    ClkselMfclkSelDisable = 0,
    #[doc = "1: ENABLE"]
    ClkselMfclkSelEnable = 1,
}
impl From<ClkselMfclkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkselMfclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL_MFCLK_SEL` reader - Selects MFCLK as clock source if enabled"]
pub type ClkselMfclkSelR = crate::BitReader<ClkselMfclkSel>;
impl ClkselMfclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkselMfclkSel {
        match self.bits {
            false => ClkselMfclkSel::ClkselMfclkSelDisable,
            true => ClkselMfclkSel::ClkselMfclkSelEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clksel_mfclk_sel_disable(&self) -> bool {
        *self == ClkselMfclkSel::ClkselMfclkSelDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clksel_mfclk_sel_enable(&self) -> bool {
        *self == ClkselMfclkSel::ClkselMfclkSelEnable
    }
}
#[doc = "Field `CLKSEL_MFCLK_SEL` writer - Selects MFCLK as clock source if enabled"]
pub type ClkselMfclkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkselMfclkSel>;
impl<'a, REG> ClkselMfclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clksel_mfclk_sel_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkselMfclkSel::ClkselMfclkSelDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clksel_mfclk_sel_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkselMfclkSel::ClkselMfclkSelEnable)
    }
}
#[doc = "Selects BUSCLK as clock source if enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkselBusclkSel {
    #[doc = "0: DISABLE"]
    ClkselBusclkSelDisable = 0,
    #[doc = "1: ENABLE"]
    ClkselBusclkSelEnable = 1,
}
impl From<ClkselBusclkSel> for bool {
    #[inline(always)]
    fn from(variant: ClkselBusclkSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL_BUSCLK_SEL` reader - Selects BUSCLK as clock source if enabled"]
pub type ClkselBusclkSelR = crate::BitReader<ClkselBusclkSel>;
impl ClkselBusclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkselBusclkSel {
        match self.bits {
            false => ClkselBusclkSel::ClkselBusclkSelDisable,
            true => ClkselBusclkSel::ClkselBusclkSelEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clksel_busclk_sel_disable(&self) -> bool {
        *self == ClkselBusclkSel::ClkselBusclkSelDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clksel_busclk_sel_enable(&self) -> bool {
        *self == ClkselBusclkSel::ClkselBusclkSelEnable
    }
}
#[doc = "Field `CLKSEL_BUSCLK_SEL` writer - Selects BUSCLK as clock source if enabled"]
pub type ClkselBusclkSelW<'a, REG> = crate::BitWriter<'a, REG, ClkselBusclkSel>;
impl<'a, REG> ClkselBusclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clksel_busclk_sel_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkselBusclkSel::ClkselBusclkSelDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clksel_busclk_sel_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkselBusclkSel::ClkselBusclkSelEnable)
    }
}
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
#[doc = "Clock Select for Ultra Low Power peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`clksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

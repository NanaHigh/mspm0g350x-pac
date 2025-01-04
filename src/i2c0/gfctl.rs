#[doc = "Register `GFCTL` reader"]
pub type R = crate::R<GfctlSpec>;
#[doc = "Register `GFCTL` writer"]
pub type W = crate::W<GfctlSpec>;
#[doc = "Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GfctlDgfsel {
    #[doc = "0: DISABLED"]
    GfctlDgfselDisabled = 0,
    #[doc = "1: CLK_1"]
    GfctlDgfselClk1 = 1,
    #[doc = "2: CLK_2"]
    GfctlDgfselClk2 = 2,
    #[doc = "3: CLK_3"]
    GfctlDgfselClk3 = 3,
    #[doc = "4: CLK_4"]
    GfctlDgfselClk4 = 4,
    #[doc = "5: CLK_8"]
    GfctlDgfselClk8 = 5,
    #[doc = "6: CLK_16"]
    GfctlDgfselClk16 = 6,
    #[doc = "7: CLK_31"]
    GfctlDgfselClk31 = 7,
}
impl From<GfctlDgfsel> for u8 {
    #[inline(always)]
    fn from(variant: GfctlDgfsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GfctlDgfsel {
    type Ux = u8;
}
impl crate::IsEnum for GfctlDgfsel {}
#[doc = "Field `GFCTL_DGFSEL` reader - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
pub type GfctlDgfselR = crate::FieldReader<GfctlDgfsel>;
impl GfctlDgfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GfctlDgfsel {
        match self.bits {
            0 => GfctlDgfsel::GfctlDgfselDisabled,
            1 => GfctlDgfsel::GfctlDgfselClk1,
            2 => GfctlDgfsel::GfctlDgfselClk2,
            3 => GfctlDgfsel::GfctlDgfselClk3,
            4 => GfctlDgfsel::GfctlDgfselClk4,
            5 => GfctlDgfsel::GfctlDgfselClk8,
            6 => GfctlDgfsel::GfctlDgfselClk16,
            7 => GfctlDgfsel::GfctlDgfselClk31,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_disabled(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselDisabled
    }
    #[doc = "CLK_1"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_1(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk1
    }
    #[doc = "CLK_2"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_2(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk2
    }
    #[doc = "CLK_3"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_3(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk3
    }
    #[doc = "CLK_4"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_4(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk4
    }
    #[doc = "CLK_8"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_8(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk8
    }
    #[doc = "CLK_16"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_16(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk16
    }
    #[doc = "CLK_31"]
    #[inline(always)]
    pub fn is_gfctl_dgfsel_clk_31(&self) -> bool {
        *self == GfctlDgfsel::GfctlDgfselClk31
    }
}
#[doc = "Field `GFCTL_DGFSEL` writer - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
pub type GfctlDgfselW<'a, REG> = crate::FieldWriter<'a, REG, 3, GfctlDgfsel, crate::Safe>;
impl<'a, REG> GfctlDgfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn gfctl_dgfsel_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselDisabled)
    }
    #[doc = "CLK_1"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_1(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk1)
    }
    #[doc = "CLK_2"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_2(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk2)
    }
    #[doc = "CLK_3"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_3(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk3)
    }
    #[doc = "CLK_4"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_4(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk4)
    }
    #[doc = "CLK_8"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_8(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk8)
    }
    #[doc = "CLK_16"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_16(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk16)
    }
    #[doc = "CLK_31"]
    #[inline(always)]
    pub fn gfctl_dgfsel_clk_31(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlDgfsel::GfctlDgfselClk31)
    }
}
#[doc = "Analog Glitch Suppression Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfctlAgfen {
    #[doc = "0: DISABLE"]
    GfctlAgfenDisable = 0,
    #[doc = "1: ENABLE"]
    GfctlAgfenEnable = 1,
}
impl From<GfctlAgfen> for bool {
    #[inline(always)]
    fn from(variant: GfctlAgfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFCTL_AGFEN` reader - Analog Glitch Suppression Enable"]
pub type GfctlAgfenR = crate::BitReader<GfctlAgfen>;
impl GfctlAgfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GfctlAgfen {
        match self.bits {
            false => GfctlAgfen::GfctlAgfenDisable,
            true => GfctlAgfen::GfctlAgfenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_gfctl_agfen_disable(&self) -> bool {
        *self == GfctlAgfen::GfctlAgfenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_gfctl_agfen_enable(&self) -> bool {
        *self == GfctlAgfen::GfctlAgfenEnable
    }
}
#[doc = "Field `GFCTL_AGFEN` writer - Analog Glitch Suppression Enable"]
pub type GfctlAgfenW<'a, REG> = crate::BitWriter<'a, REG, GfctlAgfen>;
impl<'a, REG> GfctlAgfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn gfctl_agfen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlAgfen::GfctlAgfenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn gfctl_agfen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlAgfen::GfctlAgfenEnable)
    }
}
#[doc = "Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on SCL and SDA lines. See device datasheet for exact values. (ULP I2C only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GfctlAgfsel {
    #[doc = "0: AGLIT_5"]
    GfctlAgfselAglit5 = 0,
    #[doc = "1: AGLIT_10"]
    GfctlAgfselAglit10 = 1,
    #[doc = "2: AGLIT_25"]
    GfctlAgfselAglit25 = 2,
    #[doc = "3: AGLIT_50"]
    GfctlAgfselAglit50 = 3,
}
impl From<GfctlAgfsel> for u8 {
    #[inline(always)]
    fn from(variant: GfctlAgfsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GfctlAgfsel {
    type Ux = u8;
}
impl crate::IsEnum for GfctlAgfsel {}
#[doc = "Field `GFCTL_AGFSEL` reader - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on SCL and SDA lines. See device datasheet for exact values. (ULP I2C only)"]
pub type GfctlAgfselR = crate::FieldReader<GfctlAgfsel>;
impl GfctlAgfselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GfctlAgfsel {
        match self.bits {
            0 => GfctlAgfsel::GfctlAgfselAglit5,
            1 => GfctlAgfsel::GfctlAgfselAglit10,
            2 => GfctlAgfsel::GfctlAgfselAglit25,
            3 => GfctlAgfsel::GfctlAgfselAglit50,
            _ => unreachable!(),
        }
    }
    #[doc = "AGLIT_5"]
    #[inline(always)]
    pub fn is_gfctl_agfsel_aglit_5(&self) -> bool {
        *self == GfctlAgfsel::GfctlAgfselAglit5
    }
    #[doc = "AGLIT_10"]
    #[inline(always)]
    pub fn is_gfctl_agfsel_aglit_10(&self) -> bool {
        *self == GfctlAgfsel::GfctlAgfselAglit10
    }
    #[doc = "AGLIT_25"]
    #[inline(always)]
    pub fn is_gfctl_agfsel_aglit_25(&self) -> bool {
        *self == GfctlAgfsel::GfctlAgfselAglit25
    }
    #[doc = "AGLIT_50"]
    #[inline(always)]
    pub fn is_gfctl_agfsel_aglit_50(&self) -> bool {
        *self == GfctlAgfsel::GfctlAgfselAglit50
    }
}
#[doc = "Field `GFCTL_AGFSEL` writer - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on SCL and SDA lines. See device datasheet for exact values. (ULP I2C only)"]
pub type GfctlAgfselW<'a, REG> = crate::FieldWriter<'a, REG, 2, GfctlAgfsel, crate::Safe>;
impl<'a, REG> GfctlAgfselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AGLIT_5"]
    #[inline(always)]
    pub fn gfctl_agfsel_aglit_5(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlAgfsel::GfctlAgfselAglit5)
    }
    #[doc = "AGLIT_10"]
    #[inline(always)]
    pub fn gfctl_agfsel_aglit_10(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlAgfsel::GfctlAgfselAglit10)
    }
    #[doc = "AGLIT_25"]
    #[inline(always)]
    pub fn gfctl_agfsel_aglit_25(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlAgfsel::GfctlAgfselAglit25)
    }
    #[doc = "AGLIT_50"]
    #[inline(always)]
    pub fn gfctl_agfsel_aglit_50(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlAgfsel::GfctlAgfselAglit50)
    }
}
#[doc = "Analog and digital noise filters chaining enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfctlChain {
    #[doc = "0: DISABLE"]
    GfctlChainDisable = 0,
    #[doc = "1: ENABLE"]
    GfctlChainEnable = 1,
}
impl From<GfctlChain> for bool {
    #[inline(always)]
    fn from(variant: GfctlChain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GFCTL_CHAIN` reader - Analog and digital noise filters chaining enable."]
pub type GfctlChainR = crate::BitReader<GfctlChain>;
impl GfctlChainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GfctlChain {
        match self.bits {
            false => GfctlChain::GfctlChainDisable,
            true => GfctlChain::GfctlChainEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_gfctl_chain_disable(&self) -> bool {
        *self == GfctlChain::GfctlChainDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_gfctl_chain_enable(&self) -> bool {
        *self == GfctlChain::GfctlChainEnable
    }
}
#[doc = "Field `GFCTL_CHAIN` writer - Analog and digital noise filters chaining enable."]
pub type GfctlChainW<'a, REG> = crate::BitWriter<'a, REG, GfctlChain>;
impl<'a, REG> GfctlChainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn gfctl_chain_disable(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlChain::GfctlChainDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn gfctl_chain_enable(self) -> &'a mut crate::W<REG> {
        self.variant(GfctlChain::GfctlChainEnable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
    #[inline(always)]
    pub fn gfctl_dgfsel(&self) -> GfctlDgfselR {
        GfctlDgfselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn gfctl_agfen(&self) -> GfctlAgfenR {
        GfctlAgfenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on SCL and SDA lines. See device datasheet for exact values. (ULP I2C only)"]
    #[inline(always)]
    pub fn gfctl_agfsel(&self) -> GfctlAgfselR {
        GfctlAgfselR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Analog and digital noise filters chaining enable."]
    #[inline(always)]
    pub fn gfctl_chain(&self) -> GfctlChainR {
        GfctlChainR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Glitch Suppression Pulse Width This field controls the pulse width select for glitch suppression on the SCL and SDA lines. The following values are the glitch suppression values in terms of functional clocks. (Core Domain only)"]
    #[inline(always)]
    pub fn gfctl_dgfsel(&mut self) -> GfctlDgfselW<GfctlSpec> {
        GfctlDgfselW::new(self, 0)
    }
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn gfctl_agfen(&mut self) -> GfctlAgfenW<GfctlSpec> {
        GfctlAgfenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on SCL and SDA lines. See device datasheet for exact values. (ULP I2C only)"]
    #[inline(always)]
    pub fn gfctl_agfsel(&mut self) -> GfctlAgfselW<GfctlSpec> {
        GfctlAgfselW::new(self, 9)
    }
    #[doc = "Bit 11 - Analog and digital noise filters chaining enable."]
    #[inline(always)]
    pub fn gfctl_chain(&mut self) -> GfctlChainW<GfctlSpec> {
        GfctlChainW::new(self, 11)
    }
}
#[doc = "I2C Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GfctlSpec;
impl crate::RegisterSpec for GfctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gfctl::R`](R) reader structure"]
impl crate::Readable for GfctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gfctl::W`](W) writer structure"]
impl crate::Writable for GfctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GFCTL to value 0"]
impl crate::Resettable for GfctlSpec {
    const RESET_VALUE: u32 = 0;
}

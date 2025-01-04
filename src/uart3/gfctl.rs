#[doc = "Register `GFCTL` reader"]
pub type R = crate::R<GfctlSpec>;
#[doc = "Register `GFCTL` writer"]
pub type W = crate::W<GfctlSpec>;
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
#[doc = "Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values.\n\nValue on reset: 0"]
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
#[doc = "Field `GFCTL_AGFSEL` reader - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
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
#[doc = "Field `GFCTL_AGFSEL` writer - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
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
impl R {
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn gfctl_agfen(&self) -> GfctlAgfenR {
        GfctlAgfenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
    #[inline(always)]
    pub fn gfctl_agfsel(&self) -> GfctlAgfselR {
        GfctlAgfselR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - Analog Glitch Suppression Enable"]
    #[inline(always)]
    pub fn gfctl_agfen(&mut self) -> GfctlAgfenW<GfctlSpec> {
        GfctlAgfenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Analog Glitch Suppression Pulse Width This field controls the pulse width select for the analog glitch suppression on the RX line. See device datasheet for exact values."]
    #[inline(always)]
    pub fn gfctl_agfsel(&mut self) -> GfctlAgfselW<GfctlSpec> {
        GfctlAgfselW::new(self, 9)
    }
}
#[doc = "Glitch Filter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`gfctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

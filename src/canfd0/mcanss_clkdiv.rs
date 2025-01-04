#[doc = "Register `MCANSS_CLKDIV` reader"]
pub type R = crate::R<McanssClkdivSpec>;
#[doc = "Register `MCANSS_CLKDIV` writer"]
pub type W = crate::W<McanssClkdivSpec>;
#[doc = "Clock divide ratio specification. Enables configuring clock divide settings for the MCAN functional clock input to the MCAN-SS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum McanssClkdivRatio {
    #[doc = "0: DIV_BY_1_"]
    McanssClkdivRatioDivBy1_ = 0,
    #[doc = "1: DIV_BY_2_"]
    McanssClkdivRatioDivBy2_ = 1,
    #[doc = "2: DIV_BY_4_"]
    McanssClkdivRatioDivBy4_ = 2,
}
impl From<McanssClkdivRatio> for u8 {
    #[inline(always)]
    fn from(variant: McanssClkdivRatio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for McanssClkdivRatio {
    type Ux = u8;
}
impl crate::IsEnum for McanssClkdivRatio {}
#[doc = "Field `MCANSS_CLKDIV_RATIO` reader - Clock divide ratio specification. Enables configuring clock divide settings for the MCAN functional clock input to the MCAN-SS."]
pub type McanssClkdivRatioR = crate::FieldReader<McanssClkdivRatio>;
impl McanssClkdivRatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<McanssClkdivRatio> {
        match self.bits {
            0 => Some(McanssClkdivRatio::McanssClkdivRatioDivBy1_),
            1 => Some(McanssClkdivRatio::McanssClkdivRatioDivBy2_),
            2 => Some(McanssClkdivRatio::McanssClkdivRatioDivBy4_),
            _ => None,
        }
    }
    #[doc = "DIV_BY_1_"]
    #[inline(always)]
    pub fn is_mcanss_clkdiv_ratio_div_by_1_(&self) -> bool {
        *self == McanssClkdivRatio::McanssClkdivRatioDivBy1_
    }
    #[doc = "DIV_BY_2_"]
    #[inline(always)]
    pub fn is_mcanss_clkdiv_ratio_div_by_2_(&self) -> bool {
        *self == McanssClkdivRatio::McanssClkdivRatioDivBy2_
    }
    #[doc = "DIV_BY_4_"]
    #[inline(always)]
    pub fn is_mcanss_clkdiv_ratio_div_by_4_(&self) -> bool {
        *self == McanssClkdivRatio::McanssClkdivRatioDivBy4_
    }
}
#[doc = "Field `MCANSS_CLKDIV_RATIO` writer - Clock divide ratio specification. Enables configuring clock divide settings for the MCAN functional clock input to the MCAN-SS."]
pub type McanssClkdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 2, McanssClkdivRatio>;
impl<'a, REG> McanssClkdivRatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV_BY_1_"]
    #[inline(always)]
    pub fn mcanss_clkdiv_ratio_div_by_1_(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkdivRatio::McanssClkdivRatioDivBy1_)
    }
    #[doc = "DIV_BY_2_"]
    #[inline(always)]
    pub fn mcanss_clkdiv_ratio_div_by_2_(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkdivRatio::McanssClkdivRatioDivBy2_)
    }
    #[doc = "DIV_BY_4_"]
    #[inline(always)]
    pub fn mcanss_clkdiv_ratio_div_by_4_(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkdivRatio::McanssClkdivRatioDivBy4_)
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock divide ratio specification. Enables configuring clock divide settings for the MCAN functional clock input to the MCAN-SS."]
    #[inline(always)]
    pub fn mcanss_clkdiv_ratio(&self) -> McanssClkdivRatioR {
        McanssClkdivRatioR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock divide ratio specification. Enables configuring clock divide settings for the MCAN functional clock input to the MCAN-SS."]
    #[inline(always)]
    pub fn mcanss_clkdiv_ratio(&mut self) -> McanssClkdivRatioW<McanssClkdivSpec> {
        McanssClkdivRatioW::new(self, 0)
    }
}
#[doc = "Clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssClkdivSpec;
impl crate::RegisterSpec for McanssClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_clkdiv::R`](R) reader structure"]
impl crate::Readable for McanssClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_clkdiv::W`](W) writer structure"]
impl crate::Writable for McanssClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_CLKDIV to value 0"]
impl crate::Resettable for McanssClkdivSpec {
    const RESET_VALUE: u32 = 0;
}

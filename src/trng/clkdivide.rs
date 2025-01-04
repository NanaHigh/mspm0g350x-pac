#[doc = "Register `CLKDIVIDE` reader"]
pub type R = crate::R<ClkdivideSpec>;
#[doc = "Register `CLKDIVIDE` writer"]
pub type W = crate::W<ClkdivideSpec>;
#[doc = "Selects divide ratio of module clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkdivideRatio {
    #[doc = "0: DIV_BY_1"]
    ClkdivideRatioDivBy1 = 0,
    #[doc = "1: DIV_BY_2"]
    ClkdivideRatioDivBy2 = 1,
    #[doc = "3: DIV_BY_4"]
    ClkdivideRatioDivBy4 = 3,
    #[doc = "5: DIV_BY_6"]
    ClkdivideRatioDivBy6 = 5,
    #[doc = "7: DIV_BY_8"]
    ClkdivideRatioDivBy8 = 7,
}
impl From<ClkdivideRatio> for u8 {
    #[inline(always)]
    fn from(variant: ClkdivideRatio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkdivideRatio {
    type Ux = u8;
}
impl crate::IsEnum for ClkdivideRatio {}
#[doc = "Field `CLKDIVIDE_RATIO` reader - Selects divide ratio of module clock"]
pub type ClkdivideRatioR = crate::FieldReader<ClkdivideRatio>;
impl ClkdivideRatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkdivideRatio> {
        match self.bits {
            0 => Some(ClkdivideRatio::ClkdivideRatioDivBy1),
            1 => Some(ClkdivideRatio::ClkdivideRatioDivBy2),
            3 => Some(ClkdivideRatio::ClkdivideRatioDivBy4),
            5 => Some(ClkdivideRatio::ClkdivideRatioDivBy6),
            7 => Some(ClkdivideRatio::ClkdivideRatioDivBy8),
            _ => None,
        }
    }
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn is_clkdivide_ratio_div_by_1(&self) -> bool {
        *self == ClkdivideRatio::ClkdivideRatioDivBy1
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn is_clkdivide_ratio_div_by_2(&self) -> bool {
        *self == ClkdivideRatio::ClkdivideRatioDivBy2
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn is_clkdivide_ratio_div_by_4(&self) -> bool {
        *self == ClkdivideRatio::ClkdivideRatioDivBy4
    }
    #[doc = "DIV_BY_6"]
    #[inline(always)]
    pub fn is_clkdivide_ratio_div_by_6(&self) -> bool {
        *self == ClkdivideRatio::ClkdivideRatioDivBy6
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn is_clkdivide_ratio_div_by_8(&self) -> bool {
        *self == ClkdivideRatio::ClkdivideRatioDivBy8
    }
}
#[doc = "Field `CLKDIVIDE_RATIO` writer - Selects divide ratio of module clock"]
pub type ClkdivideRatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkdivideRatio>;
impl<'a, REG> ClkdivideRatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn clkdivide_ratio_div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivideRatio::ClkdivideRatioDivBy1)
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn clkdivide_ratio_div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivideRatio::ClkdivideRatioDivBy2)
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn clkdivide_ratio_div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivideRatio::ClkdivideRatioDivBy4)
    }
    #[doc = "DIV_BY_6"]
    #[inline(always)]
    pub fn clkdivide_ratio_div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivideRatio::ClkdivideRatioDivBy6)
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn clkdivide_ratio_div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivideRatio::ClkdivideRatioDivBy8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn clkdivide_ratio(&self) -> ClkdivideRatioR {
        ClkdivideRatioR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn clkdivide_ratio(&mut self) -> ClkdivideRatioW<ClkdivideSpec> {
        ClkdivideRatioW::new(self, 0)
    }
}
#[doc = "Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdivide::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdivide::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivideSpec;
impl crate::RegisterSpec for ClkdivideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdivide::R`](R) reader structure"]
impl crate::Readable for ClkdivideSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdivide::W`](W) writer structure"]
impl crate::Writable for ClkdivideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIVIDE to value 0"]
impl crate::Resettable for ClkdivideSpec {
    const RESET_VALUE: u32 = 0;
}

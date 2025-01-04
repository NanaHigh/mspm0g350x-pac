#[doc = "Register `CLKDIV` reader"]
pub type R = crate::R<ClkdivSpec>;
#[doc = "Register `CLKDIV` writer"]
pub type W = crate::W<ClkdivSpec>;
#[doc = "Selects divide ratio of module clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkdivRatio {
    #[doc = "0: DIV_BY_1"]
    ClkdivRatioDivBy1 = 0,
    #[doc = "1: DIV_BY_2"]
    ClkdivRatioDivBy2 = 1,
    #[doc = "2: DIV_BY_3"]
    ClkdivRatioDivBy3 = 2,
    #[doc = "3: DIV_BY_4"]
    ClkdivRatioDivBy4 = 3,
    #[doc = "4: DIV_BY_5"]
    ClkdivRatioDivBy5 = 4,
    #[doc = "5: DIV_BY_6"]
    ClkdivRatioDivBy6 = 5,
    #[doc = "6: DIV_BY_7"]
    ClkdivRatioDivBy7 = 6,
    #[doc = "7: DIV_BY_8"]
    ClkdivRatioDivBy8 = 7,
}
impl From<ClkdivRatio> for u8 {
    #[inline(always)]
    fn from(variant: ClkdivRatio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkdivRatio {
    type Ux = u8;
}
impl crate::IsEnum for ClkdivRatio {}
#[doc = "Field `CLKDIV_RATIO` reader - Selects divide ratio of module clock"]
pub type ClkdivRatioR = crate::FieldReader<ClkdivRatio>;
impl ClkdivRatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkdivRatio {
        match self.bits {
            0 => ClkdivRatio::ClkdivRatioDivBy1,
            1 => ClkdivRatio::ClkdivRatioDivBy2,
            2 => ClkdivRatio::ClkdivRatioDivBy3,
            3 => ClkdivRatio::ClkdivRatioDivBy4,
            4 => ClkdivRatio::ClkdivRatioDivBy5,
            5 => ClkdivRatio::ClkdivRatioDivBy6,
            6 => ClkdivRatio::ClkdivRatioDivBy7,
            7 => ClkdivRatio::ClkdivRatioDivBy8,
            _ => unreachable!(),
        }
    }
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_1(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy1
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_2(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy2
    }
    #[doc = "DIV_BY_3"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_3(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy3
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_4(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy4
    }
    #[doc = "DIV_BY_5"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_5(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy5
    }
    #[doc = "DIV_BY_6"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_6(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy6
    }
    #[doc = "DIV_BY_7"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_7(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy7
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn is_clkdiv_ratio_div_by_8(&self) -> bool {
        *self == ClkdivRatio::ClkdivRatioDivBy8
    }
}
#[doc = "Field `CLKDIV_RATIO` writer - Selects divide ratio of module clock"]
pub type ClkdivRatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkdivRatio, crate::Safe>;
impl<'a, REG> ClkdivRatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy1)
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy2)
    }
    #[doc = "DIV_BY_3"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy3)
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy4)
    }
    #[doc = "DIV_BY_5"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_5(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy5)
    }
    #[doc = "DIV_BY_6"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy6)
    }
    #[doc = "DIV_BY_7"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_7(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy7)
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn clkdiv_ratio_div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivRatio::ClkdivRatioDivBy8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn clkdiv_ratio(&self) -> ClkdivRatioR {
        ClkdivRatioR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn clkdiv_ratio(&mut self) -> ClkdivRatioW<ClkdivSpec> {
        ClkdivRatioW::new(self, 0)
    }
}
#[doc = "Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkdivSpec;
impl crate::RegisterSpec for ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv::R`](R) reader structure"]
impl crate::Readable for ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv::W`](W) writer structure"]
impl crate::Writable for ClkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV to value 0"]
impl crate::Resettable for ClkdivSpec {
    const RESET_VALUE: u32 = 0;
}

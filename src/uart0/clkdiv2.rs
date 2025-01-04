#[doc = "Register `CLKDIV2` reader"]
pub type R = crate::R<Clkdiv2Spec>;
#[doc = "Register `CLKDIV2` writer"]
pub type W = crate::W<Clkdiv2Spec>;
#[doc = "Selects divide ratio of module clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv2Ratio {
    #[doc = "0: DIV_BY_1"]
    Clkdiv2RatioDivBy1 = 0,
    #[doc = "1: DIV_BY_2"]
    Clkdiv2RatioDivBy2 = 1,
    #[doc = "2: DIV_BY_3"]
    Clkdiv2RatioDivBy3 = 2,
    #[doc = "3: DIV_BY_4"]
    Clkdiv2RatioDivBy4 = 3,
    #[doc = "4: DIV_BY_5"]
    Clkdiv2RatioDivBy5 = 4,
    #[doc = "5: DIV_BY_6"]
    Clkdiv2RatioDivBy6 = 5,
    #[doc = "6: DIV_BY_7"]
    Clkdiv2RatioDivBy7 = 6,
    #[doc = "7: DIV_BY_8"]
    Clkdiv2RatioDivBy8 = 7,
}
impl From<Clkdiv2Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv2Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv2Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv2Ratio {}
#[doc = "Field `CLKDIV2_RATIO` reader - Selects divide ratio of module clock"]
pub type Clkdiv2RatioR = crate::FieldReader<Clkdiv2Ratio>;
impl Clkdiv2RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clkdiv2Ratio {
        match self.bits {
            0 => Clkdiv2Ratio::Clkdiv2RatioDivBy1,
            1 => Clkdiv2Ratio::Clkdiv2RatioDivBy2,
            2 => Clkdiv2Ratio::Clkdiv2RatioDivBy3,
            3 => Clkdiv2Ratio::Clkdiv2RatioDivBy4,
            4 => Clkdiv2Ratio::Clkdiv2RatioDivBy5,
            5 => Clkdiv2Ratio::Clkdiv2RatioDivBy6,
            6 => Clkdiv2Ratio::Clkdiv2RatioDivBy7,
            7 => Clkdiv2Ratio::Clkdiv2RatioDivBy8,
            _ => unreachable!(),
        }
    }
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_1(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy1
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_2(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy2
    }
    #[doc = "DIV_BY_3"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_3(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy3
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_4(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy4
    }
    #[doc = "DIV_BY_5"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_5(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy5
    }
    #[doc = "DIV_BY_6"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_6(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy6
    }
    #[doc = "DIV_BY_7"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_7(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy7
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn is_clkdiv2_ratio_div_by_8(&self) -> bool {
        *self == Clkdiv2Ratio::Clkdiv2RatioDivBy8
    }
}
#[doc = "Field `CLKDIV2_RATIO` writer - Selects divide ratio of module clock"]
pub type Clkdiv2RatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkdiv2Ratio, crate::Safe>;
impl<'a, REG> Clkdiv2RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy1)
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy2)
    }
    #[doc = "DIV_BY_3"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy3)
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy4)
    }
    #[doc = "DIV_BY_5"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_5(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy5)
    }
    #[doc = "DIV_BY_6"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy6)
    }
    #[doc = "DIV_BY_7"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_7(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy7)
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn clkdiv2_ratio_div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv2Ratio::Clkdiv2RatioDivBy8)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn clkdiv2_ratio(&self) -> Clkdiv2RatioR {
        Clkdiv2RatioR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn clkdiv2_ratio(&mut self) -> Clkdiv2RatioW<Clkdiv2Spec> {
        Clkdiv2RatioW::new(self, 0)
    }
}
#[doc = "Clock Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv2Spec;
impl crate::RegisterSpec for Clkdiv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv2::R`](R) reader structure"]
impl crate::Readable for Clkdiv2Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv2::W`](W) writer structure"]
impl crate::Writable for Clkdiv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for Clkdiv2Spec {
    const RESET_VALUE: u32 = 0;
}

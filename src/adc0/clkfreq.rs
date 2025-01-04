#[doc = "Register `CLKFREQ` reader"]
pub type R = crate::R<ClkfreqSpec>;
#[doc = "Register `CLKFREQ` writer"]
pub type W = crate::W<ClkfreqSpec>;
#[doc = "Frequency Range.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkfreqFrange {
    #[doc = "0: RANGE1TO4"]
    ClkfreqFrangeRange1to4 = 0,
    #[doc = "1: RANGE4TO8"]
    ClkfreqFrangeRange4to8 = 1,
    #[doc = "2: RANGE8TO16"]
    ClkfreqFrangeRange8to16 = 2,
    #[doc = "3: RANGE16TO20"]
    ClkfreqFrangeRange16to20 = 3,
    #[doc = "4: RANGE20TO24"]
    ClkfreqFrangeRange20to24 = 4,
    #[doc = "5: RANGE24TO32"]
    ClkfreqFrangeRange24to32 = 5,
    #[doc = "6: RANGE32TO40"]
    ClkfreqFrangeRange32to40 = 6,
    #[doc = "7: RANGE40TO48"]
    ClkfreqFrangeRange40to48 = 7,
}
impl From<ClkfreqFrange> for u8 {
    #[inline(always)]
    fn from(variant: ClkfreqFrange) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkfreqFrange {
    type Ux = u8;
}
impl crate::IsEnum for ClkfreqFrange {}
#[doc = "Field `CLKFREQ_FRANGE` reader - Frequency Range."]
pub type ClkfreqFrangeR = crate::FieldReader<ClkfreqFrange>;
impl ClkfreqFrangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkfreqFrange {
        match self.bits {
            0 => ClkfreqFrange::ClkfreqFrangeRange1to4,
            1 => ClkfreqFrange::ClkfreqFrangeRange4to8,
            2 => ClkfreqFrange::ClkfreqFrangeRange8to16,
            3 => ClkfreqFrange::ClkfreqFrangeRange16to20,
            4 => ClkfreqFrange::ClkfreqFrangeRange20to24,
            5 => ClkfreqFrange::ClkfreqFrangeRange24to32,
            6 => ClkfreqFrange::ClkfreqFrangeRange32to40,
            7 => ClkfreqFrange::ClkfreqFrangeRange40to48,
            _ => unreachable!(),
        }
    }
    #[doc = "RANGE1TO4"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range1to4(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange1to4
    }
    #[doc = "RANGE4TO8"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range4to8(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange4to8
    }
    #[doc = "RANGE8TO16"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range8to16(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange8to16
    }
    #[doc = "RANGE16TO20"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range16to20(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange16to20
    }
    #[doc = "RANGE20TO24"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range20to24(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange20to24
    }
    #[doc = "RANGE24TO32"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range24to32(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange24to32
    }
    #[doc = "RANGE32TO40"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range32to40(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange32to40
    }
    #[doc = "RANGE40TO48"]
    #[inline(always)]
    pub fn is_clkfreq_frange_range40to48(&self) -> bool {
        *self == ClkfreqFrange::ClkfreqFrangeRange40to48
    }
}
#[doc = "Field `CLKFREQ_FRANGE` writer - Frequency Range."]
pub type ClkfreqFrangeW<'a, REG> = crate::FieldWriter<'a, REG, 3, ClkfreqFrange, crate::Safe>;
impl<'a, REG> ClkfreqFrangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RANGE1TO4"]
    #[inline(always)]
    pub fn clkfreq_frange_range1to4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange1to4)
    }
    #[doc = "RANGE4TO8"]
    #[inline(always)]
    pub fn clkfreq_frange_range4to8(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange4to8)
    }
    #[doc = "RANGE8TO16"]
    #[inline(always)]
    pub fn clkfreq_frange_range8to16(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange8to16)
    }
    #[doc = "RANGE16TO20"]
    #[inline(always)]
    pub fn clkfreq_frange_range16to20(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange16to20)
    }
    #[doc = "RANGE20TO24"]
    #[inline(always)]
    pub fn clkfreq_frange_range20to24(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange20to24)
    }
    #[doc = "RANGE24TO32"]
    #[inline(always)]
    pub fn clkfreq_frange_range24to32(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange24to32)
    }
    #[doc = "RANGE32TO40"]
    #[inline(always)]
    pub fn clkfreq_frange_range32to40(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange32to40)
    }
    #[doc = "RANGE40TO48"]
    #[inline(always)]
    pub fn clkfreq_frange_range40to48(self) -> &'a mut crate::W<REG> {
        self.variant(ClkfreqFrange::ClkfreqFrangeRange40to48)
    }
}
impl R {
    #[doc = "Bits 0:2 - Frequency Range."]
    #[inline(always)]
    pub fn clkfreq_frange(&self) -> ClkfreqFrangeR {
        ClkfreqFrangeR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frequency Range."]
    #[inline(always)]
    pub fn clkfreq_frange(&mut self) -> ClkfreqFrangeW<ClkfreqSpec> {
        ClkfreqFrangeW::new(self, 0)
    }
}
#[doc = "Sample Clock Frequency Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkfreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkfreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkfreqSpec;
impl crate::RegisterSpec for ClkfreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkfreq::R`](R) reader structure"]
impl crate::Readable for ClkfreqSpec {}
#[doc = "`write(|w| ..)` method takes [`clkfreq::W`](W) writer structure"]
impl crate::Writable for ClkfreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKFREQ to value 0"]
impl crate::Resettable for ClkfreqSpec {
    const RESET_VALUE: u32 = 0;
}

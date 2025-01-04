#[doc = "Register `DAY` reader"]
pub type R = crate::R<DaySpec>;
#[doc = "Register `DAY` writer"]
pub type W = crate::W<DaySpec>;
#[doc = "Field `DAY_DOW` reader - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type DayDowR = crate::FieldReader;
#[doc = "Field `DAY_DOW` writer - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type DayDowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DAY_DOMBIN` reader - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type DayDombinR = crate::FieldReader;
#[doc = "Field `DAY_DOMBIN` writer - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type DayDombinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DAY_DOMLOWBCD` reader - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DayDomlowbcdR = crate::FieldReader;
#[doc = "Field `DAY_DOMLOWBCD` writer - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DayDomlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAY_DOMHIGHBCD` reader - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DayDomhighbcdR = crate::FieldReader;
#[doc = "Field `DAY_DOMHIGHBCD` writer - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type DayDomhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn day_dow(&self) -> DayDowR {
        DayDowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn day_dombin(&self) -> DayDombinR {
        DayDombinR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19 - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn day_domlowbcd(&self) -> DayDomlowbcdR {
        DayDomlowbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn day_domhighbcd(&self) -> DayDomhighbcdR {
        DayDomhighbcdR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn day_dow(&mut self) -> DayDowW<DaySpec> {
        DayDowW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Day of month Binary (1 to 28, 29, 30, 31). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn day_dombin(&mut self) -> DayDombinW<DaySpec> {
        DayDombinW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn day_domlowbcd(&mut self) -> DayDomlowbcdW<DaySpec> {
        DayDomlowbcdW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn day_domhighbcd(&mut self) -> DayDomhighbcdW<DaySpec> {
        DayDomhighbcdW::new(self, 20)
    }
}
#[doc = "RTC Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaySpec;
impl crate::RegisterSpec for DaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`day::R`](R) reader structure"]
impl crate::Readable for DaySpec {}
#[doc = "`write(|w| ..)` method takes [`day::W`](W) writer structure"]
impl crate::Writable for DaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAY to value 0"]
impl crate::Resettable for DaySpec {
    const RESET_VALUE: u32 = 0;
}

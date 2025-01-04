#[doc = "Register `HOUR` reader"]
pub type R = crate::R<HourSpec>;
#[doc = "Register `HOUR` writer"]
pub type W = crate::W<HourSpec>;
#[doc = "Field `HOUR_HOURBIN` reader - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type HourHourbinR = crate::FieldReader;
#[doc = "Field `HOUR_HOURBIN` writer - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type HourHourbinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HOUR_HOURLOWBCD` reader - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourHourlowbcdR = crate::FieldReader;
#[doc = "Field `HOUR_HOURLOWBCD` writer - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourHourlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOUR_HOURHIGHBCD` reader - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourHourhighbcdR = crate::FieldReader;
#[doc = "Field `HOUR_HOURHIGHBCD` writer - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type HourHourhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hour_hourbin(&self) -> HourHourbinR {
        HourHourbinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hour_hourlowbcd(&self) -> HourHourlowbcdR {
        HourHourlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hour_hourhighbcd(&self) -> HourHourhighbcdR {
        HourHourhighbcdR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hour_hourbin(&mut self) -> HourHourbinW<HourSpec> {
        HourHourbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hour_hourlowbcd(&mut self) -> HourHourlowbcdW<HourSpec> {
        HourHourlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn hour_hourhighbcd(&mut self) -> HourHourhighbcdW<HourSpec> {
        HourHourhighbcdW::new(self, 12)
    }
}
#[doc = "RTC Hours Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HourSpec;
impl crate::RegisterSpec for HourSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hour::R`](R) reader structure"]
impl crate::Readable for HourSpec {}
#[doc = "`write(|w| ..)` method takes [`hour::W`](W) writer structure"]
impl crate::Writable for HourSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOUR to value 0"]
impl crate::Resettable for HourSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `YEAR` reader"]
pub type R = crate::R<YearSpec>;
#[doc = "Register `YEAR` writer"]
pub type W = crate::W<YearSpec>;
#[doc = "Field `YEAR_YEARLOWBIN` reader - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearYearlowbinR = crate::FieldReader;
#[doc = "Field `YEAR_YEARLOWBIN` writer - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearYearlowbinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEAR_YEARHIGHBIN` reader - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearYearhighbinR = crate::FieldReader;
#[doc = "Field `YEAR_YEARHIGHBIN` writer - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type YearYearhighbinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR_YEARLOWESTBCD` reader - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearYearlowestbcdR = crate::FieldReader;
#[doc = "Field `YEAR_YEARLOWESTBCD` writer - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearYearlowestbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR_DECADEBCD` reader - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearDecadebcdR = crate::FieldReader;
#[doc = "Field `YEAR_DECADEBCD` writer - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearDecadebcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR_CENTLOWBCD` reader - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearCentlowbcdR = crate::FieldReader;
#[doc = "Field `YEAR_CENTLOWBCD` writer - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearCentlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEAR_CENTHIGHBCD` reader - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearCenthighbcdR = crate::FieldReader;
#[doc = "Field `YEAR_CENTHIGHBCD` writer - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type YearCenthighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_yearlowbin(&self) -> YearYearlowbinR {
        YearYearlowbinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_yearhighbin(&self) -> YearYearhighbinR {
        YearYearhighbinR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_yearlowestbcd(&self) -> YearYearlowestbcdR {
        YearYearlowestbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_decadebcd(&self) -> YearDecadebcdR {
        YearDecadebcdR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_centlowbcd(&self) -> YearCentlowbcdR {
        YearCentlowbcdR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_centhighbcd(&self) -> YearCenthighbcdR {
        YearCenthighbcdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Year Binary low byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_yearlowbin(&mut self) -> YearYearlowbinW<YearSpec> {
        YearYearlowbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Year Binary high byte. Valid values for Year are 0 to 4095. If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_yearhighbin(&mut self) -> YearYearhighbinW<YearSpec> {
        YearYearhighbinW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Year BCD lowest digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_yearlowestbcd(&mut self) -> YearYearlowestbcdW<YearSpec> {
        YearYearlowestbcdW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Decade BCD (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_decadebcd(&mut self) -> YearDecadebcdW<YearSpec> {
        YearDecadebcdW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Century BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_centlowbcd(&mut self) -> YearCentlowbcdW<YearSpec> {
        YearCentlowbcdW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Century BCD high digit (0 to 4). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn year_centhighbcd(&mut self) -> YearCenthighbcdW<YearSpec> {
        YearCenthighbcdW::new(self, 28)
    }
}
#[doc = "RTC Year Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YearSpec;
impl crate::RegisterSpec for YearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`year::R`](R) reader structure"]
impl crate::Readable for YearSpec {}
#[doc = "`write(|w| ..)` method takes [`year::W`](W) writer structure"]
impl crate::Writable for YearSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets YEAR to value 0"]
impl crate::Resettable for YearSpec {
    const RESET_VALUE: u32 = 0;
}

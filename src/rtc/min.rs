#[doc = "Register `MIN` reader"]
pub type R = crate::R<MinSpec>;
#[doc = "Register `MIN` writer"]
pub type W = crate::W<MinSpec>;
#[doc = "Field `MIN_MINBIN` reader - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MinMinbinR = crate::FieldReader;
#[doc = "Field `MIN_MINBIN` writer - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MinMinbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MIN_MINLOWBCD` reader - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinMinlowbcdR = crate::FieldReader;
#[doc = "Field `MIN_MINLOWBCD` writer - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinMinlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIN_MINHIGHBCD` reader - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinMinhighbcdR = crate::FieldReader;
#[doc = "Field `MIN_MINHIGHBCD` writer - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MinMinhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn min_minbin(&self) -> MinMinbinR {
        MinMinbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn min_minlowbcd(&self) -> MinMinlowbcdR {
        MinMinlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn min_minhighbcd(&self) -> MinMinhighbcdR {
        MinMinhighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn min_minbin(&mut self) -> MinMinbinW<MinSpec> {
        MinMinbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn min_minlowbcd(&mut self) -> MinMinlowbcdW<MinSpec> {
        MinMinlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn min_minhighbcd(&mut self) -> MinMinhighbcdW<MinSpec> {
        MinMinhighbcdW::new(self, 12)
    }
}
#[doc = "RTC Minutes Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MinSpec;
impl crate::RegisterSpec for MinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`min::R`](R) reader structure"]
impl crate::Readable for MinSpec {}
#[doc = "`write(|w| ..)` method takes [`min::W`](W) writer structure"]
impl crate::Writable for MinSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIN to value 0"]
impl crate::Resettable for MinSpec {
    const RESET_VALUE: u32 = 0;
}

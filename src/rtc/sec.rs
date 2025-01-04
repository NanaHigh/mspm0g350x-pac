#[doc = "Register `SEC` reader"]
pub type R = crate::R<SecSpec>;
#[doc = "Register `SEC` writer"]
pub type W = crate::W<SecSpec>;
#[doc = "Field `SEC_SECBIN` reader - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type SecSecbinR = crate::FieldReader;
#[doc = "Field `SEC_SECBIN` writer - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type SecSecbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SEC_SECLOWBCD` reader - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SecSeclowbcdR = crate::FieldReader;
#[doc = "Field `SEC_SECLOWBCD` writer - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SecSeclowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEC_SECHIGHBCD` reader - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SecSechighbcdR = crate::FieldReader;
#[doc = "Field `SEC_SECHIGHBCD` writer - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type SecSechighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sec_secbin(&self) -> SecSecbinR {
        SecSecbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sec_seclowbcd(&self) -> SecSeclowbcdR {
        SecSeclowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sec_sechighbcd(&self) -> SecSechighbcdR {
        SecSechighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sec_secbin(&mut self) -> SecSecbinW<SecSpec> {
        SecSecbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Seconds BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sec_seclowbcd(&mut self) -> SecSeclowbcdW<SecSpec> {
        SecSeclowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Seconds BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn sec_sechighbcd(&mut self) -> SecSechighbcdW<SecSpec> {
        SecSechighbcdW::new(self, 12)
    }
}
#[doc = "RTC Seconds Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSpec;
impl crate::RegisterSpec for SecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec::R`](R) reader structure"]
impl crate::Readable for SecSpec {}
#[doc = "`write(|w| ..)` method takes [`sec::W`](W) writer structure"]
impl crate::Writable for SecSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SecSpec {
    const RESET_VALUE: u32 = 0;
}

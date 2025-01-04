#[doc = "Register `MON` reader"]
pub type R = crate::R<MonSpec>;
#[doc = "Register `MON` writer"]
pub type W = crate::W<MonSpec>;
#[doc = "Field `MON_MONBIN` reader - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MonMonbinR = crate::FieldReader;
#[doc = "Field `MON_MONBIN` writer - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type MonMonbinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MON_MONLOWBCD` reader - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonMonlowbcdR = crate::FieldReader;
#[doc = "Field `MON_MONLOWBCD` writer - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonMonlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MON_MONHIGHBCD` reader - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonMonhighbcdR = crate::BitReader;
#[doc = "Field `MON_MONHIGHBCD` writer - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type MonMonhighbcdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn mon_monbin(&self) -> MonMonbinR {
        MonMonbinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn mon_monlowbcd(&self) -> MonMonlowbcdR {
        MonMonlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn mon_monhighbcd(&self) -> MonMonhighbcdR {
        MonMonhighbcdR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Month Binary (1 to 12). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn mon_monbin(&mut self) -> MonMonbinW<MonSpec> {
        MonMonbinW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn mon_monlowbcd(&mut self) -> MonMonlowbcdW<MonSpec> {
        MonMonlowbcdW::new(self, 8)
    }
    #[doc = "Bit 12 - Month BCD high digit (0 or 1). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn mon_monhighbcd(&mut self) -> MonMonhighbcdW<MonSpec> {
        MonMonhighbcdW::new(self, 12)
    }
}
#[doc = "RTC Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonSpec;
impl crate::RegisterSpec for MonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mon::R`](R) reader structure"]
impl crate::Readable for MonSpec {}
#[doc = "`write(|w| ..)` method takes [`mon::W`](W) writer structure"]
impl crate::Writable for MonSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MON to value 0"]
impl crate::Resettable for MonSpec {
    const RESET_VALUE: u32 = 0;
}

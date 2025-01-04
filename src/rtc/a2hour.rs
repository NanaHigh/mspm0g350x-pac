#[doc = "Register `A2HOUR` reader"]
pub type R = crate::R<A2hourSpec>;
#[doc = "Register `A2HOUR` writer"]
pub type W = crate::W<A2hourSpec>;
#[doc = "Field `A2HOUR_AHOURBIN` reader - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A2hourAhourbinR = crate::FieldReader;
#[doc = "Field `A2HOUR_AHOURBIN` writer - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A2hourAhourbinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2hourAhouraebin {
    #[doc = "0: DISABLE"]
    A2hourAhouraebinDisable = 0,
    #[doc = "1: ENABLE"]
    A2hourAhouraebinEnable = 1,
}
impl From<A2hourAhouraebin> for bool {
    #[inline(always)]
    fn from(variant: A2hourAhouraebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2HOUR_AHOURAEBIN` reader - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A2hourAhouraebinR = crate::BitReader<A2hourAhouraebin>;
impl A2hourAhouraebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2hourAhouraebin {
        match self.bits {
            false => A2hourAhouraebin::A2hourAhouraebinDisable,
            true => A2hourAhouraebin::A2hourAhouraebinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2hour_ahouraebin_disable(&self) -> bool {
        *self == A2hourAhouraebin::A2hourAhouraebinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2hour_ahouraebin_enable(&self) -> bool {
        *self == A2hourAhouraebin::A2hourAhouraebinEnable
    }
}
#[doc = "Field `A2HOUR_AHOURAEBIN` writer - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A2hourAhouraebinW<'a, REG> = crate::BitWriter<'a, REG, A2hourAhouraebin>;
impl<'a, REG> A2hourAhouraebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2hour_ahouraebin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2hourAhouraebin::A2hourAhouraebinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2hour_ahouraebin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2hourAhouraebin::A2hourAhouraebinEnable)
    }
}
#[doc = "Field `A2HOUR_AHOURLOWBCD` reader - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2hourAhourlowbcdR = crate::FieldReader;
#[doc = "Field `A2HOUR_AHOURLOWBCD` writer - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2hourAhourlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A2HOUR_AHOURHIGHBCD` reader - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
pub type A2hourAhourhighbcdR = crate::FieldReader;
#[doc = "Field `A2HOUR_AHOURHIGHBCD` writer - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
pub type A2hourAhourhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2hourAhouraebcd {
    #[doc = "0: DISABLE"]
    A2hourAhouraebcdDisable = 0,
    #[doc = "1: ENABLE"]
    A2hourAhouraebcdEnable = 1,
}
impl From<A2hourAhouraebcd> for bool {
    #[inline(always)]
    fn from(variant: A2hourAhouraebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2HOUR_AHOURAEBCD` reader - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A2hourAhouraebcdR = crate::BitReader<A2hourAhouraebcd>;
impl A2hourAhouraebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2hourAhouraebcd {
        match self.bits {
            false => A2hourAhouraebcd::A2hourAhouraebcdDisable,
            true => A2hourAhouraebcd::A2hourAhouraebcdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2hour_ahouraebcd_disable(&self) -> bool {
        *self == A2hourAhouraebcd::A2hourAhouraebcdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2hour_ahouraebcd_enable(&self) -> bool {
        *self == A2hourAhouraebcd::A2hourAhouraebcdEnable
    }
}
#[doc = "Field `A2HOUR_AHOURAEBCD` writer - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A2hourAhouraebcdW<'a, REG> = crate::BitWriter<'a, REG, A2hourAhouraebcd>;
impl<'a, REG> A2hourAhouraebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2hour_ahouraebcd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2hourAhouraebcd::A2hourAhouraebcdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2hour_ahouraebcd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2hourAhouraebcd::A2hourAhouraebcdEnable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2hour_ahourbin(&self) -> A2hourAhourbinR {
        A2hourAhourbinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2hour_ahouraebin(&self) -> A2hourAhouraebinR {
        A2hourAhouraebinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2hour_ahourlowbcd(&self) -> A2hourAhourlowbcdR {
        A2hourAhourlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
    #[inline(always)]
    pub fn a2hour_ahourhighbcd(&self) -> A2hourAhourhighbcdR {
        A2hourAhourhighbcdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2hour_ahouraebcd(&self) -> A2hourAhouraebcdR {
        A2hourAhouraebcdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2hour_ahourbin(&mut self) -> A2hourAhourbinW<A2hourSpec> {
        A2hourAhourbinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2hour_ahouraebin(&mut self) -> A2hourAhouraebinW<A2hourSpec> {
        A2hourAhouraebinW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2hour_ahourlowbcd(&mut self) -> A2hourAhourlowbcdW<A2hourSpec> {
        A2hourAhourlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
    #[inline(always)]
    pub fn a2hour_ahourhighbcd(&mut self) -> A2hourAhourhighbcdW<A2hourSpec> {
        A2hourAhourhighbcdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2hour_ahouraebcd(&mut self) -> A2hourAhouraebcdW<A2hourSpec> {
        A2hourAhouraebcdW::new(self, 15)
    }
}
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a2hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2hourSpec;
impl crate::RegisterSpec for A2hourSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2hour::R`](R) reader structure"]
impl crate::Readable for A2hourSpec {}
#[doc = "`write(|w| ..)` method takes [`a2hour::W`](W) writer structure"]
impl crate::Writable for A2hourSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A2HOUR to value 0"]
impl crate::Resettable for A2hourSpec {
    const RESET_VALUE: u32 = 0;
}

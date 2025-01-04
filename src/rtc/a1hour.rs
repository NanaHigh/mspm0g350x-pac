#[doc = "Register `A1HOUR` reader"]
pub type R = crate::R<A1hourSpec>;
#[doc = "Register `A1HOUR` writer"]
pub type W = crate::W<A1hourSpec>;
#[doc = "Field `A1HOUR_AHOURBIN` reader - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A1hourAhourbinR = crate::FieldReader;
#[doc = "Field `A1HOUR_AHOURBIN` writer - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A1hourAhourbinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1hourAhouraebin {
    #[doc = "0: DISABLE"]
    A1hourAhouraebinDisable = 0,
    #[doc = "1: ENABLE"]
    A1hourAhouraebinEnable = 1,
}
impl From<A1hourAhouraebin> for bool {
    #[inline(always)]
    fn from(variant: A1hourAhouraebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1HOUR_AHOURAEBIN` reader - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A1hourAhouraebinR = crate::BitReader<A1hourAhouraebin>;
impl A1hourAhouraebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1hourAhouraebin {
        match self.bits {
            false => A1hourAhouraebin::A1hourAhouraebinDisable,
            true => A1hourAhouraebin::A1hourAhouraebinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1hour_ahouraebin_disable(&self) -> bool {
        *self == A1hourAhouraebin::A1hourAhouraebinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1hour_ahouraebin_enable(&self) -> bool {
        *self == A1hourAhouraebin::A1hourAhouraebinEnable
    }
}
#[doc = "Field `A1HOUR_AHOURAEBIN` writer - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A1hourAhouraebinW<'a, REG> = crate::BitWriter<'a, REG, A1hourAhouraebin>;
impl<'a, REG> A1hourAhouraebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1hour_ahouraebin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1hourAhouraebin::A1hourAhouraebinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1hour_ahouraebin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1hourAhouraebin::A1hourAhouraebinEnable)
    }
}
#[doc = "Field `A1HOUR_AHOURLOWBCD` reader - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1hourAhourlowbcdR = crate::FieldReader;
#[doc = "Field `A1HOUR_AHOURLOWBCD` writer - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1hourAhourlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A1HOUR_AHOURHIGHBCD` reader - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
pub type A1hourAhourhighbcdR = crate::FieldReader;
#[doc = "Field `A1HOUR_AHOURHIGHBCD` writer - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
pub type A1hourAhourhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1hourAhouraebcd {
    #[doc = "0: DISABLE"]
    A1hourAhouraebcdDisable = 0,
    #[doc = "1: ENABLE"]
    A1hourAhouraebcdEnable = 1,
}
impl From<A1hourAhouraebcd> for bool {
    #[inline(always)]
    fn from(variant: A1hourAhouraebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1HOUR_AHOURAEBCD` reader - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A1hourAhouraebcdR = crate::BitReader<A1hourAhouraebcd>;
impl A1hourAhouraebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1hourAhouraebcd {
        match self.bits {
            false => A1hourAhouraebcd::A1hourAhouraebcdDisable,
            true => A1hourAhouraebcd::A1hourAhouraebcdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1hour_ahouraebcd_disable(&self) -> bool {
        *self == A1hourAhouraebcd::A1hourAhouraebcdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1hour_ahouraebcd_enable(&self) -> bool {
        *self == A1hourAhouraebcd::A1hourAhouraebcdEnable
    }
}
#[doc = "Field `A1HOUR_AHOURAEBCD` writer - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A1hourAhouraebcdW<'a, REG> = crate::BitWriter<'a, REG, A1hourAhouraebcd>;
impl<'a, REG> A1hourAhouraebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1hour_ahouraebcd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1hourAhouraebcd::A1hourAhouraebcdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1hour_ahouraebcd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1hourAhouraebcd::A1hourAhouraebcdEnable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1hour_ahourbin(&self) -> A1hourAhourbinR {
        A1hourAhourbinR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1hour_ahouraebin(&self) -> A1hourAhouraebinR {
        A1hourAhouraebinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1hour_ahourlowbcd(&self) -> A1hourAhourlowbcdR {
        A1hourAhourlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
    #[inline(always)]
    pub fn a1hour_ahourhighbcd(&self) -> A1hourAhourhighbcdR {
        A1hourAhourhighbcdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1hour_ahouraebcd(&self) -> A1hourAhouraebcdR {
        A1hourAhouraebcdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Alarm Hours Binary (0 to 23). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1hour_ahourbin(&mut self) -> A1hourAhourbinW<A1hourSpec> {
        A1hourAhourbinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Hours Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1hour_ahouraebin(&mut self) -> A1hourAhouraebinW<A1hourSpec> {
        A1hourAhouraebinW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Alarm Hours BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1hour_ahourlowbcd(&mut self) -> A1hourAhourlowbcdW<A1hourSpec> {
        A1hourAhourlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Alarm Hours BCD high digit (0 to 2). If RTCBCD=0 write to these bits will be ignored and read give the value 0.."]
    #[inline(always)]
    pub fn a1hour_ahourhighbcd(&mut self) -> A1hourAhourhighbcdW<A1hourSpec> {
        A1hourAhourhighbcdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm Hours BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1hour_ahouraebcd(&mut self) -> A1hourAhouraebcdW<A1hourSpec> {
        A1hourAhouraebcdW::new(self, 15)
    }
}
#[doc = "RTC Hours Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a1hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1hourSpec;
impl crate::RegisterSpec for A1hourSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a1hour::R`](R) reader structure"]
impl crate::Readable for A1hourSpec {}
#[doc = "`write(|w| ..)` method takes [`a1hour::W`](W) writer structure"]
impl crate::Writable for A1hourSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A1HOUR to value 0"]
impl crate::Resettable for A1hourSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `A2DAY` reader"]
pub type R = crate::R<A2daySpec>;
#[doc = "Register `A2DAY` writer"]
pub type W = crate::W<A2daySpec>;
#[doc = "Field `A2DAY_ADOW` reader - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type A2dayAdowR = crate::FieldReader;
#[doc = "Field `A2DAY_ADOW` writer - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type A2dayAdowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2dayAdowae {
    #[doc = "0: DISABLE"]
    A2dayAdowaeDisable = 0,
    #[doc = "1: ENABLE"]
    A2dayAdowaeEnable = 1,
}
impl From<A2dayAdowae> for bool {
    #[inline(always)]
    fn from(variant: A2dayAdowae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2DAY_ADOWAE` reader - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
pub type A2dayAdowaeR = crate::BitReader<A2dayAdowae>;
impl A2dayAdowaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2dayAdowae {
        match self.bits {
            false => A2dayAdowae::A2dayAdowaeDisable,
            true => A2dayAdowae::A2dayAdowaeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2day_adowae_disable(&self) -> bool {
        *self == A2dayAdowae::A2dayAdowaeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2day_adowae_enable(&self) -> bool {
        *self == A2dayAdowae::A2dayAdowaeEnable
    }
}
#[doc = "Field `A2DAY_ADOWAE` writer - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
pub type A2dayAdowaeW<'a, REG> = crate::BitWriter<'a, REG, A2dayAdowae>;
impl<'a, REG> A2dayAdowaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2day_adowae_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2dayAdowae::A2dayAdowaeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2day_adowae_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2dayAdowae::A2dayAdowaeEnable)
    }
}
#[doc = "Field `A2DAY_ADOMBIN` reader - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A2dayAdombinR = crate::FieldReader;
#[doc = "Field `A2DAY_ADOMBIN` writer - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A2dayAdombinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2dayAdomaebin {
    #[doc = "0: DISABLE"]
    A2dayAdomaebinDisable = 0,
    #[doc = "1: ENABLE"]
    A2dayAdomaebinEnable = 1,
}
impl From<A2dayAdomaebin> for bool {
    #[inline(always)]
    fn from(variant: A2dayAdomaebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2DAY_ADOMAEBIN` reader - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A2dayAdomaebinR = crate::BitReader<A2dayAdomaebin>;
impl A2dayAdomaebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2dayAdomaebin {
        match self.bits {
            false => A2dayAdomaebin::A2dayAdomaebinDisable,
            true => A2dayAdomaebin::A2dayAdomaebinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2day_adomaebin_disable(&self) -> bool {
        *self == A2dayAdomaebin::A2dayAdomaebinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2day_adomaebin_enable(&self) -> bool {
        *self == A2dayAdomaebin::A2dayAdomaebinEnable
    }
}
#[doc = "Field `A2DAY_ADOMAEBIN` writer - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A2dayAdomaebinW<'a, REG> = crate::BitWriter<'a, REG, A2dayAdomaebin>;
impl<'a, REG> A2dayAdomaebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2day_adomaebin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2dayAdomaebin::A2dayAdomaebinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2day_adomaebin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2dayAdomaebin::A2dayAdomaebinEnable)
    }
}
#[doc = "Field `A2DAY_ADOMLOWBCD` reader - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2dayAdomlowbcdR = crate::FieldReader;
#[doc = "Field `A2DAY_ADOMLOWBCD` writer - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2dayAdomlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A2DAY_ADOMHIGHBCD` reader - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2dayAdomhighbcdR = crate::FieldReader;
#[doc = "Field `A2DAY_ADOMHIGHBCD` writer - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2dayAdomhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2dayAdomaebcd {
    #[doc = "0: DISABLE"]
    A2dayAdomaebcdDisable = 0,
    #[doc = "1: ENABLE"]
    A2dayAdomaebcdEnable = 1,
}
impl From<A2dayAdomaebcd> for bool {
    #[inline(always)]
    fn from(variant: A2dayAdomaebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2DAY_ADOMAEBCD` reader - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A2dayAdomaebcdR = crate::BitReader<A2dayAdomaebcd>;
impl A2dayAdomaebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2dayAdomaebcd {
        match self.bits {
            false => A2dayAdomaebcd::A2dayAdomaebcdDisable,
            true => A2dayAdomaebcd::A2dayAdomaebcdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2day_adomaebcd_disable(&self) -> bool {
        *self == A2dayAdomaebcd::A2dayAdomaebcdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2day_adomaebcd_enable(&self) -> bool {
        *self == A2dayAdomaebcd::A2dayAdomaebcdEnable
    }
}
#[doc = "Field `A2DAY_ADOMAEBCD` writer - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A2dayAdomaebcdW<'a, REG> = crate::BitWriter<'a, REG, A2dayAdomaebcd>;
impl<'a, REG> A2dayAdomaebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2day_adomaebcd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2dayAdomaebcd::A2dayAdomaebcdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2day_adomaebcd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2dayAdomaebcd::A2dayAdomaebcdEnable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a2day_adow(&self) -> A2dayAdowR {
        A2dayAdowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a2day_adowae(&self) -> A2dayAdowaeR {
        A2dayAdowaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2day_adombin(&self) -> A2dayAdombinR {
        A2dayAdombinR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2day_adomaebin(&self) -> A2dayAdomaebinR {
        A2dayAdomaebinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2day_adomlowbcd(&self) -> A2dayAdomlowbcdR {
        A2dayAdomlowbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2day_adomhighbcd(&self) -> A2dayAdomhighbcdR {
        A2dayAdomhighbcdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2day_adomaebcd(&self) -> A2dayAdomaebcdR {
        A2dayAdomaebcdR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a2day_adow(&mut self) -> A2dayAdowW<A2daySpec> {
        A2dayAdowW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a2day_adowae(&mut self) -> A2dayAdowaeW<A2daySpec> {
        A2dayAdowaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2day_adombin(&mut self) -> A2dayAdombinW<A2daySpec> {
        A2dayAdombinW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2day_adomaebin(&mut self) -> A2dayAdomaebinW<A2daySpec> {
        A2dayAdomaebinW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2day_adomlowbcd(&mut self) -> A2dayAdomlowbcdW<A2daySpec> {
        A2dayAdomlowbcdW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2day_adomhighbcd(&mut self) -> A2dayAdomhighbcdW<A2daySpec> {
        A2dayAdomhighbcdW::new(self, 20)
    }
    #[doc = "Bit 23 - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2day_adomaebcd(&mut self) -> A2dayAdomaebcdW<A2daySpec> {
        A2dayAdomaebcdW::new(self, 23)
    }
}
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a2day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2daySpec;
impl crate::RegisterSpec for A2daySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2day::R`](R) reader structure"]
impl crate::Readable for A2daySpec {}
#[doc = "`write(|w| ..)` method takes [`a2day::W`](W) writer structure"]
impl crate::Writable for A2daySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A2DAY to value 0"]
impl crate::Resettable for A2daySpec {
    const RESET_VALUE: u32 = 0;
}

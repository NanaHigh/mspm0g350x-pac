#[doc = "Register `A1DAY` reader"]
pub type R = crate::R<A1daySpec>;
#[doc = "Register `A1DAY` writer"]
pub type W = crate::W<A1daySpec>;
#[doc = "Field `A1DAY_ADOW` reader - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type A1dayAdowR = crate::FieldReader;
#[doc = "Field `A1DAY_ADOW` writer - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
pub type A1dayAdowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1dayAdowae {
    #[doc = "0: DISABLE"]
    A1dayAdowaeDisable = 0,
    #[doc = "1: ENABLE"]
    A1dayAdowaeEnable = 1,
}
impl From<A1dayAdowae> for bool {
    #[inline(always)]
    fn from(variant: A1dayAdowae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1DAY_ADOWAE` reader - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
pub type A1dayAdowaeR = crate::BitReader<A1dayAdowae>;
impl A1dayAdowaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1dayAdowae {
        match self.bits {
            false => A1dayAdowae::A1dayAdowaeDisable,
            true => A1dayAdowae::A1dayAdowaeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1day_adowae_disable(&self) -> bool {
        *self == A1dayAdowae::A1dayAdowaeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1day_adowae_enable(&self) -> bool {
        *self == A1dayAdowae::A1dayAdowaeEnable
    }
}
#[doc = "Field `A1DAY_ADOWAE` writer - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
pub type A1dayAdowaeW<'a, REG> = crate::BitWriter<'a, REG, A1dayAdowae>;
impl<'a, REG> A1dayAdowaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1day_adowae_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1dayAdowae::A1dayAdowaeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1day_adowae_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1dayAdowae::A1dayAdowaeEnable)
    }
}
#[doc = "Field `A1DAY_ADOMBIN` reader - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A1dayAdombinR = crate::FieldReader;
#[doc = "Field `A1DAY_ADOMBIN` writer - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A1dayAdombinW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1dayAdomaebin {
    #[doc = "0: DISABLE"]
    A1dayAdomaebinDisable = 0,
    #[doc = "1: ENABLE"]
    A1dayAdomaebinEnable = 1,
}
impl From<A1dayAdomaebin> for bool {
    #[inline(always)]
    fn from(variant: A1dayAdomaebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1DAY_ADOMAEBIN` reader - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A1dayAdomaebinR = crate::BitReader<A1dayAdomaebin>;
impl A1dayAdomaebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1dayAdomaebin {
        match self.bits {
            false => A1dayAdomaebin::A1dayAdomaebinDisable,
            true => A1dayAdomaebin::A1dayAdomaebinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1day_adomaebin_disable(&self) -> bool {
        *self == A1dayAdomaebin::A1dayAdomaebinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1day_adomaebin_enable(&self) -> bool {
        *self == A1dayAdomaebin::A1dayAdomaebinEnable
    }
}
#[doc = "Field `A1DAY_ADOMAEBIN` writer - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A1dayAdomaebinW<'a, REG> = crate::BitWriter<'a, REG, A1dayAdomaebin>;
impl<'a, REG> A1dayAdomaebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1day_adomaebin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1dayAdomaebin::A1dayAdomaebinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1day_adomaebin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1dayAdomaebin::A1dayAdomaebinEnable)
    }
}
#[doc = "Field `A1DAY_ADOMLOWBCD` reader - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1dayAdomlowbcdR = crate::FieldReader;
#[doc = "Field `A1DAY_ADOMLOWBCD` writer - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1dayAdomlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A1DAY_ADOMHIGHBCD` reader - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1dayAdomhighbcdR = crate::FieldReader;
#[doc = "Field `A1DAY_ADOMHIGHBCD` writer - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1dayAdomhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1dayAdomaebcd {
    #[doc = "0: DISABLE"]
    A1dayAdomaebcdDisable = 0,
    #[doc = "1: ENABLE"]
    A1dayAdomaebcdEnable = 1,
}
impl From<A1dayAdomaebcd> for bool {
    #[inline(always)]
    fn from(variant: A1dayAdomaebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1DAY_ADOMAEBCD` reader - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A1dayAdomaebcdR = crate::BitReader<A1dayAdomaebcd>;
impl A1dayAdomaebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1dayAdomaebcd {
        match self.bits {
            false => A1dayAdomaebcd::A1dayAdomaebcdDisable,
            true => A1dayAdomaebcd::A1dayAdomaebcdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1day_adomaebcd_disable(&self) -> bool {
        *self == A1dayAdomaebcd::A1dayAdomaebcdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1day_adomaebcd_enable(&self) -> bool {
        *self == A1dayAdomaebcd::A1dayAdomaebcdEnable
    }
}
#[doc = "Field `A1DAY_ADOMAEBCD` writer - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A1dayAdomaebcdW<'a, REG> = crate::BitWriter<'a, REG, A1dayAdomaebcd>;
impl<'a, REG> A1dayAdomaebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1day_adomaebcd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1dayAdomaebcd::A1dayAdomaebcdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1day_adomaebcd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1dayAdomaebcd::A1dayAdomaebcdEnable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a1day_adow(&self) -> A1dayAdowR {
        A1dayAdowR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a1day_adowae(&self) -> A1dayAdowaeR {
        A1dayAdowaeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1day_adombin(&self) -> A1dayAdombinR {
        A1dayAdombinR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1day_adomaebin(&self) -> A1dayAdomaebinR {
        A1dayAdomaebinR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1day_adomlowbcd(&self) -> A1dayAdomlowbcdR {
        A1dayAdomlowbcdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1day_adomhighbcd(&self) -> A1dayAdomhighbcdR {
        A1dayAdomhighbcdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1day_adomaebcd(&self) -> A1dayAdomaebcdR {
        A1dayAdomaebcdR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alarm Day of week (0 to 6). These bits are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a1day_adow(&mut self) -> A1dayAdowW<A1daySpec> {
        A1dayAdowW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Day of week enable. This bit are valid if RTCBCD=1 or RTCBCD=0."]
    #[inline(always)]
    pub fn a1day_adowae(&mut self) -> A1dayAdowaeW<A1daySpec> {
        A1dayAdowaeW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Alarm Day of month Binary (1 to 28, 29, 30, 31) If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1day_adombin(&mut self) -> A1dayAdombinW<A1daySpec> {
        A1dayAdombinW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm Day of month Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1day_adomaebin(&mut self) -> A1dayAdomaebinW<A1daySpec> {
        A1dayAdomaebinW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Alarm Day of month BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1day_adomlowbcd(&mut self) -> A1dayAdomlowbcdW<A1daySpec> {
        A1dayAdomlowbcdW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Alarm Day of month BCD high digit (0 to 3). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1day_adomhighbcd(&mut self) -> A1dayAdomhighbcdW<A1daySpec> {
        A1dayAdomhighbcdW::new(self, 20)
    }
    #[doc = "Bit 23 - Alarm Day of month BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1day_adomaebcd(&mut self) -> A1dayAdomaebcdW<A1daySpec> {
        A1dayAdomaebcdW::new(self, 23)
    }
}
#[doc = "RTC Alarm Day Of Week / Month Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a1day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1daySpec;
impl crate::RegisterSpec for A1daySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a1day::R`](R) reader structure"]
impl crate::Readable for A1daySpec {}
#[doc = "`write(|w| ..)` method takes [`a1day::W`](W) writer structure"]
impl crate::Writable for A1daySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A1DAY to value 0"]
impl crate::Resettable for A1daySpec {
    const RESET_VALUE: u32 = 0;
}

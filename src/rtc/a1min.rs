#[doc = "Register `A1MIN` reader"]
pub type R = crate::R<A1minSpec>;
#[doc = "Register `A1MIN` writer"]
pub type W = crate::W<A1minSpec>;
#[doc = "Field `A1MIN_AMINBIN` reader - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A1minAminbinR = crate::FieldReader;
#[doc = "Field `A1MIN_AMINBIN` writer - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A1minAminbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1minAminaebin {
    #[doc = "0: DISABLE"]
    A1minAminaebinDisable = 0,
    #[doc = "1: ENABLE"]
    A1minAminaebinEnable = 1,
}
impl From<A1minAminaebin> for bool {
    #[inline(always)]
    fn from(variant: A1minAminaebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1MIN_AMINAEBIN` reader - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A1minAminaebinR = crate::BitReader<A1minAminaebin>;
impl A1minAminaebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1minAminaebin {
        match self.bits {
            false => A1minAminaebin::A1minAminaebinDisable,
            true => A1minAminaebin::A1minAminaebinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1min_aminaebin_disable(&self) -> bool {
        *self == A1minAminaebin::A1minAminaebinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1min_aminaebin_enable(&self) -> bool {
        *self == A1minAminaebin::A1minAminaebinEnable
    }
}
#[doc = "Field `A1MIN_AMINAEBIN` writer - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A1minAminaebinW<'a, REG> = crate::BitWriter<'a, REG, A1minAminaebin>;
impl<'a, REG> A1minAminaebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1min_aminaebin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1minAminaebin::A1minAminaebinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1min_aminaebin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1minAminaebin::A1minAminaebinEnable)
    }
}
#[doc = "Field `A1MIN_AMINLOWBCD` reader - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1minAminlowbcdR = crate::FieldReader;
#[doc = "Field `A1MIN_AMINLOWBCD` writer - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1minAminlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A1MIN_AMINHIGHBCD` reader - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1minAminhighbcdR = crate::FieldReader;
#[doc = "Field `A1MIN_AMINHIGHBCD` writer - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A1minAminhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A1minAminaebcd {
    #[doc = "0: DISABLE"]
    A1minAminaebcdDisable = 0,
    #[doc = "1: ENABLE"]
    A1minAminaebcdEnable = 1,
}
impl From<A1minAminaebcd> for bool {
    #[inline(always)]
    fn from(variant: A1minAminaebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A1MIN_AMINAEBCD` reader - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A1minAminaebcdR = crate::BitReader<A1minAminaebcd>;
impl A1minAminaebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A1minAminaebcd {
        match self.bits {
            false => A1minAminaebcd::A1minAminaebcdDisable,
            true => A1minAminaebcd::A1minAminaebcdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a1min_aminaebcd_disable(&self) -> bool {
        *self == A1minAminaebcd::A1minAminaebcdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a1min_aminaebcd_enable(&self) -> bool {
        *self == A1minAminaebcd::A1minAminaebcdEnable
    }
}
#[doc = "Field `A1MIN_AMINAEBCD` writer - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A1minAminaebcdW<'a, REG> = crate::BitWriter<'a, REG, A1minAminaebcd>;
impl<'a, REG> A1minAminaebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a1min_aminaebcd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A1minAminaebcd::A1minAminaebcdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a1min_aminaebcd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A1minAminaebcd::A1minAminaebcdEnable)
    }
}
impl R {
    #[doc = "Bits 0:5 - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1min_aminbin(&self) -> A1minAminbinR {
        A1minAminbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1min_aminaebin(&self) -> A1minAminaebinR {
        A1minAminaebinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1min_aminlowbcd(&self) -> A1minAminlowbcdR {
        A1minAminlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1min_aminhighbcd(&self) -> A1minAminhighbcdR {
        A1minAminhighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1min_aminaebcd(&self) -> A1minAminaebcdR {
        A1minAminaebcdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1min_aminbin(&mut self) -> A1minAminbinW<A1minSpec> {
        A1minAminbinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1min_aminaebin(&mut self) -> A1minAminaebinW<A1minSpec> {
        A1minAminaebinW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1min_aminlowbcd(&mut self) -> A1minAminlowbcdW<A1minSpec> {
        A1minAminlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a1min_aminhighbcd(&mut self) -> A1minAminhighbcdW<A1minSpec> {
        A1minAminhighbcdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a1min_aminaebcd(&mut self) -> A1minAminaebcdW<A1minSpec> {
        A1minAminaebcdW::new(self, 15)
    }
}
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a1min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1minSpec;
impl crate::RegisterSpec for A1minSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a1min::R`](R) reader structure"]
impl crate::Readable for A1minSpec {}
#[doc = "`write(|w| ..)` method takes [`a1min::W`](W) writer structure"]
impl crate::Writable for A1minSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A1MIN to value 0"]
impl crate::Resettable for A1minSpec {
    const RESET_VALUE: u32 = 0;
}

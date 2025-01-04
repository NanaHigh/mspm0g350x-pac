#[doc = "Register `A2MIN` reader"]
pub type R = crate::R<A2minSpec>;
#[doc = "Register `A2MIN` writer"]
pub type W = crate::W<A2minSpec>;
#[doc = "Field `A2MIN_AMINBIN` reader - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A2minAminbinR = crate::FieldReader;
#[doc = "Field `A2MIN_AMINBIN` writer - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
pub type A2minAminbinW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2minAminaebin {
    #[doc = "0: DISABLE"]
    A2minAminaebinDisable = 0,
    #[doc = "1: ENABLE"]
    A2minAminaebinEnable = 1,
}
impl From<A2minAminaebin> for bool {
    #[inline(always)]
    fn from(variant: A2minAminaebin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2MIN_AMINAEBIN` reader - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A2minAminaebinR = crate::BitReader<A2minAminaebin>;
impl A2minAminaebinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2minAminaebin {
        match self.bits {
            false => A2minAminaebin::A2minAminaebinDisable,
            true => A2minAminaebin::A2minAminaebinEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2min_aminaebin_disable(&self) -> bool {
        *self == A2minAminaebin::A2minAminaebinDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2min_aminaebin_enable(&self) -> bool {
        *self == A2minAminaebin::A2minAminaebinEnable
    }
}
#[doc = "Field `A2MIN_AMINAEBIN` writer - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
pub type A2minAminaebinW<'a, REG> = crate::BitWriter<'a, REG, A2minAminaebin>;
impl<'a, REG> A2minAminaebinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2min_aminaebin_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2minAminaebin::A2minAminaebinDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2min_aminaebin_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2minAminaebin::A2minAminaebinEnable)
    }
}
#[doc = "Field `A2MIN_AMINLOWBCD` reader - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2minAminlowbcdR = crate::FieldReader;
#[doc = "Field `A2MIN_AMINLOWBCD` writer - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2minAminlowbcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `A2MIN_AMINHIGHBCD` reader - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2minAminhighbcdR = crate::FieldReader;
#[doc = "Field `A2MIN_AMINHIGHBCD` writer - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
pub type A2minAminhighbcdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2minAminaebcd {
    #[doc = "0: DISABLE"]
    A2minAminaebcdDisable = 0,
    #[doc = "1: ENABLE"]
    A2minAminaebcdEnable = 1,
}
impl From<A2minAminaebcd> for bool {
    #[inline(always)]
    fn from(variant: A2minAminaebcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `A2MIN_AMINAEBCD` reader - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A2minAminaebcdR = crate::BitReader<A2minAminaebcd>;
impl A2minAminaebcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> A2minAminaebcd {
        match self.bits {
            false => A2minAminaebcd::A2minAminaebcdDisable,
            true => A2minAminaebcd::A2minAminaebcdEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_a2min_aminaebcd_disable(&self) -> bool {
        *self == A2minAminaebcd::A2minAminaebcdDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_a2min_aminaebcd_enable(&self) -> bool {
        *self == A2minAminaebcd::A2minAminaebcdEnable
    }
}
#[doc = "Field `A2MIN_AMINAEBCD` writer - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
pub type A2minAminaebcdW<'a, REG> = crate::BitWriter<'a, REG, A2minAminaebcd>;
impl<'a, REG> A2minAminaebcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn a2min_aminaebcd_disable(self) -> &'a mut crate::W<REG> {
        self.variant(A2minAminaebcd::A2minAminaebcdDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn a2min_aminaebcd_enable(self) -> &'a mut crate::W<REG> {
        self.variant(A2minAminaebcd::A2minAminaebcdEnable)
    }
}
impl R {
    #[doc = "Bits 0:5 - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2min_aminbin(&self) -> A2minAminbinR {
        A2minAminbinR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2min_aminaebin(&self) -> A2minAminaebinR {
        A2minAminaebinR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2min_aminlowbcd(&self) -> A2minAminlowbcdR {
        A2minAminlowbcdR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2min_aminhighbcd(&self) -> A2minAminhighbcdR {
        A2minAminhighbcdR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2min_aminaebcd(&self) -> A2minAminaebcdR {
        A2minAminaebcdR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Minutes Binary (0 to 59). If RTCBCD=1 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2min_aminbin(&mut self) -> A2minAminbinW<A2minSpec> {
        A2minAminbinW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Minutes Binary enable. If RTCBCD=1 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2min_aminaebin(&mut self) -> A2minAminaebinW<A2minSpec> {
        A2minAminaebinW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Alarm Minutes BCD low digit (0 to 9). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2min_aminlowbcd(&mut self) -> A2minAminlowbcdW<A2minSpec> {
        A2minAminlowbcdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Alarm Minutes BCD high digit (0 to 5). If RTCBCD=0 write to these bits will be ignored and read give the value 0."]
    #[inline(always)]
    pub fn a2min_aminhighbcd(&mut self) -> A2minAminhighbcdW<A2minSpec> {
        A2minAminhighbcdW::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm Minutes BCD enable. If RTCBCD=0 this bit is always 0. Write to this bit will be ignored."]
    #[inline(always)]
    pub fn a2min_aminaebcd(&mut self) -> A2minAminaebcdW<A2minSpec> {
        A2minAminaebcdW::new(self, 15)
    }
}
#[doc = "RTC Minute Alarm Register - Calendar Mode With Binary / BCD Format\n\nYou can [`read`](crate::Reg::read) this register and get [`a2min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2minSpec;
impl crate::RegisterSpec for A2minSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a2min::R`](R) reader structure"]
impl crate::Readable for A2minSpec {}
#[doc = "`write(|w| ..)` method takes [`a2min::W`](W) writer structure"]
impl crate::Writable for A2minSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A2MIN to value 0"]
impl crate::Resettable for A2minSpec {
    const RESET_VALUE: u32 = 0;
}

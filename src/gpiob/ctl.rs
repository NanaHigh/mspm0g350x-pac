#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "FASTWAKEONLY for the global control of fastwake\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlFastwakeonly {
    #[doc = "0: NOT_GLOBAL_EN"]
    CtlFastwakeonlyNotGlobalEn = 0,
    #[doc = "1: GLOBAL_EN"]
    CtlFastwakeonlyGlobalEn = 1,
}
impl From<CtlFastwakeonly> for bool {
    #[inline(always)]
    fn from(variant: CtlFastwakeonly) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_FASTWAKEONLY` reader - FASTWAKEONLY for the global control of fastwake"]
pub type CtlFastwakeonlyR = crate::BitReader<CtlFastwakeonly>;
impl CtlFastwakeonlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlFastwakeonly {
        match self.bits {
            false => CtlFastwakeonly::CtlFastwakeonlyNotGlobalEn,
            true => CtlFastwakeonly::CtlFastwakeonlyGlobalEn,
        }
    }
    #[doc = "NOT_GLOBAL_EN"]
    #[inline(always)]
    pub fn is_ctl_fastwakeonly_not_global_en(&self) -> bool {
        *self == CtlFastwakeonly::CtlFastwakeonlyNotGlobalEn
    }
    #[doc = "GLOBAL_EN"]
    #[inline(always)]
    pub fn is_ctl_fastwakeonly_global_en(&self) -> bool {
        *self == CtlFastwakeonly::CtlFastwakeonlyGlobalEn
    }
}
#[doc = "Field `CTL_FASTWAKEONLY` writer - FASTWAKEONLY for the global control of fastwake"]
pub type CtlFastwakeonlyW<'a, REG> = crate::BitWriter<'a, REG, CtlFastwakeonly>;
impl<'a, REG> CtlFastwakeonlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOT_GLOBAL_EN"]
    #[inline(always)]
    pub fn ctl_fastwakeonly_not_global_en(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFastwakeonly::CtlFastwakeonlyNotGlobalEn)
    }
    #[doc = "GLOBAL_EN"]
    #[inline(always)]
    pub fn ctl_fastwakeonly_global_en(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFastwakeonly::CtlFastwakeonlyGlobalEn)
    }
}
impl R {
    #[doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"]
    #[inline(always)]
    pub fn ctl_fastwakeonly(&self) -> CtlFastwakeonlyR {
        CtlFastwakeonlyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FASTWAKEONLY for the global control of fastwake"]
    #[inline(always)]
    pub fn ctl_fastwakeonly(&mut self) -> CtlFastwakeonlyW<CtlSpec> {
        CtlFastwakeonlyW::new(self, 0)
    }
}
#[doc = "FAST WAKE GLOBAL EN\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}

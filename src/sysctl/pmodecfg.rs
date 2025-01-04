#[doc = "Register `PMODECFG` reader"]
pub type R = crate::R<PmodecfgSpec>;
#[doc = "Register `PMODECFG` writer"]
pub type W = crate::W<PmodecfgSpec>;
#[doc = "DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PmodecfgDsleep {
    #[doc = "0: STOP"]
    PmodecfgDsleepStop = 0,
    #[doc = "1: STANDBY"]
    PmodecfgDsleepStandby = 1,
    #[doc = "2: SHUTDOWN"]
    PmodecfgDsleepShutdown = 2,
}
impl From<PmodecfgDsleep> for u8 {
    #[inline(always)]
    fn from(variant: PmodecfgDsleep) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PmodecfgDsleep {
    type Ux = u8;
}
impl crate::IsEnum for PmodecfgDsleep {}
#[doc = "Field `PMODECFG_DSLEEP` reader - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
pub type PmodecfgDsleepR = crate::FieldReader<PmodecfgDsleep>;
impl PmodecfgDsleepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PmodecfgDsleep> {
        match self.bits {
            0 => Some(PmodecfgDsleep::PmodecfgDsleepStop),
            1 => Some(PmodecfgDsleep::PmodecfgDsleepStandby),
            2 => Some(PmodecfgDsleep::PmodecfgDsleepShutdown),
            _ => None,
        }
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn is_pmodecfg_dsleep_stop(&self) -> bool {
        *self == PmodecfgDsleep::PmodecfgDsleepStop
    }
    #[doc = "STANDBY"]
    #[inline(always)]
    pub fn is_pmodecfg_dsleep_standby(&self) -> bool {
        *self == PmodecfgDsleep::PmodecfgDsleepStandby
    }
    #[doc = "SHUTDOWN"]
    #[inline(always)]
    pub fn is_pmodecfg_dsleep_shutdown(&self) -> bool {
        *self == PmodecfgDsleep::PmodecfgDsleepShutdown
    }
}
#[doc = "Field `PMODECFG_DSLEEP` writer - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
pub type PmodecfgDsleepW<'a, REG> = crate::FieldWriter<'a, REG, 2, PmodecfgDsleep>;
impl<'a, REG> PmodecfgDsleepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STOP"]
    #[inline(always)]
    pub fn pmodecfg_dsleep_stop(self) -> &'a mut crate::W<REG> {
        self.variant(PmodecfgDsleep::PmodecfgDsleepStop)
    }
    #[doc = "STANDBY"]
    #[inline(always)]
    pub fn pmodecfg_dsleep_standby(self) -> &'a mut crate::W<REG> {
        self.variant(PmodecfgDsleep::PmodecfgDsleepStandby)
    }
    #[doc = "SHUTDOWN"]
    #[inline(always)]
    pub fn pmodecfg_dsleep_shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(PmodecfgDsleep::PmodecfgDsleepShutdown)
    }
}
impl R {
    #[doc = "Bits 0:1 - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
    #[inline(always)]
    pub fn pmodecfg_dsleep(&self) -> PmodecfgDsleepR {
        PmodecfgDsleepR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DSLEEP selects the operating mode to enter upon a DEEPSLEEP request from the CPU."]
    #[inline(always)]
    pub fn pmodecfg_dsleep(&mut self) -> PmodecfgDsleepW<PmodecfgSpec> {
        PmodecfgDsleepW::new(self, 0)
    }
}
#[doc = "Power mode configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`pmodecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmodecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmodecfgSpec;
impl crate::RegisterSpec for PmodecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmodecfg::R`](R) reader structure"]
impl crate::Readable for PmodecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pmodecfg::W`](W) writer structure"]
impl crate::Writable for PmodecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMODECFG to value 0"]
impl crate::Resettable for PmodecfgSpec {
    const RESET_VALUE: u32 = 0;
}

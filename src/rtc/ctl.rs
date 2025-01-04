#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Real-time clock time event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtlRtctevtx {
    #[doc = "0: MINUTE"]
    CtlRtctevtxMinute = 0,
    #[doc = "1: HOUR"]
    CtlRtctevtxHour = 1,
    #[doc = "2: MIDNIGHT"]
    CtlRtctevtxMidnight = 2,
    #[doc = "3: NOON"]
    CtlRtctevtxNoon = 3,
}
impl From<CtlRtctevtx> for u8 {
    #[inline(always)]
    fn from(variant: CtlRtctevtx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtlRtctevtx {
    type Ux = u8;
}
impl crate::IsEnum for CtlRtctevtx {}
#[doc = "Field `CTL_RTCTEVTX` reader - Real-time clock time event."]
pub type CtlRtctevtxR = crate::FieldReader<CtlRtctevtx>;
impl CtlRtctevtxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlRtctevtx {
        match self.bits {
            0 => CtlRtctevtx::CtlRtctevtxMinute,
            1 => CtlRtctevtx::CtlRtctevtxHour,
            2 => CtlRtctevtx::CtlRtctevtxMidnight,
            3 => CtlRtctevtx::CtlRtctevtxNoon,
            _ => unreachable!(),
        }
    }
    #[doc = "MINUTE"]
    #[inline(always)]
    pub fn is_ctl_rtctevtx_minute(&self) -> bool {
        *self == CtlRtctevtx::CtlRtctevtxMinute
    }
    #[doc = "HOUR"]
    #[inline(always)]
    pub fn is_ctl_rtctevtx_hour(&self) -> bool {
        *self == CtlRtctevtx::CtlRtctevtxHour
    }
    #[doc = "MIDNIGHT"]
    #[inline(always)]
    pub fn is_ctl_rtctevtx_midnight(&self) -> bool {
        *self == CtlRtctevtx::CtlRtctevtxMidnight
    }
    #[doc = "NOON"]
    #[inline(always)]
    pub fn is_ctl_rtctevtx_noon(&self) -> bool {
        *self == CtlRtctevtx::CtlRtctevtxNoon
    }
}
#[doc = "Field `CTL_RTCTEVTX` writer - Real-time clock time event."]
pub type CtlRtctevtxW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtlRtctevtx, crate::Safe>;
impl<'a, REG> CtlRtctevtxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MINUTE"]
    #[inline(always)]
    pub fn ctl_rtctevtx_minute(self) -> &'a mut crate::W<REG> {
        self.variant(CtlRtctevtx::CtlRtctevtxMinute)
    }
    #[doc = "HOUR"]
    #[inline(always)]
    pub fn ctl_rtctevtx_hour(self) -> &'a mut crate::W<REG> {
        self.variant(CtlRtctevtx::CtlRtctevtxHour)
    }
    #[doc = "MIDNIGHT"]
    #[inline(always)]
    pub fn ctl_rtctevtx_midnight(self) -> &'a mut crate::W<REG> {
        self.variant(CtlRtctevtx::CtlRtctevtxMidnight)
    }
    #[doc = "NOON"]
    #[inline(always)]
    pub fn ctl_rtctevtx_noon(self) -> &'a mut crate::W<REG> {
        self.variant(CtlRtctevtx::CtlRtctevtxNoon)
    }
}
#[doc = "Real-time clock BCD select. Selects BCD counting for real-time clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlRtcbcd {
    #[doc = "0: BINARY"]
    CtlRtcbcdBinary = 0,
    #[doc = "1: BCD"]
    CtlRtcbcdBcd = 1,
}
impl From<CtlRtcbcd> for bool {
    #[inline(always)]
    fn from(variant: CtlRtcbcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_RTCBCD` reader - Real-time clock BCD select. Selects BCD counting for real-time clock."]
pub type CtlRtcbcdR = crate::BitReader<CtlRtcbcd>;
impl CtlRtcbcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlRtcbcd {
        match self.bits {
            false => CtlRtcbcd::CtlRtcbcdBinary,
            true => CtlRtcbcd::CtlRtcbcdBcd,
        }
    }
    #[doc = "BINARY"]
    #[inline(always)]
    pub fn is_ctl_rtcbcd_binary(&self) -> bool {
        *self == CtlRtcbcd::CtlRtcbcdBinary
    }
    #[doc = "BCD"]
    #[inline(always)]
    pub fn is_ctl_rtcbcd_bcd(&self) -> bool {
        *self == CtlRtcbcd::CtlRtcbcdBcd
    }
}
#[doc = "Field `CTL_RTCBCD` writer - Real-time clock BCD select. Selects BCD counting for real-time clock."]
pub type CtlRtcbcdW<'a, REG> = crate::BitWriter<'a, REG, CtlRtcbcd>;
impl<'a, REG> CtlRtcbcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BINARY"]
    #[inline(always)]
    pub fn ctl_rtcbcd_binary(self) -> &'a mut crate::W<REG> {
        self.variant(CtlRtcbcd::CtlRtcbcdBinary)
    }
    #[doc = "BCD"]
    #[inline(always)]
    pub fn ctl_rtcbcd_bcd(self) -> &'a mut crate::W<REG> {
        self.variant(CtlRtcbcd::CtlRtcbcdBcd)
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-time clock time event."]
    #[inline(always)]
    pub fn ctl_rtctevtx(&self) -> CtlRtctevtxR {
        CtlRtctevtxR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Real-time clock BCD select. Selects BCD counting for real-time clock."]
    #[inline(always)]
    pub fn ctl_rtcbcd(&self) -> CtlRtcbcdR {
        CtlRtcbcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-time clock time event."]
    #[inline(always)]
    pub fn ctl_rtctevtx(&mut self) -> CtlRtctevtxW<CtlSpec> {
        CtlRtctevtxW::new(self, 0)
    }
    #[doc = "Bit 7 - Real-time clock BCD select. Selects BCD counting for real-time clock."]
    #[inline(always)]
    pub fn ctl_rtcbcd(&mut self) -> CtlRtcbcdW<CtlSpec> {
        CtlRtcbcdW::new(self, 7)
    }
}
#[doc = "RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

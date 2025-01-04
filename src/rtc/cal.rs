#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `CAL_RTCOCALX` reader - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
pub type CalRtcocalxR = crate::FieldReader;
#[doc = "Field `CAL_RTCOCALX` writer - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
pub type CalRtcocalxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalRtcocals {
    #[doc = "0: DOWN"]
    CalRtcocalsDown = 0,
    #[doc = "1: UP"]
    CalRtcocalsUp = 1,
}
impl From<CalRtcocals> for bool {
    #[inline(always)]
    fn from(variant: CalRtcocals) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_RTCOCALS` reader - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
pub type CalRtcocalsR = crate::BitReader<CalRtcocals>;
impl CalRtcocalsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalRtcocals {
        match self.bits {
            false => CalRtcocals::CalRtcocalsDown,
            true => CalRtcocals::CalRtcocalsUp,
        }
    }
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn is_cal_rtcocals_down(&self) -> bool {
        *self == CalRtcocals::CalRtcocalsDown
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn is_cal_rtcocals_up(&self) -> bool {
        *self == CalRtcocals::CalRtcocalsUp
    }
}
#[doc = "Field `CAL_RTCOCALS` writer - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
pub type CalRtcocalsW<'a, REG> = crate::BitWriter<'a, REG, CalRtcocals>;
impl<'a, REG> CalRtcocalsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn cal_rtcocals_down(self) -> &'a mut crate::W<REG> {
        self.variant(CalRtcocals::CalRtcocalsDown)
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn cal_rtcocals_up(self) -> &'a mut crate::W<REG> {
        self.variant(CalRtcocals::CalRtcocalsUp)
    }
}
#[doc = "Real-time clock calibration frequency. Selects frequency output to RTC_OUT pin for calibration measurement. The corresponding port must be configured for the peripheral module function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CalRtccalfx {
    #[doc = "0: OFF"]
    CalRtccalfxOff = 0,
    #[doc = "1: F512HZ"]
    CalRtccalfxF512hz = 1,
    #[doc = "2: F256HZ"]
    CalRtccalfxF256hz = 2,
    #[doc = "3: F1HZ"]
    CalRtccalfxF1hz = 3,
}
impl From<CalRtccalfx> for u8 {
    #[inline(always)]
    fn from(variant: CalRtccalfx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CalRtccalfx {
    type Ux = u8;
}
impl crate::IsEnum for CalRtccalfx {}
#[doc = "Field `CAL_RTCCALFX` reader - Real-time clock calibration frequency. Selects frequency output to RTC_OUT pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
pub type CalRtccalfxR = crate::FieldReader<CalRtccalfx>;
impl CalRtccalfxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalRtccalfx {
        match self.bits {
            0 => CalRtccalfx::CalRtccalfxOff,
            1 => CalRtccalfx::CalRtccalfxF512hz,
            2 => CalRtccalfx::CalRtccalfxF256hz,
            3 => CalRtccalfx::CalRtccalfxF1hz,
            _ => unreachable!(),
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_cal_rtccalfx_off(&self) -> bool {
        *self == CalRtccalfx::CalRtccalfxOff
    }
    #[doc = "F512HZ"]
    #[inline(always)]
    pub fn is_cal_rtccalfx_f512hz(&self) -> bool {
        *self == CalRtccalfx::CalRtccalfxF512hz
    }
    #[doc = "F256HZ"]
    #[inline(always)]
    pub fn is_cal_rtccalfx_f256hz(&self) -> bool {
        *self == CalRtccalfx::CalRtccalfxF256hz
    }
    #[doc = "F1HZ"]
    #[inline(always)]
    pub fn is_cal_rtccalfx_f1hz(&self) -> bool {
        *self == CalRtccalfx::CalRtccalfxF1hz
    }
}
#[doc = "Field `CAL_RTCCALFX` writer - Real-time clock calibration frequency. Selects frequency output to RTC_OUT pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
pub type CalRtccalfxW<'a, REG> = crate::FieldWriter<'a, REG, 2, CalRtccalfx, crate::Safe>;
impl<'a, REG> CalRtccalfxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn cal_rtccalfx_off(self) -> &'a mut crate::W<REG> {
        self.variant(CalRtccalfx::CalRtccalfxOff)
    }
    #[doc = "F512HZ"]
    #[inline(always)]
    pub fn cal_rtccalfx_f512hz(self) -> &'a mut crate::W<REG> {
        self.variant(CalRtccalfx::CalRtccalfxF512hz)
    }
    #[doc = "F256HZ"]
    #[inline(always)]
    pub fn cal_rtccalfx_f256hz(self) -> &'a mut crate::W<REG> {
        self.variant(CalRtccalfx::CalRtccalfxF256hz)
    }
    #[doc = "F1HZ"]
    #[inline(always)]
    pub fn cal_rtccalfx_f1hz(self) -> &'a mut crate::W<REG> {
        self.variant(CalRtccalfx::CalRtccalfxF1hz)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
    #[inline(always)]
    pub fn cal_rtcocalx(&self) -> CalRtcocalxR {
        CalRtcocalxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
    #[inline(always)]
    pub fn cal_rtcocals(&self) -> CalRtcocalsR {
        CalRtcocalsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Real-time clock calibration frequency. Selects frequency output to RTC_OUT pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
    #[inline(always)]
    pub fn cal_rtccalfx(&self) -> CalRtccalfxR {
        CalRtccalfxR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration. Each LSB represents approximately +1ppm (RTCOCALXS = 1) or -1ppm (RTCOCALXS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm will be ignored by hardware."]
    #[inline(always)]
    pub fn cal_rtcocalx(&mut self) -> CalRtcocalxW<CalSpec> {
        CalRtcocalxW::new(self, 0)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign. This bit decides the sign of offset error calibration."]
    #[inline(always)]
    pub fn cal_rtcocals(&mut self) -> CalRtcocalsW<CalSpec> {
        CalRtcocalsW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Real-time clock calibration frequency. Selects frequency output to RTC_OUT pin for calibration measurement. The corresponding port must be configured for the peripheral module function."]
    #[inline(always)]
    pub fn cal_rtccalfx(&mut self) -> CalRtccalfxW<CalSpec> {
        CalRtccalfxW::new(self, 16)
    }
}
#[doc = "RTC Clock Offset Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL to value 0"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0;
}

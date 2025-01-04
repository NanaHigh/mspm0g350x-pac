#[doc = "Register `TCMP` reader"]
pub type R = crate::R<TcmpSpec>;
#[doc = "Register `TCMP` writer"]
pub type W = crate::W<TcmpSpec>;
#[doc = "Field `TCMP_RTCTCMPX` reader - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
pub type TcmpRtctcmpxR = crate::FieldReader;
#[doc = "Field `TCMP_RTCTCMPX` writer - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
pub type TcmpRtctcmpxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcmpRtctcmps {
    #[doc = "0: DOWN"]
    TcmpRtctcmpsDown = 0,
    #[doc = "1: UP"]
    TcmpRtctcmpsUp = 1,
}
impl From<TcmpRtctcmps> for bool {
    #[inline(always)]
    fn from(variant: TcmpRtctcmps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCMP_RTCTCMPS` reader - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
pub type TcmpRtctcmpsR = crate::BitReader<TcmpRtctcmps>;
impl TcmpRtctcmpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TcmpRtctcmps {
        match self.bits {
            false => TcmpRtctcmps::TcmpRtctcmpsDown,
            true => TcmpRtctcmps::TcmpRtctcmpsUp,
        }
    }
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn is_tcmp_rtctcmps_down(&self) -> bool {
        *self == TcmpRtctcmps::TcmpRtctcmpsDown
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn is_tcmp_rtctcmps_up(&self) -> bool {
        *self == TcmpRtctcmps::TcmpRtctcmpsUp
    }
}
#[doc = "Field `TCMP_RTCTCMPS` writer - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
pub type TcmpRtctcmpsW<'a, REG> = crate::BitWriter<'a, REG, TcmpRtctcmps>;
impl<'a, REG> TcmpRtctcmpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn tcmp_rtctcmps_down(self) -> &'a mut crate::W<REG> {
        self.variant(TcmpRtctcmps::TcmpRtctcmpsDown)
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn tcmp_rtctcmps_up(self) -> &'a mut crate::W<REG> {
        self.variant(TcmpRtctcmps::TcmpRtctcmpsUp)
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
    #[inline(always)]
    pub fn tcmp_rtctcmpx(&self) -> TcmpRtctcmpxR {
        TcmpRtctcmpxR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
    #[inline(always)]
    pub fn tcmp_rtctcmps(&self) -> TcmpRtctcmpsR {
        TcmpRtctcmpsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock temperature compensation. Value written into this register is used for temperature compensation of RTC. Each LSB represents approximately +1ppm (RTCTCMPS = 1) or -1ppm (RTCTCMPS = 0) adjustment in frequency. Maximum effective calibration value is +/-240ppm. Excess values written above +/-240ppm are ignored by hardware. Reading from RTCTCMP register at any time returns the cumulative value which is the signed addition of RTCOCALx and RTCTCMPX values, and the updated sign bit (RTCTCMPS) of the addition result."]
    #[inline(always)]
    pub fn tcmp_rtctcmpx(&mut self) -> TcmpRtctcmpxW<TcmpSpec> {
        TcmpRtctcmpxW::new(self, 0)
    }
    #[doc = "Bit 15 - Real-time clock temperature compensation sign. This bit decides the sign of temperature compensation."]
    #[inline(always)]
    pub fn tcmp_rtctcmps(&mut self) -> TcmpRtctcmpsW<TcmpSpec> {
        TcmpRtctcmpsW::new(self, 15)
    }
}
#[doc = "RTC Temperature Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcmpSpec;
impl crate::RegisterSpec for TcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcmp::R`](R) reader structure"]
impl crate::Readable for TcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`tcmp::W`](W) writer structure"]
impl crate::Writable for TcmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCMP to value 0"]
impl crate::Resettable for TcmpSpec {
    const RESET_VALUE: u32 = 0;
}

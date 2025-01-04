#[doc = "Register `STA` reader"]
pub type R = crate::R<StaSpec>;
#[doc = "Real-time clock ready. This bit indicates when the real-time clock time values are safe for reading.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaRtcrdy {
    #[doc = "0: NOT_READY"]
    StaRtcrdyNotReady = 0,
    #[doc = "1: READY"]
    StaRtcrdyReady = 1,
}
impl From<StaRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: StaRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STA_RTCRDY` reader - Real-time clock ready. This bit indicates when the real-time clock time values are safe for reading."]
pub type StaRtcrdyR = crate::BitReader<StaRtcrdy>;
impl StaRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StaRtcrdy {
        match self.bits {
            false => StaRtcrdy::StaRtcrdyNotReady,
            true => StaRtcrdy::StaRtcrdyReady,
        }
    }
    #[doc = "NOT_READY"]
    #[inline(always)]
    pub fn is_sta_rtcrdy_not_ready(&self) -> bool {
        *self == StaRtcrdy::StaRtcrdyNotReady
    }
    #[doc = "READY"]
    #[inline(always)]
    pub fn is_sta_rtcrdy_ready(&self) -> bool {
        *self == StaRtcrdy::StaRtcrdyReady
    }
}
#[doc = "Real-time clock temperature compensation ready. This is a read only bit that indicates when the RTCTCMPx can be written. Write to RTCTCMPx should be avoided when RTCTCRDY is reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaRtctcrdy {
    #[doc = "0: NOT_READY"]
    StaRtctcrdyNotReady = 0,
    #[doc = "1: READY"]
    StaRtctcrdyReady = 1,
}
impl From<StaRtctcrdy> for bool {
    #[inline(always)]
    fn from(variant: StaRtctcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STA_RTCTCRDY` reader - Real-time clock temperature compensation ready. This is a read only bit that indicates when the RTCTCMPx can be written. Write to RTCTCMPx should be avoided when RTCTCRDY is reset."]
pub type StaRtctcrdyR = crate::BitReader<StaRtctcrdy>;
impl StaRtctcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StaRtctcrdy {
        match self.bits {
            false => StaRtctcrdy::StaRtctcrdyNotReady,
            true => StaRtctcrdy::StaRtctcrdyReady,
        }
    }
    #[doc = "NOT_READY"]
    #[inline(always)]
    pub fn is_sta_rtctcrdy_not_ready(&self) -> bool {
        *self == StaRtctcrdy::StaRtctcrdyNotReady
    }
    #[doc = "READY"]
    #[inline(always)]
    pub fn is_sta_rtctcrdy_ready(&self) -> bool {
        *self == StaRtctcrdy::StaRtctcrdyReady
    }
}
#[doc = "Real-time clock temperature compensation write OK. This is a read-only bit that indicates if the write to RTCTCMP is successful or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaRtctcok {
    #[doc = "0: NOT_OK"]
    StaRtctcokNotOk = 0,
    #[doc = "1: OK"]
    StaRtctcokOk = 1,
}
impl From<StaRtctcok> for bool {
    #[inline(always)]
    fn from(variant: StaRtctcok) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STA_RTCTCOK` reader - Real-time clock temperature compensation write OK. This is a read-only bit that indicates if the write to RTCTCMP is successful or not."]
pub type StaRtctcokR = crate::BitReader<StaRtctcok>;
impl StaRtctcokR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StaRtctcok {
        match self.bits {
            false => StaRtctcok::StaRtctcokNotOk,
            true => StaRtctcok::StaRtctcokOk,
        }
    }
    #[doc = "NOT_OK"]
    #[inline(always)]
    pub fn is_sta_rtctcok_not_ok(&self) -> bool {
        *self == StaRtctcok::StaRtctcokNotOk
    }
    #[doc = "OK"]
    #[inline(always)]
    pub fn is_sta_rtctcok_ok(&self) -> bool {
        *self == StaRtctcok::StaRtctcokOk
    }
}
impl R {
    #[doc = "Bit 0 - Real-time clock ready. This bit indicates when the real-time clock time values are safe for reading."]
    #[inline(always)]
    pub fn sta_rtcrdy(&self) -> StaRtcrdyR {
        StaRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Real-time clock temperature compensation ready. This is a read only bit that indicates when the RTCTCMPx can be written. Write to RTCTCMPx should be avoided when RTCTCRDY is reset."]
    #[inline(always)]
    pub fn sta_rtctcrdy(&self) -> StaRtctcrdyR {
        StaRtctcrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time clock temperature compensation write OK. This is a read-only bit that indicates if the write to RTCTCMP is successful or not."]
    #[inline(always)]
    pub fn sta_rtctcok(&self) -> StaRtctcokR {
        StaRtctcokR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "RTC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaSpec;
impl crate::RegisterSpec for StaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sta::R`](R) reader structure"]
impl crate::Readable for StaSpec {}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for StaSpec {
    const RESET_VALUE: u32 = 0;
}

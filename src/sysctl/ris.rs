#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Raw status of the LFOSCGOOD interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisLfoscgood {
    #[doc = "0: FALSE"]
    RisLfoscgoodFalse = 0,
    #[doc = "1: TRUE"]
    RisLfoscgoodTrue = 1,
}
impl From<RisLfoscgood> for bool {
    #[inline(always)]
    fn from(variant: RisLfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_LFOSCGOOD` reader - Raw status of the LFOSCGOOD interrupt."]
pub type RisLfoscgoodR = crate::BitReader<RisLfoscgood>;
impl RisLfoscgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisLfoscgood {
        match self.bits {
            false => RisLfoscgood::RisLfoscgoodFalse,
            true => RisLfoscgood::RisLfoscgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_lfoscgood_false(&self) -> bool {
        *self == RisLfoscgood::RisLfoscgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_lfoscgood_true(&self) -> bool {
        *self == RisLfoscgood::RisLfoscgoodTrue
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisAnaclkerr {
    #[doc = "0: FALSE"]
    RisAnaclkerrFalse = 0,
    #[doc = "1: TRUE"]
    RisAnaclkerrTrue = 1,
}
impl From<RisAnaclkerr> for bool {
    #[inline(always)]
    fn from(variant: RisAnaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_ANACLKERR` reader - Analog Clocking Consistency Error"]
pub type RisAnaclkerrR = crate::BitReader<RisAnaclkerr>;
impl RisAnaclkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisAnaclkerr {
        match self.bits {
            false => RisAnaclkerr::RisAnaclkerrFalse,
            true => RisAnaclkerr::RisAnaclkerrTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_anaclkerr_false(&self) -> bool {
        *self == RisAnaclkerr::RisAnaclkerrFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_anaclkerr_true(&self) -> bool {
        *self == RisAnaclkerr::RisAnaclkerrTrue
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFlashsec {
    #[doc = "0: FALSE"]
    RisFlashsecFalse = 0,
    #[doc = "1: TRUE"]
    RisFlashsecTrue = 1,
}
impl From<RisFlashsec> for bool {
    #[inline(always)]
    fn from(variant: RisFlashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FLASHSEC` reader - Flash Single Error Correct"]
pub type RisFlashsecR = crate::BitReader<RisFlashsec>;
impl RisFlashsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFlashsec {
        match self.bits {
            false => RisFlashsec::RisFlashsecFalse,
            true => RisFlashsec::RisFlashsecTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_flashsec_false(&self) -> bool {
        *self == RisFlashsec::RisFlashsecFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_flashsec_true(&self) -> bool {
        *self == RisFlashsec::RisFlashsecTrue
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisSramsec {
    #[doc = "0: FALSE"]
    RisSramsecFalse = 0,
    #[doc = "1: TRUE"]
    RisSramsecTrue = 1,
}
impl From<RisSramsec> for bool {
    #[inline(always)]
    fn from(variant: RisSramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_SRAMSEC` reader - SRAM Single Error Correct"]
pub type RisSramsecR = crate::BitReader<RisSramsec>;
impl RisSramsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisSramsec {
        match self.bits {
            false => RisSramsec::RisSramsecFalse,
            true => RisSramsec::RisSramsecTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_sramsec_false(&self) -> bool {
        *self == RisSramsec::RisSramsecFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_sramsec_true(&self) -> bool {
        *self == RisSramsec::RisSramsecTrue
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisLfxtgood {
    #[doc = "0: FALSE"]
    RisLfxtgoodFalse = 0,
    #[doc = "1: TRUE"]
    RisLfxtgoodTrue = 1,
}
impl From<RisLfxtgood> for bool {
    #[inline(always)]
    fn from(variant: RisLfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_LFXTGOOD` reader - LFXT GOOD"]
pub type RisLfxtgoodR = crate::BitReader<RisLfxtgood>;
impl RisLfxtgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisLfxtgood {
        match self.bits {
            false => RisLfxtgood::RisLfxtgoodFalse,
            true => RisLfxtgood::RisLfxtgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_lfxtgood_false(&self) -> bool {
        *self == RisLfxtgood::RisLfxtgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_lfxtgood_true(&self) -> bool {
        *self == RisLfxtgood::RisLfxtgoodTrue
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisHfclkgood {
    #[doc = "0: FALSE"]
    RisHfclkgoodFalse = 0,
    #[doc = "1: TRUE"]
    RisHfclkgoodTrue = 1,
}
impl From<RisHfclkgood> for bool {
    #[inline(always)]
    fn from(variant: RisHfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_HFCLKGOOD` reader - HFCLK GOOD"]
pub type RisHfclkgoodR = crate::BitReader<RisHfclkgood>;
impl RisHfclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisHfclkgood {
        match self.bits {
            false => RisHfclkgood::RisHfclkgoodFalse,
            true => RisHfclkgood::RisHfclkgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_hfclkgood_false(&self) -> bool {
        *self == RisHfclkgood::RisHfclkgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_hfclkgood_true(&self) -> bool {
        *self == RisHfclkgood::RisHfclkgoodTrue
    }
}
#[doc = "SYSPLL GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisSyspllgood {
    #[doc = "0: FALSE"]
    RisSyspllgoodFalse = 0,
    #[doc = "1: TRUE"]
    RisSyspllgoodTrue = 1,
}
impl From<RisSyspllgood> for bool {
    #[inline(always)]
    fn from(variant: RisSyspllgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_SYSPLLGOOD` reader - SYSPLL GOOD"]
pub type RisSyspllgoodR = crate::BitReader<RisSyspllgood>;
impl RisSyspllgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisSyspllgood {
        match self.bits {
            false => RisSyspllgood::RisSyspllgoodFalse,
            true => RisSyspllgood::RisSyspllgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_syspllgood_false(&self) -> bool {
        *self == RisSyspllgood::RisSyspllgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_syspllgood_true(&self) -> bool {
        *self == RisSyspllgood::RisSyspllgoodTrue
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisHsclkgood {
    #[doc = "0: FALSE"]
    RisHsclkgoodFalse = 0,
    #[doc = "1: TRUE"]
    RisHsclkgoodTrue = 1,
}
impl From<RisHsclkgood> for bool {
    #[inline(always)]
    fn from(variant: RisHsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_HSCLKGOOD` reader - HSCLK GOOD"]
pub type RisHsclkgoodR = crate::BitReader<RisHsclkgood>;
impl RisHsclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisHsclkgood {
        match self.bits {
            false => RisHsclkgood::RisHsclkgoodFalse,
            true => RisHsclkgood::RisHsclkgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_ris_hsclkgood_false(&self) -> bool {
        *self == RisHsclkgood::RisHsclkgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_ris_hsclkgood_true(&self) -> bool {
        *self == RisHsclkgood::RisHsclkgoodTrue
    }
}
impl R {
    #[doc = "Bit 0 - Raw status of the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn ris_lfoscgood(&self) -> RisLfoscgoodR {
        RisLfoscgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn ris_anaclkerr(&self) -> RisAnaclkerrR {
        RisAnaclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn ris_flashsec(&self) -> RisFlashsecR {
        RisFlashsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn ris_sramsec(&self) -> RisSramsecR {
        RisSramsecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn ris_lfxtgood(&self) -> RisLfxtgoodR {
        RisLfxtgoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn ris_hfclkgood(&self) -> RisHfclkgoodR {
        RisHfclkgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn ris_syspllgood(&self) -> RisSyspllgoodR {
        RisSyspllgoodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSCLK GOOD"]
    #[inline(always)]
    pub fn ris_hsclkgood(&self) -> RisHsclkgoodR {
        RisHsclkgoodR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SYSCTL raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}

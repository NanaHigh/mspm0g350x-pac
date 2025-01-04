#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Masked status of the LFOSCGOOD interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisLfoscgood {
    #[doc = "0: FALSE"]
    MisLfoscgoodFalse = 0,
    #[doc = "1: TRUE"]
    MisLfoscgoodTrue = 1,
}
impl From<MisLfoscgood> for bool {
    #[inline(always)]
    fn from(variant: MisLfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_LFOSCGOOD` reader - Masked status of the LFOSCGOOD interrupt."]
pub type MisLfoscgoodR = crate::BitReader<MisLfoscgood>;
impl MisLfoscgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisLfoscgood {
        match self.bits {
            false => MisLfoscgood::MisLfoscgoodFalse,
            true => MisLfoscgood::MisLfoscgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_lfoscgood_false(&self) -> bool {
        *self == MisLfoscgood::MisLfoscgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_lfoscgood_true(&self) -> bool {
        *self == MisLfoscgood::MisLfoscgoodTrue
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisAnaclkerr {
    #[doc = "0: FALSE"]
    MisAnaclkerrFalse = 0,
    #[doc = "1: TRUE"]
    MisAnaclkerrTrue = 1,
}
impl From<MisAnaclkerr> for bool {
    #[inline(always)]
    fn from(variant: MisAnaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_ANACLKERR` reader - Analog Clocking Consistency Error"]
pub type MisAnaclkerrR = crate::BitReader<MisAnaclkerr>;
impl MisAnaclkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisAnaclkerr {
        match self.bits {
            false => MisAnaclkerr::MisAnaclkerrFalse,
            true => MisAnaclkerr::MisAnaclkerrTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_anaclkerr_false(&self) -> bool {
        *self == MisAnaclkerr::MisAnaclkerrFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_anaclkerr_true(&self) -> bool {
        *self == MisAnaclkerr::MisAnaclkerrTrue
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFlashsec {
    #[doc = "0: FALSE"]
    MisFlashsecFalse = 0,
    #[doc = "1: TRUE"]
    MisFlashsecTrue = 1,
}
impl From<MisFlashsec> for bool {
    #[inline(always)]
    fn from(variant: MisFlashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FLASHSEC` reader - Flash Single Error Correct"]
pub type MisFlashsecR = crate::BitReader<MisFlashsec>;
impl MisFlashsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFlashsec {
        match self.bits {
            false => MisFlashsec::MisFlashsecFalse,
            true => MisFlashsec::MisFlashsecTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_flashsec_false(&self) -> bool {
        *self == MisFlashsec::MisFlashsecFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_flashsec_true(&self) -> bool {
        *self == MisFlashsec::MisFlashsecTrue
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisSramsec {
    #[doc = "0: FALSE"]
    MisSramsecFalse = 0,
    #[doc = "1: TRUE"]
    MisSramsecTrue = 1,
}
impl From<MisSramsec> for bool {
    #[inline(always)]
    fn from(variant: MisSramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_SRAMSEC` reader - SRAM Single Error Correct"]
pub type MisSramsecR = crate::BitReader<MisSramsec>;
impl MisSramsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisSramsec {
        match self.bits {
            false => MisSramsec::MisSramsecFalse,
            true => MisSramsec::MisSramsecTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_sramsec_false(&self) -> bool {
        *self == MisSramsec::MisSramsecFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_sramsec_true(&self) -> bool {
        *self == MisSramsec::MisSramsecTrue
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisLfxtgood {
    #[doc = "0: FALSE"]
    MisLfxtgoodFalse = 0,
    #[doc = "1: TRUE"]
    MisLfxtgoodTrue = 1,
}
impl From<MisLfxtgood> for bool {
    #[inline(always)]
    fn from(variant: MisLfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_LFXTGOOD` reader - LFXT GOOD"]
pub type MisLfxtgoodR = crate::BitReader<MisLfxtgood>;
impl MisLfxtgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisLfxtgood {
        match self.bits {
            false => MisLfxtgood::MisLfxtgoodFalse,
            true => MisLfxtgood::MisLfxtgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_lfxtgood_false(&self) -> bool {
        *self == MisLfxtgood::MisLfxtgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_lfxtgood_true(&self) -> bool {
        *self == MisLfxtgood::MisLfxtgoodTrue
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisHfclkgood {
    #[doc = "0: FALSE"]
    MisHfclkgoodFalse = 0,
    #[doc = "1: TRUE"]
    MisHfclkgoodTrue = 1,
}
impl From<MisHfclkgood> for bool {
    #[inline(always)]
    fn from(variant: MisHfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_HFCLKGOOD` reader - HFCLK GOOD"]
pub type MisHfclkgoodR = crate::BitReader<MisHfclkgood>;
impl MisHfclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisHfclkgood {
        match self.bits {
            false => MisHfclkgood::MisHfclkgoodFalse,
            true => MisHfclkgood::MisHfclkgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_hfclkgood_false(&self) -> bool {
        *self == MisHfclkgood::MisHfclkgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_hfclkgood_true(&self) -> bool {
        *self == MisHfclkgood::MisHfclkgoodTrue
    }
}
#[doc = "SYSPLL GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisSyspllgood {
    #[doc = "0: FALSE"]
    MisSyspllgoodFalse = 0,
    #[doc = "1: TRUE"]
    MisSyspllgoodTrue = 1,
}
impl From<MisSyspllgood> for bool {
    #[inline(always)]
    fn from(variant: MisSyspllgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_SYSPLLGOOD` reader - SYSPLL GOOD"]
pub type MisSyspllgoodR = crate::BitReader<MisSyspllgood>;
impl MisSyspllgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisSyspllgood {
        match self.bits {
            false => MisSyspllgood::MisSyspllgoodFalse,
            true => MisSyspllgood::MisSyspllgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_syspllgood_false(&self) -> bool {
        *self == MisSyspllgood::MisSyspllgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_syspllgood_true(&self) -> bool {
        *self == MisSyspllgood::MisSyspllgoodTrue
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisHsclkgood {
    #[doc = "0: FALSE"]
    MisHsclkgoodFalse = 0,
    #[doc = "1: TRUE"]
    MisHsclkgoodTrue = 1,
}
impl From<MisHsclkgood> for bool {
    #[inline(always)]
    fn from(variant: MisHsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_HSCLKGOOD` reader - HSCLK GOOD"]
pub type MisHsclkgoodR = crate::BitReader<MisHsclkgood>;
impl MisHsclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisHsclkgood {
        match self.bits {
            false => MisHsclkgood::MisHsclkgoodFalse,
            true => MisHsclkgood::MisHsclkgoodTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_mis_hsclkgood_false(&self) -> bool {
        *self == MisHsclkgood::MisHsclkgoodFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_mis_hsclkgood_true(&self) -> bool {
        *self == MisHsclkgood::MisHsclkgoodTrue
    }
}
impl R {
    #[doc = "Bit 0 - Masked status of the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn mis_lfoscgood(&self) -> MisLfoscgoodR {
        MisLfoscgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn mis_anaclkerr(&self) -> MisAnaclkerrR {
        MisAnaclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn mis_flashsec(&self) -> MisFlashsecR {
        MisFlashsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn mis_sramsec(&self) -> MisSramsecR {
        MisSramsecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn mis_lfxtgood(&self) -> MisLfxtgoodR {
        MisLfxtgoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn mis_hfclkgood(&self) -> MisHfclkgoodR {
        MisHfclkgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn mis_syspllgood(&self) -> MisSyspllgoodR {
        MisSyspllgoodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSCLK GOOD"]
    #[inline(always)]
    pub fn mis_hsclkgood(&self) -> MisHsclkgoodR {
        MisHsclkgoodR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "SYSCTL masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}

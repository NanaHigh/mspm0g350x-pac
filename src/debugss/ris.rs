#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Raw interrupt status for TXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisTxifg {
    #[doc = "0: CLR"]
    RisTxifgClr = 0,
    #[doc = "1: SET"]
    RisTxifgSet = 1,
}
impl From<RisTxifg> for bool {
    #[inline(always)]
    fn from(variant: RisTxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_TXIFG` reader - Raw interrupt status for TXIFG"]
pub type RisTxifgR = crate::BitReader<RisTxifg>;
impl RisTxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisTxifg {
        match self.bits {
            false => RisTxifg::RisTxifgClr,
            true => RisTxifg::RisTxifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_txifg_clr(&self) -> bool {
        *self == RisTxifg::RisTxifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_txifg_set(&self) -> bool {
        *self == RisTxifg::RisTxifgSet
    }
}
#[doc = "Raw interrupt status for RXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisRxifg {
    #[doc = "0: CLR"]
    RisRxifgClr = 0,
    #[doc = "1: SET"]
    RisRxifgSet = 1,
}
impl From<RisRxifg> for bool {
    #[inline(always)]
    fn from(variant: RisRxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_RXIFG` reader - Raw interrupt status for RXIFG"]
pub type RisRxifgR = crate::BitReader<RisRxifg>;
impl RisRxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisRxifg {
        match self.bits {
            false => RisRxifg::RisRxifgClr,
            true => RisRxifg::RisRxifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_rxifg_clr(&self) -> bool {
        *self == RisRxifg::RisRxifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_rxifg_set(&self) -> bool {
        *self == RisRxifg::RisRxifgSet
    }
}
#[doc = "Raw interrupt status for PWRUPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisPwrupifg {
    #[doc = "0: CLR"]
    RisPwrupifgClr = 0,
    #[doc = "1: SET"]
    RisPwrupifgSet = 1,
}
impl From<RisPwrupifg> for bool {
    #[inline(always)]
    fn from(variant: RisPwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_PWRUPIFG` reader - Raw interrupt status for PWRUPIFG"]
pub type RisPwrupifgR = crate::BitReader<RisPwrupifg>;
impl RisPwrupifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisPwrupifg {
        match self.bits {
            false => RisPwrupifg::RisPwrupifgClr,
            true => RisPwrupifg::RisPwrupifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_pwrupifg_clr(&self) -> bool {
        *self == RisPwrupifg::RisPwrupifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_pwrupifg_set(&self) -> bool {
        *self == RisPwrupifg::RisPwrupifgSet
    }
}
#[doc = "Raw interrupt status for PWRDWNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisPwrdwnifg {
    #[doc = "0: CLR"]
    RisPwrdwnifgClr = 0,
    #[doc = "1: SET"]
    RisPwrdwnifgSet = 1,
}
impl From<RisPwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: RisPwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_PWRDWNIFG` reader - Raw interrupt status for PWRDWNIFG"]
pub type RisPwrdwnifgR = crate::BitReader<RisPwrdwnifg>;
impl RisPwrdwnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisPwrdwnifg {
        match self.bits {
            false => RisPwrdwnifg::RisPwrdwnifgClr,
            true => RisPwrdwnifg::RisPwrdwnifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_pwrdwnifg_clr(&self) -> bool {
        *self == RisPwrdwnifg::RisPwrdwnifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_pwrdwnifg_set(&self) -> bool {
        *self == RisPwrdwnifg::RisPwrdwnifgSet
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt status for TXIFG"]
    #[inline(always)]
    pub fn ris_txifg(&self) -> RisTxifgR {
        RisTxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt status for RXIFG"]
    #[inline(always)]
    pub fn ris_rxifg(&self) -> RisRxifgR {
        RisRxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status for PWRUPIFG"]
    #[inline(always)]
    pub fn ris_pwrupifg(&self) -> RisPwrupifgR {
        RisPwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status for PWRDWNIFG"]
    #[inline(always)]
    pub fn ris_pwrdwnifg(&self) -> RisPwrdwnifgR {
        RisPwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

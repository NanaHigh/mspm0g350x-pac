#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Masked interrupt status for TXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisTxifg {
    #[doc = "0: CLR"]
    MisTxifgClr = 0,
    #[doc = "1: SET"]
    MisTxifgSet = 1,
}
impl From<MisTxifg> for bool {
    #[inline(always)]
    fn from(variant: MisTxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_TXIFG` reader - Masked interrupt status for TXIFG"]
pub type MisTxifgR = crate::BitReader<MisTxifg>;
impl MisTxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisTxifg {
        match self.bits {
            false => MisTxifg::MisTxifgClr,
            true => MisTxifg::MisTxifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_txifg_clr(&self) -> bool {
        *self == MisTxifg::MisTxifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_txifg_set(&self) -> bool {
        *self == MisTxifg::MisTxifgSet
    }
}
#[doc = "Masked interrupt status for RXIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisRxifg {
    #[doc = "0: CLR"]
    MisRxifgClr = 0,
    #[doc = "1: SET"]
    MisRxifgSet = 1,
}
impl From<MisRxifg> for bool {
    #[inline(always)]
    fn from(variant: MisRxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_RXIFG` reader - Masked interrupt status for RXIFG"]
pub type MisRxifgR = crate::BitReader<MisRxifg>;
impl MisRxifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisRxifg {
        match self.bits {
            false => MisRxifg::MisRxifgClr,
            true => MisRxifg::MisRxifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_rxifg_clr(&self) -> bool {
        *self == MisRxifg::MisRxifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_rxifg_set(&self) -> bool {
        *self == MisRxifg::MisRxifgSet
    }
}
#[doc = "Masked interrupt status for PWRUPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisPwrupifg {
    #[doc = "0: CLR"]
    MisPwrupifgClr = 0,
    #[doc = "1: SET"]
    MisPwrupifgSet = 1,
}
impl From<MisPwrupifg> for bool {
    #[inline(always)]
    fn from(variant: MisPwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_PWRUPIFG` reader - Masked interrupt status for PWRUPIFG"]
pub type MisPwrupifgR = crate::BitReader<MisPwrupifg>;
impl MisPwrupifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisPwrupifg {
        match self.bits {
            false => MisPwrupifg::MisPwrupifgClr,
            true => MisPwrupifg::MisPwrupifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_pwrupifg_clr(&self) -> bool {
        *self == MisPwrupifg::MisPwrupifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_pwrupifg_set(&self) -> bool {
        *self == MisPwrupifg::MisPwrupifgSet
    }
}
#[doc = "Masked interrupt status for PWRDWNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisPwrdwnifg {
    #[doc = "0: CLR"]
    MisPwrdwnifgClr = 0,
    #[doc = "1: SET"]
    MisPwrdwnifgSet = 1,
}
impl From<MisPwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: MisPwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_PWRDWNIFG` reader - Masked interrupt status for PWRDWNIFG"]
pub type MisPwrdwnifgR = crate::BitReader<MisPwrdwnifg>;
impl MisPwrdwnifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisPwrdwnifg {
        match self.bits {
            false => MisPwrdwnifg::MisPwrdwnifgClr,
            true => MisPwrdwnifg::MisPwrdwnifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_pwrdwnifg_clr(&self) -> bool {
        *self == MisPwrdwnifg::MisPwrdwnifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_pwrdwnifg_set(&self) -> bool {
        *self == MisPwrdwnifg::MisPwrdwnifgSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt status for TXIFG"]
    #[inline(always)]
    pub fn mis_txifg(&self) -> MisTxifgR {
        MisTxifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt status for RXIFG"]
    #[inline(always)]
    pub fn mis_rxifg(&self) -> MisRxifgR {
        MisRxifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt status for PWRUPIFG"]
    #[inline(always)]
    pub fn mis_pwrupifg(&self) -> MisPwrupifgR {
        MisPwrupifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt status for PWRDWNIFG"]
    #[inline(always)]
    pub fn mis_pwrdwnifg(&self) -> MisPwrdwnifgR {
        MisPwrdwnifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

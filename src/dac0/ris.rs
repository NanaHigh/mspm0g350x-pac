#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Raw interrupt status for MODRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisModrdyifg {
    #[doc = "0: CLR"]
    RisModrdyifgClr = 0,
    #[doc = "1: SET"]
    RisModrdyifgSet = 1,
}
impl From<RisModrdyifg> for bool {
    #[inline(always)]
    fn from(variant: RisModrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_MODRDYIFG` reader - Raw interrupt status for MODRDYIFG"]
pub type RisModrdyifgR = crate::BitReader<RisModrdyifg>;
impl RisModrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisModrdyifg {
        match self.bits {
            false => RisModrdyifg::RisModrdyifgClr,
            true => RisModrdyifg::RisModrdyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_modrdyifg_clr(&self) -> bool {
        *self == RisModrdyifg::RisModrdyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_modrdyifg_set(&self) -> bool {
        *self == RisModrdyifg::RisModrdyifgSet
    }
}
#[doc = "Raw interrupt status for FIFOFULLIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFifofullifg {
    #[doc = "0: CLR"]
    RisFifofullifgClr = 0,
    #[doc = "1: SET"]
    RisFifofullifgSet = 1,
}
impl From<RisFifofullifg> for bool {
    #[inline(always)]
    fn from(variant: RisFifofullifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FIFOFULLIFG` reader - Raw interrupt status for FIFOFULLIFG"]
pub type RisFifofullifgR = crate::BitReader<RisFifofullifg>;
impl RisFifofullifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFifofullifg {
        match self.bits {
            false => RisFifofullifg::RisFifofullifgClr,
            true => RisFifofullifg::RisFifofullifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_fifofullifg_clr(&self) -> bool {
        *self == RisFifofullifg::RisFifofullifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_fifofullifg_set(&self) -> bool {
        *self == RisFifofullifg::RisFifofullifgSet
    }
}
#[doc = "Raw interrupt status for FIFO1B4IFG\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFifo1b4ifg {
    #[doc = "0: CLR"]
    RisFifo1b4ifgClr = 0,
    #[doc = "1: SET"]
    RisFifo1b4ifgSet = 1,
}
impl From<RisFifo1b4ifg> for bool {
    #[inline(always)]
    fn from(variant: RisFifo1b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FIFO1B4IFG` reader - Raw interrupt status for FIFO1B4IFG"]
pub type RisFifo1b4ifgR = crate::BitReader<RisFifo1b4ifg>;
impl RisFifo1b4ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFifo1b4ifg {
        match self.bits {
            false => RisFifo1b4ifg::RisFifo1b4ifgClr,
            true => RisFifo1b4ifg::RisFifo1b4ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_fifo1b4ifg_clr(&self) -> bool {
        *self == RisFifo1b4ifg::RisFifo1b4ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_fifo1b4ifg_set(&self) -> bool {
        *self == RisFifo1b4ifg::RisFifo1b4ifgSet
    }
}
#[doc = "Raw interrupt status for FIFO1B2IFG\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFifo1b2ifg {
    #[doc = "0: CLR"]
    RisFifo1b2ifgClr = 0,
    #[doc = "1: SET"]
    RisFifo1b2ifgSet = 1,
}
impl From<RisFifo1b2ifg> for bool {
    #[inline(always)]
    fn from(variant: RisFifo1b2ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FIFO1B2IFG` reader - Raw interrupt status for FIFO1B2IFG"]
pub type RisFifo1b2ifgR = crate::BitReader<RisFifo1b2ifg>;
impl RisFifo1b2ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFifo1b2ifg {
        match self.bits {
            false => RisFifo1b2ifg::RisFifo1b2ifgClr,
            true => RisFifo1b2ifg::RisFifo1b2ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_fifo1b2ifg_clr(&self) -> bool {
        *self == RisFifo1b2ifg::RisFifo1b2ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_fifo1b2ifg_set(&self) -> bool {
        *self == RisFifo1b2ifg::RisFifo1b2ifgSet
    }
}
#[doc = "Raw interrupt status for FIFO3B4IFG\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFifo3b4ifg {
    #[doc = "0: CLR"]
    RisFifo3b4ifgClr = 0,
    #[doc = "1: SET"]
    RisFifo3b4ifgSet = 1,
}
impl From<RisFifo3b4ifg> for bool {
    #[inline(always)]
    fn from(variant: RisFifo3b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FIFO3B4IFG` reader - Raw interrupt status for FIFO3B4IFG"]
pub type RisFifo3b4ifgR = crate::BitReader<RisFifo3b4ifg>;
impl RisFifo3b4ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFifo3b4ifg {
        match self.bits {
            false => RisFifo3b4ifg::RisFifo3b4ifgClr,
            true => RisFifo3b4ifg::RisFifo3b4ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_fifo3b4ifg_clr(&self) -> bool {
        *self == RisFifo3b4ifg::RisFifo3b4ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_fifo3b4ifg_set(&self) -> bool {
        *self == RisFifo3b4ifg::RisFifo3b4ifgSet
    }
}
#[doc = "Raw interrupt status for FIFOEMPTYIFG\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFifoemptyifg {
    #[doc = "0: CLR"]
    RisFifoemptyifgClr = 0,
    #[doc = "1: SET"]
    RisFifoemptyifgSet = 1,
}
impl From<RisFifoemptyifg> for bool {
    #[inline(always)]
    fn from(variant: RisFifoemptyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FIFOEMPTYIFG` reader - Raw interrupt status for FIFOEMPTYIFG"]
pub type RisFifoemptyifgR = crate::BitReader<RisFifoemptyifg>;
impl RisFifoemptyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFifoemptyifg {
        match self.bits {
            false => RisFifoemptyifg::RisFifoemptyifgClr,
            true => RisFifoemptyifg::RisFifoemptyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_fifoemptyifg_clr(&self) -> bool {
        *self == RisFifoemptyifg::RisFifoemptyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_fifoemptyifg_set(&self) -> bool {
        *self == RisFifoemptyifg::RisFifoemptyifgSet
    }
}
#[doc = "Raw interrupt status for FIFOURUNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisFifourunifg {
    #[doc = "0: CLR"]
    RisFifourunifgClr = 0,
    #[doc = "1: SET"]
    RisFifourunifgSet = 1,
}
impl From<RisFifourunifg> for bool {
    #[inline(always)]
    fn from(variant: RisFifourunifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_FIFOURUNIFG` reader - Raw interrupt status for FIFOURUNIFG"]
pub type RisFifourunifgR = crate::BitReader<RisFifourunifg>;
impl RisFifourunifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisFifourunifg {
        match self.bits {
            false => RisFifourunifg::RisFifourunifgClr,
            true => RisFifourunifg::RisFifourunifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_fifourunifg_clr(&self) -> bool {
        *self == RisFifourunifg::RisFifourunifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_fifourunifg_set(&self) -> bool {
        *self == RisFifourunifg::RisFifourunifgSet
    }
}
#[doc = "Raw interrupt status for DMADONEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmadoneifg {
    #[doc = "0: CLR"]
    RisDmadoneifgClr = 0,
    #[doc = "1: SET"]
    RisDmadoneifgSet = 1,
}
impl From<RisDmadoneifg> for bool {
    #[inline(always)]
    fn from(variant: RisDmadoneifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMADONEIFG` reader - Raw interrupt status for DMADONEIFG"]
pub type RisDmadoneifgR = crate::BitReader<RisDmadoneifg>;
impl RisDmadoneifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmadoneifg {
        match self.bits {
            false => RisDmadoneifg::RisDmadoneifgClr,
            true => RisDmadoneifg::RisDmadoneifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmadoneifg_clr(&self) -> bool {
        *self == RisDmadoneifg::RisDmadoneifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmadoneifg_set(&self) -> bool {
        *self == RisDmadoneifg::RisDmadoneifgSet
    }
}
impl R {
    #[doc = "Bit 1 - Raw interrupt status for MODRDYIFG"]
    #[inline(always)]
    pub fn ris_modrdyifg(&self) -> RisModrdyifgR {
        RisModrdyifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for FIFOFULLIFG"]
    #[inline(always)]
    pub fn ris_fifofullifg(&self) -> RisFifofullifgR {
        RisFifofullifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for FIFO1B4IFG"]
    #[inline(always)]
    pub fn ris_fifo1b4ifg(&self) -> RisFifo1b4ifgR {
        RisFifo1b4ifgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for FIFO1B2IFG"]
    #[inline(always)]
    pub fn ris_fifo1b2ifg(&self) -> RisFifo1b2ifgR {
        RisFifo1b2ifgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for FIFO3B4IFG"]
    #[inline(always)]
    pub fn ris_fifo3b4ifg(&self) -> RisFifo3b4ifgR {
        RisFifo3b4ifgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for FIFOEMPTYIFG"]
    #[inline(always)]
    pub fn ris_fifoemptyifg(&self) -> RisFifoemptyifgR {
        RisFifoemptyifgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for FIFOURUNIFG"]
    #[inline(always)]
    pub fn ris_fifourunifg(&self) -> RisFifourunifgR {
        RisFifourunifgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for DMADONEIFG"]
    #[inline(always)]
    pub fn ris_dmadoneifg(&self) -> RisDmadoneifgR {
        RisDmadoneifgR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0x1e00"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0x1e00;
}

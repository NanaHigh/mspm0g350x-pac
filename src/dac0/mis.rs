#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Masked interrupt status for MODRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisModrdyifg {
    #[doc = "0: CLR"]
    MisModrdyifgClr = 0,
    #[doc = "1: SET"]
    MisModrdyifgSet = 1,
}
impl From<MisModrdyifg> for bool {
    #[inline(always)]
    fn from(variant: MisModrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_MODRDYIFG` reader - Masked interrupt status for MODRDYIFG"]
pub type MisModrdyifgR = crate::BitReader<MisModrdyifg>;
impl MisModrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisModrdyifg {
        match self.bits {
            false => MisModrdyifg::MisModrdyifgClr,
            true => MisModrdyifg::MisModrdyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_modrdyifg_clr(&self) -> bool {
        *self == MisModrdyifg::MisModrdyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_modrdyifg_set(&self) -> bool {
        *self == MisModrdyifg::MisModrdyifgSet
    }
}
#[doc = "Masked interrupt status for FIFOFULLIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFifofullifg {
    #[doc = "0: CLR"]
    MisFifofullifgClr = 0,
    #[doc = "1: SET"]
    MisFifofullifgSet = 1,
}
impl From<MisFifofullifg> for bool {
    #[inline(always)]
    fn from(variant: MisFifofullifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FIFOFULLIFG` reader - Masked interrupt status for FIFOFULLIFG"]
pub type MisFifofullifgR = crate::BitReader<MisFifofullifg>;
impl MisFifofullifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFifofullifg {
        match self.bits {
            false => MisFifofullifg::MisFifofullifgClr,
            true => MisFifofullifg::MisFifofullifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_fifofullifg_clr(&self) -> bool {
        *self == MisFifofullifg::MisFifofullifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_fifofullifg_set(&self) -> bool {
        *self == MisFifofullifg::MisFifofullifgSet
    }
}
#[doc = "Masked interrupt status for FIFO1B4IFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFifo1b4ifg {
    #[doc = "0: CLR"]
    MisFifo1b4ifgClr = 0,
    #[doc = "1: SET"]
    MisFifo1b4ifgSet = 1,
}
impl From<MisFifo1b4ifg> for bool {
    #[inline(always)]
    fn from(variant: MisFifo1b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FIFO1B4IFG` reader - Masked interrupt status for FIFO1B4IFG"]
pub type MisFifo1b4ifgR = crate::BitReader<MisFifo1b4ifg>;
impl MisFifo1b4ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFifo1b4ifg {
        match self.bits {
            false => MisFifo1b4ifg::MisFifo1b4ifgClr,
            true => MisFifo1b4ifg::MisFifo1b4ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_fifo1b4ifg_clr(&self) -> bool {
        *self == MisFifo1b4ifg::MisFifo1b4ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_fifo1b4ifg_set(&self) -> bool {
        *self == MisFifo1b4ifg::MisFifo1b4ifgSet
    }
}
#[doc = "Masked interrupt status for FIFO1B2IFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFifo1b2ifg {
    #[doc = "0: CLR"]
    MisFifo1b2ifgClr = 0,
    #[doc = "1: SET"]
    MisFifo1b2ifgSet = 1,
}
impl From<MisFifo1b2ifg> for bool {
    #[inline(always)]
    fn from(variant: MisFifo1b2ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FIFO1B2IFG` reader - Masked interrupt status for FIFO1B2IFG"]
pub type MisFifo1b2ifgR = crate::BitReader<MisFifo1b2ifg>;
impl MisFifo1b2ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFifo1b2ifg {
        match self.bits {
            false => MisFifo1b2ifg::MisFifo1b2ifgClr,
            true => MisFifo1b2ifg::MisFifo1b2ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_fifo1b2ifg_clr(&self) -> bool {
        *self == MisFifo1b2ifg::MisFifo1b2ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_fifo1b2ifg_set(&self) -> bool {
        *self == MisFifo1b2ifg::MisFifo1b2ifgSet
    }
}
#[doc = "Masked interrupt status for FIFO3B4IFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFifo3b4ifg {
    #[doc = "0: CLR"]
    MisFifo3b4ifgClr = 0,
    #[doc = "1: SET"]
    MisFifo3b4ifgSet = 1,
}
impl From<MisFifo3b4ifg> for bool {
    #[inline(always)]
    fn from(variant: MisFifo3b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FIFO3B4IFG` reader - Masked interrupt status for FIFO3B4IFG"]
pub type MisFifo3b4ifgR = crate::BitReader<MisFifo3b4ifg>;
impl MisFifo3b4ifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFifo3b4ifg {
        match self.bits {
            false => MisFifo3b4ifg::MisFifo3b4ifgClr,
            true => MisFifo3b4ifg::MisFifo3b4ifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_fifo3b4ifg_clr(&self) -> bool {
        *self == MisFifo3b4ifg::MisFifo3b4ifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_fifo3b4ifg_set(&self) -> bool {
        *self == MisFifo3b4ifg::MisFifo3b4ifgSet
    }
}
#[doc = "Masked interrupt status for FIFOEMPTYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFifoemptyifg {
    #[doc = "0: CLR"]
    MisFifoemptyifgClr = 0,
    #[doc = "1: SET"]
    MisFifoemptyifgSet = 1,
}
impl From<MisFifoemptyifg> for bool {
    #[inline(always)]
    fn from(variant: MisFifoemptyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FIFOEMPTYIFG` reader - Masked interrupt status for FIFOEMPTYIFG"]
pub type MisFifoemptyifgR = crate::BitReader<MisFifoemptyifg>;
impl MisFifoemptyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFifoemptyifg {
        match self.bits {
            false => MisFifoemptyifg::MisFifoemptyifgClr,
            true => MisFifoemptyifg::MisFifoemptyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_fifoemptyifg_clr(&self) -> bool {
        *self == MisFifoemptyifg::MisFifoemptyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_fifoemptyifg_set(&self) -> bool {
        *self == MisFifoemptyifg::MisFifoemptyifgSet
    }
}
#[doc = "Masked interrupt status for FIFOURUNIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisFifourunifg {
    #[doc = "0: CLR"]
    MisFifourunifgClr = 0,
    #[doc = "1: SET"]
    MisFifourunifgSet = 1,
}
impl From<MisFifourunifg> for bool {
    #[inline(always)]
    fn from(variant: MisFifourunifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_FIFOURUNIFG` reader - Masked interrupt status for FIFOURUNIFG"]
pub type MisFifourunifgR = crate::BitReader<MisFifourunifg>;
impl MisFifourunifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisFifourunifg {
        match self.bits {
            false => MisFifourunifg::MisFifourunifgClr,
            true => MisFifourunifg::MisFifourunifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_fifourunifg_clr(&self) -> bool {
        *self == MisFifourunifg::MisFifourunifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_fifourunifg_set(&self) -> bool {
        *self == MisFifourunifg::MisFifourunifgSet
    }
}
#[doc = "Masked interrupt status for DMADONEIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmadoneifg {
    #[doc = "0: CLR"]
    MisDmadoneifgClr = 0,
    #[doc = "1: SET"]
    MisDmadoneifgSet = 1,
}
impl From<MisDmadoneifg> for bool {
    #[inline(always)]
    fn from(variant: MisDmadoneifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMADONEIFG` reader - Masked interrupt status for DMADONEIFG"]
pub type MisDmadoneifgR = crate::BitReader<MisDmadoneifg>;
impl MisDmadoneifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmadoneifg {
        match self.bits {
            false => MisDmadoneifg::MisDmadoneifgClr,
            true => MisDmadoneifg::MisDmadoneifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmadoneifg_clr(&self) -> bool {
        *self == MisDmadoneifg::MisDmadoneifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmadoneifg_set(&self) -> bool {
        *self == MisDmadoneifg::MisDmadoneifgSet
    }
}
impl R {
    #[doc = "Bit 1 - Masked interrupt status for MODRDYIFG"]
    #[inline(always)]
    pub fn mis_modrdyifg(&self) -> MisModrdyifgR {
        MisModrdyifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked interrupt status for FIFOFULLIFG"]
    #[inline(always)]
    pub fn mis_fifofullifg(&self) -> MisFifofullifgR {
        MisFifofullifgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked interrupt status for FIFO1B4IFG"]
    #[inline(always)]
    pub fn mis_fifo1b4ifg(&self) -> MisFifo1b4ifgR {
        MisFifo1b4ifgR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Masked interrupt status for FIFO1B2IFG"]
    #[inline(always)]
    pub fn mis_fifo1b2ifg(&self) -> MisFifo1b2ifgR {
        MisFifo1b2ifgR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Masked interrupt status for FIFO3B4IFG"]
    #[inline(always)]
    pub fn mis_fifo3b4ifg(&self) -> MisFifo3b4ifgR {
        MisFifo3b4ifgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Masked interrupt status for FIFOEMPTYIFG"]
    #[inline(always)]
    pub fn mis_fifoemptyifg(&self) -> MisFifoemptyifgR {
        MisFifoemptyifgR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Masked interrupt status for FIFOURUNIFG"]
    #[inline(always)]
    pub fn mis_fifourunifg(&self) -> MisFifourunifgR {
        MisFifourunifgR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Masked interrupt status for DMADONEIFG"]
    #[inline(always)]
    pub fn mis_dmadoneifg(&self) -> MisDmadoneifgR {
        MisDmadoneifgR::new(((self.bits >> 14) & 1) != 0)
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

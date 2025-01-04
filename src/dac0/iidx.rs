#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "2: MODRDYIFG"]
    IidxStatModrdyifg = 2,
    #[doc = "9: FIFOFULLIFG"]
    IidxStatFifofullifg = 9,
    #[doc = "10: FIFO1B4IFG"]
    IidxStatFifo1b4ifg = 10,
    #[doc = "11: FIFO1B2IFG"]
    IidxStatFifo1b2ifg = 11,
    #[doc = "12: FIFO3B4IFG"]
    IidxStatFifo3b4ifg = 12,
    #[doc = "13: FIFOEMPTYIFG"]
    IidxStatFifoemptyifg = 13,
    #[doc = "14: FIFOURUNIFG"]
    IidxStatFifourunifg = 14,
    #[doc = "15: DMADONEIFG"]
    IidxStatDmadoneifg = 15,
}
impl From<IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IidxStat {}
#[doc = "Field `IIDX_STAT` reader - Interrupt index status"]
pub type IidxStatR = crate::FieldReader<IidxStat>;
impl IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IidxStat> {
        match self.bits {
            0 => Some(IidxStat::IidxStatNoIntr),
            2 => Some(IidxStat::IidxStatModrdyifg),
            9 => Some(IidxStat::IidxStatFifofullifg),
            10 => Some(IidxStat::IidxStatFifo1b4ifg),
            11 => Some(IidxStat::IidxStatFifo1b2ifg),
            12 => Some(IidxStat::IidxStatFifo3b4ifg),
            13 => Some(IidxStat::IidxStatFifoemptyifg),
            14 => Some(IidxStat::IidxStatFifourunifg),
            15 => Some(IidxStat::IidxStatDmadoneifg),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "MODRDYIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_modrdyifg(&self) -> bool {
        *self == IidxStat::IidxStatModrdyifg
    }
    #[doc = "FIFOFULLIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_fifofullifg(&self) -> bool {
        *self == IidxStat::IidxStatFifofullifg
    }
    #[doc = "FIFO1B4IFG"]
    #[inline(always)]
    pub fn is_iidx_stat_fifo1b4ifg(&self) -> bool {
        *self == IidxStat::IidxStatFifo1b4ifg
    }
    #[doc = "FIFO1B2IFG"]
    #[inline(always)]
    pub fn is_iidx_stat_fifo1b2ifg(&self) -> bool {
        *self == IidxStat::IidxStatFifo1b2ifg
    }
    #[doc = "FIFO3B4IFG"]
    #[inline(always)]
    pub fn is_iidx_stat_fifo3b4ifg(&self) -> bool {
        *self == IidxStat::IidxStatFifo3b4ifg
    }
    #[doc = "FIFOEMPTYIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_fifoemptyifg(&self) -> bool {
        *self == IidxStat::IidxStatFifoemptyifg
    }
    #[doc = "FIFOURUNIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_fifourunifg(&self) -> bool {
        *self == IidxStat::IidxStatFifourunifg
    }
    #[doc = "DMADONEIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_dmadoneifg(&self) -> bool {
        *self == IidxStat::IidxStatDmadoneifg
    }
}
impl R {
    #[doc = "Bits 0:3 - Interrupt index status"]
    #[inline(always)]
    pub fn iidx_stat(&self) -> IidxStatR {
        IidxStatR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {
    const RESET_VALUE: u32 = 0;
}

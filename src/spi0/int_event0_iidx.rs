#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: RXFIFO_OFV_EVT"]
    IntEvent0IidxStatRxfifoOfvEvt = 1,
    #[doc = "2: PER_EVT"]
    IntEvent0IidxStatPerEvt = 2,
    #[doc = "3: RTOUT_EVT"]
    IntEvent0IidxStatRtoutEvt = 3,
    #[doc = "4: RX_EVT"]
    IntEvent0IidxStatRxEvt = 4,
    #[doc = "5: TX_EVT"]
    IntEvent0IidxStatTxEvt = 5,
    #[doc = "6: TX_EMPTY"]
    IntEvent0IidxStatTxEmpty = 6,
    #[doc = "7: IDLE_EVT"]
    IntEvent0IidxStatIdleEvt = 7,
    #[doc = "8: DMA_DONE_RX_EVT"]
    IntEvent0IidxStatDmaDoneRxEvt = 8,
    #[doc = "9: DMA_DONE_TX_EVT"]
    IntEvent0IidxStatDmaDoneTxEvt = 9,
    #[doc = "10: TXFIFO_UNF_EVT"]
    IntEvent0IidxStatTxfifoUnfEvt = 10,
    #[doc = "11: RXFULL_EVT"]
    IntEvent0IidxStatRxfullEvt = 11,
}
impl From<IntEvent0IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent0IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent0IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent0IidxStat {}
#[doc = "Field `INT_EVENT0_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent0IidxStatR = crate::FieldReader<IntEvent0IidxStat>;
impl IntEvent0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent0IidxStat> {
        match self.bits {
            0 => Some(IntEvent0IidxStat::IntEvent0IidxStatNoIntr),
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatRxfifoOfvEvt),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatPerEvt),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatRtoutEvt),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatRxEvt),
            5 => Some(IntEvent0IidxStat::IntEvent0IidxStatTxEvt),
            6 => Some(IntEvent0IidxStat::IntEvent0IidxStatTxEmpty),
            7 => Some(IntEvent0IidxStat::IntEvent0IidxStatIdleEvt),
            8 => Some(IntEvent0IidxStat::IntEvent0IidxStatDmaDoneRxEvt),
            9 => Some(IntEvent0IidxStat::IntEvent0IidxStatDmaDoneTxEvt),
            10 => Some(IntEvent0IidxStat::IntEvent0IidxStatTxfifoUnfEvt),
            11 => Some(IntEvent0IidxStat::IntEvent0IidxStatRxfullEvt),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "RXFIFO_OFV_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rxfifo_ofv_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRxfifoOfvEvt
    }
    #[doc = "PER_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_per_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatPerEvt
    }
    #[doc = "RTOUT_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rtout_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRtoutEvt
    }
    #[doc = "RX_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rx_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRxEvt
    }
    #[doc = "TX_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_tx_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTxEvt
    }
    #[doc = "TX_EMPTY"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_tx_empty(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTxEmpty
    }
    #[doc = "IDLE_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_idle_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatIdleEvt
    }
    #[doc = "DMA_DONE_RX_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma_done_rx_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDmaDoneRxEvt
    }
    #[doc = "DMA_DONE_TX_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma_done_tx_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDmaDoneTxEvt
    }
    #[doc = "TXFIFO_UNF_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_txfifo_unf_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTxfifoUnfEvt
    }
    #[doc = "RXFULL_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rxfull_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRxfullEvt
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event0_iidx_stat(&self) -> IntEvent0IidxStatR {
        IntEvent0IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IidxSpec;
impl crate::RegisterSpec for IntEvent0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent0IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT0_IIDX to value 0"]
impl crate::Resettable for IntEvent0IidxSpec {
    const RESET_VALUE: u32 = 0;
}

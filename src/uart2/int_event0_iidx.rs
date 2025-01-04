#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MIS registers. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: RTFG"]
    IntEvent0IidxStatRtfg = 1,
    #[doc = "2: FEFG"]
    IntEvent0IidxStatFefg = 2,
    #[doc = "3: PEFG"]
    IntEvent0IidxStatPefg = 3,
    #[doc = "4: BEFG"]
    IntEvent0IidxStatBefg = 4,
    #[doc = "5: OEFG"]
    IntEvent0IidxStatOefg = 5,
    #[doc = "6: RXNE"]
    IntEvent0IidxStatRxne = 6,
    #[doc = "7: RXPE"]
    IntEvent0IidxStatRxpe = 7,
    #[doc = "8: LINC0"]
    IntEvent0IidxStatLinc0 = 8,
    #[doc = "9: LINC1"]
    IntEvent0IidxStatLinc1 = 9,
    #[doc = "10: LINOVF"]
    IntEvent0IidxStatLinovf = 10,
    #[doc = "11: RXIFG"]
    IntEvent0IidxStatRxifg = 11,
    #[doc = "12: TXIFG"]
    IntEvent0IidxStatTxifg = 12,
    #[doc = "13: EOT"]
    IntEvent0IidxStatEot = 13,
    #[doc = "14: MODE_9B"]
    IntEvent0IidxStatMode9b = 14,
    #[doc = "15: CTS"]
    IntEvent0IidxStatCts = 15,
    #[doc = "16: DMA_DONE_RX"]
    IntEvent0IidxStatDmaDoneRx = 16,
    #[doc = "17: DMA_DONE_TX"]
    IntEvent0IidxStatDmaDoneTx = 17,
    #[doc = "18: NERR_EVT"]
    IntEvent0IidxStatNerrEvt = 18,
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
#[doc = "Field `INT_EVENT0_IIDX_STAT` reader - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MIS registers. 15h-1Fh = Reserved"]
pub type IntEvent0IidxStatR = crate::FieldReader<IntEvent0IidxStat>;
impl IntEvent0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent0IidxStat> {
        match self.bits {
            0 => Some(IntEvent0IidxStat::IntEvent0IidxStatNoIntr),
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatRtfg),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatFefg),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatPefg),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatBefg),
            5 => Some(IntEvent0IidxStat::IntEvent0IidxStatOefg),
            6 => Some(IntEvent0IidxStat::IntEvent0IidxStatRxne),
            7 => Some(IntEvent0IidxStat::IntEvent0IidxStatRxpe),
            8 => Some(IntEvent0IidxStat::IntEvent0IidxStatLinc0),
            9 => Some(IntEvent0IidxStat::IntEvent0IidxStatLinc1),
            10 => Some(IntEvent0IidxStat::IntEvent0IidxStatLinovf),
            11 => Some(IntEvent0IidxStat::IntEvent0IidxStatRxifg),
            12 => Some(IntEvent0IidxStat::IntEvent0IidxStatTxifg),
            13 => Some(IntEvent0IidxStat::IntEvent0IidxStatEot),
            14 => Some(IntEvent0IidxStat::IntEvent0IidxStatMode9b),
            15 => Some(IntEvent0IidxStat::IntEvent0IidxStatCts),
            16 => Some(IntEvent0IidxStat::IntEvent0IidxStatDmaDoneRx),
            17 => Some(IntEvent0IidxStat::IntEvent0IidxStatDmaDoneTx),
            18 => Some(IntEvent0IidxStat::IntEvent0IidxStatNerrEvt),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "RTFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rtfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRtfg
    }
    #[doc = "FEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_fefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatFefg
    }
    #[doc = "PEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_pefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatPefg
    }
    #[doc = "BEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_befg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatBefg
    }
    #[doc = "OEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_oefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatOefg
    }
    #[doc = "RXNE"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rxne(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRxne
    }
    #[doc = "RXPE"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rxpe(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRxpe
    }
    #[doc = "LINC0"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_linc0(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatLinc0
    }
    #[doc = "LINC1"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_linc1(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatLinc1
    }
    #[doc = "LINOVF"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_linovf(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatLinovf
    }
    #[doc = "RXIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rxifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRxifg
    }
    #[doc = "TXIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_txifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTxifg
    }
    #[doc = "EOT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_eot(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatEot
    }
    #[doc = "MODE_9B"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mode_9b(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMode9b
    }
    #[doc = "CTS"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_cts(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatCts
    }
    #[doc = "DMA_DONE_RX"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma_done_rx(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDmaDoneRx
    }
    #[doc = "DMA_DONE_TX"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma_done_tx(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDmaDoneTx
    }
    #[doc = "NERR_EVT"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_nerr_evt(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNerrEvt
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MIS registers. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn int_event0_iidx_stat(&self) -> IntEvent0IidxStatR {
        IntEvent0IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

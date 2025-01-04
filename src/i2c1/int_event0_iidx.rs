#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: MRXDONEFG"]
    IntEvent0IidxStatMrxdonefg = 1,
    #[doc = "2: MTXDONEFG"]
    IntEvent0IidxStatMtxdonefg = 2,
    #[doc = "3: MRXFIFOTRG"]
    IntEvent0IidxStatMrxfifotrg = 3,
    #[doc = "4: MTXFIFOTRG"]
    IntEvent0IidxStatMtxfifotrg = 4,
    #[doc = "5: MRXFIFOFULL"]
    IntEvent0IidxStatMrxfifofull = 5,
    #[doc = "6: MTX_EMPTY"]
    IntEvent0IidxStatMtxEmpty = 6,
    #[doc = "8: MNACKFG"]
    IntEvent0IidxStatMnackfg = 8,
    #[doc = "9: MSTARTFG"]
    IntEvent0IidxStatMstartfg = 9,
    #[doc = "10: MSTOPFG"]
    IntEvent0IidxStatMstopfg = 10,
    #[doc = "11: MARBLOSTFG"]
    IntEvent0IidxStatMarblostfg = 11,
    #[doc = "12: MDMA_DONE1_CH2"]
    IntEvent0IidxStatMdmaDone1Ch2 = 12,
    #[doc = "13: MDMA_DONE1_CH3"]
    IntEvent0IidxStatMdmaDone1Ch3 = 13,
    #[doc = "14: MPEC_RX_ERR"]
    IntEvent0IidxStatMpecRxErr = 14,
    #[doc = "15: TIMEOUTA"]
    IntEvent0IidxStatTimeouta = 15,
    #[doc = "16: TIMEOUTB"]
    IntEvent0IidxStatTimeoutb = 16,
    #[doc = "17: SRXDONEFG"]
    IntEvent0IidxStatSrxdonefg = 17,
    #[doc = "18: STXDONEFG"]
    IntEvent0IidxStatStxdonefg = 18,
    #[doc = "19: SRXFIFOTRG"]
    IntEvent0IidxStatSrxfifotrg = 19,
    #[doc = "20: STXFIFOTRG"]
    IntEvent0IidxStatStxfifotrg = 20,
    #[doc = "21: SRXFIFOFULL"]
    IntEvent0IidxStatSrxfifofull = 21,
    #[doc = "22: STXEMPTY"]
    IntEvent0IidxStatStxempty = 22,
    #[doc = "23: SSTARTFG"]
    IntEvent0IidxStatSstartfg = 23,
    #[doc = "24: SSTOPFG"]
    IntEvent0IidxStatSstopfg = 24,
    #[doc = "25: SGENCALL"]
    IntEvent0IidxStatSgencall = 25,
    #[doc = "26: SDMA_DONE1_CH2"]
    IntEvent0IidxStatSdmaDone1Ch2 = 26,
    #[doc = "27: SDMA_DONE1_CH3"]
    IntEvent0IidxStatSdmaDone1Ch3 = 27,
    #[doc = "28: SPEC_RX_ERR"]
    IntEvent0IidxStatSpecRxErr = 28,
    #[doc = "29: STX_UNFL"]
    IntEvent0IidxStatStxUnfl = 29,
    #[doc = "30: SRX_OVFL"]
    IntEvent0IidxStatSrxOvfl = 30,
    #[doc = "31: SARBLOST"]
    IntEvent0IidxStatSarblost = 31,
    #[doc = "32: INTR_OVFL"]
    IntEvent0IidxStatIntrOvfl = 32,
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
#[doc = "Field `INT_EVENT0_IIDX_STAT` reader - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
pub type IntEvent0IidxStatR = crate::FieldReader<IntEvent0IidxStat>;
impl IntEvent0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent0IidxStat> {
        match self.bits {
            0 => Some(IntEvent0IidxStat::IntEvent0IidxStatNoIntr),
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatMrxdonefg),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatMtxdonefg),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatMrxfifotrg),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatMtxfifotrg),
            5 => Some(IntEvent0IidxStat::IntEvent0IidxStatMrxfifofull),
            6 => Some(IntEvent0IidxStat::IntEvent0IidxStatMtxEmpty),
            8 => Some(IntEvent0IidxStat::IntEvent0IidxStatMnackfg),
            9 => Some(IntEvent0IidxStat::IntEvent0IidxStatMstartfg),
            10 => Some(IntEvent0IidxStat::IntEvent0IidxStatMstopfg),
            11 => Some(IntEvent0IidxStat::IntEvent0IidxStatMarblostfg),
            12 => Some(IntEvent0IidxStat::IntEvent0IidxStatMdmaDone1Ch2),
            13 => Some(IntEvent0IidxStat::IntEvent0IidxStatMdmaDone1Ch3),
            14 => Some(IntEvent0IidxStat::IntEvent0IidxStatMpecRxErr),
            15 => Some(IntEvent0IidxStat::IntEvent0IidxStatTimeouta),
            16 => Some(IntEvent0IidxStat::IntEvent0IidxStatTimeoutb),
            17 => Some(IntEvent0IidxStat::IntEvent0IidxStatSrxdonefg),
            18 => Some(IntEvent0IidxStat::IntEvent0IidxStatStxdonefg),
            19 => Some(IntEvent0IidxStat::IntEvent0IidxStatSrxfifotrg),
            20 => Some(IntEvent0IidxStat::IntEvent0IidxStatStxfifotrg),
            21 => Some(IntEvent0IidxStat::IntEvent0IidxStatSrxfifofull),
            22 => Some(IntEvent0IidxStat::IntEvent0IidxStatStxempty),
            23 => Some(IntEvent0IidxStat::IntEvent0IidxStatSstartfg),
            24 => Some(IntEvent0IidxStat::IntEvent0IidxStatSstopfg),
            25 => Some(IntEvent0IidxStat::IntEvent0IidxStatSgencall),
            26 => Some(IntEvent0IidxStat::IntEvent0IidxStatSdmaDone1Ch2),
            27 => Some(IntEvent0IidxStat::IntEvent0IidxStatSdmaDone1Ch3),
            28 => Some(IntEvent0IidxStat::IntEvent0IidxStatSpecRxErr),
            29 => Some(IntEvent0IidxStat::IntEvent0IidxStatStxUnfl),
            30 => Some(IntEvent0IidxStat::IntEvent0IidxStatSrxOvfl),
            31 => Some(IntEvent0IidxStat::IntEvent0IidxStatSarblost),
            32 => Some(IntEvent0IidxStat::IntEvent0IidxStatIntrOvfl),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "MRXDONEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mrxdonefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMrxdonefg
    }
    #[doc = "MTXDONEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mtxdonefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMtxdonefg
    }
    #[doc = "MRXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mrxfifotrg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMrxfifotrg
    }
    #[doc = "MTXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mtxfifotrg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMtxfifotrg
    }
    #[doc = "MRXFIFOFULL"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mrxfifofull(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMrxfifofull
    }
    #[doc = "MTX_EMPTY"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mtx_empty(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMtxEmpty
    }
    #[doc = "MNACKFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mnackfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMnackfg
    }
    #[doc = "MSTARTFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mstartfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMstartfg
    }
    #[doc = "MSTOPFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mstopfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMstopfg
    }
    #[doc = "MARBLOSTFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_marblostfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMarblostfg
    }
    #[doc = "MDMA_DONE1_CH2"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mdma_done1_ch2(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMdmaDone1Ch2
    }
    #[doc = "MDMA_DONE1_CH3"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mdma_done1_ch3(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMdmaDone1Ch3
    }
    #[doc = "MPEC_RX_ERR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_mpec_rx_err(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMpecRxErr
    }
    #[doc = "TIMEOUTA"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_timeouta(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTimeouta
    }
    #[doc = "TIMEOUTB"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_timeoutb(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTimeoutb
    }
    #[doc = "SRXDONEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_srxdonefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSrxdonefg
    }
    #[doc = "STXDONEFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_stxdonefg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatStxdonefg
    }
    #[doc = "SRXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_srxfifotrg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSrxfifotrg
    }
    #[doc = "STXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_stxfifotrg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatStxfifotrg
    }
    #[doc = "SRXFIFOFULL"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_srxfifofull(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSrxfifofull
    }
    #[doc = "STXEMPTY"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_stxempty(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatStxempty
    }
    #[doc = "SSTARTFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_sstartfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSstartfg
    }
    #[doc = "SSTOPFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_sstopfg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSstopfg
    }
    #[doc = "SGENCALL"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_sgencall(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSgencall
    }
    #[doc = "SDMA_DONE1_CH2"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_sdma_done1_ch2(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSdmaDone1Ch2
    }
    #[doc = "SDMA_DONE1_CH3"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_sdma_done1_ch3(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSdmaDone1Ch3
    }
    #[doc = "SPEC_RX_ERR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_spec_rx_err(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSpecRxErr
    }
    #[doc = "STX_UNFL"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_stx_unfl(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatStxUnfl
    }
    #[doc = "SRX_OVFL"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_srx_ovfl(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSrxOvfl
    }
    #[doc = "SARBLOST"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_sarblost(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatSarblost
    }
    #[doc = "INTR_OVFL"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_intr_ovfl(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatIntrOvfl
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
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

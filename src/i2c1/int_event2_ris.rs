#[doc = "Register `INT_EVENT2_RIS` reader"]
pub type R = crate::R<IntEvent2RisSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2RisMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMrxfifotrgSet = 1,
}
impl From<IntEvent2RisMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent2RisMrxfifotrgR = crate::BitReader<IntEvent2RisMrxfifotrg>;
impl IntEvent2RisMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMrxfifotrg {
        match self.bits {
            false => IntEvent2RisMrxfifotrg::IntEvent2RisMrxfifotrgClr,
            true => IntEvent2RisMrxfifotrg::IntEvent2RisMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent2RisMrxfifotrg::IntEvent2RisMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent2RisMrxfifotrg::IntEvent2RisMrxfifotrgSet
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2RisMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2RisMtxfifotrgSet = 1,
}
impl From<IntEvent2RisMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent2RisMtxfifotrgR = crate::BitReader<IntEvent2RisMtxfifotrg>;
impl IntEvent2RisMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisMtxfifotrg {
        match self.bits {
            false => IntEvent2RisMtxfifotrg::IntEvent2RisMtxfifotrgClr,
            true => IntEvent2RisMtxfifotrg::IntEvent2RisMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent2RisMtxfifotrg::IntEvent2RisMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent2RisMtxfifotrg::IntEvent2RisMtxfifotrgSet
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2RisSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2RisSrxfifotrgSet = 1,
}
impl From<IntEvent2RisSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent2RisSrxfifotrgR = crate::BitReader<IntEvent2RisSrxfifotrg>;
impl IntEvent2RisSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisSrxfifotrg {
        match self.bits {
            false => IntEvent2RisSrxfifotrg::IntEvent2RisSrxfifotrgClr,
            true => IntEvent2RisSrxfifotrg::IntEvent2RisSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent2RisSrxfifotrg::IntEvent2RisSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_srxfifotrg_set(&self) -> bool {
        *self == IntEvent2RisSrxfifotrg::IntEvent2RisSrxfifotrgSet
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2RisStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2RisStxfifotrgSet = 1,
}
impl From<IntEvent2RisStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent2RisStxfifotrgR = crate::BitReader<IntEvent2RisStxfifotrg>;
impl IntEvent2RisStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisStxfifotrg {
        match self.bits {
            false => IntEvent2RisStxfifotrg::IntEvent2RisStxfifotrgClr,
            true => IntEvent2RisStxfifotrg::IntEvent2RisStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent2RisStxfifotrg::IntEvent2RisStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_stxfifotrg_set(&self) -> bool {
        *self == IntEvent2RisStxfifotrg::IntEvent2RisStxfifotrgSet
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_ris_mrxfifotrg(&self) -> IntEvent2RisMrxfifotrgR {
        IntEvent2RisMrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_ris_mtxfifotrg(&self) -> IntEvent2RisMtxfifotrgR {
        IntEvent2RisMtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_ris_srxfifotrg(&self) -> IntEvent2RisSrxfifotrgR {
        IntEvent2RisSrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_ris_stxfifotrg(&self) -> IntEvent2RisStxfifotrgR {
        IntEvent2RisStxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2RisSpec;
impl crate::RegisterSpec for IntEvent2RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent2RisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_RIS to value 0"]
impl crate::Resettable for IntEvent2RisSpec {
    const RESET_VALUE: u32 = 0;
}

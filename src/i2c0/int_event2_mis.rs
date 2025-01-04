#[doc = "Register `INT_EVENT2_MIS` reader"]
pub type R = crate::R<IntEvent2MisSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2MisMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMrxfifotrgSet = 1,
}
impl From<IntEvent2MisMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent2MisMrxfifotrgR = crate::BitReader<IntEvent2MisMrxfifotrg>;
impl IntEvent2MisMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMrxfifotrg {
        match self.bits {
            false => IntEvent2MisMrxfifotrg::IntEvent2MisMrxfifotrgClr,
            true => IntEvent2MisMrxfifotrg::IntEvent2MisMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent2MisMrxfifotrg::IntEvent2MisMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent2MisMrxfifotrg::IntEvent2MisMrxfifotrgSet
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2MisMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2MisMtxfifotrgSet = 1,
}
impl From<IntEvent2MisMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent2MisMtxfifotrgR = crate::BitReader<IntEvent2MisMtxfifotrg>;
impl IntEvent2MisMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisMtxfifotrg {
        match self.bits {
            false => IntEvent2MisMtxfifotrg::IntEvent2MisMtxfifotrgClr,
            true => IntEvent2MisMtxfifotrg::IntEvent2MisMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent2MisMtxfifotrg::IntEvent2MisMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent2MisMtxfifotrg::IntEvent2MisMtxfifotrgSet
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2MisSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2MisSrxfifotrgSet = 1,
}
impl From<IntEvent2MisSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent2MisSrxfifotrgR = crate::BitReader<IntEvent2MisSrxfifotrg>;
impl IntEvent2MisSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisSrxfifotrg {
        match self.bits {
            false => IntEvent2MisSrxfifotrg::IntEvent2MisSrxfifotrgClr,
            true => IntEvent2MisSrxfifotrg::IntEvent2MisSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent2MisSrxfifotrg::IntEvent2MisSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_srxfifotrg_set(&self) -> bool {
        *self == IntEvent2MisSrxfifotrg::IntEvent2MisSrxfifotrgSet
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent2MisStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent2MisStxfifotrgSet = 1,
}
impl From<IntEvent2MisStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent2MisStxfifotrgR = crate::BitReader<IntEvent2MisStxfifotrg>;
impl IntEvent2MisStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisStxfifotrg {
        match self.bits {
            false => IntEvent2MisStxfifotrg::IntEvent2MisStxfifotrgClr,
            true => IntEvent2MisStxfifotrg::IntEvent2MisStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent2MisStxfifotrg::IntEvent2MisStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_stxfifotrg_set(&self) -> bool {
        *self == IntEvent2MisStxfifotrg::IntEvent2MisStxfifotrgSet
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_mis_mrxfifotrg(&self) -> IntEvent2MisMrxfifotrgR {
        IntEvent2MisMrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event2_mis_mtxfifotrg(&self) -> IntEvent2MisMtxfifotrgR {
        IntEvent2MisMtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_mis_srxfifotrg(&self) -> IntEvent2MisSrxfifotrgR {
        IntEvent2MisSrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event2_mis_stxfifotrg(&self) -> IntEvent2MisStxfifotrgR {
        IntEvent2MisStxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2MisSpec;
impl crate::RegisterSpec for IntEvent2MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent2MisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_MIS to value 0"]
impl crate::Resettable for IntEvent2MisSpec {
    const RESET_VALUE: u32 = 0;
}

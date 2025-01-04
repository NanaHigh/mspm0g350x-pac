#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1RisMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisMrxfifotrgSet = 1,
}
impl From<IntEvent1RisMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent1RisMrxfifotrgR = crate::BitReader<IntEvent1RisMrxfifotrg>;
impl IntEvent1RisMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisMrxfifotrg {
        match self.bits {
            false => IntEvent1RisMrxfifotrg::IntEvent1RisMrxfifotrgClr,
            true => IntEvent1RisMrxfifotrg::IntEvent1RisMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent1RisMrxfifotrg::IntEvent1RisMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent1RisMrxfifotrg::IntEvent1RisMrxfifotrgSet
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1RisMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisMtxfifotrgSet = 1,
}
impl From<IntEvent1RisMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent1RisMtxfifotrgR = crate::BitReader<IntEvent1RisMtxfifotrg>;
impl IntEvent1RisMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisMtxfifotrg {
        match self.bits {
            false => IntEvent1RisMtxfifotrg::IntEvent1RisMtxfifotrgClr,
            true => IntEvent1RisMtxfifotrg::IntEvent1RisMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent1RisMtxfifotrg::IntEvent1RisMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent1RisMtxfifotrg::IntEvent1RisMtxfifotrgSet
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1RisSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisSrxfifotrgSet = 1,
}
impl From<IntEvent1RisSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent1RisSrxfifotrgR = crate::BitReader<IntEvent1RisSrxfifotrg>;
impl IntEvent1RisSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisSrxfifotrg {
        match self.bits {
            false => IntEvent1RisSrxfifotrg::IntEvent1RisSrxfifotrgClr,
            true => IntEvent1RisSrxfifotrg::IntEvent1RisSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent1RisSrxfifotrg::IntEvent1RisSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_srxfifotrg_set(&self) -> bool {
        *self == IntEvent1RisSrxfifotrg::IntEvent1RisSrxfifotrgSet
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1RisStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisStxfifotrgSet = 1,
}
impl From<IntEvent1RisStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent1RisStxfifotrgR = crate::BitReader<IntEvent1RisStxfifotrg>;
impl IntEvent1RisStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisStxfifotrg {
        match self.bits {
            false => IntEvent1RisStxfifotrg::IntEvent1RisStxfifotrgClr,
            true => IntEvent1RisStxfifotrg::IntEvent1RisStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent1RisStxfifotrg::IntEvent1RisStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_stxfifotrg_set(&self) -> bool {
        *self == IntEvent1RisStxfifotrg::IntEvent1RisStxfifotrgSet
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_ris_mrxfifotrg(&self) -> IntEvent1RisMrxfifotrgR {
        IntEvent1RisMrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_ris_mtxfifotrg(&self) -> IntEvent1RisMtxfifotrgR {
        IntEvent1RisMtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_ris_srxfifotrg(&self) -> IntEvent1RisSrxfifotrgR {
        IntEvent1RisSrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_ris_stxfifotrg(&self) -> IntEvent1RisStxfifotrgR {
        IntEvent1RisStxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1RisSpec;
impl crate::RegisterSpec for IntEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent1RisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_RIS to value 0"]
impl crate::Resettable for IntEvent1RisSpec {
    const RESET_VALUE: u32 = 0;
}

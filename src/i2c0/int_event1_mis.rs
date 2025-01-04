#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1MisMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisMrxfifotrgSet = 1,
}
impl From<IntEvent1MisMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent1MisMrxfifotrgR = crate::BitReader<IntEvent1MisMrxfifotrg>;
impl IntEvent1MisMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisMrxfifotrg {
        match self.bits {
            false => IntEvent1MisMrxfifotrg::IntEvent1MisMrxfifotrgClr,
            true => IntEvent1MisMrxfifotrg::IntEvent1MisMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent1MisMrxfifotrg::IntEvent1MisMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent1MisMrxfifotrg::IntEvent1MisMrxfifotrgSet
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1MisMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisMtxfifotrgSet = 1,
}
impl From<IntEvent1MisMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent1MisMtxfifotrgR = crate::BitReader<IntEvent1MisMtxfifotrg>;
impl IntEvent1MisMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisMtxfifotrg {
        match self.bits {
            false => IntEvent1MisMtxfifotrg::IntEvent1MisMtxfifotrgClr,
            true => IntEvent1MisMtxfifotrg::IntEvent1MisMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent1MisMtxfifotrg::IntEvent1MisMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent1MisMtxfifotrg::IntEvent1MisMtxfifotrgSet
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1MisSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisSrxfifotrgSet = 1,
}
impl From<IntEvent1MisSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent1MisSrxfifotrgR = crate::BitReader<IntEvent1MisSrxfifotrg>;
impl IntEvent1MisSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisSrxfifotrg {
        match self.bits {
            false => IntEvent1MisSrxfifotrg::IntEvent1MisSrxfifotrgClr,
            true => IntEvent1MisSrxfifotrg::IntEvent1MisSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent1MisSrxfifotrg::IntEvent1MisSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_srxfifotrg_set(&self) -> bool {
        *self == IntEvent1MisSrxfifotrg::IntEvent1MisSrxfifotrgSet
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent1MisStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisStxfifotrgSet = 1,
}
impl From<IntEvent1MisStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent1MisStxfifotrgR = crate::BitReader<IntEvent1MisStxfifotrg>;
impl IntEvent1MisStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisStxfifotrg {
        match self.bits {
            false => IntEvent1MisStxfifotrg::IntEvent1MisStxfifotrgClr,
            true => IntEvent1MisStxfifotrg::IntEvent1MisStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent1MisStxfifotrg::IntEvent1MisStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_stxfifotrg_set(&self) -> bool {
        *self == IntEvent1MisStxfifotrg::IntEvent1MisStxfifotrgSet
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_mis_mrxfifotrg(&self) -> IntEvent1MisMrxfifotrgR {
        IntEvent1MisMrxfifotrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event1_mis_mtxfifotrg(&self) -> IntEvent1MisMtxfifotrgR {
        IntEvent1MisMtxfifotrgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_mis_srxfifotrg(&self) -> IntEvent1MisSrxfifotrgR {
        IntEvent1MisSrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event1_mis_stxfifotrg(&self) -> IntEvent1MisStxfifotrgR {
        IntEvent1MisStxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1MisSpec;
impl crate::RegisterSpec for IntEvent1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent1MisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_MIS to value 0"]
impl crate::Resettable for IntEvent1MisSpec {
    const RESET_VALUE: u32 = 0;
}

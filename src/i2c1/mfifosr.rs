#[doc = "Register `MFIFOSR` reader"]
pub type R = crate::R<MfifosrSpec>;
#[doc = "Field `MFIFOSR_RXFIFOCNT` reader - Number of Bytes which could be read from the RX FIFO"]
pub type MfifosrRxfifocntR = crate::FieldReader;
#[doc = "RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfifosrRxflush {
    #[doc = "0: INACTIVE"]
    MfifosrRxflushInactive = 0,
    #[doc = "1: ACTIVE"]
    MfifosrRxflushActive = 1,
}
impl From<MfifosrRxflush> for bool {
    #[inline(always)]
    fn from(variant: MfifosrRxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFIFOSR_RXFLUSH` reader - RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop."]
pub type MfifosrRxflushR = crate::BitReader<MfifosrRxflush>;
impl MfifosrRxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MfifosrRxflush {
        match self.bits {
            false => MfifosrRxflush::MfifosrRxflushInactive,
            true => MfifosrRxflush::MfifosrRxflushActive,
        }
    }
    #[doc = "INACTIVE"]
    #[inline(always)]
    pub fn is_mfifosr_rxflush_inactive(&self) -> bool {
        *self == MfifosrRxflush::MfifosrRxflushInactive
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_mfifosr_rxflush_active(&self) -> bool {
        *self == MfifosrRxflush::MfifosrRxflushActive
    }
}
#[doc = "Field `MFIFOSR_TXFIFOCNT` reader - Number of Bytes which could be put into the TX FIFO"]
pub type MfifosrTxfifocntR = crate::FieldReader;
#[doc = "TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfifosrTxflush {
    #[doc = "0: INACTIVE"]
    MfifosrTxflushInactive = 0,
    #[doc = "1: ACTIVE"]
    MfifosrTxflushActive = 1,
}
impl From<MfifosrTxflush> for bool {
    #[inline(always)]
    fn from(variant: MfifosrTxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MFIFOSR_TXFLUSH` reader - TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop."]
pub type MfifosrTxflushR = crate::BitReader<MfifosrTxflush>;
impl MfifosrTxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MfifosrTxflush {
        match self.bits {
            false => MfifosrTxflush::MfifosrTxflushInactive,
            true => MfifosrTxflush::MfifosrTxflushActive,
        }
    }
    #[doc = "INACTIVE"]
    #[inline(always)]
    pub fn is_mfifosr_txflush_inactive(&self) -> bool {
        *self == MfifosrTxflush::MfifosrTxflushInactive
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_mfifosr_txflush_active(&self) -> bool {
        *self == MfifosrTxflush::MfifosrTxflushActive
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Bytes which could be read from the RX FIFO"]
    #[inline(always)]
    pub fn mfifosr_rxfifocnt(&self) -> MfifosrRxfifocntR {
        MfifosrRxfifocntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop."]
    #[inline(always)]
    pub fn mfifosr_rxflush(&self) -> MfifosrRxflushR {
        MfifosrRxflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bytes which could be put into the TX FIFO"]
    #[inline(always)]
    pub fn mfifosr_txfifocnt(&self) -> MfifosrTxfifocntR {
        MfifosrTxfifocntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop."]
    #[inline(always)]
    pub fn mfifosr_txflush(&self) -> MfifosrTxflushR {
        MfifosrTxflushR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "I2C Master FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mfifosr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MfifosrSpec;
impl crate::RegisterSpec for MfifosrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mfifosr::R`](R) reader structure"]
impl crate::Readable for MfifosrSpec {}
#[doc = "`reset()` method sets MFIFOSR to value 0x0800"]
impl crate::Resettable for MfifosrSpec {
    const RESET_VALUE: u32 = 0x0800;
}

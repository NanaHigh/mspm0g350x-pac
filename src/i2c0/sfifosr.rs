#[doc = "Register `SFIFOSR` reader"]
pub type R = crate::R<SfifosrSpec>;
#[doc = "Field `SFIFOSR_RXFIFOCNT` reader - Number of Bytes which could be read from the RX FIFO"]
pub type SfifosrRxfifocntR = crate::FieldReader;
#[doc = "RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfifosrRxflush {
    #[doc = "0: INACTIVE"]
    SfifosrRxflushInactive = 0,
    #[doc = "1: ACTIVE"]
    SfifosrRxflushActive = 1,
}
impl From<SfifosrRxflush> for bool {
    #[inline(always)]
    fn from(variant: SfifosrRxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFIFOSR_RXFLUSH` reader - RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop."]
pub type SfifosrRxflushR = crate::BitReader<SfifosrRxflush>;
impl SfifosrRxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfifosrRxflush {
        match self.bits {
            false => SfifosrRxflush::SfifosrRxflushInactive,
            true => SfifosrRxflush::SfifosrRxflushActive,
        }
    }
    #[doc = "INACTIVE"]
    #[inline(always)]
    pub fn is_sfifosr_rxflush_inactive(&self) -> bool {
        *self == SfifosrRxflush::SfifosrRxflushInactive
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_sfifosr_rxflush_active(&self) -> bool {
        *self == SfifosrRxflush::SfifosrRxflushActive
    }
}
#[doc = "Field `SFIFOSR_TXFIFOCNT` reader - Number of Bytes which could be put into the TX FIFO"]
pub type SfifosrTxfifocntR = crate::FieldReader;
#[doc = "TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfifosrTxflush {
    #[doc = "0: INACTIVE"]
    SfifosrTxflushInactive = 0,
    #[doc = "1: ACTIVE"]
    SfifosrTxflushActive = 1,
}
impl From<SfifosrTxflush> for bool {
    #[inline(always)]
    fn from(variant: SfifosrTxflush) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFIFOSR_TXFLUSH` reader - TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop."]
pub type SfifosrTxflushR = crate::BitReader<SfifosrTxflush>;
impl SfifosrTxflushR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SfifosrTxflush {
        match self.bits {
            false => SfifosrTxflush::SfifosrTxflushInactive,
            true => SfifosrTxflush::SfifosrTxflushActive,
        }
    }
    #[doc = "INACTIVE"]
    #[inline(always)]
    pub fn is_sfifosr_txflush_inactive(&self) -> bool {
        *self == SfifosrTxflush::SfifosrTxflushInactive
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_sfifosr_txflush_active(&self) -> bool {
        *self == SfifosrTxflush::SfifosrTxflushActive
    }
}
impl R {
    #[doc = "Bits 0:3 - Number of Bytes which could be read from the RX FIFO"]
    #[inline(always)]
    pub fn sfifosr_rxfifocnt(&self) -> SfifosrRxfifocntR {
        SfifosrRxfifocntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - RX FIFO Flush When this bit is set a Flush operation for the RX FIFO is active. Clear the RXFLUSH bit in the control register to stop."]
    #[inline(always)]
    pub fn sfifosr_rxflush(&self) -> SfifosrRxflushR {
        SfifosrRxflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Bytes which could be put into the TX FIFO"]
    #[inline(always)]
    pub fn sfifosr_txfifocnt(&self) -> SfifosrTxfifocntR {
        SfifosrTxfifocntR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - TX FIFO Flush When this bit is set a Flush operation for the TX FIFO is active. Clear the TXFLUSH bit in the control register to stop."]
    #[inline(always)]
    pub fn sfifosr_txflush(&self) -> SfifosrTxflushR {
        SfifosrTxflushR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "I2C Slave FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifosr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SfifosrSpec;
impl crate::RegisterSpec for SfifosrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sfifosr::R`](R) reader structure"]
impl crate::Readable for SfifosrSpec {}
#[doc = "`reset()` method sets SFIFOSR to value 0x0800"]
impl crate::Resettable for SfifosrSpec {
    const RESET_VALUE: u32 = 0x0800;
}

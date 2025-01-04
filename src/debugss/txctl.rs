#[doc = "Register `TXCTL` reader"]
pub type R = crate::R<TxctlSpec>;
#[doc = "Indicates data request in DSSM.TXD, set on write via Debug AP to DSSM.TXD. A read of the DSSM.TXD register by SW will clear the TX field. The tool can check that TXD is empty by reading this field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxctlTransmit {
    #[doc = "0: EMPTY"]
    TxctlTransmitEmpty = 0,
    #[doc = "1: FULL"]
    TxctlTransmitFull = 1,
}
impl From<TxctlTransmit> for bool {
    #[inline(always)]
    fn from(variant: TxctlTransmit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXCTL_TRANSMIT` reader - Indicates data request in DSSM.TXD, set on write via Debug AP to DSSM.TXD. A read of the DSSM.TXD register by SW will clear the TX field. The tool can check that TXD is empty by reading this field."]
pub type TxctlTransmitR = crate::BitReader<TxctlTransmit>;
impl TxctlTransmitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxctlTransmit {
        match self.bits {
            false => TxctlTransmit::TxctlTransmitEmpty,
            true => TxctlTransmit::TxctlTransmitFull,
        }
    }
    #[doc = "EMPTY"]
    #[inline(always)]
    pub fn is_txctl_transmit_empty(&self) -> bool {
        *self == TxctlTransmit::TxctlTransmitEmpty
    }
    #[doc = "FULL"]
    #[inline(always)]
    pub fn is_txctl_transmit_full(&self) -> bool {
        *self == TxctlTransmit::TxctlTransmitFull
    }
}
#[doc = "Field `TXCTL_TRANSMIT_FLAGS` reader - Generic TX flags that can be set by external debug tool. Functionality is defined by SW."]
pub type TxctlTransmitFlagsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Indicates data request in DSSM.TXD, set on write via Debug AP to DSSM.TXD. A read of the DSSM.TXD register by SW will clear the TX field. The tool can check that TXD is empty by reading this field."]
    #[inline(always)]
    pub fn txctl_transmit(&self) -> TxctlTransmitR {
        TxctlTransmitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Generic TX flags that can be set by external debug tool. Functionality is defined by SW."]
    #[inline(always)]
    pub fn txctl_transmit_flags(&self) -> TxctlTransmitFlagsR {
        TxctlTransmitFlagsR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
#[doc = "Transmit control register\n\nYou can [`read`](crate::Reg::read) this register and get [`txctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxctlSpec;
impl crate::RegisterSpec for TxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctl::R`](R) reader structure"]
impl crate::Readable for TxctlSpec {}
#[doc = "`reset()` method sets TXCTL to value 0"]
impl crate::Resettable for TxctlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `TXD` reader"]
pub type R = crate::R<TxdSpec>;
#[doc = "Field `TXD_TX_DATA` reader - Contains data written by an external debug tool to the SEC-AP TXDATA register"]
pub type TxdTxDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Contains data written by an external debug tool to the SEC-AP TXDATA register"]
    #[inline(always)]
    pub fn txd_tx_data(&self) -> TxdTxDataR {
        TxdTxDataR::new(self.bits)
    }
}
#[doc = "Transmit data register\n\nYou can [`read`](crate::Reg::read) this register and get [`txd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdSpec;
impl crate::RegisterSpec for TxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txd::R`](R) reader structure"]
impl crate::Readable for TxdSpec {}
#[doc = "`reset()` method sets TXD to value 0"]
impl crate::Resettable for TxdSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MCAN_TXFQS` reader"]
pub type R = crate::R<McanTxfqsSpec>;
#[doc = "Field `MCAN_TXFQS_TFFL` reader - Tx FIFO Free Level. Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1')."]
pub type McanTxfqsTfflR = crate::FieldReader;
#[doc = "Field `MCAN_TXFQS_TFGI` reader - Tx FIFO Get Index. Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1'). Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
pub type McanTxfqsTfgiR = crate::FieldReader;
#[doc = "Field `MCAN_TXFQS_TFQP` reader - Tx FIFO/Queue Put Index. Tx FIFO/Queue write index pointer, range 0 to 31. Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
pub type McanTxfqsTfqpR = crate::FieldReader;
#[doc = "Field `MCAN_TXFQS_TFQF` reader - Tx FIFO/Queue Full 0 Tx FIFO/Queue not full 1 Tx FIFO/Queue full"]
pub type McanTxfqsTfqfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Tx FIFO Free Level. Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 32. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1')."]
    #[inline(always)]
    pub fn mcan_txfqs_tffl(&self) -> McanTxfqsTfflR {
        McanTxfqsTfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Tx FIFO Get Index. Tx FIFO read index pointer, range 0 to 31. Read as zero when Tx Queue operation is configured (TXBC.TFQM = '1'). Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
    #[inline(always)]
    pub fn mcan_txfqs_tfgi(&self) -> McanTxfqsTfgiR {
        McanTxfqsTfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Tx FIFO/Queue Put Index. Tx FIFO/Queue write index pointer, range 0 to 31. Note: In case of mixed configurations where dedicated Tx Buffers are combined with a Tx FIFO or a Tx Queue, the Put and Get Indices indicate the number of the Tx Buffer starting with the first dedicated Tx Buffers. Example: For a configuration of 12 dedicated Tx Buffers and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth buffer of the Tx FIFO."]
    #[inline(always)]
    pub fn mcan_txfqs_tfqp(&self) -> McanTxfqsTfqpR {
        McanTxfqsTfqpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Tx FIFO/Queue Full 0 Tx FIFO/Queue not full 1 Tx FIFO/Queue full"]
    #[inline(always)]
    pub fn mcan_txfqs_tfqf(&self) -> McanTxfqsTfqfR {
        McanTxfqsTfqfR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "MCAN Tx FIFO / Queue Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txfqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxfqsSpec;
impl crate::RegisterSpec for McanTxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txfqs::R`](R) reader structure"]
impl crate::Readable for McanTxfqsSpec {}
#[doc = "`reset()` method sets MCAN_TXFQS to value 0"]
impl crate::Resettable for McanTxfqsSpec {
    const RESET_VALUE: u32 = 0;
}

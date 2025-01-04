#[doc = "Register `MCAN_TXEFS` reader"]
pub type R = crate::R<McanTxefsSpec>;
#[doc = "Field `MCAN_TXEFS_EFFL` reader - Event FIFO Fill Level. Number of elements stored in Tx Event FIFO, range 0 to 32."]
pub type McanTxefsEfflR = crate::FieldReader;
#[doc = "Field `MCAN_TXEFS_EFGI` reader - Event FIFO Get Index. Tx Event FIFO read index pointer, range 0 to 31."]
pub type McanTxefsEfgiR = crate::FieldReader;
#[doc = "Field `MCAN_TXEFS_EFPI` reader - Event FIFO Put Index.Tx Event FIFO write index pointer, range 0 to 31."]
pub type McanTxefsEfpiR = crate::FieldReader;
#[doc = "Field `MCAN_TXEFS_EFF` reader - Event FIFO Full 0 Tx Event FIFO not full 1 Tx Event FIFO full"]
pub type McanTxefsEffR = crate::BitReader;
#[doc = "Field `MCAN_TXEFS_TEFL` reader - Tx Event FIFO Element Lost. This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0 No Tx Event FIFO element lost 1 Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
pub type McanTxefsTeflR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level. Number of elements stored in Tx Event FIFO, range 0 to 32."]
    #[inline(always)]
    pub fn mcan_txefs_effl(&self) -> McanTxefsEfflR {
        McanTxefsEfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index. Tx Event FIFO read index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn mcan_txefs_efgi(&self) -> McanTxefsEfgiR {
        McanTxefsEfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO Put Index.Tx Event FIFO write index pointer, range 0 to 31."]
    #[inline(always)]
    pub fn mcan_txefs_efpi(&self) -> McanTxefsEfpiR {
        McanTxefsEfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full 0 Tx Event FIFO not full 1 Tx Event FIFO full"]
    #[inline(always)]
    pub fn mcan_txefs_eff(&self) -> McanTxefsEffR {
        McanTxefsEffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost. This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset, this bit is also reset. 0 No Tx Event FIFO element lost 1 Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero."]
    #[inline(always)]
    pub fn mcan_txefs_tefl(&self) -> McanTxefsTeflR {
        McanTxefsTeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "MCAN Tx Event FIFO Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txefs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxefsSpec;
impl crate::RegisterSpec for McanTxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txefs::R`](R) reader structure"]
impl crate::Readable for McanTxefsSpec {}
#[doc = "`reset()` method sets MCAN_TXEFS to value 0"]
impl crate::Resettable for McanTxefsSpec {
    const RESET_VALUE: u32 = 0;
}

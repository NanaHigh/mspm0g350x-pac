#[doc = "Register `MCAN_RXF1S` reader"]
pub type R = crate::R<McanRxf1sSpec>;
#[doc = "Field `MCAN_RXF1S_F1FL` reader - Rx FIFO 1 Fill Level. Number of elements stored in Rx FIFO 1, range 0 to 64."]
pub type McanRxf1sF1flR = crate::FieldReader;
#[doc = "Field `MCAN_RXF1S_F1GI` reader - Rx FIFO 1 Get Index. Rx FIFO 1 read index pointer, range 0 to 63."]
pub type McanRxf1sF1giR = crate::FieldReader;
#[doc = "Field `MCAN_RXF1S_F1PI` reader - Rx FIFO 1 Put Index. Rx FIFO 1 write index pointer, range 0 to 63."]
pub type McanRxf1sF1piR = crate::FieldReader;
#[doc = "Field `MCAN_RXF1S_F1F` reader - Rx FIFO 1 Full 0 Rx FIFO 1 not full 1 Rx FIFO 1 full"]
pub type McanRxf1sF1fR = crate::BitReader;
#[doc = "Field `MCAN_RXF1S_RF1L` reader - Rx FIFO 1 Message Lost. This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0 No Rx FIFO 1 message lost 1 Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = '1' will not set this flag."]
pub type McanRxf1sRf1lR = crate::BitReader;
#[doc = "Field `MCAN_RXF1S_DMS` reader - Debug Message Status 00 Idle state, wait for reception of debug messages 01 Debug message A received 10 Debug messages A, B received 11 Debug messages A, B, C received"]
pub type McanRxf1sDmsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level. Number of elements stored in Rx FIFO 1, range 0 to 64."]
    #[inline(always)]
    pub fn mcan_rxf1s_f1fl(&self) -> McanRxf1sF1flR {
        McanRxf1sF1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 1 Get Index. Rx FIFO 1 read index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn mcan_rxf1s_f1gi(&self) -> McanRxf1sF1giR {
        McanRxf1sF1giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 1 Put Index. Rx FIFO 1 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn mcan_rxf1s_f1pi(&self) -> McanRxf1sF1piR {
        McanRxf1sF1piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full 0 Rx FIFO 1 not full 1 Rx FIFO 1 full"]
    #[inline(always)]
    pub fn mcan_rxf1s_f1f(&self) -> McanRxf1sF1fR {
        McanRxf1sF1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost. This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset, this bit is also reset. 0 No Rx FIFO 1 message lost 1 Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero Note: Overwriting the oldest message when RXF1C.F1OM = '1' will not set this flag."]
    #[inline(always)]
    pub fn mcan_rxf1s_rf1l(&self) -> McanRxf1sRf1lR {
        McanRxf1sRf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status 00 Idle state, wait for reception of debug messages 01 Debug message A received 10 Debug messages A, B received 11 Debug messages A, B, C received"]
    #[inline(always)]
    pub fn mcan_rxf1s_dms(&self) -> McanRxf1sDmsR {
        McanRxf1sDmsR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MCAN Rx FIFO 1 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf1s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxf1sSpec;
impl crate::RegisterSpec for McanRxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxf1s::R`](R) reader structure"]
impl crate::Readable for McanRxf1sSpec {}
#[doc = "`reset()` method sets MCAN_RXF1S to value 0"]
impl crate::Resettable for McanRxf1sSpec {
    const RESET_VALUE: u32 = 0;
}

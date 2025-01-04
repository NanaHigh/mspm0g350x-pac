#[doc = "Register `MCAN_RXF0S` reader"]
pub type R = crate::R<McanRxf0sSpec>;
#[doc = "Field `MCAN_RXF0S_F0FL` reader - Rx FIFO 0 Fill Level. Number of elements stored in Rx FIFO 0, range 0 to 64."]
pub type McanRxf0sF0flR = crate::FieldReader;
#[doc = "Field `MCAN_RXF0S_F0GI` reader - Rx FIFO 0 Get Index. Rx FIFO 0 read index pointer, range 0 to 63."]
pub type McanRxf0sF0giR = crate::FieldReader;
#[doc = "Field `MCAN_RXF0S_F0PI` reader - Rx FIFO 0 Put Index. Rx FIFO 0 write index pointer, range 0 to 63."]
pub type McanRxf0sF0piR = crate::FieldReader;
#[doc = "Field `MCAN_RXF0S_F0F` reader - Rx FIFO 0 Full 0 Rx FIFO 0 not full 1 Rx FIFO 0 full"]
pub type McanRxf0sF0fR = crate::BitReader;
#[doc = "Field `MCAN_RXF0S_RF0L` reader - Rx FIFO 0 Message Lost. This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0 No Rx FIFO 0 message lost 1 Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = '1' will not set this flag."]
pub type McanRxf0sRf0lR = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level. Number of elements stored in Rx FIFO 0, range 0 to 64."]
    #[inline(always)]
    pub fn mcan_rxf0s_f0fl(&self) -> McanRxf0sF0flR {
        McanRxf0sF0flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index. Rx FIFO 0 read index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn mcan_rxf0s_f0gi(&self) -> McanRxf0sF0giR {
        McanRxf0sF0giR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index. Rx FIFO 0 write index pointer, range 0 to 63."]
    #[inline(always)]
    pub fn mcan_rxf0s_f0pi(&self) -> McanRxf0sF0piR {
        McanRxf0sF0piR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full 0 Rx FIFO 0 not full 1 Rx FIFO 0 full"]
    #[inline(always)]
    pub fn mcan_rxf0s_f0f(&self) -> McanRxf0sF0fR {
        McanRxf0sF0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost. This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset, this bit is also reset. 0 No Rx FIFO 0 message lost 1 Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero Note: Overwriting the oldest message when RXF0C.F0OM = '1' will not set this flag."]
    #[inline(always)]
    pub fn mcan_rxf0s_rf0l(&self) -> McanRxf0sRf0lR {
        McanRxf0sRf0lR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "MCAN Rx FIFO 0 Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf0s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxf0sSpec;
impl crate::RegisterSpec for McanRxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxf0s::R`](R) reader structure"]
impl crate::Readable for McanRxf0sSpec {}
#[doc = "`reset()` method sets MCAN_RXF0S to value 0"]
impl crate::Resettable for McanRxf0sSpec {
    const RESET_VALUE: u32 = 0;
}

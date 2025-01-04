#[doc = "Register `MCAN_GFC` reader"]
pub type R = crate::R<McanGfcSpec>;
#[doc = "Register `MCAN_GFC` writer"]
pub type W = crate::W<McanGfcSpec>;
#[doc = "Field `MCAN_GFC_RRFE` reader - Reject Remote Frames Extended 0 Filter remote frames with 29-bit extended IDs 1 Reject all remote frames with 29-bit extended IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcRrfeR = crate::BitReader;
#[doc = "Field `MCAN_GFC_RRFE` writer - Reject Remote Frames Extended 0 Filter remote frames with 29-bit extended IDs 1 Reject all remote frames with 29-bit extended IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcRrfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_GFC_RRFS` reader - Reject Remote Frames Standard 0 Filter remote frames with 11-bit standard IDs 1 Reject all remote frames with 11-bit standard IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcRrfsR = crate::BitReader;
#[doc = "Field `MCAN_GFC_RRFS` writer - Reject Remote Frames Standard 0 Filter remote frames with 11-bit standard IDs 1 Reject all remote frames with 11-bit standard IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcRrfsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_GFC_ANFE` reader - Accept Non-matching Frames Extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcAnfeR = crate::FieldReader;
#[doc = "Field `MCAN_GFC_ANFE` writer - Accept Non-matching Frames Extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcAnfeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCAN_GFC_ANFS` reader - Accept Non-matching Frames Standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcAnfsR = crate::FieldReader;
#[doc = "Field `MCAN_GFC_ANFS` writer - Accept Non-matching Frames Standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanGfcAnfsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Reject Remote Frames Extended 0 Filter remote frames with 29-bit extended IDs 1 Reject all remote frames with 29-bit extended IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_rrfe(&self) -> McanGfcRrfeR {
        McanGfcRrfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard 0 Filter remote frames with 11-bit standard IDs 1 Reject all remote frames with 11-bit standard IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_rrfs(&self) -> McanGfcRrfsR {
        McanGfcRrfsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_anfe(&self) -> McanGfcAnfeR {
        McanGfcAnfeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_anfs(&self) -> McanGfcAnfsR {
        McanGfcAnfsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reject Remote Frames Extended 0 Filter remote frames with 29-bit extended IDs 1 Reject all remote frames with 29-bit extended IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_rrfe(&mut self) -> McanGfcRrfeW<McanGfcSpec> {
        McanGfcRrfeW::new(self, 0)
    }
    #[doc = "Bit 1 - Reject Remote Frames Standard 0 Filter remote frames with 11-bit standard IDs 1 Reject all remote frames with 11-bit standard IDs Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_rrfs(&mut self) -> McanGfcRrfsW<McanGfcSpec> {
        McanGfcRrfsW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Accept Non-matching Frames Extended. Defines how received messages with 29-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_anfe(&mut self) -> McanGfcAnfeW<McanGfcSpec> {
        McanGfcAnfeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Accept Non-matching Frames Standard. Defines how received messages with 11-bit IDs that do not match any element of the filter list are treated. 00 Accept in Rx FIFO 0 01 Accept in Rx FIFO 1 10 Reject 11 Reject Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_gfc_anfs(&mut self) -> McanGfcAnfsW<McanGfcSpec> {
        McanGfcAnfsW::new(self, 4)
    }
}
#[doc = "MCAN Global Filter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanGfcSpec;
impl crate::RegisterSpec for McanGfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_gfc::R`](R) reader structure"]
impl crate::Readable for McanGfcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_gfc::W`](W) writer structure"]
impl crate::Writable for McanGfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_GFC to value 0"]
impl crate::Resettable for McanGfcSpec {
    const RESET_VALUE: u32 = 0;
}

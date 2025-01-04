#[doc = "Register `MCAN_RXF1A` reader"]
pub type R = crate::R<McanRxf1aSpec>;
#[doc = "Register `MCAN_RXF1A` writer"]
pub type W = crate::W<McanRxf1aSpec>;
#[doc = "Field `MCAN_RXF1A_F1AI` reader - Rx FIFO 1 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
pub type McanRxf1aF1aiR = crate::FieldReader;
#[doc = "Field `MCAN_RXF1A_F1AI` writer - Rx FIFO 1 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
pub type McanRxf1aF1aiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
    #[inline(always)]
    pub fn mcan_rxf1a_f1ai(&self) -> McanRxf1aF1aiR {
        McanRxf1aF1aiR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 1 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI + 1 and update the FIFO 1 Fill Level RXF1S.F1FL."]
    #[inline(always)]
    pub fn mcan_rxf1a_f1ai(&mut self) -> McanRxf1aF1aiW<McanRxf1aSpec> {
        McanRxf1aF1aiW::new(self, 0)
    }
}
#[doc = "MCAN Rx FIFO 1 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf1a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf1a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxf1aSpec;
impl crate::RegisterSpec for McanRxf1aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxf1a::R`](R) reader structure"]
impl crate::Readable for McanRxf1aSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_rxf1a::W`](W) writer structure"]
impl crate::Writable for McanRxf1aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_RXF1A to value 0"]
impl crate::Resettable for McanRxf1aSpec {
    const RESET_VALUE: u32 = 0;
}

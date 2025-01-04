#[doc = "Register `MCAN_RXF0A` reader"]
pub type R = crate::R<McanRxf0aSpec>;
#[doc = "Register `MCAN_RXF0A` writer"]
pub type W = crate::W<McanRxf0aSpec>;
#[doc = "Field `MCAN_RXF0A_F0AI` reader - Rx FIFO 0 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
pub type McanRxf0aF0aiR = crate::FieldReader;
#[doc = "Field `MCAN_RXF0A_F0AI` writer - Rx FIFO 0 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
pub type McanRxf0aF0aiW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
    #[inline(always)]
    pub fn mcan_rxf0a_f0ai(&self) -> McanRxf0aF0aiR {
        McanRxf0aF0aiR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index. After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI + 1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
    #[inline(always)]
    pub fn mcan_rxf0a_f0ai(&mut self) -> McanRxf0aF0aiW<McanRxf0aSpec> {
        McanRxf0aF0aiW::new(self, 0)
    }
}
#[doc = "MCAN Rx FIFO 0 Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxf0aSpec;
impl crate::RegisterSpec for McanRxf0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxf0a::R`](R) reader structure"]
impl crate::Readable for McanRxf0aSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_rxf0a::W`](W) writer structure"]
impl crate::Writable for McanRxf0aSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_RXF0A to value 0"]
impl crate::Resettable for McanRxf0aSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `MCAN_TXEFA` reader"]
pub type R = crate::R<McanTxefaSpec>;
#[doc = "Register `MCAN_TXEFA` writer"]
pub type W = crate::W<McanTxefaSpec>;
#[doc = "Field `MCAN_TXEFA_EFAI` reader - Event FIFO Acknowledge Index. After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
pub type McanTxefaEfaiR = crate::FieldReader;
#[doc = "Field `MCAN_TXEFA_EFAI` writer - Event FIFO Acknowledge Index. After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
pub type McanTxefaEfaiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index. After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
    #[inline(always)]
    pub fn mcan_txefa_efai(&self) -> McanTxefaEfaiR {
        McanTxefaEfaiR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Event FIFO Acknowledge Index. After the Host has read an element or a sequence of elements from the Tx Event FIFO it has to write the index of the last element read from Tx Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI to EFAI + 1 and update the Event FIFO Fill Level TXEFS.EFFL."]
    #[inline(always)]
    pub fn mcan_txefa_efai(&mut self) -> McanTxefaEfaiW<McanTxefaSpec> {
        McanTxefaEfaiW::new(self, 0)
    }
}
#[doc = "MCAN Tx Event FIFO Acknowledge\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txefa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txefa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxefaSpec;
impl crate::RegisterSpec for McanTxefaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txefa::R`](R) reader structure"]
impl crate::Readable for McanTxefaSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_txefa::W`](W) writer structure"]
impl crate::Writable for McanTxefaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TXEFA to value 0"]
impl crate::Resettable for McanTxefaSpec {
    const RESET_VALUE: u32 = 0;
}

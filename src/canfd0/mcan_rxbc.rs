#[doc = "Register `MCAN_RXBC` reader"]
pub type R = crate::R<McanRxbcSpec>;
#[doc = "Register `MCAN_RXBC` writer"]
pub type W = crate::W<McanRxbcSpec>;
#[doc = "Field `MCAN_RXBC_RBSA` reader - Rx Buffer Start Address. Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address). +I466"]
pub type McanRxbcRbsaR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_RXBC_RBSA` writer - Rx Buffer Start Address. Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address). +I466"]
pub type McanRxbcRbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 2:15 - Rx Buffer Start Address. Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address). +I466"]
    #[inline(always)]
    pub fn mcan_rxbc_rbsa(&self) -> McanRxbcRbsaR {
        McanRxbcRbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx Buffer Start Address. Configures the start address of the Rx Buffers section in the Message RAM (32-bit word address). +I466"]
    #[inline(always)]
    pub fn mcan_rxbc_rbsa(&mut self) -> McanRxbcRbsaW<McanRxbcSpec> {
        McanRxbcRbsaW::new(self, 2)
    }
}
#[doc = "MCAN Rx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxbcSpec;
impl crate::RegisterSpec for McanRxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxbc::R`](R) reader structure"]
impl crate::Readable for McanRxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_rxbc::W`](W) writer structure"]
impl crate::Writable for McanRxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_RXBC to value 0"]
impl crate::Resettable for McanRxbcSpec {
    const RESET_VALUE: u32 = 0;
}

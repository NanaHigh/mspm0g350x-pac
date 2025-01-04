#[doc = "Register `MCAN_TXEFC` reader"]
pub type R = crate::R<McanTxefcSpec>;
#[doc = "Register `MCAN_TXEFC` writer"]
pub type W = crate::W<McanTxefcSpec>;
#[doc = "Field `MCAN_TXEFC_EFSA` reader - Event FIFO Start Address. Start address of Tx Event FIFO in Message RAM (32-bit word address)."]
pub type McanTxefcEfsaR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_TXEFC_EFSA` writer - Event FIFO Start Address. Start address of Tx Event FIFO in Message RAM (32-bit word address)."]
pub type McanTxefcEfsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MCAN_TXEFC_EFS` reader - Event FIFO Size. The Tx Event FIFO elements are indexed from 0 to EFS - 1. 0 Tx Event FIFO disabled 1-32 Number of Tx Event FIFO elements &gt;32 Values greater than 32 are interpreted as 32"]
pub type McanTxefcEfsR = crate::FieldReader;
#[doc = "Field `MCAN_TXEFC_EFS` writer - Event FIFO Size. The Tx Event FIFO elements are indexed from 0 to EFS - 1. 0 Tx Event FIFO disabled 1-32 Number of Tx Event FIFO elements &gt;32 Values greater than 32 are interpreted as 32"]
pub type McanTxefcEfsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCAN_TXEFC_EFWM` reader - Event FIFO Watermark 0 Watermark interrupt disabled 1-32 Level for Tx Event FIFO watermark interrupt (IR.TEFW) &gt;32 Watermark interrupt disabled"]
pub type McanTxefcEfwmR = crate::FieldReader;
#[doc = "Field `MCAN_TXEFC_EFWM` writer - Event FIFO Watermark 0 Watermark interrupt disabled 1-32 Level for Tx Event FIFO watermark interrupt (IR.TEFW) &gt;32 Watermark interrupt disabled"]
pub type McanTxefcEfwmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 2:15 - Event FIFO Start Address. Start address of Tx Event FIFO in Message RAM (32-bit word address)."]
    #[inline(always)]
    pub fn mcan_txefc_efsa(&self) -> McanTxefcEfsaR {
        McanTxefcEfsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size. The Tx Event FIFO elements are indexed from 0 to EFS - 1. 0 Tx Event FIFO disabled 1-32 Number of Tx Event FIFO elements &gt;32 Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub fn mcan_txefc_efs(&self) -> McanTxefcEfsR {
        McanTxefcEfsR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark 0 Watermark interrupt disabled 1-32 Level for Tx Event FIFO watermark interrupt (IR.TEFW) &gt;32 Watermark interrupt disabled"]
    #[inline(always)]
    pub fn mcan_txefc_efwm(&self) -> McanTxefcEfwmR {
        McanTxefcEfwmR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Event FIFO Start Address. Start address of Tx Event FIFO in Message RAM (32-bit word address)."]
    #[inline(always)]
    pub fn mcan_txefc_efsa(&mut self) -> McanTxefcEfsaW<McanTxefcSpec> {
        McanTxefcEfsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - Event FIFO Size. The Tx Event FIFO elements are indexed from 0 to EFS - 1. 0 Tx Event FIFO disabled 1-32 Number of Tx Event FIFO elements &gt;32 Values greater than 32 are interpreted as 32"]
    #[inline(always)]
    pub fn mcan_txefc_efs(&mut self) -> McanTxefcEfsW<McanTxefcSpec> {
        McanTxefcEfsW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark 0 Watermark interrupt disabled 1-32 Level for Tx Event FIFO watermark interrupt (IR.TEFW) &gt;32 Watermark interrupt disabled"]
    #[inline(always)]
    pub fn mcan_txefc_efwm(&mut self) -> McanTxefcEfwmW<McanTxefcSpec> {
        McanTxefcEfwmW::new(self, 24)
    }
}
#[doc = "MCAN Tx Event FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txefc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txefc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxefcSpec;
impl crate::RegisterSpec for McanTxefcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txefc::R`](R) reader structure"]
impl crate::Readable for McanTxefcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_txefc::W`](W) writer structure"]
impl crate::Writable for McanTxefcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TXEFC to value 0"]
impl crate::Resettable for McanTxefcSpec {
    const RESET_VALUE: u32 = 0;
}

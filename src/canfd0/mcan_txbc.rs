#[doc = "Register `MCAN_TXBC` reader"]
pub type R = crate::R<McanTxbcSpec>;
#[doc = "Register `MCAN_TXBC` writer"]
pub type W = crate::W<McanTxbcSpec>;
#[doc = "Field `MCAN_TXBC_TBSA` reader - Tx Buffers Start Address. Start address of Tx Buffers section in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcTbsaR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_TXBC_TBSA` writer - Tx Buffers Start Address. Start address of Tx Buffers section in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcTbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MCAN_TXBC_NDTB` reader - Number of Dedicated Transmit Buffers 0 No Dedicated Tx Buffers 1-32 Number of Dedicated Tx Buffers &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcNdtbR = crate::FieldReader;
#[doc = "Field `MCAN_TXBC_NDTB` writer - Number of Dedicated Transmit Buffers 0 No Dedicated Tx Buffers 1-32 Number of Dedicated Tx Buffers &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcNdtbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCAN_TXBC_TFQS` reader - Transmit FIFO/Queue Size 0 No Tx FIFO/Queue 1-32 Number of Tx Buffers used for Tx FIFO/Queue &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcTfqsR = crate::FieldReader;
#[doc = "Field `MCAN_TXBC_TFQS` writer - Transmit FIFO/Queue Size 0 No Tx FIFO/Queue 1-32 Number of Tx Buffers used for Tx FIFO/Queue &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcTfqsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MCAN_TXBC_TFQM` reader - Tx FIFO/Queue Mode 0 Tx FIFO operation 1 Tx Queue operation Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcTfqmR = crate::BitReader;
#[doc = "Field `MCAN_TXBC_TFQM` writer - Tx FIFO/Queue Mode 0 Tx FIFO operation 1 Tx Queue operation Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxbcTfqmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Tx Buffers Start Address. Start address of Tx Buffers section in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_tbsa(&self) -> McanTxbcTbsaR {
        McanTxbcTbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers 0 No Dedicated Tx Buffers 1-32 Number of Dedicated Tx Buffers &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_ndtb(&self) -> McanTxbcNdtbR {
        McanTxbcNdtbR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size 0 No Tx FIFO/Queue 1-32 Number of Tx Buffers used for Tx FIFO/Queue &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_tfqs(&self) -> McanTxbcTfqsR {
        McanTxbcTfqsR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode 0 Tx FIFO operation 1 Tx Queue operation Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_tfqm(&self) -> McanTxbcTfqmR {
        McanTxbcTfqmR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Tx Buffers Start Address. Start address of Tx Buffers section in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_tbsa(&mut self) -> McanTxbcTbsaW<McanTxbcSpec> {
        McanTxbcTbsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers 0 No Dedicated Tx Buffers 1-32 Number of Dedicated Tx Buffers &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_ndtb(&mut self) -> McanTxbcNdtbW<McanTxbcSpec> {
        McanTxbcNdtbW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size 0 No Tx FIFO/Queue 1-32 Number of Tx Buffers used for Tx FIFO/Queue &gt;32 Values greater than 32 are interpreted as 32 Note: Be aware that the sum of TFQS and NDTB may be not greater than 32. There is no check for erroneous configurations. The Tx Buffers section in the Message RAM starts with the dedicated Tx Buffers. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_tfqs(&mut self) -> McanTxbcTfqsW<McanTxbcSpec> {
        McanTxbcTfqsW::new(self, 24)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode 0 Tx FIFO operation 1 Tx Queue operation Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txbc_tfqm(&mut self) -> McanTxbcTfqmW<McanTxbcSpec> {
        McanTxbcTfqmW::new(self, 30)
    }
}
#[doc = "MCAN Tx Buffer Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxbcSpec;
impl crate::RegisterSpec for McanTxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txbc::R`](R) reader structure"]
impl crate::Readable for McanTxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_txbc::W`](W) writer structure"]
impl crate::Writable for McanTxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TXBC to value 0"]
impl crate::Resettable for McanTxbcSpec {
    const RESET_VALUE: u32 = 0;
}

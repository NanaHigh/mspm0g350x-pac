#[doc = "Register `MCAN_TXESC` reader"]
pub type R = crate::R<McanTxescSpec>;
#[doc = "Register `MCAN_TXESC` writer"]
pub type W = crate::W<McanTxescSpec>;
#[doc = "Field `MCAN_TXESC_TBDS` reader - Tx Buffer Data Field Size 000 8 byte data field 001 12 byte data field 010 16 byte data field 011 20 byte data field 100 24 byte data field 101 32 byte data field 110 48 byte data field 111 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as &amp;quot;0xCC&amp;quot; (padding bytes). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxescTbdsR = crate::FieldReader;
#[doc = "Field `MCAN_TXESC_TBDS` writer - Tx Buffer Data Field Size 000 8 byte data field 001 12 byte data field 010 16 byte data field 011 20 byte data field 100 24 byte data field 101 32 byte data field 110 48 byte data field 111 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as &amp;quot;0xCC&amp;quot; (padding bytes). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTxescTbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size 000 8 byte data field 001 12 byte data field 010 16 byte data field 011 20 byte data field 100 24 byte data field 101 32 byte data field 110 48 byte data field 111 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as &amp;quot;0xCC&amp;quot; (padding bytes). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txesc_tbds(&self) -> McanTxescTbdsR {
        McanTxescTbdsR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size 000 8 byte data field 001 12 byte data field 010 16 byte data field 011 20 byte data field 100 24 byte data field 101 32 byte data field 110 48 byte data field 111 64 byte data field Note: In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS, the bytes not defined by the Tx Buffer are transmitted as &amp;quot;0xCC&amp;quot; (padding bytes). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_txesc_tbds(&mut self) -> McanTxescTbdsW<McanTxescSpec> {
        McanTxescTbdsW::new(self, 0)
    }
}
#[doc = "MCAN Tx Buffer Element Size Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxescSpec;
impl crate::RegisterSpec for McanTxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txesc::R`](R) reader structure"]
impl crate::Readable for McanTxescSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_txesc::W`](W) writer structure"]
impl crate::Writable for McanTxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TXESC to value 0"]
impl crate::Resettable for McanTxescSpec {
    const RESET_VALUE: u32 = 0;
}

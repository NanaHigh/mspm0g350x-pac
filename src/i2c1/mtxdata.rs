#[doc = "Register `MTXDATA` reader"]
pub type R = crate::R<MtxdataSpec>;
#[doc = "Register `MTXDATA` writer"]
pub type W = crate::W<MtxdataSpec>;
#[doc = "Field `MTXDATA_VALUE` reader - Transmit Data This byte contains the data to be transferred during the next transaction."]
pub type MtxdataValueR = crate::FieldReader;
#[doc = "Field `MTXDATA_VALUE` writer - Transmit Data This byte contains the data to be transferred during the next transaction."]
pub type MtxdataValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Data This byte contains the data to be transferred during the next transaction."]
    #[inline(always)]
    pub fn mtxdata_value(&self) -> MtxdataValueR {
        MtxdataValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data This byte contains the data to be transferred during the next transaction."]
    #[inline(always)]
    pub fn mtxdata_value(&mut self) -> MtxdataValueW<MtxdataSpec> {
        MtxdataValueW::new(self, 0)
    }
}
#[doc = "I2C Master TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`mtxdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtxdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtxdataSpec;
impl crate::RegisterSpec for MtxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtxdata::R`](R) reader structure"]
impl crate::Readable for MtxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`mtxdata::W`](W) writer structure"]
impl crate::Writable for MtxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTXDATA to value 0"]
impl crate::Resettable for MtxdataSpec {
    const RESET_VALUE: u32 = 0;
}

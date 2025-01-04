#[doc = "Register `STXDATA` reader"]
pub type R = crate::R<StxdataSpec>;
#[doc = "Register `STXDATA` writer"]
pub type W = crate::W<StxdataSpec>;
#[doc = "Field `STXDATA_VALUE` reader - Transmit Data This byte contains the data to be transferred during the next transaction."]
pub type StxdataValueR = crate::FieldReader;
#[doc = "Field `STXDATA_VALUE` writer - Transmit Data This byte contains the data to be transferred during the next transaction."]
pub type StxdataValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Data This byte contains the data to be transferred during the next transaction."]
    #[inline(always)]
    pub fn stxdata_value(&self) -> StxdataValueR {
        StxdataValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data This byte contains the data to be transferred during the next transaction."]
    #[inline(always)]
    pub fn stxdata_value(&mut self) -> StxdataValueW<StxdataSpec> {
        StxdataValueW::new(self, 0)
    }
}
#[doc = "I2C Slave TXData\n\nYou can [`read`](crate::Reg::read) this register and get [`stxdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stxdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StxdataSpec;
impl crate::RegisterSpec for StxdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stxdata::R`](R) reader structure"]
impl crate::Readable for StxdataSpec {}
#[doc = "`write(|w| ..)` method takes [`stxdata::W`](W) writer structure"]
impl crate::Writable for StxdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STXDATA to value 0"]
impl crate::Resettable for StxdataSpec {
    const RESET_VALUE: u32 = 0;
}

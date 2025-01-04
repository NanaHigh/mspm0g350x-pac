#[doc = "Register `RXD` reader"]
pub type R = crate::R<RxdSpec>;
#[doc = "Register `RXD` writer"]
pub type W = crate::W<RxdSpec>;
#[doc = "Field `RXD_RX_DATA` reader - Contains data written by SM/OW."]
pub type RxdRxDataR = crate::FieldReader<u32>;
#[doc = "Field `RXD_RX_DATA` writer - Contains data written by SM/OW."]
pub type RxdRxDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Contains data written by SM/OW."]
    #[inline(always)]
    pub fn rxd_rx_data(&self) -> RxdRxDataR {
        RxdRxDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains data written by SM/OW."]
    #[inline(always)]
    pub fn rxd_rx_data(&mut self) -> RxdRxDataW<RxdSpec> {
        RxdRxDataW::new(self, 0)
    }
}
#[doc = "Receive data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdSpec;
impl crate::RegisterSpec for RxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd::R`](R) reader structure"]
impl crate::Readable for RxdSpec {}
#[doc = "`write(|w| ..)` method takes [`rxd::W`](W) writer structure"]
impl crate::Writable for RxdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RxdSpec {
    const RESET_VALUE: u32 = 0;
}

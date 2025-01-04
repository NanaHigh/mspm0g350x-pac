#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `ADDR_VALUE` reader - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
pub type AddrValueR = crate::FieldReader;
#[doc = "Field `ADDR_VALUE` writer - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
pub type AddrValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
    #[inline(always)]
    pub fn addr_value(&self) -> AddrValueR {
        AddrValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address for 9-Bit Mode This field contains the address that should be matched when UARTxAMASK is FFh."]
    #[inline(always)]
    pub fn addr_value(&mut self) -> AddrValueW<AddrSpec> {
        AddrValueW::new(self, 0)
    }
}
#[doc = "Self Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `AMASK` reader"]
pub type R = crate::R<AmaskSpec>;
#[doc = "Register `AMASK` writer"]
pub type W = crate::W<AmaskSpec>;
#[doc = "Field `AMASK_VALUE` reader - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
pub type AmaskValueR = crate::FieldReader;
#[doc = "Field `AMASK_VALUE` writer - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
pub type AmaskValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
    #[inline(always)]
    pub fn amask_value(&self) -> AmaskValueR {
        AmaskValueR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a set of addresses that should be matched. A 0 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register is don't care. A 1 bit in the MSK bitfield configures, that the corresponding bit in the ADDR bitfield of the UARTxADDR register must match."]
    #[inline(always)]
    pub fn amask_value(&mut self) -> AmaskValueW<AmaskSpec> {
        AmaskValueW::new(self, 0)
    }
}
#[doc = "Self Address Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmaskSpec;
impl crate::RegisterSpec for AmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amask::R`](R) reader structure"]
impl crate::Readable for AmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`amask::W`](W) writer structure"]
impl crate::Writable for AmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMASK to value 0xff"]
impl crate::Resettable for AmaskSpec {
    const RESET_VALUE: u32 = 0xff;
}

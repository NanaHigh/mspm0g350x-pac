#[doc = "Register `DATA0` reader"]
pub type R = crate::R<Data0Spec>;
#[doc = "Register `DATA0` writer"]
pub type W = crate::W<Data0Spec>;
#[doc = "Field `DATA0_DATA_VALUE` reader - This is the data written for digital to analog conversion."]
pub type Data0DataValueR = crate::FieldReader<u16>;
#[doc = "Field `DATA0_DATA_VALUE` writer - This is the data written for digital to analog conversion."]
pub type Data0DataValueW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - This is the data written for digital to analog conversion."]
    #[inline(always)]
    pub fn data0_data_value(&self) -> Data0DataValueR {
        Data0DataValueR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This is the data written for digital to analog conversion."]
    #[inline(always)]
    pub fn data0_data_value(&mut self) -> Data0DataValueW<Data0Spec> {
        Data0DataValueW::new(self, 0)
    }
}
#[doc = "Data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Spec;
impl crate::RegisterSpec for Data0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for Data0Spec {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for Data0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for Data0Spec {
    const RESET_VALUE: u32 = 0;
}

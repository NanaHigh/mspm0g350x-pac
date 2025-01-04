#[doc = "Register `CPS` reader"]
pub type R = crate::R<CpsSpec>;
#[doc = "Register `CPS` writer"]
pub type W = crate::W<CpsSpec>;
#[doc = "Field `CPS_PCNT` reader - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
pub type CpsPcntR = crate::FieldReader;
#[doc = "Field `CPS_PCNT` writer - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
pub type CpsPcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
    #[inline(always)]
    pub fn cps_pcnt(&self) -> CpsPcntR {
        CpsPcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pre-Scale Count This field specifies the pre-scale count value. The selected TIMCLK source is divided by a value of (PCNT+1). A PCNT value of 0 divides TIMCLK by 1, effectively bypassing the divider. A PCNT value of greater than 0 divides the TIMCLK source generating a slower clock"]
    #[inline(always)]
    pub fn cps_pcnt(&mut self) -> CpsPcntW<CpsSpec> {
        CpsPcntW::new(self, 0)
    }
}
#[doc = "Clock Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsSpec;
impl crate::RegisterSpec for CpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cps::R`](R) reader structure"]
impl crate::Readable for CpsSpec {}
#[doc = "`write(|w| ..)` method takes [`cps::W`](W) writer structure"]
impl crate::Writable for CpsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPS to value 0"]
impl crate::Resettable for CpsSpec {
    const RESET_VALUE: u32 = 0;
}

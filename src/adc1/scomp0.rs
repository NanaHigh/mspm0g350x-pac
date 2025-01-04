#[doc = "Register `SCOMP0` reader"]
pub type R = crate::R<Scomp0Spec>;
#[doc = "Register `SCOMP0` writer"]
pub type W = crate::W<Scomp0Spec>;
#[doc = "Field `SCOMP0_VAL` reader - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type Scomp0ValR = crate::FieldReader<u16>;
#[doc = "Field `SCOMP0_VAL` writer - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type Scomp0ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn scomp0_val(&self) -> Scomp0ValR {
        Scomp0ValR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn scomp0_val(&mut self) -> Scomp0ValW<Scomp0Spec> {
        Scomp0ValW::new(self, 0)
    }
}
#[doc = "Sample Time Compare 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scomp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scomp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scomp0Spec;
impl crate::RegisterSpec for Scomp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scomp0::R`](R) reader structure"]
impl crate::Readable for Scomp0Spec {}
#[doc = "`write(|w| ..)` method takes [`scomp0::W`](W) writer structure"]
impl crate::Writable for Scomp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCOMP0 to value 0"]
impl crate::Resettable for Scomp0Spec {
    const RESET_VALUE: u32 = 0;
}

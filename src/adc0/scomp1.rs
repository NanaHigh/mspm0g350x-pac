#[doc = "Register `SCOMP1` reader"]
pub type R = crate::R<Scomp1Spec>;
#[doc = "Register `SCOMP1` writer"]
pub type W = crate::W<Scomp1Spec>;
#[doc = "Field `SCOMP1_VAL` reader - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type Scomp1ValR = crate::FieldReader<u16>;
#[doc = "Field `SCOMP1_VAL` writer - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
pub type Scomp1ValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn scomp1_val(&self) -> Scomp1ValR {
        Scomp1ValR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specifies the number of sample clocks. When VAL = 0 or 1, number of sample clocks = Sample clock divide value. When VAL &gt; 1, number of sample clocks = VAL x Sample clock divide value. Note: Sample clock divide value is not the value written to SCLKDIV but the actual divide value (SCLKDIV = 2 implies divide value is 4). Example: VAL = 4, SCLKDIV = 3 implies 32 sample clock cycles."]
    #[inline(always)]
    pub fn scomp1_val(&mut self) -> Scomp1ValW<Scomp1Spec> {
        Scomp1ValW::new(self, 0)
    }
}
#[doc = "Sample Time Compare 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scomp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scomp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scomp1Spec;
impl crate::RegisterSpec for Scomp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scomp1::R`](R) reader structure"]
impl crate::Readable for Scomp1Spec {}
#[doc = "`write(|w| ..)` method takes [`scomp1::W`](W) writer structure"]
impl crate::Writable for Scomp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCOMP1 to value 0"]
impl crate::Resettable for Scomp1Spec {
    const RESET_VALUE: u32 = 0;
}

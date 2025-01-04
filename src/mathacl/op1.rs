#[doc = "Register `OP1` reader"]
pub type R = crate::R<Op1Spec>;
#[doc = "Register `OP1` writer"]
pub type W = crate::W<Op1Spec>;
#[doc = "Field `OP1_DATA` reader - Operand 1 register."]
pub type Op1DataR = crate::FieldReader<u32>;
#[doc = "Field `OP1_DATA` writer - Operand 1 register."]
pub type Op1DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Operand 1 register."]
    #[inline(always)]
    pub fn op1_data(&self) -> Op1DataR {
        Op1DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Operand 1 register."]
    #[inline(always)]
    pub fn op1_data(&mut self) -> Op1DataW<Op1Spec> {
        Op1DataW::new(self, 0)
    }
}
#[doc = "Operand 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`op1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op1Spec;
impl crate::RegisterSpec for Op1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op1::R`](R) reader structure"]
impl crate::Readable for Op1Spec {}
#[doc = "`write(|w| ..)` method takes [`op1::W`](W) writer structure"]
impl crate::Writable for Op1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OP1 to value 0"]
impl crate::Resettable for Op1Spec {
    const RESET_VALUE: u32 = 0;
}

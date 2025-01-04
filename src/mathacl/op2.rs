#[doc = "Register `OP2` reader"]
pub type R = crate::R<Op2Spec>;
#[doc = "Register `OP2` writer"]
pub type W = crate::W<Op2Spec>;
#[doc = "Field `OP2_DATA` reader - Operand 2 Register"]
pub type Op2DataR = crate::FieldReader<u32>;
#[doc = "Field `OP2_DATA` writer - Operand 2 Register"]
pub type Op2DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Operand 2 Register"]
    #[inline(always)]
    pub fn op2_data(&self) -> Op2DataR {
        Op2DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Operand 2 Register"]
    #[inline(always)]
    pub fn op2_data(&mut self) -> Op2DataW<Op2Spec> {
        Op2DataW::new(self, 0)
    }
}
#[doc = "Operand 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`op2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`op2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Op2Spec;
impl crate::RegisterSpec for Op2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`op2::R`](R) reader structure"]
impl crate::Readable for Op2Spec {}
#[doc = "`write(|w| ..)` method takes [`op2::W`](W) writer structure"]
impl crate::Writable for Op2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OP2 to value 0"]
impl crate::Resettable for Op2Spec {
    const RESET_VALUE: u32 = 0;
}

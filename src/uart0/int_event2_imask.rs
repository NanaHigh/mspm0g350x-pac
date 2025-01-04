#[doc = "Register `INT_EVENT2_IMASK` reader"]
pub type R = crate::R<IntEvent2ImaskSpec>;
#[doc = "Register `INT_EVENT2_IMASK` writer"]
pub type W = crate::W<IntEvent2ImaskSpec>;
#[doc = "Enable UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskTxint {
    #[doc = "0: CLR"]
    IntEvent2ImaskTxintClr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskTxintSet = 1,
}
impl From<IntEvent2ImaskTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_TXINT` reader - Enable UART Transmit Interrupt."]
pub type IntEvent2ImaskTxintR = crate::BitReader<IntEvent2ImaskTxint>;
impl IntEvent2ImaskTxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskTxint {
        match self.bits {
            false => IntEvent2ImaskTxint::IntEvent2ImaskTxintClr,
            true => IntEvent2ImaskTxint::IntEvent2ImaskTxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_txint_clr(&self) -> bool {
        *self == IntEvent2ImaskTxint::IntEvent2ImaskTxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_txint_set(&self) -> bool {
        *self == IntEvent2ImaskTxint::IntEvent2ImaskTxintSet
    }
}
#[doc = "Field `INT_EVENT2_IMASK_TXINT` writer - Enable UART Transmit Interrupt."]
pub type IntEvent2ImaskTxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskTxint>;
impl<'a, REG> IntEvent2ImaskTxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_txint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskTxint::IntEvent2ImaskTxintClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_txint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskTxint::IntEvent2ImaskTxintSet)
    }
}
impl R {
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event2_imask_txint(&self) -> IntEvent2ImaskTxintR {
        IntEvent2ImaskTxintR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - Enable UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event2_imask_txint(&mut self) -> IntEvent2ImaskTxintW<IntEvent2ImaskSpec> {
        IntEvent2ImaskTxintW::new(self, 11)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2ImaskSpec;
impl crate::RegisterSpec for IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for IntEvent2ImaskSpec {
    const RESET_VALUE: u32 = 0;
}

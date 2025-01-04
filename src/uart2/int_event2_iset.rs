#[doc = "Register `INT_EVENT2_ISET` writer"]
pub type W = crate::W<IntEvent2IsetSpec>;
#[doc = "Set UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetTxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetTxintNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetTxintSet = 1,
}
impl From<IntEvent2IsetTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_TXINT` writer - Set UART Transmit Interrupt."]
pub type IntEvent2IsetTxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetTxint>;
impl<'a, REG> IntEvent2IsetTxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_txint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetTxint::IntEvent2IsetTxintNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_txint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetTxint::IntEvent2IsetTxintSet)
    }
}
impl W {
    #[doc = "Bit 11 - Set UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event2_iset_txint(&mut self) -> IntEvent2IsetTxintW<IntEvent2IsetSpec> {
        IntEvent2IsetTxintW::new(self, 11)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IsetSpec;
impl crate::RegisterSpec for IntEvent2IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent2IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ISET to value 0"]
impl crate::Resettable for IntEvent2IsetSpec {
    const RESET_VALUE: u32 = 0;
}

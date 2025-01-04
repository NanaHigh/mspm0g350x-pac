#[doc = "Register `INT_EVENT2_ICLR` writer"]
pub type W = crate::W<IntEvent2IclrSpec>;
#[doc = "Clear UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrTxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrTxintNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrTxintClr = 1,
}
impl From<IntEvent2IclrTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_TXINT` writer - Clear UART Transmit Interrupt."]
pub type IntEvent2IclrTxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrTxint>;
impl<'a, REG> IntEvent2IclrTxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_txint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrTxint::IntEvent2IclrTxintNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_txint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrTxint::IntEvent2IclrTxintClr)
    }
}
impl W {
    #[doc = "Bit 11 - Clear UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event2_iclr_txint(&mut self) -> IntEvent2IclrTxintW<IntEvent2IclrSpec> {
        IntEvent2IclrTxintW::new(self, 11)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IclrSpec;
impl crate::RegisterSpec for IntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent2IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for IntEvent2IclrSpec {
    const RESET_VALUE: u32 = 0;
}

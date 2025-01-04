#[doc = "Register `INT_EVENT2_ISET` writer"]
pub type W = crate::W<IntEvent2IsetSpec>;
#[doc = "Set Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetTx {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetTxNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetTxSet = 1,
}
impl From<IntEvent2IsetTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_TX` writer - Set Transmit FIFO event."]
pub type IntEvent2IsetTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetTx>;
impl<'a, REG> IntEvent2IsetTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_tx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetTx::IntEvent2IsetTxNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetTx::IntEvent2IsetTxSet)
    }
}
impl W {
    #[doc = "Bit 4 - Set Transmit FIFO event."]
    #[inline(always)]
    pub fn int_event2_iset_tx(&mut self) -> IntEvent2IsetTxW<IntEvent2IsetSpec> {
        IntEvent2IsetTxW::new(self, 4)
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

#[doc = "Register `INT_EVENT2_IMASK` reader"]
pub type R = crate::R<IntEvent2ImaskSpec>;
#[doc = "Register `INT_EVENT2_IMASK` writer"]
pub type W = crate::W<IntEvent2ImaskSpec>;
#[doc = "Transmit FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskTx {
    #[doc = "0: CLR"]
    IntEvent2ImaskTxClr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskTxSet = 1,
}
impl From<IntEvent2ImaskTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_TX` reader - Transmit FIFO event mask."]
pub type IntEvent2ImaskTxR = crate::BitReader<IntEvent2ImaskTx>;
impl IntEvent2ImaskTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskTx {
        match self.bits {
            false => IntEvent2ImaskTx::IntEvent2ImaskTxClr,
            true => IntEvent2ImaskTx::IntEvent2ImaskTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_tx_clr(&self) -> bool {
        *self == IntEvent2ImaskTx::IntEvent2ImaskTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_tx_set(&self) -> bool {
        *self == IntEvent2ImaskTx::IntEvent2ImaskTxSet
    }
}
#[doc = "Field `INT_EVENT2_IMASK_TX` writer - Transmit FIFO event mask."]
pub type IntEvent2ImaskTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskTx>;
impl<'a, REG> IntEvent2ImaskTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_tx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskTx::IntEvent2ImaskTxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskTx::IntEvent2ImaskTxSet)
    }
}
impl R {
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn int_event2_imask_tx(&self) -> IntEvent2ImaskTxR {
        IntEvent2ImaskTxR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn int_event2_imask_tx(&mut self) -> IntEvent2ImaskTxW<IntEvent2ImaskSpec> {
        IntEvent2ImaskTxW::new(self, 4)
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

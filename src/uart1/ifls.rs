#[doc = "Register `IFLS` reader"]
pub type R = crate::R<IflsSpec>;
#[doc = "Register `IFLS` writer"]
pub type W = crate::W<IflsSpec>;
#[doc = "Field `IFLS_TXIFLSEL` reader - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
pub type IflsTxiflselR = crate::FieldReader;
#[doc = "Field `IFLS_TXIFLSEL` writer - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
pub type IflsTxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IFLS_RXIFLSEL` reader - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
pub type IflsRxiflselR = crate::FieldReader;
#[doc = "Field `IFLS_RXIFLSEL` writer - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
pub type IflsRxiflselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IFLS_RXTOSEL` reader - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
pub type IflsRxtoselR = crate::FieldReader;
#[doc = "Field `IFLS_RXTOSEL` writer - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
pub type IflsRxtoselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn ifls_txiflsel(&self) -> IflsTxiflselR {
        IflsTxiflselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn ifls_rxiflsel(&self) -> IflsRxiflselR {
        IflsRxiflselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
    #[inline(always)]
    pub fn ifls_rxtosel(&self) -> IflsRxtoselR {
        IflsRxtoselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: Note: for undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn ifls_txiflsel(&mut self) -> IflsTxiflselW<IflsSpec> {
        IflsTxiflselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - UART Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows: Note: In ULP domain the trigger levels are used for: 0: LVL_1_4 4: LVL_FULL For undefined settings the default configuration is used."]
    #[inline(always)]
    pub fn ifls_rxiflsel(&mut self) -> IflsRxiflselW<IflsSpec> {
        IflsRxiflselW::new(self, 4)
    }
    #[doc = "Bits 8:11 - UART Receive Interrupt Timeout Select. When receiving no start edge for an additional character within the set bittimes a RX interrupt is set even if the FIFO level is not reached. A value of 0 disables this function."]
    #[inline(always)]
    pub fn ifls_rxtosel(&mut self) -> IflsRxtoselW<IflsSpec> {
        IflsRxtoselW::new(self, 8)
    }
}
#[doc = "UART Interrupt FIFO Level Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifls::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifls::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IflsSpec;
impl crate::RegisterSpec for IflsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ifls::R`](R) reader structure"]
impl crate::Readable for IflsSpec {}
#[doc = "`write(|w| ..)` method takes [`ifls::W`](W) writer structure"]
impl crate::Writable for IflsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFLS to value 0x22"]
impl crate::Resettable for IflsSpec {
    const RESET_VALUE: u32 = 0x22;
}

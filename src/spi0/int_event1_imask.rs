#[doc = "Register `INT_EVENT1_IMASK` reader"]
pub type R = crate::R<IntEvent1ImaskSpec>;
#[doc = "Register `INT_EVENT1_IMASK` writer"]
pub type W = crate::W<IntEvent1ImaskSpec>;
#[doc = "SPI Receive Time-Out event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRtout {
    #[doc = "0: CLR"]
    IntEvent1ImaskRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRtoutSet = 1,
}
impl From<IntEvent1ImaskRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTOUT` reader - SPI Receive Time-Out event mask."]
pub type IntEvent1ImaskRtoutR = crate::BitReader<IntEvent1ImaskRtout>;
impl IntEvent1ImaskRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRtout {
        match self.bits {
            false => IntEvent1ImaskRtout::IntEvent1ImaskRtoutClr,
            true => IntEvent1ImaskRtout::IntEvent1ImaskRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtout_clr(&self) -> bool {
        *self == IntEvent1ImaskRtout::IntEvent1ImaskRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rtout_set(&self) -> bool {
        *self == IntEvent1ImaskRtout::IntEvent1ImaskRtoutSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RTOUT` writer - SPI Receive Time-Out event mask."]
pub type IntEvent1ImaskRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRtout>;
impl<'a, REG> IntEvent1ImaskRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rtout_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtout::IntEvent1ImaskRtoutClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rtout_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRtout::IntEvent1ImaskRtoutSet)
    }
}
#[doc = "Receive FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskRx {
    #[doc = "0: CLR"]
    IntEvent1ImaskRxClr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskRxSet = 1,
}
impl From<IntEvent1ImaskRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RX` reader - Receive FIFO event mask."]
pub type IntEvent1ImaskRxR = crate::BitReader<IntEvent1ImaskRx>;
impl IntEvent1ImaskRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskRx {
        match self.bits {
            false => IntEvent1ImaskRx::IntEvent1ImaskRxClr,
            true => IntEvent1ImaskRx::IntEvent1ImaskRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_rx_clr(&self) -> bool {
        *self == IntEvent1ImaskRx::IntEvent1ImaskRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_rx_set(&self) -> bool {
        *self == IntEvent1ImaskRx::IntEvent1ImaskRxSet
    }
}
#[doc = "Field `INT_EVENT1_IMASK_RX` writer - Receive FIFO event mask."]
pub type IntEvent1ImaskRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskRx>;
impl<'a, REG> IntEvent1ImaskRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRx::IntEvent1ImaskRxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_rx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskRx::IntEvent1ImaskRxSet)
    }
}
impl R {
    #[doc = "Bit 2 - SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn int_event1_imask_rtout(&self) -> IntEvent1ImaskRtoutR {
        IntEvent1ImaskRtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event mask."]
    #[inline(always)]
    pub fn int_event1_imask_rx(&self) -> IntEvent1ImaskRxR {
        IntEvent1ImaskRxR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn int_event1_imask_rtout(&mut self) -> IntEvent1ImaskRtoutW<IntEvent1ImaskSpec> {
        IntEvent1ImaskRtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO event mask."]
    #[inline(always)]
    pub fn int_event1_imask_rx(&mut self) -> IntEvent1ImaskRxW<IntEvent1ImaskSpec> {
        IntEvent1ImaskRxW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1ImaskSpec;
impl crate::RegisterSpec for IntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for IntEvent1ImaskSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "SPI Receive Time-Out Event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRtout {
    #[doc = "0: CLR"]
    IntEvent1RisRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRtoutSet = 1,
}
impl From<IntEvent1RisRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RTOUT` reader - SPI Receive Time-Out Event."]
pub type IntEvent1RisRtoutR = crate::BitReader<IntEvent1RisRtout>;
impl IntEvent1RisRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRtout {
        match self.bits {
            false => IntEvent1RisRtout::IntEvent1RisRtoutClr,
            true => IntEvent1RisRtout::IntEvent1RisRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtout_clr(&self) -> bool {
        *self == IntEvent1RisRtout::IntEvent1RisRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtout_set(&self) -> bool {
        *self == IntEvent1RisRtout::IntEvent1RisRtoutSet
    }
}
#[doc = "Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRx {
    #[doc = "0: CLR"]
    IntEvent1RisRxClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRxSet = 1,
}
impl From<IntEvent1RisRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RX` reader - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type IntEvent1RisRxR = crate::BitReader<IntEvent1RisRx>;
impl IntEvent1RisRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRx {
        match self.bits {
            false => IntEvent1RisRx::IntEvent1RisRxClr,
            true => IntEvent1RisRx::IntEvent1RisRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rx_clr(&self) -> bool {
        *self == IntEvent1RisRx::IntEvent1RisRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rx_set(&self) -> bool {
        *self == IntEvent1RisRx::IntEvent1RisRxSet
    }
}
impl R {
    #[doc = "Bit 2 - SPI Receive Time-Out Event."]
    #[inline(always)]
    pub fn int_event1_ris_rtout(&self) -> IntEvent1RisRtoutR {
        IntEvent1RisRtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn int_event1_ris_rx(&self) -> IntEvent1RisRxR {
        IntEvent1RisRxR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1RisSpec;
impl crate::RegisterSpec for IntEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent1RisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_RIS to value 0"]
impl crate::Resettable for IntEvent1RisSpec {
    const RESET_VALUE: u32 = 0;
}

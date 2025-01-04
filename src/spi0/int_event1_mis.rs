#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "SPI Receive Time-Out event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRtout {
    #[doc = "0: CLR"]
    IntEvent1MisRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRtoutSet = 1,
}
impl From<IntEvent1MisRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RTOUT` reader - SPI Receive Time-Out event mask."]
pub type IntEvent1MisRtoutR = crate::BitReader<IntEvent1MisRtout>;
impl IntEvent1MisRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRtout {
        match self.bits {
            false => IntEvent1MisRtout::IntEvent1MisRtoutClr,
            true => IntEvent1MisRtout::IntEvent1MisRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtout_clr(&self) -> bool {
        *self == IntEvent1MisRtout::IntEvent1MisRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtout_set(&self) -> bool {
        *self == IntEvent1MisRtout::IntEvent1MisRtoutSet
    }
}
#[doc = "Receive FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRx {
    #[doc = "0: CLR"]
    IntEvent1MisRxClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRxSet = 1,
}
impl From<IntEvent1MisRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RX` reader - Receive FIFO event mask."]
pub type IntEvent1MisRxR = crate::BitReader<IntEvent1MisRx>;
impl IntEvent1MisRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRx {
        match self.bits {
            false => IntEvent1MisRx::IntEvent1MisRxClr,
            true => IntEvent1MisRx::IntEvent1MisRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rx_clr(&self) -> bool {
        *self == IntEvent1MisRx::IntEvent1MisRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rx_set(&self) -> bool {
        *self == IntEvent1MisRx::IntEvent1MisRxSet
    }
}
impl R {
    #[doc = "Bit 2 - SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn int_event1_mis_rtout(&self) -> IntEvent1MisRtoutR {
        IntEvent1MisRtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event mask."]
    #[inline(always)]
    pub fn int_event1_mis_rx(&self) -> IntEvent1MisRxR {
        IntEvent1MisRxR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1MisSpec;
impl crate::RegisterSpec for IntEvent1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent1MisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_MIS to value 0"]
impl crate::Resettable for IntEvent1MisSpec {
    const RESET_VALUE: u32 = 0;
}

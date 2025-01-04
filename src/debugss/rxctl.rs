#[doc = "Register `RXCTL` reader"]
pub type R = crate::R<RxctlSpec>;
#[doc = "Register `RXCTL` writer"]
pub type W = crate::W<RxctlSpec>;
#[doc = "Indicates SW write to the DSSM.RXD register. A read of the DSSM.RXD register by SWD Access Port will clear the RX field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxctlReceive {
    #[doc = "0: EMPTY"]
    RxctlReceiveEmpty = 0,
    #[doc = "1: FULL"]
    RxctlReceiveFull = 1,
}
impl From<RxctlReceive> for bool {
    #[inline(always)]
    fn from(variant: RxctlReceive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXCTL_RECEIVE` reader - Indicates SW write to the DSSM.RXD register. A read of the DSSM.RXD register by SWD Access Port will clear the RX field."]
pub type RxctlReceiveR = crate::BitReader<RxctlReceive>;
impl RxctlReceiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RxctlReceive {
        match self.bits {
            false => RxctlReceive::RxctlReceiveEmpty,
            true => RxctlReceive::RxctlReceiveFull,
        }
    }
    #[doc = "EMPTY"]
    #[inline(always)]
    pub fn is_rxctl_receive_empty(&self) -> bool {
        *self == RxctlReceive::RxctlReceiveEmpty
    }
    #[doc = "FULL"]
    #[inline(always)]
    pub fn is_rxctl_receive_full(&self) -> bool {
        *self == RxctlReceive::RxctlReceiveFull
    }
}
#[doc = "Field `RXCTL_RECEIVE_FLAGS` reader - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
pub type RxctlReceiveFlagsR = crate::FieldReader;
#[doc = "Field `RXCTL_RECEIVE_FLAGS` writer - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
pub type RxctlReceiveFlagsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Indicates SW write to the DSSM.RXD register. A read of the DSSM.RXD register by SWD Access Port will clear the RX field."]
    #[inline(always)]
    pub fn rxctl_receive(&self) -> RxctlReceiveR {
        RxctlReceiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
    #[inline(always)]
    pub fn rxctl_receive_flags(&self) -> RxctlReceiveFlagsR {
        RxctlReceiveFlagsR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Generic RX flags that can be set by SW and read by external debug tool. Functionality is defined by SW."]
    #[inline(always)]
    pub fn rxctl_receive_flags(&mut self) -> RxctlReceiveFlagsW<RxctlSpec> {
        RxctlReceiveFlagsW::new(self, 1)
    }
}
#[doc = "Receive control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxctlSpec;
impl crate::RegisterSpec for RxctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxctl::R`](R) reader structure"]
impl crate::Readable for RxctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rxctl::W`](W) writer structure"]
impl crate::Writable for RxctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXCTL to value 0"]
impl crate::Resettable for RxctlSpec {
    const RESET_VALUE: u32 = 0;
}

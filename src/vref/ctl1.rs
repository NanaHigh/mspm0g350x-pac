#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "These bits defines status of VREF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Ready {
    #[doc = "0: NOTRDY"]
    Ctl1ReadyNotrdy = 0,
    #[doc = "1: RDY"]
    Ctl1ReadyRdy = 1,
}
impl From<Ctl1Ready> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Ready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_READY` reader - These bits defines status of VREF"]
pub type Ctl1ReadyR = crate::BitReader<Ctl1Ready>;
impl Ctl1ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Ready {
        match self.bits {
            false => Ctl1Ready::Ctl1ReadyNotrdy,
            true => Ctl1Ready::Ctl1ReadyRdy,
        }
    }
    #[doc = "NOTRDY"]
    #[inline(always)]
    pub fn is_ctl1_ready_notrdy(&self) -> bool {
        *self == Ctl1Ready::Ctl1ReadyNotrdy
    }
    #[doc = "RDY"]
    #[inline(always)]
    pub fn is_ctl1_ready_rdy(&self) -> bool {
        *self == Ctl1Ready::Ctl1ReadyRdy
    }
}
impl R {
    #[doc = "Bit 0 - These bits defines status of VREF"]
    #[inline(always)]
    pub fn ctl1_ready(&self) -> Ctl1ReadyR {
        Ctl1ReadyR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}

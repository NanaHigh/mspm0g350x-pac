#[doc = "Register `FSUB_0` reader"]
pub type R = crate::R<Fsub0Spec>;
#[doc = "Register `FSUB_0` writer"]
pub type W = crate::W<Fsub0Spec>;
#[doc = "0 = disconnected. others = connected to channel_ID = CHANID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsub0Chanid {
    #[doc = "0: UNCONNECTED"]
    Fsub0ChanidUnconnected = 0,
}
impl From<Fsub0Chanid> for u8 {
    #[inline(always)]
    fn from(variant: Fsub0Chanid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsub0Chanid {
    type Ux = u8;
}
impl crate::IsEnum for Fsub0Chanid {}
#[doc = "Field `FSUB_0_CHANID` reader - 0 = disconnected. others = connected to channel_ID = CHANID."]
pub type Fsub0ChanidR = crate::FieldReader<Fsub0Chanid>;
impl Fsub0ChanidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsub0Chanid> {
        match self.bits {
            0 => Some(Fsub0Chanid::Fsub0ChanidUnconnected),
            _ => None,
        }
    }
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn is_fsub_0_chanid_unconnected(&self) -> bool {
        *self == Fsub0Chanid::Fsub0ChanidUnconnected
    }
}
#[doc = "Field `FSUB_0_CHANID` writer - 0 = disconnected. others = connected to channel_ID = CHANID."]
pub type Fsub0ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fsub0Chanid>;
impl<'a, REG> Fsub0ChanidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn fsub_0_chanid_unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Fsub0Chanid::Fsub0ChanidUnconnected)
    }
}
impl R {
    #[doc = "Bits 0:3 - 0 = disconnected. others = connected to channel_ID = CHANID."]
    #[inline(always)]
    pub fn fsub_0_chanid(&self) -> Fsub0ChanidR {
        Fsub0ChanidR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0 = disconnected. others = connected to channel_ID = CHANID."]
    #[inline(always)]
    pub fn fsub_0_chanid(&mut self) -> Fsub0ChanidW<Fsub0Spec> {
        Fsub0ChanidW::new(self, 0)
    }
}
#[doc = "Subscriber Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsub0Spec;
impl crate::RegisterSpec for Fsub0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsub_0::R`](R) reader structure"]
impl crate::Readable for Fsub0Spec {}
#[doc = "`write(|w| ..)` method takes [`fsub_0::W`](W) writer structure"]
impl crate::Writable for Fsub0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSUB_0 to value 0"]
impl crate::Resettable for Fsub0Spec {
    const RESET_VALUE: u32 = 0;
}

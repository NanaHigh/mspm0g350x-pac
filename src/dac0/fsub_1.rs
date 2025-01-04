#[doc = "Register `FSUB_1` reader"]
pub type R = crate::R<Fsub1Spec>;
#[doc = "Register `FSUB_1` writer"]
pub type W = crate::W<Fsub1Spec>;
#[doc = "0 = disconnected. others = connected to channel_ID = CHANID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsub1Chanid {
    #[doc = "0: UNCONNECTED"]
    Fsub1ChanidUnconnected = 0,
}
impl From<Fsub1Chanid> for u8 {
    #[inline(always)]
    fn from(variant: Fsub1Chanid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsub1Chanid {
    type Ux = u8;
}
impl crate::IsEnum for Fsub1Chanid {}
#[doc = "Field `FSUB_1_CHANID` reader - 0 = disconnected. others = connected to channel_ID = CHANID."]
pub type Fsub1ChanidR = crate::FieldReader<Fsub1Chanid>;
impl Fsub1ChanidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fsub1Chanid> {
        match self.bits {
            0 => Some(Fsub1Chanid::Fsub1ChanidUnconnected),
            _ => None,
        }
    }
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn is_fsub_1_chanid_unconnected(&self) -> bool {
        *self == Fsub1Chanid::Fsub1ChanidUnconnected
    }
}
#[doc = "Field `FSUB_1_CHANID` writer - 0 = disconnected. others = connected to channel_ID = CHANID."]
pub type Fsub1ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fsub1Chanid>;
impl<'a, REG> Fsub1ChanidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn fsub_1_chanid_unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Fsub1Chanid::Fsub1ChanidUnconnected)
    }
}
impl R {
    #[doc = "Bits 0:3 - 0 = disconnected. others = connected to channel_ID = CHANID."]
    #[inline(always)]
    pub fn fsub_1_chanid(&self) -> Fsub1ChanidR {
        Fsub1ChanidR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0 = disconnected. others = connected to channel_ID = CHANID."]
    #[inline(always)]
    pub fn fsub_1_chanid(&mut self) -> Fsub1ChanidW<Fsub1Spec> {
        Fsub1ChanidW::new(self, 0)
    }
}
#[doc = "Subscriber Port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fsub_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsub_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsub1Spec;
impl crate::RegisterSpec for Fsub1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsub_1::R`](R) reader structure"]
impl crate::Readable for Fsub1Spec {}
#[doc = "`write(|w| ..)` method takes [`fsub_1::W`](W) writer structure"]
impl crate::Writable for Fsub1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSUB_1 to value 0"]
impl crate::Resettable for Fsub1Spec {
    const RESET_VALUE: u32 = 0;
}

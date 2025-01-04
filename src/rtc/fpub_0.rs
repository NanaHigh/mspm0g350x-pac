#[doc = "Register `FPUB_0` reader"]
pub type R = crate::R<Fpub0Spec>;
#[doc = "Register `FPUB_0` writer"]
pub type W = crate::W<Fpub0Spec>;
#[doc = "0 = disconnected. 1-15 = connected to channelID = CHANID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fpub0Chanid {
    #[doc = "0: UNCONNECTED"]
    Fpub0ChanidUnconnected = 0,
}
impl From<Fpub0Chanid> for u8 {
    #[inline(always)]
    fn from(variant: Fpub0Chanid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fpub0Chanid {
    type Ux = u8;
}
impl crate::IsEnum for Fpub0Chanid {}
#[doc = "Field `FPUB_0_CHANID` reader - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
pub type Fpub0ChanidR = crate::FieldReader<Fpub0Chanid>;
impl Fpub0ChanidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fpub0Chanid> {
        match self.bits {
            0 => Some(Fpub0Chanid::Fpub0ChanidUnconnected),
            _ => None,
        }
    }
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn is_fpub_0_chanid_unconnected(&self) -> bool {
        *self == Fpub0Chanid::Fpub0ChanidUnconnected
    }
}
#[doc = "Field `FPUB_0_CHANID` writer - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
pub type Fpub0ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fpub0Chanid>;
impl<'a, REG> Fpub0ChanidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn fpub_0_chanid_unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Fpub0Chanid::Fpub0ChanidUnconnected)
    }
}
impl R {
    #[doc = "Bits 0:3 - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
    #[inline(always)]
    pub fn fpub_0_chanid(&self) -> Fpub0ChanidR {
        Fpub0ChanidR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
    #[inline(always)]
    pub fn fpub_0_chanid(&mut self) -> Fpub0ChanidW<Fpub0Spec> {
        Fpub0ChanidW::new(self, 0)
    }
}
#[doc = "Publisher Port 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpub0Spec;
impl crate::RegisterSpec for Fpub0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpub_0::R`](R) reader structure"]
impl crate::Readable for Fpub0Spec {}
#[doc = "`write(|w| ..)` method takes [`fpub_0::W`](W) writer structure"]
impl crate::Writable for Fpub0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPUB_0 to value 0"]
impl crate::Resettable for Fpub0Spec {
    const RESET_VALUE: u32 = 0;
}

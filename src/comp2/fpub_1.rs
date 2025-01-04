#[doc = "Register `FPUB_1` reader"]
pub type R = crate::R<Fpub1Spec>;
#[doc = "Register `FPUB_1` writer"]
pub type W = crate::W<Fpub1Spec>;
#[doc = "0 = disconnected. 1-15 = connected to channelID = CHANID.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fpub1Chanid {
    #[doc = "0: UNCONNECTED"]
    Fpub1ChanidUnconnected = 0,
}
impl From<Fpub1Chanid> for u8 {
    #[inline(always)]
    fn from(variant: Fpub1Chanid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fpub1Chanid {
    type Ux = u8;
}
impl crate::IsEnum for Fpub1Chanid {}
#[doc = "Field `FPUB_1_CHANID` reader - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
pub type Fpub1ChanidR = crate::FieldReader<Fpub1Chanid>;
impl Fpub1ChanidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fpub1Chanid> {
        match self.bits {
            0 => Some(Fpub1Chanid::Fpub1ChanidUnconnected),
            _ => None,
        }
    }
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn is_fpub_1_chanid_unconnected(&self) -> bool {
        *self == Fpub1Chanid::Fpub1ChanidUnconnected
    }
}
#[doc = "Field `FPUB_1_CHANID` writer - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
pub type Fpub1ChanidW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fpub1Chanid>;
impl<'a, REG> Fpub1ChanidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UNCONNECTED"]
    #[inline(always)]
    pub fn fpub_1_chanid_unconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Fpub1Chanid::Fpub1ChanidUnconnected)
    }
}
impl R {
    #[doc = "Bits 0:3 - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
    #[inline(always)]
    pub fn fpub_1_chanid(&self) -> Fpub1ChanidR {
        Fpub1ChanidR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0 = disconnected. 1-15 = connected to channelID = CHANID."]
    #[inline(always)]
    pub fn fpub_1_chanid(&mut self) -> Fpub1ChanidW<Fpub1Spec> {
        Fpub1ChanidW::new(self, 0)
    }
}
#[doc = "Publisher port 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fpub_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpub_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fpub1Spec;
impl crate::RegisterSpec for Fpub1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpub_1::R`](R) reader structure"]
impl crate::Readable for Fpub1Spec {}
#[doc = "`write(|w| ..)` method takes [`fpub_1::W`](W) writer structure"]
impl crate::Writable for Fpub1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPUB_1 to value 0"]
impl crate::Resettable for Fpub1Spec {
    const RESET_VALUE: u32 = 0;
}

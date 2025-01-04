#[doc = "Register `SYSPLLPARAM1` reader"]
pub type R = crate::R<Syspllparam1Spec>;
#[doc = "Register `SYSPLLPARAM1` writer"]
pub type W = crate::W<Syspllparam1Spec>;
#[doc = "Field `SYSPLLPARAM1_LPFCAPA` reader - Loop filter Cap A"]
pub type Syspllparam1LpfcapaR = crate::FieldReader;
#[doc = "Field `SYSPLLPARAM1_LPFCAPA` writer - Loop filter Cap A"]
pub type Syspllparam1LpfcapaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYSPLLPARAM1_LPFRESA` reader - Loop filter Res A"]
pub type Syspllparam1LpfresaR = crate::FieldReader<u16>;
#[doc = "Field `SYSPLLPARAM1_LPFRESA` writer - Loop filter Res A"]
pub type Syspllparam1LpfresaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SYSPLLPARAM1_LPFRESC` reader - Loop filter Res C"]
pub type Syspllparam1LpfrescR = crate::FieldReader;
#[doc = "Field `SYSPLLPARAM1_LPFRESC` writer - Loop filter Res C"]
pub type Syspllparam1LpfrescW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - Loop filter Cap A"]
    #[inline(always)]
    pub fn syspllparam1_lpfcapa(&self) -> Syspllparam1LpfcapaR {
        Syspllparam1LpfcapaR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:17 - Loop filter Res A"]
    #[inline(always)]
    pub fn syspllparam1_lpfresa(&self) -> Syspllparam1LpfresaR {
        Syspllparam1LpfresaR::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - Loop filter Res C"]
    #[inline(always)]
    pub fn syspllparam1_lpfresc(&self) -> Syspllparam1LpfrescR {
        Syspllparam1LpfrescR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Loop filter Cap A"]
    #[inline(always)]
    pub fn syspllparam1_lpfcapa(&mut self) -> Syspllparam1LpfcapaW<Syspllparam1Spec> {
        Syspllparam1LpfcapaW::new(self, 0)
    }
    #[doc = "Bits 8:17 - Loop filter Res A"]
    #[inline(always)]
    pub fn syspllparam1_lpfresa(&mut self) -> Syspllparam1LpfresaW<Syspllparam1Spec> {
        Syspllparam1LpfresaW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Loop filter Res C"]
    #[inline(always)]
    pub fn syspllparam1_lpfresc(&mut self) -> Syspllparam1LpfrescW<Syspllparam1Spec> {
        Syspllparam1LpfrescW::new(self, 24)
    }
}
#[doc = "SYSPLL PARAM1 (load from FACTORY region)\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllparam1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllparam1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllparam1Spec;
impl crate::RegisterSpec for Syspllparam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllparam1::R`](R) reader structure"]
impl crate::Readable for Syspllparam1Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllparam1::W`](W) writer structure"]
impl crate::Writable for Syspllparam1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLLPARAM1 to value 0"]
impl crate::Resettable for Syspllparam1Spec {
    const RESET_VALUE: u32 = 0;
}

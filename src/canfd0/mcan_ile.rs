#[doc = "Register `MCAN_ILE` reader"]
pub type R = crate::R<McanIleSpec>;
#[doc = "Register `MCAN_ILE` writer"]
pub type W = crate::W<McanIleSpec>;
#[doc = "Field `MCAN_ILE_EINT0` reader - Enable Interrupt Line 0 0 Interrupt Line 0 is disabled 1 Interrupt Line 0 is enabled"]
pub type McanIleEint0R = crate::BitReader;
#[doc = "Field `MCAN_ILE_EINT0` writer - Enable Interrupt Line 0 0 Interrupt Line 0 is disabled 1 Interrupt Line 0 is enabled"]
pub type McanIleEint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_ILE_EINT1` reader - Enable Interrupt Line 1 0 Interrupt Line 1 is disabled 1 Interrupt Line 1 is enabled"]
pub type McanIleEint1R = crate::BitReader;
#[doc = "Field `MCAN_ILE_EINT1` writer - Enable Interrupt Line 1 0 Interrupt Line 1 is disabled 1 Interrupt Line 1 is enabled"]
pub type McanIleEint1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt Line 0 0 Interrupt Line 0 is disabled 1 Interrupt Line 0 is enabled"]
    #[inline(always)]
    pub fn mcan_ile_eint0(&self) -> McanIleEint0R {
        McanIleEint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1 0 Interrupt Line 1 is disabled 1 Interrupt Line 1 is enabled"]
    #[inline(always)]
    pub fn mcan_ile_eint1(&self) -> McanIleEint1R {
        McanIleEint1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0 0 Interrupt Line 0 is disabled 1 Interrupt Line 0 is enabled"]
    #[inline(always)]
    pub fn mcan_ile_eint0(&mut self) -> McanIleEint0W<McanIleSpec> {
        McanIleEint0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1 0 Interrupt Line 1 is disabled 1 Interrupt Line 1 is enabled"]
    #[inline(always)]
    pub fn mcan_ile_eint1(&mut self) -> McanIleEint1W<McanIleSpec> {
        McanIleEint1W::new(self, 1)
    }
}
#[doc = "MCAN Interrupt Line Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanIleSpec;
impl crate::RegisterSpec for McanIleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_ile::R`](R) reader structure"]
impl crate::Readable for McanIleSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_ile::W`](W) writer structure"]
impl crate::Writable for McanIleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_ILE to value 0"]
impl crate::Resettable for McanIleSpec {
    const RESET_VALUE: u32 = 0;
}

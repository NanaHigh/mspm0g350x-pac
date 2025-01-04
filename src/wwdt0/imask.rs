#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskInttim {
    #[doc = "0: CLR"]
    ImaskInttimClr = 0,
    #[doc = "1: SET"]
    ImaskInttimSet = 1,
}
impl From<ImaskInttim> for bool {
    #[inline(always)]
    fn from(variant: ImaskInttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_INTTIM` reader - Interval Timer Interrupt."]
pub type ImaskInttimR = crate::BitReader<ImaskInttim>;
impl ImaskInttimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskInttim {
        match self.bits {
            false => ImaskInttim::ImaskInttimClr,
            true => ImaskInttim::ImaskInttimSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_inttim_clr(&self) -> bool {
        *self == ImaskInttim::ImaskInttimClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_inttim_set(&self) -> bool {
        *self == ImaskInttim::ImaskInttimSet
    }
}
#[doc = "Field `IMASK_INTTIM` writer - Interval Timer Interrupt."]
pub type ImaskInttimW<'a, REG> = crate::BitWriter<'a, REG, ImaskInttim>;
impl<'a, REG> ImaskInttimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_inttim_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskInttim::ImaskInttimClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_inttim_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskInttim::ImaskInttimSet)
    }
}
impl R {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn imask_inttim(&self) -> ImaskInttimR {
        ImaskInttimR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn imask_inttim(&mut self) -> ImaskInttimW<ImaskSpec> {
        ImaskInttimW::new(self, 0)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}

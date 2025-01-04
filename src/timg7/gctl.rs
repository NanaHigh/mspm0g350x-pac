#[doc = "Register `GCTL` reader"]
pub type R = crate::R<GctlSpec>;
#[doc = "Register `GCTL` writer"]
pub type W = crate::W<GctlSpec>;
#[doc = "Enables shadow to active load of bufferred registers and register fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GctlShdwlden {
    #[doc = "0: DISABLE"]
    GctlShdwldenDisable = 0,
    #[doc = "1: ENABLE"]
    GctlShdwldenEnable = 1,
}
impl From<GctlShdwlden> for bool {
    #[inline(always)]
    fn from(variant: GctlShdwlden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCTL_SHDWLDEN` reader - Enables shadow to active load of bufferred registers and register fields."]
pub type GctlShdwldenR = crate::BitReader<GctlShdwlden>;
impl GctlShdwldenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GctlShdwlden {
        match self.bits {
            false => GctlShdwlden::GctlShdwldenDisable,
            true => GctlShdwlden::GctlShdwldenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_gctl_shdwlden_disable(&self) -> bool {
        *self == GctlShdwlden::GctlShdwldenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_gctl_shdwlden_enable(&self) -> bool {
        *self == GctlShdwlden::GctlShdwldenEnable
    }
}
#[doc = "Field `GCTL_SHDWLDEN` writer - Enables shadow to active load of bufferred registers and register fields."]
pub type GctlShdwldenW<'a, REG> = crate::BitWriter<'a, REG, GctlShdwlden>;
impl<'a, REG> GctlShdwldenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn gctl_shdwlden_disable(self) -> &'a mut crate::W<REG> {
        self.variant(GctlShdwlden::GctlShdwldenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn gctl_shdwlden_enable(self) -> &'a mut crate::W<REG> {
        self.variant(GctlShdwlden::GctlShdwldenEnable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables shadow to active load of bufferred registers and register fields."]
    #[inline(always)]
    pub fn gctl_shdwlden(&self) -> GctlShdwldenR {
        GctlShdwldenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadow to active load of bufferred registers and register fields."]
    #[inline(always)]
    pub fn gctl_shdwlden(&mut self) -> GctlShdwldenW<GctlSpec> {
        GctlShdwldenW::new(self, 0)
    }
}
#[doc = "Shadow to active load mask\n\nYou can [`read`](crate::Reg::read) this register and get [`gctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GctlSpec;
impl crate::RegisterSpec for GctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gctl::R`](R) reader structure"]
impl crate::Readable for GctlSpec {}
#[doc = "`write(|w| ..)` method takes [`gctl::W`](W) writer structure"]
impl crate::Writable for GctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCTL to value 0"]
impl crate::Resettable for GctlSpec {
    const RESET_VALUE: u32 = 0;
}

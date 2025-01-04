#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Masks COMPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCompifg {
    #[doc = "0: CLR"]
    ImaskCompifgClr = 0,
    #[doc = "1: SET"]
    ImaskCompifgSet = 1,
}
impl From<ImaskCompifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskCompifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_COMPIFG` reader - Masks COMPIFG"]
pub type ImaskCompifgR = crate::BitReader<ImaskCompifg>;
impl ImaskCompifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCompifg {
        match self.bits {
            false => ImaskCompifg::ImaskCompifgClr,
            true => ImaskCompifg::ImaskCompifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_compifg_clr(&self) -> bool {
        *self == ImaskCompifg::ImaskCompifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_compifg_set(&self) -> bool {
        *self == ImaskCompifg::ImaskCompifgSet
    }
}
#[doc = "Field `IMASK_COMPIFG` writer - Masks COMPIFG"]
pub type ImaskCompifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskCompifg>;
impl<'a, REG> ImaskCompifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_compifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCompifg::ImaskCompifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_compifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCompifg::ImaskCompifgSet)
    }
}
#[doc = "Masks COMPINVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskCompinvifg {
    #[doc = "0: CLR"]
    ImaskCompinvifgClr = 0,
    #[doc = "1: SET"]
    ImaskCompinvifgSet = 1,
}
impl From<ImaskCompinvifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskCompinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_COMPINVIFG` reader - Masks COMPINVIFG"]
pub type ImaskCompinvifgR = crate::BitReader<ImaskCompinvifg>;
impl ImaskCompinvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskCompinvifg {
        match self.bits {
            false => ImaskCompinvifg::ImaskCompinvifgClr,
            true => ImaskCompinvifg::ImaskCompinvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_compinvifg_clr(&self) -> bool {
        *self == ImaskCompinvifg::ImaskCompinvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_compinvifg_set(&self) -> bool {
        *self == ImaskCompinvifg::ImaskCompinvifgSet
    }
}
#[doc = "Field `IMASK_COMPINVIFG` writer - Masks COMPINVIFG"]
pub type ImaskCompinvifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskCompinvifg>;
impl<'a, REG> ImaskCompinvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_compinvifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCompinvifg::ImaskCompinvifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_compinvifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskCompinvifg::ImaskCompinvifgSet)
    }
}
#[doc = "Masks OUTRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskOutrdyifg {
    #[doc = "0: CLR"]
    ImaskOutrdyifgClr = 0,
    #[doc = "1: SET"]
    ImaskOutrdyifgSet = 1,
}
impl From<ImaskOutrdyifg> for bool {
    #[inline(always)]
    fn from(variant: ImaskOutrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_OUTRDYIFG` reader - Masks OUTRDYIFG"]
pub type ImaskOutrdyifgR = crate::BitReader<ImaskOutrdyifg>;
impl ImaskOutrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskOutrdyifg {
        match self.bits {
            false => ImaskOutrdyifg::ImaskOutrdyifgClr,
            true => ImaskOutrdyifg::ImaskOutrdyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_outrdyifg_clr(&self) -> bool {
        *self == ImaskOutrdyifg::ImaskOutrdyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_outrdyifg_set(&self) -> bool {
        *self == ImaskOutrdyifg::ImaskOutrdyifgSet
    }
}
#[doc = "Field `IMASK_OUTRDYIFG` writer - Masks OUTRDYIFG"]
pub type ImaskOutrdyifgW<'a, REG> = crate::BitWriter<'a, REG, ImaskOutrdyifg>;
impl<'a, REG> ImaskOutrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_outrdyifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskOutrdyifg::ImaskOutrdyifgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_outrdyifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskOutrdyifg::ImaskOutrdyifgSet)
    }
}
impl R {
    #[doc = "Bit 1 - Masks COMPIFG"]
    #[inline(always)]
    pub fn imask_compifg(&self) -> ImaskCompifgR {
        ImaskCompifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks COMPINVIFG"]
    #[inline(always)]
    pub fn imask_compinvifg(&self) -> ImaskCompinvifgR {
        ImaskCompinvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks OUTRDYIFG"]
    #[inline(always)]
    pub fn imask_outrdyifg(&self) -> ImaskOutrdyifgR {
        ImaskOutrdyifgR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Masks COMPIFG"]
    #[inline(always)]
    pub fn imask_compifg(&mut self) -> ImaskCompifgW<ImaskSpec> {
        ImaskCompifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Masks COMPINVIFG"]
    #[inline(always)]
    pub fn imask_compinvifg(&mut self) -> ImaskCompinvifgW<ImaskSpec> {
        ImaskCompinvifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Masks OUTRDYIFG"]
    #[inline(always)]
    pub fn imask_outrdyifg(&mut self) -> ImaskOutrdyifgW<ImaskSpec> {
        ImaskOutrdyifgW::new(self, 3)
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

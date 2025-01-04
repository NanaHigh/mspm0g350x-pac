#[doc = "Register `SYSPLLCFG1` reader"]
pub type R = crate::R<Syspllcfg1Spec>;
#[doc = "Register `SYSPLLCFG1` writer"]
pub type W = crate::W<Syspllcfg1Spec>;
#[doc = "PDIV selects the SYSPLL reference clock prescale divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syspllcfg1Pdiv {
    #[doc = "0: REFDIV1"]
    Syspllcfg1PdivRefdiv1 = 0,
    #[doc = "1: REFDIV2"]
    Syspllcfg1PdivRefdiv2 = 1,
    #[doc = "2: REFDIV4"]
    Syspllcfg1PdivRefdiv4 = 2,
    #[doc = "3: REFDIV8"]
    Syspllcfg1PdivRefdiv8 = 3,
}
impl From<Syspllcfg1Pdiv> for u8 {
    #[inline(always)]
    fn from(variant: Syspllcfg1Pdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syspllcfg1Pdiv {
    type Ux = u8;
}
impl crate::IsEnum for Syspllcfg1Pdiv {}
#[doc = "Field `SYSPLLCFG1_PDIV` reader - PDIV selects the SYSPLL reference clock prescale divider."]
pub type Syspllcfg1PdivR = crate::FieldReader<Syspllcfg1Pdiv>;
impl Syspllcfg1PdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg1Pdiv {
        match self.bits {
            0 => Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv1,
            1 => Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv2,
            2 => Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv4,
            3 => Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv8,
            _ => unreachable!(),
        }
    }
    #[doc = "REFDIV1"]
    #[inline(always)]
    pub fn is_syspllcfg1_pdiv_refdiv1(&self) -> bool {
        *self == Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv1
    }
    #[doc = "REFDIV2"]
    #[inline(always)]
    pub fn is_syspllcfg1_pdiv_refdiv2(&self) -> bool {
        *self == Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv2
    }
    #[doc = "REFDIV4"]
    #[inline(always)]
    pub fn is_syspllcfg1_pdiv_refdiv4(&self) -> bool {
        *self == Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv4
    }
    #[doc = "REFDIV8"]
    #[inline(always)]
    pub fn is_syspllcfg1_pdiv_refdiv8(&self) -> bool {
        *self == Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv8
    }
}
#[doc = "Field `SYSPLLCFG1_PDIV` writer - PDIV selects the SYSPLL reference clock prescale divider."]
pub type Syspllcfg1PdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Syspllcfg1Pdiv, crate::Safe>;
impl<'a, REG> Syspllcfg1PdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "REFDIV1"]
    #[inline(always)]
    pub fn syspllcfg1_pdiv_refdiv1(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv1)
    }
    #[doc = "REFDIV2"]
    #[inline(always)]
    pub fn syspllcfg1_pdiv_refdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv2)
    }
    #[doc = "REFDIV4"]
    #[inline(always)]
    pub fn syspllcfg1_pdiv_refdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv4)
    }
    #[doc = "REFDIV8"]
    #[inline(always)]
    pub fn syspllcfg1_pdiv_refdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Pdiv::Syspllcfg1PdivRefdiv8)
    }
}
#[doc = "QDIV selects the SYSPLL feedback path divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syspllcfg1Qdiv {
    #[doc = "0: INVALID"]
    Syspllcfg1QdivInvalid = 0,
    #[doc = "1: QDIVMIN"]
    Syspllcfg1QdivQdivmin = 1,
    #[doc = "126: QDIVMAX"]
    Syspllcfg1QdivQdivmax = 126,
}
impl From<Syspllcfg1Qdiv> for u8 {
    #[inline(always)]
    fn from(variant: Syspllcfg1Qdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syspllcfg1Qdiv {
    type Ux = u8;
}
impl crate::IsEnum for Syspllcfg1Qdiv {}
#[doc = "Field `SYSPLLCFG1_QDIV` reader - QDIV selects the SYSPLL feedback path divider."]
pub type Syspllcfg1QdivR = crate::FieldReader<Syspllcfg1Qdiv>;
impl Syspllcfg1QdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syspllcfg1Qdiv> {
        match self.bits {
            0 => Some(Syspllcfg1Qdiv::Syspllcfg1QdivInvalid),
            1 => Some(Syspllcfg1Qdiv::Syspllcfg1QdivQdivmin),
            126 => Some(Syspllcfg1Qdiv::Syspllcfg1QdivQdivmax),
            _ => None,
        }
    }
    #[doc = "INVALID"]
    #[inline(always)]
    pub fn is_syspllcfg1_qdiv_invalid(&self) -> bool {
        *self == Syspllcfg1Qdiv::Syspllcfg1QdivInvalid
    }
    #[doc = "QDIVMIN"]
    #[inline(always)]
    pub fn is_syspllcfg1_qdiv_qdivmin(&self) -> bool {
        *self == Syspllcfg1Qdiv::Syspllcfg1QdivQdivmin
    }
    #[doc = "QDIVMAX"]
    #[inline(always)]
    pub fn is_syspllcfg1_qdiv_qdivmax(&self) -> bool {
        *self == Syspllcfg1Qdiv::Syspllcfg1QdivQdivmax
    }
}
#[doc = "Field `SYSPLLCFG1_QDIV` writer - QDIV selects the SYSPLL feedback path divider."]
pub type Syspllcfg1QdivW<'a, REG> = crate::FieldWriter<'a, REG, 7, Syspllcfg1Qdiv>;
impl<'a, REG> Syspllcfg1QdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "INVALID"]
    #[inline(always)]
    pub fn syspllcfg1_qdiv_invalid(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Qdiv::Syspllcfg1QdivInvalid)
    }
    #[doc = "QDIVMIN"]
    #[inline(always)]
    pub fn syspllcfg1_qdiv_qdivmin(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Qdiv::Syspllcfg1QdivQdivmin)
    }
    #[doc = "QDIVMAX"]
    #[inline(always)]
    pub fn syspllcfg1_qdiv_qdivmax(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg1Qdiv::Syspllcfg1QdivQdivmax)
    }
}
impl R {
    #[doc = "Bits 0:1 - PDIV selects the SYSPLL reference clock prescale divider."]
    #[inline(always)]
    pub fn syspllcfg1_pdiv(&self) -> Syspllcfg1PdivR {
        Syspllcfg1PdivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:14 - QDIV selects the SYSPLL feedback path divider."]
    #[inline(always)]
    pub fn syspllcfg1_qdiv(&self) -> Syspllcfg1QdivR {
        Syspllcfg1QdivR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDIV selects the SYSPLL reference clock prescale divider."]
    #[inline(always)]
    pub fn syspllcfg1_pdiv(&mut self) -> Syspllcfg1PdivW<Syspllcfg1Spec> {
        Syspllcfg1PdivW::new(self, 0)
    }
    #[doc = "Bits 8:14 - QDIV selects the SYSPLL feedback path divider."]
    #[inline(always)]
    pub fn syspllcfg1_qdiv(&mut self) -> Syspllcfg1QdivW<Syspllcfg1Spec> {
        Syspllcfg1QdivW::new(self, 8)
    }
}
#[doc = "SYSPLL reference and feedback divider\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllcfg1Spec;
impl crate::RegisterSpec for Syspllcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllcfg1::R`](R) reader structure"]
impl crate::Readable for Syspllcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllcfg1::W`](W) writer structure"]
impl crate::Writable for Syspllcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLLCFG1 to value 0"]
impl crate::Resettable for Syspllcfg1Spec {
    const RESET_VALUE: u32 = 0;
}

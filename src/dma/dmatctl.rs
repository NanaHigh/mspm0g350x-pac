#[doc = "Register `DMATCTL` reader"]
pub type R = crate::R<DmatctlSpec>;
#[doc = "Register `DMATCTL` writer"]
pub type W = crate::W<DmatctlSpec>;
#[doc = "DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmatctlDmatsel {
    #[doc = "0: DMAREQ"]
    DmatctlDmatselDmareq = 0,
}
impl From<DmatctlDmatsel> for u8 {
    #[inline(always)]
    fn from(variant: DmatctlDmatsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmatctlDmatsel {
    type Ux = u8;
}
impl crate::IsEnum for DmatctlDmatsel {}
#[doc = "Field `DMATCTL_DMATSEL` reader - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
pub type DmatctlDmatselR = crate::FieldReader<DmatctlDmatsel>;
impl DmatctlDmatselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmatctlDmatsel> {
        match self.bits {
            0 => Some(DmatctlDmatsel::DmatctlDmatselDmareq),
            _ => None,
        }
    }
    #[doc = "DMAREQ"]
    #[inline(always)]
    pub fn is_dmatctl_dmatsel_dmareq(&self) -> bool {
        *self == DmatctlDmatsel::DmatctlDmatselDmareq
    }
}
#[doc = "Field `DMATCTL_DMATSEL` writer - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
pub type DmatctlDmatselW<'a, REG> = crate::FieldWriter<'a, REG, 6, DmatctlDmatsel>;
impl<'a, REG> DmatctlDmatselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMAREQ"]
    #[inline(always)]
    pub fn dmatctl_dmatsel_dmareq(self) -> &'a mut crate::W<REG> {
        self.variant(DmatctlDmatsel::DmatctlDmatselDmareq)
    }
}
#[doc = "DMA Trigger by Internal Channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmatctlDmatint {
    #[doc = "0: EXTERNAL"]
    DmatctlDmatintExternal = 0,
    #[doc = "1: INTERNAL"]
    DmatctlDmatintInternal = 1,
}
impl From<DmatctlDmatint> for bool {
    #[inline(always)]
    fn from(variant: DmatctlDmatint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMATCTL_DMATINT` reader - DMA Trigger by Internal Channel"]
pub type DmatctlDmatintR = crate::BitReader<DmatctlDmatint>;
impl DmatctlDmatintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmatctlDmatint {
        match self.bits {
            false => DmatctlDmatint::DmatctlDmatintExternal,
            true => DmatctlDmatint::DmatctlDmatintInternal,
        }
    }
    #[doc = "EXTERNAL"]
    #[inline(always)]
    pub fn is_dmatctl_dmatint_external(&self) -> bool {
        *self == DmatctlDmatint::DmatctlDmatintExternal
    }
    #[doc = "INTERNAL"]
    #[inline(always)]
    pub fn is_dmatctl_dmatint_internal(&self) -> bool {
        *self == DmatctlDmatint::DmatctlDmatintInternal
    }
}
#[doc = "Field `DMATCTL_DMATINT` writer - DMA Trigger by Internal Channel"]
pub type DmatctlDmatintW<'a, REG> = crate::BitWriter<'a, REG, DmatctlDmatint>;
impl<'a, REG> DmatctlDmatintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "EXTERNAL"]
    #[inline(always)]
    pub fn dmatctl_dmatint_external(self) -> &'a mut crate::W<REG> {
        self.variant(DmatctlDmatint::DmatctlDmatintExternal)
    }
    #[doc = "INTERNAL"]
    #[inline(always)]
    pub fn dmatctl_dmatint_internal(self) -> &'a mut crate::W<REG> {
        self.variant(DmatctlDmatint::DmatctlDmatintInternal)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
    #[inline(always)]
    pub fn dmatctl_dmatsel(&self) -> DmatctlDmatselR {
        DmatctlDmatselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - DMA Trigger by Internal Channel"]
    #[inline(always)]
    pub fn dmatctl_dmatint(&self) -> DmatctlDmatintR {
        DmatctlDmatintR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Trigger Select Note: Reference the datasheet of the device to see the specific trigger mapping."]
    #[inline(always)]
    pub fn dmatctl_dmatsel(&mut self) -> DmatctlDmatselW<DmatctlSpec> {
        DmatctlDmatselW::new(self, 0)
    }
    #[doc = "Bit 7 - DMA Trigger by Internal Channel"]
    #[inline(always)]
    pub fn dmatctl_dmatint(&mut self) -> DmatctlDmatintW<DmatctlSpec> {
        DmatctlDmatintW::new(self, 7)
    }
}
#[doc = "DMA Trigger Select\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatctlSpec;
impl crate::RegisterSpec for DmatctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatctl::R`](R) reader structure"]
impl crate::Readable for DmatctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatctl::W`](W) writer structure"]
impl crate::Writable for DmatctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATCTL to value 0"]
impl crate::Resettable for DmatctlSpec {
    const RESET_VALUE: u32 = 0;
}

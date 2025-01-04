#[doc = "Register `GENCLKEN` reader"]
pub type R = crate::R<GenclkenSpec>;
#[doc = "Register `GENCLKEN` writer"]
pub type W = crate::W<GenclkenSpec>;
#[doc = "EXCLKEN enables the CLK_OUT external clock output block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenclkenExclken {
    #[doc = "0: DISABLE"]
    GenclkenExclkenDisable = 0,
    #[doc = "1: ENABLE"]
    GenclkenExclkenEnable = 1,
}
impl From<GenclkenExclken> for bool {
    #[inline(always)]
    fn from(variant: GenclkenExclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCLKEN_EXCLKEN` reader - EXCLKEN enables the CLK_OUT external clock output block."]
pub type GenclkenExclkenR = crate::BitReader<GenclkenExclken>;
impl GenclkenExclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GenclkenExclken {
        match self.bits {
            false => GenclkenExclken::GenclkenExclkenDisable,
            true => GenclkenExclken::GenclkenExclkenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_genclken_exclken_disable(&self) -> bool {
        *self == GenclkenExclken::GenclkenExclkenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_genclken_exclken_enable(&self) -> bool {
        *self == GenclkenExclken::GenclkenExclkenEnable
    }
}
#[doc = "Field `GENCLKEN_EXCLKEN` writer - EXCLKEN enables the CLK_OUT external clock output block."]
pub type GenclkenExclkenW<'a, REG> = crate::BitWriter<'a, REG, GenclkenExclken>;
impl<'a, REG> GenclkenExclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn genclken_exclken_disable(self) -> &'a mut crate::W<REG> {
        self.variant(GenclkenExclken::GenclkenExclkenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn genclken_exclken_enable(self) -> &'a mut crate::W<REG> {
        self.variant(GenclkenExclken::GenclkenExclkenEnable)
    }
}
#[doc = "MFPCLKEN enables the middle frequency precision clock (MFPCLK).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenclkenMfpclken {
    #[doc = "0: DISABLE"]
    GenclkenMfpclkenDisable = 0,
    #[doc = "1: ENABLE"]
    GenclkenMfpclkenEnable = 1,
}
impl From<GenclkenMfpclken> for bool {
    #[inline(always)]
    fn from(variant: GenclkenMfpclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GENCLKEN_MFPCLKEN` reader - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
pub type GenclkenMfpclkenR = crate::BitReader<GenclkenMfpclken>;
impl GenclkenMfpclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GenclkenMfpclken {
        match self.bits {
            false => GenclkenMfpclken::GenclkenMfpclkenDisable,
            true => GenclkenMfpclken::GenclkenMfpclkenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_genclken_mfpclken_disable(&self) -> bool {
        *self == GenclkenMfpclken::GenclkenMfpclkenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_genclken_mfpclken_enable(&self) -> bool {
        *self == GenclkenMfpclken::GenclkenMfpclkenEnable
    }
}
#[doc = "Field `GENCLKEN_MFPCLKEN` writer - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
pub type GenclkenMfpclkenW<'a, REG> = crate::BitWriter<'a, REG, GenclkenMfpclken>;
impl<'a, REG> GenclkenMfpclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn genclken_mfpclken_disable(self) -> &'a mut crate::W<REG> {
        self.variant(GenclkenMfpclken::GenclkenMfpclkenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn genclken_mfpclken_enable(self) -> &'a mut crate::W<REG> {
        self.variant(GenclkenMfpclken::GenclkenMfpclkenEnable)
    }
}
impl R {
    #[doc = "Bit 0 - EXCLKEN enables the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn genclken_exclken(&self) -> GenclkenExclkenR {
        GenclkenExclkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
    #[inline(always)]
    pub fn genclken_mfpclken(&self) -> GenclkenMfpclkenR {
        GenclkenMfpclkenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EXCLKEN enables the CLK_OUT external clock output block."]
    #[inline(always)]
    pub fn genclken_exclken(&mut self) -> GenclkenExclkenW<GenclkenSpec> {
        GenclkenExclkenW::new(self, 0)
    }
    #[doc = "Bit 4 - MFPCLKEN enables the middle frequency precision clock (MFPCLK)."]
    #[inline(always)]
    pub fn genclken_mfpclken(&mut self) -> GenclkenMfpclkenW<GenclkenSpec> {
        GenclkenMfpclkenW::new(self, 4)
    }
}
#[doc = "General clock enable control\n\nYou can [`read`](crate::Reg::read) this register and get [`genclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`genclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GenclkenSpec;
impl crate::RegisterSpec for GenclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`genclken::R`](R) reader structure"]
impl crate::Readable for GenclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`genclken::W`](W) writer structure"]
impl crate::Writable for GenclkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GENCLKEN to value 0"]
impl crate::Resettable for GenclkenSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CCPD` reader"]
pub type R = crate::R<CcpdSpec>;
#[doc = "Register `CCPD` writer"]
pub type W = crate::W<CcpdSpec>;
#[doc = "Counter CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcpdC0ccp0 {
    #[doc = "0: INPUT"]
    CcpdC0ccp0Input = 0,
    #[doc = "1: OUTPUT"]
    CcpdC0ccp0Output = 1,
}
impl From<CcpdC0ccp0> for bool {
    #[inline(always)]
    fn from(variant: CcpdC0ccp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPD_C0CCP0` reader - Counter CCP0"]
pub type CcpdC0ccp0R = crate::BitReader<CcpdC0ccp0>;
impl CcpdC0ccp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcpdC0ccp0 {
        match self.bits {
            false => CcpdC0ccp0::CcpdC0ccp0Input,
            true => CcpdC0ccp0::CcpdC0ccp0Output,
        }
    }
    #[doc = "INPUT"]
    #[inline(always)]
    pub fn is_ccpd_c0ccp0_input(&self) -> bool {
        *self == CcpdC0ccp0::CcpdC0ccp0Input
    }
    #[doc = "OUTPUT"]
    #[inline(always)]
    pub fn is_ccpd_c0ccp0_output(&self) -> bool {
        *self == CcpdC0ccp0::CcpdC0ccp0Output
    }
}
#[doc = "Field `CCPD_C0CCP0` writer - Counter CCP0"]
pub type CcpdC0ccp0W<'a, REG> = crate::BitWriter<'a, REG, CcpdC0ccp0>;
impl<'a, REG> CcpdC0ccp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INPUT"]
    #[inline(always)]
    pub fn ccpd_c0ccp0_input(self) -> &'a mut crate::W<REG> {
        self.variant(CcpdC0ccp0::CcpdC0ccp0Input)
    }
    #[doc = "OUTPUT"]
    #[inline(always)]
    pub fn ccpd_c0ccp0_output(self) -> &'a mut crate::W<REG> {
        self.variant(CcpdC0ccp0::CcpdC0ccp0Output)
    }
}
#[doc = "Counter CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcpdC0ccp1 {
    #[doc = "0: INPUT"]
    CcpdC0ccp1Input = 0,
    #[doc = "1: OUTPUT"]
    CcpdC0ccp1Output = 1,
}
impl From<CcpdC0ccp1> for bool {
    #[inline(always)]
    fn from(variant: CcpdC0ccp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPD_C0CCP1` reader - Counter CCP1"]
pub type CcpdC0ccp1R = crate::BitReader<CcpdC0ccp1>;
impl CcpdC0ccp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcpdC0ccp1 {
        match self.bits {
            false => CcpdC0ccp1::CcpdC0ccp1Input,
            true => CcpdC0ccp1::CcpdC0ccp1Output,
        }
    }
    #[doc = "INPUT"]
    #[inline(always)]
    pub fn is_ccpd_c0ccp1_input(&self) -> bool {
        *self == CcpdC0ccp1::CcpdC0ccp1Input
    }
    #[doc = "OUTPUT"]
    #[inline(always)]
    pub fn is_ccpd_c0ccp1_output(&self) -> bool {
        *self == CcpdC0ccp1::CcpdC0ccp1Output
    }
}
#[doc = "Field `CCPD_C0CCP1` writer - Counter CCP1"]
pub type CcpdC0ccp1W<'a, REG> = crate::BitWriter<'a, REG, CcpdC0ccp1>;
impl<'a, REG> CcpdC0ccp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INPUT"]
    #[inline(always)]
    pub fn ccpd_c0ccp1_input(self) -> &'a mut crate::W<REG> {
        self.variant(CcpdC0ccp1::CcpdC0ccp1Input)
    }
    #[doc = "OUTPUT"]
    #[inline(always)]
    pub fn ccpd_c0ccp1_output(self) -> &'a mut crate::W<REG> {
        self.variant(CcpdC0ccp1::CcpdC0ccp1Output)
    }
}
impl R {
    #[doc = "Bit 0 - Counter CCP0"]
    #[inline(always)]
    pub fn ccpd_c0ccp0(&self) -> CcpdC0ccp0R {
        CcpdC0ccp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter CCP1"]
    #[inline(always)]
    pub fn ccpd_c0ccp1(&self) -> CcpdC0ccp1R {
        CcpdC0ccp1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter CCP0"]
    #[inline(always)]
    pub fn ccpd_c0ccp0(&mut self) -> CcpdC0ccp0W<CcpdSpec> {
        CcpdC0ccp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Counter CCP1"]
    #[inline(always)]
    pub fn ccpd_c0ccp1(&mut self) -> CcpdC0ccp1W<CcpdSpec> {
        CcpdC0ccp1W::new(self, 1)
    }
}
#[doc = "CCP Direction\n\nYou can [`read`](crate::Reg::read) this register and get [`ccpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcpdSpec;
impl crate::RegisterSpec for CcpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccpd::R`](R) reader structure"]
impl crate::Readable for CcpdSpec {}
#[doc = "`write(|w| ..)` method takes [`ccpd::W`](W) writer structure"]
impl crate::Writable for CcpdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCPD to value 0"]
impl crate::Resettable for CcpdSpec {
    const RESET_VALUE: u32 = 0;
}

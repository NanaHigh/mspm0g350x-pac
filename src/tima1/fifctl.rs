#[doc = "Register `FIFCTL` reader"]
pub type R = crate::R<FifctlSpec>;
#[doc = "Register `FIFCTL` writer"]
pub type W = crate::W<FifctlSpec>;
#[doc = "Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FifctlFp {
    #[doc = "0: PER_3"]
    FifctlFpPer3 = 0,
    #[doc = "1: PER_5"]
    FifctlFpPer5 = 1,
    #[doc = "2: PER_8"]
    FifctlFpPer8 = 2,
}
impl From<FifctlFp> for u8 {
    #[inline(always)]
    fn from(variant: FifctlFp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FifctlFp {
    type Ux = u8;
}
impl crate::IsEnum for FifctlFp {}
#[doc = "Field `FIFCTL_FP` reader - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type FifctlFpR = crate::FieldReader<FifctlFp>;
impl FifctlFpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FifctlFp> {
        match self.bits {
            0 => Some(FifctlFp::FifctlFpPer3),
            1 => Some(FifctlFp::FifctlFpPer5),
            2 => Some(FifctlFp::FifctlFpPer8),
            _ => None,
        }
    }
    #[doc = "PER_3"]
    #[inline(always)]
    pub fn is_fifctl_fp_per_3(&self) -> bool {
        *self == FifctlFp::FifctlFpPer3
    }
    #[doc = "PER_5"]
    #[inline(always)]
    pub fn is_fifctl_fp_per_5(&self) -> bool {
        *self == FifctlFp::FifctlFpPer5
    }
    #[doc = "PER_8"]
    #[inline(always)]
    pub fn is_fifctl_fp_per_8(&self) -> bool {
        *self == FifctlFp::FifctlFpPer8
    }
}
#[doc = "Field `FIFCTL_FP` writer - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
pub type FifctlFpW<'a, REG> = crate::FieldWriter<'a, REG, 2, FifctlFp>;
impl<'a, REG> FifctlFpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PER_3"]
    #[inline(always)]
    pub fn fifctl_fp_per_3(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlFp::FifctlFpPer3)
    }
    #[doc = "PER_5"]
    #[inline(always)]
    pub fn fifctl_fp_per_5(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlFp::FifctlFpPer5)
    }
    #[doc = "PER_8"]
    #[inline(always)]
    pub fn fifctl_fp_per_8(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlFp::FifctlFpPer8)
    }
}
#[doc = "Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifctlCpv {
    #[doc = "0: CONSEC_PER"]
    FifctlCpvConsecPer = 0,
    #[doc = "1: VOTING"]
    FifctlCpvVoting = 1,
}
impl From<FifctlCpv> for bool {
    #[inline(always)]
    fn from(variant: FifctlCpv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFCTL_CPV` reader - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type FifctlCpvR = crate::BitReader<FifctlCpv>;
impl FifctlCpvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifctlCpv {
        match self.bits {
            false => FifctlCpv::FifctlCpvConsecPer,
            true => FifctlCpv::FifctlCpvVoting,
        }
    }
    #[doc = "CONSEC_PER"]
    #[inline(always)]
    pub fn is_fifctl_cpv_consec_per(&self) -> bool {
        *self == FifctlCpv::FifctlCpvConsecPer
    }
    #[doc = "VOTING"]
    #[inline(always)]
    pub fn is_fifctl_cpv_voting(&self) -> bool {
        *self == FifctlCpv::FifctlCpvVoting
    }
}
#[doc = "Field `FIFCTL_CPV` writer - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
pub type FifctlCpvW<'a, REG> = crate::BitWriter<'a, REG, FifctlCpv>;
impl<'a, REG> FifctlCpvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CONSEC_PER"]
    #[inline(always)]
    pub fn fifctl_cpv_consec_per(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlCpv::FifctlCpvConsecPer)
    }
    #[doc = "VOTING"]
    #[inline(always)]
    pub fn fifctl_cpv_voting(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlCpv::FifctlCpvVoting)
    }
}
#[doc = "Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FifctlFilten {
    #[doc = "0: BYPASS"]
    FifctlFiltenBypass = 0,
    #[doc = "1: FILTERED"]
    FifctlFiltenFiltered = 1,
}
impl From<FifctlFilten> for bool {
    #[inline(always)]
    fn from(variant: FifctlFilten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFCTL_FILTEN` reader - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
pub type FifctlFiltenR = crate::BitReader<FifctlFilten>;
impl FifctlFiltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FifctlFilten {
        match self.bits {
            false => FifctlFilten::FifctlFiltenBypass,
            true => FifctlFilten::FifctlFiltenFiltered,
        }
    }
    #[doc = "BYPASS"]
    #[inline(always)]
    pub fn is_fifctl_filten_bypass(&self) -> bool {
        *self == FifctlFilten::FifctlFiltenBypass
    }
    #[doc = "FILTERED"]
    #[inline(always)]
    pub fn is_fifctl_filten_filtered(&self) -> bool {
        *self == FifctlFilten::FifctlFiltenFiltered
    }
}
#[doc = "Field `FIFCTL_FILTEN` writer - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
pub type FifctlFiltenW<'a, REG> = crate::BitWriter<'a, REG, FifctlFilten>;
impl<'a, REG> FifctlFiltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "BYPASS"]
    #[inline(always)]
    pub fn fifctl_filten_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlFilten::FifctlFiltenBypass)
    }
    #[doc = "FILTERED"]
    #[inline(always)]
    pub fn fifctl_filten_filtered(self) -> &'a mut crate::W<REG> {
        self.variant(FifctlFilten::FifctlFiltenFiltered)
    }
}
impl R {
    #[doc = "Bits 0:1 - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn fifctl_fp(&self) -> FifctlFpR {
        FifctlFpR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn fifctl_cpv(&self) -> FifctlCpvR {
        FifctlCpvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
    #[inline(always)]
    pub fn fifctl_filten(&self) -> FifctlFiltenR {
        FifctlFiltenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Filter Period This field specifies the sample period for the input filter. I.e. The input is sampled for FP timer clocks during filtering."]
    #[inline(always)]
    pub fn fifctl_fp(&mut self) -> FifctlFpW<FifctlSpec> {
        FifctlFpW::new(self, 0)
    }
    #[doc = "Bit 3 - Consecutive Period/Voting Select This bit controls whether the input filter uses a stricter consecutive period count or majority voting."]
    #[inline(always)]
    pub fn fifctl_cpv(&mut self) -> FifctlCpvW<FifctlSpec> {
        FifctlCpvW::new(self, 3)
    }
    #[doc = "Bit 4 - Filter Enable This bit controls whether the input is filtered by the input filter or bypasses to go directly to the optional pre-scale filter and then to the edge detect."]
    #[inline(always)]
    pub fn fifctl_filten(&mut self) -> FifctlFiltenW<FifctlSpec> {
        FifctlFiltenW::new(self, 4)
    }
}
#[doc = "Fault input Filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifctlSpec;
impl crate::RegisterSpec for FifctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifctl::R`](R) reader structure"]
impl crate::Readable for FifctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifctl::W`](W) writer structure"]
impl crate::Writable for FifctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFCTL to value 0"]
impl crate::Resettable for FifctlSpec {
    const RESET_VALUE: u32 = 0;
}

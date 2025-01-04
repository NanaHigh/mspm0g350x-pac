#[doc = "Register `BORTHRESHOLD` reader"]
pub type R = crate::R<BorthresholdSpec>;
#[doc = "Register `BORTHRESHOLD` writer"]
pub type W = crate::W<BorthresholdSpec>;
#[doc = "LEVEL specifies the desired BOR threshold and BOR mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BorthresholdLevel {
    #[doc = "0: BORMIN"]
    BorthresholdLevelBormin = 0,
    #[doc = "1: BORLEVEL1"]
    BorthresholdLevelBorlevel1 = 1,
    #[doc = "2: BORLEVEL2"]
    BorthresholdLevelBorlevel2 = 2,
    #[doc = "3: BORLEVEL3"]
    BorthresholdLevelBorlevel3 = 3,
}
impl From<BorthresholdLevel> for u8 {
    #[inline(always)]
    fn from(variant: BorthresholdLevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BorthresholdLevel {
    type Ux = u8;
}
impl crate::IsEnum for BorthresholdLevel {}
#[doc = "Field `BORTHRESHOLD_LEVEL` reader - LEVEL specifies the desired BOR threshold and BOR mode."]
pub type BorthresholdLevelR = crate::FieldReader<BorthresholdLevel>;
impl BorthresholdLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BorthresholdLevel {
        match self.bits {
            0 => BorthresholdLevel::BorthresholdLevelBormin,
            1 => BorthresholdLevel::BorthresholdLevelBorlevel1,
            2 => BorthresholdLevel::BorthresholdLevelBorlevel2,
            3 => BorthresholdLevel::BorthresholdLevelBorlevel3,
            _ => unreachable!(),
        }
    }
    #[doc = "BORMIN"]
    #[inline(always)]
    pub fn is_borthreshold_level_bormin(&self) -> bool {
        *self == BorthresholdLevel::BorthresholdLevelBormin
    }
    #[doc = "BORLEVEL1"]
    #[inline(always)]
    pub fn is_borthreshold_level_borlevel1(&self) -> bool {
        *self == BorthresholdLevel::BorthresholdLevelBorlevel1
    }
    #[doc = "BORLEVEL2"]
    #[inline(always)]
    pub fn is_borthreshold_level_borlevel2(&self) -> bool {
        *self == BorthresholdLevel::BorthresholdLevelBorlevel2
    }
    #[doc = "BORLEVEL3"]
    #[inline(always)]
    pub fn is_borthreshold_level_borlevel3(&self) -> bool {
        *self == BorthresholdLevel::BorthresholdLevelBorlevel3
    }
}
#[doc = "Field `BORTHRESHOLD_LEVEL` writer - LEVEL specifies the desired BOR threshold and BOR mode."]
pub type BorthresholdLevelW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, BorthresholdLevel, crate::Safe>;
impl<'a, REG> BorthresholdLevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BORMIN"]
    #[inline(always)]
    pub fn borthreshold_level_bormin(self) -> &'a mut crate::W<REG> {
        self.variant(BorthresholdLevel::BorthresholdLevelBormin)
    }
    #[doc = "BORLEVEL1"]
    #[inline(always)]
    pub fn borthreshold_level_borlevel1(self) -> &'a mut crate::W<REG> {
        self.variant(BorthresholdLevel::BorthresholdLevelBorlevel1)
    }
    #[doc = "BORLEVEL2"]
    #[inline(always)]
    pub fn borthreshold_level_borlevel2(self) -> &'a mut crate::W<REG> {
        self.variant(BorthresholdLevel::BorthresholdLevelBorlevel2)
    }
    #[doc = "BORLEVEL3"]
    #[inline(always)]
    pub fn borthreshold_level_borlevel3(self) -> &'a mut crate::W<REG> {
        self.variant(BorthresholdLevel::BorthresholdLevelBorlevel3)
    }
}
impl R {
    #[doc = "Bits 0:1 - LEVEL specifies the desired BOR threshold and BOR mode."]
    #[inline(always)]
    pub fn borthreshold_level(&self) -> BorthresholdLevelR {
        BorthresholdLevelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - LEVEL specifies the desired BOR threshold and BOR mode."]
    #[inline(always)]
    pub fn borthreshold_level(&mut self) -> BorthresholdLevelW<BorthresholdSpec> {
        BorthresholdLevelW::new(self, 0)
    }
}
#[doc = "BOR threshold selection\n\nYou can [`read`](crate::Reg::read) this register and get [`borthreshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`borthreshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorthresholdSpec;
impl crate::RegisterSpec for BorthresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`borthreshold::R`](R) reader structure"]
impl crate::Readable for BorthresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`borthreshold::W`](W) writer structure"]
impl crate::Writable for BorthresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BORTHRESHOLD to value 0"]
impl crate::Resettable for BorthresholdSpec {
    const RESET_VALUE: u32 = 0;
}

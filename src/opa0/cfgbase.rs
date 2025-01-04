#[doc = "Register `CFGBASE` reader"]
pub type R = crate::R<CfgbaseSpec>;
#[doc = "Register `CFGBASE` writer"]
pub type W = crate::W<CfgbaseSpec>;
#[doc = "Select gain bandwidth which affects current as well the gain bandwidth. The lower gain bandwidth has lower current. See device specific datasheet for values. Can only be modified when STAT.BUSY=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfgbaseGbw {
    #[doc = "0: LOWGAIN"]
    CfgbaseGbwLowgain = 0,
    #[doc = "1: HIGHGAIN"]
    CfgbaseGbwHighgain = 1,
}
impl From<CfgbaseGbw> for bool {
    #[inline(always)]
    fn from(variant: CfgbaseGbw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGBASE_GBW` reader - Select gain bandwidth which affects current as well the gain bandwidth. The lower gain bandwidth has lower current. See device specific datasheet for values. Can only be modified when STAT.BUSY=0."]
pub type CfgbaseGbwR = crate::BitReader<CfgbaseGbw>;
impl CfgbaseGbwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CfgbaseGbw {
        match self.bits {
            false => CfgbaseGbw::CfgbaseGbwLowgain,
            true => CfgbaseGbw::CfgbaseGbwHighgain,
        }
    }
    #[doc = "LOWGAIN"]
    #[inline(always)]
    pub fn is_cfgbase_gbw_lowgain(&self) -> bool {
        *self == CfgbaseGbw::CfgbaseGbwLowgain
    }
    #[doc = "HIGHGAIN"]
    #[inline(always)]
    pub fn is_cfgbase_gbw_highgain(&self) -> bool {
        *self == CfgbaseGbw::CfgbaseGbwHighgain
    }
}
#[doc = "Field `CFGBASE_GBW` writer - Select gain bandwidth which affects current as well the gain bandwidth. The lower gain bandwidth has lower current. See device specific datasheet for values. Can only be modified when STAT.BUSY=0."]
pub type CfgbaseGbwW<'a, REG> = crate::BitWriter<'a, REG, CfgbaseGbw>;
impl<'a, REG> CfgbaseGbwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWGAIN"]
    #[inline(always)]
    pub fn cfgbase_gbw_lowgain(self) -> &'a mut crate::W<REG> {
        self.variant(CfgbaseGbw::CfgbaseGbwLowgain)
    }
    #[doc = "HIGHGAIN"]
    #[inline(always)]
    pub fn cfgbase_gbw_highgain(self) -> &'a mut crate::W<REG> {
        self.variant(CfgbaseGbw::CfgbaseGbwHighgain)
    }
}
#[doc = "Rail-to-rail input enable. Can only be modified when STAT.BUSY=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CfgbaseRri {
    #[doc = "0: OFF"]
    CfgbaseRriOff = 0,
    #[doc = "1: ON"]
    CfgbaseRriOn = 1,
}
impl From<CfgbaseRri> for bool {
    #[inline(always)]
    fn from(variant: CfgbaseRri) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGBASE_RRI` reader - Rail-to-rail input enable. Can only be modified when STAT.BUSY=0"]
pub type CfgbaseRriR = crate::BitReader<CfgbaseRri>;
impl CfgbaseRriR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CfgbaseRri {
        match self.bits {
            false => CfgbaseRri::CfgbaseRriOff,
            true => CfgbaseRri::CfgbaseRriOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_cfgbase_rri_off(&self) -> bool {
        *self == CfgbaseRri::CfgbaseRriOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_cfgbase_rri_on(&self) -> bool {
        *self == CfgbaseRri::CfgbaseRriOn
    }
}
#[doc = "Field `CFGBASE_RRI` writer - Rail-to-rail input enable. Can only be modified when STAT.BUSY=0"]
pub type CfgbaseRriW<'a, REG> = crate::BitWriter<'a, REG, CfgbaseRri>;
impl<'a, REG> CfgbaseRriW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn cfgbase_rri_off(self) -> &'a mut crate::W<REG> {
        self.variant(CfgbaseRri::CfgbaseRriOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn cfgbase_rri_on(self) -> &'a mut crate::W<REG> {
        self.variant(CfgbaseRri::CfgbaseRriOn)
    }
}
impl R {
    #[doc = "Bit 0 - Select gain bandwidth which affects current as well the gain bandwidth. The lower gain bandwidth has lower current. See device specific datasheet for values. Can only be modified when STAT.BUSY=0."]
    #[inline(always)]
    pub fn cfgbase_gbw(&self) -> CfgbaseGbwR {
        CfgbaseGbwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Rail-to-rail input enable. Can only be modified when STAT.BUSY=0"]
    #[inline(always)]
    pub fn cfgbase_rri(&self) -> CfgbaseRriR {
        CfgbaseRriR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select gain bandwidth which affects current as well the gain bandwidth. The lower gain bandwidth has lower current. See device specific datasheet for values. Can only be modified when STAT.BUSY=0."]
    #[inline(always)]
    pub fn cfgbase_gbw(&mut self) -> CfgbaseGbwW<CfgbaseSpec> {
        CfgbaseGbwW::new(self, 0)
    }
    #[doc = "Bit 2 - Rail-to-rail input enable. Can only be modified when STAT.BUSY=0"]
    #[inline(always)]
    pub fn cfgbase_rri(&mut self) -> CfgbaseRriW<CfgbaseSpec> {
        CfgbaseRriW::new(self, 2)
    }
}
#[doc = "Configuration Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgbaseSpec;
impl crate::RegisterSpec for CfgbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgbase::R`](R) reader structure"]
impl crate::Readable for CfgbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgbase::W`](W) writer structure"]
impl crate::Writable for CfgbaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGBASE to value 0"]
impl crate::Resettable for CfgbaseSpec {
    const RESET_VALUE: u32 = 0;
}

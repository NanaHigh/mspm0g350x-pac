#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McrActive {
    #[doc = "0: DISABLE"]
    McrActiveDisable = 0,
    #[doc = "1: ENABLE"]
    McrActiveEnable = 1,
}
impl From<McrActive> for bool {
    #[inline(always)]
    fn from(variant: McrActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCR_ACTIVE` reader - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
pub type McrActiveR = crate::BitReader<McrActive>;
impl McrActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McrActive {
        match self.bits {
            false => McrActive::McrActiveDisable,
            true => McrActive::McrActiveEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcr_active_disable(&self) -> bool {
        *self == McrActive::McrActiveDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcr_active_enable(&self) -> bool {
        *self == McrActive::McrActiveEnable
    }
}
#[doc = "Field `MCR_ACTIVE` writer - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
pub type McrActiveW<'a, REG> = crate::BitWriter<'a, REG, McrActive>;
impl<'a, REG> McrActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcr_active_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McrActive::McrActiveDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcr_active_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McrActive::McrActiveEnable)
    }
}
#[doc = "Multimaster mode. In Multimaster mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McrMmst {
    #[doc = "0: DISABLE"]
    McrMmstDisable = 0,
    #[doc = "1: ENABLE"]
    McrMmstEnable = 1,
}
impl From<McrMmst> for bool {
    #[inline(always)]
    fn from(variant: McrMmst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCR_MMST` reader - Multimaster mode. In Multimaster mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
pub type McrMmstR = crate::BitReader<McrMmst>;
impl McrMmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McrMmst {
        match self.bits {
            false => McrMmst::McrMmstDisable,
            true => McrMmst::McrMmstEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcr_mmst_disable(&self) -> bool {
        *self == McrMmst::McrMmstDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcr_mmst_enable(&self) -> bool {
        *self == McrMmst::McrMmstEnable
    }
}
#[doc = "Field `MCR_MMST` writer - Multimaster mode. In Multimaster mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
pub type McrMmstW<'a, REG> = crate::BitWriter<'a, REG, McrMmst>;
impl<'a, REG> McrMmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcr_mmst_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McrMmst::McrMmstDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcr_mmst_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McrMmst::McrMmstEnable)
    }
}
#[doc = "Clock Stretching. This bit controls the support for clock stretching of the I2C bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McrClkstretch {
    #[doc = "0: DISABLE"]
    McrClkstretchDisable = 0,
    #[doc = "1: ENABLE"]
    McrClkstretchEnable = 1,
}
impl From<McrClkstretch> for bool {
    #[inline(always)]
    fn from(variant: McrClkstretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCR_CLKSTRETCH` reader - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
pub type McrClkstretchR = crate::BitReader<McrClkstretch>;
impl McrClkstretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McrClkstretch {
        match self.bits {
            false => McrClkstretch::McrClkstretchDisable,
            true => McrClkstretch::McrClkstretchEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcr_clkstretch_disable(&self) -> bool {
        *self == McrClkstretch::McrClkstretchDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcr_clkstretch_enable(&self) -> bool {
        *self == McrClkstretch::McrClkstretchEnable
    }
}
#[doc = "Field `MCR_CLKSTRETCH` writer - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
pub type McrClkstretchW<'a, REG> = crate::BitWriter<'a, REG, McrClkstretch>;
impl<'a, REG> McrClkstretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcr_clkstretch_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McrClkstretch::McrClkstretchDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcr_clkstretch_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McrClkstretch::McrClkstretchEnable)
    }
}
#[doc = "I2C Loopback\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McrLpbk {
    #[doc = "0: DISABLE"]
    McrLpbkDisable = 0,
    #[doc = "1: ENABLE"]
    McrLpbkEnable = 1,
}
impl From<McrLpbk> for bool {
    #[inline(always)]
    fn from(variant: McrLpbk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCR_LPBK` reader - I2C Loopback"]
pub type McrLpbkR = crate::BitReader<McrLpbk>;
impl McrLpbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McrLpbk {
        match self.bits {
            false => McrLpbk::McrLpbkDisable,
            true => McrLpbk::McrLpbkEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcr_lpbk_disable(&self) -> bool {
        *self == McrLpbk::McrLpbkDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcr_lpbk_enable(&self) -> bool {
        *self == McrLpbk::McrLpbkEnable
    }
}
#[doc = "Field `MCR_LPBK` writer - I2C Loopback"]
pub type McrLpbkW<'a, REG> = crate::BitWriter<'a, REG, McrLpbk>;
impl<'a, REG> McrLpbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcr_lpbk_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McrLpbk::McrLpbkDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcr_lpbk_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McrLpbk::McrLpbkEnable)
    }
}
impl R {
    #[doc = "Bit 0 - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
    #[inline(always)]
    pub fn mcr_active(&self) -> McrActiveR {
        McrActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Multimaster mode. In Multimaster mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
    #[inline(always)]
    pub fn mcr_mmst(&self) -> McrMmstR {
        McrMmstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
    #[inline(always)]
    pub fn mcr_clkstretch(&self) -> McrClkstretchR {
        McrClkstretchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C Loopback"]
    #[inline(always)]
    pub fn mcr_lpbk(&self) -> McrLpbkR {
        McrLpbkR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Active After this bit has been set, it should not be set again unless it has been cleared by writing a 0 or by a reset, otherwise transfer failures may occur."]
    #[inline(always)]
    pub fn mcr_active(&mut self) -> McrActiveW<McrSpec> {
        McrActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - Multimaster mode. In Multimaster mode the SCL high time counts once the SCL line has been detected high. If this is not enabled the high time counts as soon as the SCL line has been set high by the I2C controller."]
    #[inline(always)]
    pub fn mcr_mmst(&mut self) -> McrMmstW<McrSpec> {
        McrMmstW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Stretching. This bit controls the support for clock stretching of the I2C bus."]
    #[inline(always)]
    pub fn mcr_clkstretch(&mut self) -> McrClkstretchW<McrSpec> {
        McrClkstretchW::new(self, 2)
    }
    #[doc = "Bit 8 - I2C Loopback"]
    #[inline(always)]
    pub fn mcr_lpbk(&mut self) -> McrLpbkW<McrSpec> {
        McrLpbkW::new(self, 8)
    }
}
#[doc = "I2C Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}

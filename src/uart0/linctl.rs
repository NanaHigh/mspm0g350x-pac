#[doc = "Register `LINCTL` reader"]
pub type R = crate::R<LinctlSpec>;
#[doc = "Register `LINCTL` writer"]
pub type W = crate::W<LinctlSpec>;
#[doc = "LIN Counter Enable. LIN counter will only count when enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinctlCtrena {
    #[doc = "0: DISABLE"]
    LinctlCtrenaDisable = 0,
    #[doc = "1: ENABLE"]
    LinctlCtrenaEnable = 1,
}
impl From<LinctlCtrena> for bool {
    #[inline(always)]
    fn from(variant: LinctlCtrena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCTL_CTRENA` reader - LIN Counter Enable. LIN counter will only count when enabled."]
pub type LinctlCtrenaR = crate::BitReader<LinctlCtrena>;
impl LinctlCtrenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinctlCtrena {
        match self.bits {
            false => LinctlCtrena::LinctlCtrenaDisable,
            true => LinctlCtrena::LinctlCtrenaEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_linctl_ctrena_disable(&self) -> bool {
        *self == LinctlCtrena::LinctlCtrenaDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_linctl_ctrena_enable(&self) -> bool {
        *self == LinctlCtrena::LinctlCtrenaEnable
    }
}
#[doc = "Field `LINCTL_CTRENA` writer - LIN Counter Enable. LIN counter will only count when enabled."]
pub type LinctlCtrenaW<'a, REG> = crate::BitWriter<'a, REG, LinctlCtrena>;
impl<'a, REG> LinctlCtrenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn linctl_ctrena_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlCtrena::LinctlCtrenaDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn linctl_ctrena_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlCtrena::LinctlCtrenaEnable)
    }
}
#[doc = "Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinctlZerone {
    #[doc = "0: DISABLE"]
    LinctlZeroneDisable = 0,
    #[doc = "1: ENABLE"]
    LinctlZeroneEnable = 1,
}
impl From<LinctlZerone> for bool {
    #[inline(always)]
    fn from(variant: LinctlZerone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCTL_ZERONE` reader - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
pub type LinctlZeroneR = crate::BitReader<LinctlZerone>;
impl LinctlZeroneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinctlZerone {
        match self.bits {
            false => LinctlZerone::LinctlZeroneDisable,
            true => LinctlZerone::LinctlZeroneEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_linctl_zerone_disable(&self) -> bool {
        *self == LinctlZerone::LinctlZeroneDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_linctl_zerone_enable(&self) -> bool {
        *self == LinctlZerone::LinctlZeroneEnable
    }
}
#[doc = "Field `LINCTL_ZERONE` writer - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
pub type LinctlZeroneW<'a, REG> = crate::BitWriter<'a, REG, LinctlZerone>;
impl<'a, REG> LinctlZeroneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn linctl_zerone_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlZerone::LinctlZeroneDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn linctl_zerone_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlZerone::LinctlZeroneEnable)
    }
}
#[doc = "Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinctlCntrxlow {
    #[doc = "0: DISABLE"]
    LinctlCntrxlowDisable = 0,
    #[doc = "1: ENABLE"]
    LinctlCntrxlowEnable = 1,
}
impl From<LinctlCntrxlow> for bool {
    #[inline(always)]
    fn from(variant: LinctlCntrxlow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCTL_CNTRXLOW` reader - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
pub type LinctlCntrxlowR = crate::BitReader<LinctlCntrxlow>;
impl LinctlCntrxlowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinctlCntrxlow {
        match self.bits {
            false => LinctlCntrxlow::LinctlCntrxlowDisable,
            true => LinctlCntrxlow::LinctlCntrxlowEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_linctl_cntrxlow_disable(&self) -> bool {
        *self == LinctlCntrxlow::LinctlCntrxlowDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_linctl_cntrxlow_enable(&self) -> bool {
        *self == LinctlCntrxlow::LinctlCntrxlowEnable
    }
}
#[doc = "Field `LINCTL_CNTRXLOW` writer - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
pub type LinctlCntrxlowW<'a, REG> = crate::BitWriter<'a, REG, LinctlCntrxlow>;
impl<'a, REG> LinctlCntrxlowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn linctl_cntrxlow_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlCntrxlow::LinctlCntrxlowDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn linctl_cntrxlow_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlCntrxlow::LinctlCntrxlowEnable)
    }
}
#[doc = "Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinctlLinc0cap {
    #[doc = "0: DISABLE"]
    LinctlLinc0capDisable = 0,
    #[doc = "1: ENABLE"]
    LinctlLinc0capEnable = 1,
}
impl From<LinctlLinc0cap> for bool {
    #[inline(always)]
    fn from(variant: LinctlLinc0cap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCTL_LINC0CAP` reader - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
pub type LinctlLinc0capR = crate::BitReader<LinctlLinc0cap>;
impl LinctlLinc0capR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinctlLinc0cap {
        match self.bits {
            false => LinctlLinc0cap::LinctlLinc0capDisable,
            true => LinctlLinc0cap::LinctlLinc0capEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_linctl_linc0cap_disable(&self) -> bool {
        *self == LinctlLinc0cap::LinctlLinc0capDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_linctl_linc0cap_enable(&self) -> bool {
        *self == LinctlLinc0cap::LinctlLinc0capEnable
    }
}
#[doc = "Field `LINCTL_LINC0CAP` writer - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
pub type LinctlLinc0capW<'a, REG> = crate::BitWriter<'a, REG, LinctlLinc0cap>;
impl<'a, REG> LinctlLinc0capW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn linctl_linc0cap_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlLinc0cap::LinctlLinc0capDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn linctl_linc0cap_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlLinc0cap::LinctlLinc0capEnable)
    }
}
#[doc = "Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinctlLinc1cap {
    #[doc = "0: DISABLE"]
    LinctlLinc1capDisable = 0,
    #[doc = "1: ENABLE"]
    LinctlLinc1capEnable = 1,
}
impl From<LinctlLinc1cap> for bool {
    #[inline(always)]
    fn from(variant: LinctlLinc1cap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCTL_LINC1CAP` reader - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
pub type LinctlLinc1capR = crate::BitReader<LinctlLinc1cap>;
impl LinctlLinc1capR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinctlLinc1cap {
        match self.bits {
            false => LinctlLinc1cap::LinctlLinc1capDisable,
            true => LinctlLinc1cap::LinctlLinc1capEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_linctl_linc1cap_disable(&self) -> bool {
        *self == LinctlLinc1cap::LinctlLinc1capDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_linctl_linc1cap_enable(&self) -> bool {
        *self == LinctlLinc1cap::LinctlLinc1capEnable
    }
}
#[doc = "Field `LINCTL_LINC1CAP` writer - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
pub type LinctlLinc1capW<'a, REG> = crate::BitWriter<'a, REG, LinctlLinc1cap>;
impl<'a, REG> LinctlLinc1capW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn linctl_linc1cap_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlLinc1cap::LinctlLinc1capDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn linctl_linc1cap_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlLinc1cap::LinctlLinc1capEnable)
    }
}
#[doc = "Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinctlLinc0Match {
    #[doc = "0: DISABLE"]
    LinctlLinc0MatchDisable = 0,
    #[doc = "1: ENABLE"]
    LinctlLinc0MatchEnable = 1,
}
impl From<LinctlLinc0Match> for bool {
    #[inline(always)]
    fn from(variant: LinctlLinc0Match) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINCTL_LINC0_MATCH` reader - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
pub type LinctlLinc0MatchR = crate::BitReader<LinctlLinc0Match>;
impl LinctlLinc0MatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LinctlLinc0Match {
        match self.bits {
            false => LinctlLinc0Match::LinctlLinc0MatchDisable,
            true => LinctlLinc0Match::LinctlLinc0MatchEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_linctl_linc0_match_disable(&self) -> bool {
        *self == LinctlLinc0Match::LinctlLinc0MatchDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_linctl_linc0_match_enable(&self) -> bool {
        *self == LinctlLinc0Match::LinctlLinc0MatchEnable
    }
}
#[doc = "Field `LINCTL_LINC0_MATCH` writer - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
pub type LinctlLinc0MatchW<'a, REG> = crate::BitWriter<'a, REG, LinctlLinc0Match>;
impl<'a, REG> LinctlLinc0MatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn linctl_linc0_match_disable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlLinc0Match::LinctlLinc0MatchDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn linctl_linc0_match_enable(self) -> &'a mut crate::W<REG> {
        self.variant(LinctlLinc0Match::LinctlLinc0MatchEnable)
    }
}
impl R {
    #[doc = "Bit 0 - LIN Counter Enable. LIN counter will only count when enabled."]
    #[inline(always)]
    pub fn linctl_ctrena(&self) -> LinctlCtrenaR {
        LinctlCtrenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
    #[inline(always)]
    pub fn linctl_zerone(&self) -> LinctlZeroneR {
        LinctlZeroneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
    #[inline(always)]
    pub fn linctl_cntrxlow(&self) -> LinctlCntrxlowR {
        LinctlCntrxlowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linctl_linc0cap(&self) -> LinctlLinc0capR {
        LinctlLinc0capR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linctl_linc1cap(&self) -> LinctlLinc1capR {
        LinctlLinc1capR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
    #[inline(always)]
    pub fn linctl_linc0_match(&self) -> LinctlLinc0MatchR {
        LinctlLinc0MatchR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LIN Counter Enable. LIN counter will only count when enabled."]
    #[inline(always)]
    pub fn linctl_ctrena(&mut self) -> LinctlCtrenaW<LinctlSpec> {
        LinctlCtrenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Zero on negative Edge of RXD. When enabled the counter is set to 0 and starts counting on a negative edge of RXD"]
    #[inline(always)]
    pub fn linctl_zerone(&mut self) -> LinctlZeroneW<LinctlSpec> {
        LinctlZeroneW::new(self, 1)
    }
    #[doc = "Bit 2 - Count while low Signal on RXD When counter is enabled and the signal on RXD is low, the counter increments."]
    #[inline(always)]
    pub fn linctl_cntrxlow(&mut self) -> LinctlCntrxlowW<LinctlSpec> {
        LinctlCntrxlowW::new(self, 2)
    }
    #[doc = "Bit 4 - Capture Counter on negative RXD Edge. When enabled the counter value is captured to LINC0 register on each negative RXD edge. A LINC0 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linctl_linc0cap(&mut self) -> LinctlLinc0capW<LinctlSpec> {
        LinctlLinc0capW::new(self, 4)
    }
    #[doc = "Bit 5 - Capture Counter on positive RXD Edge. When enabled the counter value is captured to LINC1 register on each positive RXD edge. A LINC1 interrupt is triggered when enabled."]
    #[inline(always)]
    pub fn linctl_linc1cap(&mut self) -> LinctlLinc1capW<LinctlSpec> {
        LinctlLinc1capW::new(self, 5)
    }
    #[doc = "Bit 6 - Counter Compare Match Mode When this bit is set to 1 a counter compare match with LINC0 register triggers an LINC0 interrupt when enabled."]
    #[inline(always)]
    pub fn linctl_linc0_match(&mut self) -> LinctlLinc0MatchW<LinctlSpec> {
        LinctlLinc0MatchW::new(self, 6)
    }
}
#[doc = "UART LIN Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinctlSpec;
impl crate::RegisterSpec for LinctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linctl::R`](R) reader structure"]
impl crate::Readable for LinctlSpec {}
#[doc = "`write(|w| ..)` method takes [`linctl::W`](W) writer structure"]
impl crate::Writable for LinctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINCTL to value 0"]
impl crate::Resettable for LinctlSpec {
    const RESET_VALUE: u32 = 0;
}

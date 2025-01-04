#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "AMP_EN - output amplifier enabled or disabled 0 : disabled 1 : enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Ampen {
    #[doc = "0: DISABLE"]
    Ctl1AmpenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1AmpenEnable = 1,
}
impl From<Ctl1Ampen> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Ampen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_AMPEN` reader - AMP_EN - output amplifier enabled or disabled 0 : disabled 1 : enabled"]
pub type Ctl1AmpenR = crate::BitReader<Ctl1Ampen>;
impl Ctl1AmpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Ampen {
        match self.bits {
            false => Ctl1Ampen::Ctl1AmpenDisable,
            true => Ctl1Ampen::Ctl1AmpenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_ampen_disable(&self) -> bool {
        *self == Ctl1Ampen::Ctl1AmpenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_ampen_enable(&self) -> bool {
        *self == Ctl1Ampen::Ctl1AmpenEnable
    }
}
#[doc = "Field `CTL1_AMPEN` writer - AMP_EN - output amplifier enabled or disabled 0 : disabled 1 : enabled"]
pub type Ctl1AmpenW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Ampen>;
impl<'a, REG> Ctl1AmpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_ampen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ampen::Ctl1AmpenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_ampen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ampen::Ctl1AmpenEnable)
    }
}
#[doc = "AMPHIZ - amplifier output value 0 : amplifier output is high impedance 1 : amplifier output is pulled down to ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Amphiz {
    #[doc = "0: HIZ"]
    Ctl1AmphizHiz = 0,
    #[doc = "1: PULLDOWN"]
    Ctl1AmphizPulldown = 1,
}
impl From<Ctl1Amphiz> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Amphiz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_AMPHIZ` reader - AMPHIZ - amplifier output value 0 : amplifier output is high impedance 1 : amplifier output is pulled down to ground"]
pub type Ctl1AmphizR = crate::BitReader<Ctl1Amphiz>;
impl Ctl1AmphizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Amphiz {
        match self.bits {
            false => Ctl1Amphiz::Ctl1AmphizHiz,
            true => Ctl1Amphiz::Ctl1AmphizPulldown,
        }
    }
    #[doc = "HIZ"]
    #[inline(always)]
    pub fn is_ctl1_amphiz_hiz(&self) -> bool {
        *self == Ctl1Amphiz::Ctl1AmphizHiz
    }
    #[doc = "PULLDOWN"]
    #[inline(always)]
    pub fn is_ctl1_amphiz_pulldown(&self) -> bool {
        *self == Ctl1Amphiz::Ctl1AmphizPulldown
    }
}
#[doc = "Field `CTL1_AMPHIZ` writer - AMPHIZ - amplifier output value 0 : amplifier output is high impedance 1 : amplifier output is pulled down to ground"]
pub type Ctl1AmphizW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Amphiz>;
impl<'a, REG> Ctl1AmphizW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIZ"]
    #[inline(always)]
    pub fn ctl1_amphiz_hiz(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Amphiz::Ctl1AmphizHiz)
    }
    #[doc = "PULLDOWN"]
    #[inline(always)]
    pub fn ctl1_amphiz_pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Amphiz::Ctl1AmphizPulldown)
    }
}
#[doc = "This bit selects the DAC voltage reference source + input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Refsp {
    #[doc = "0: VDDA"]
    Ctl1RefspVdda = 0,
    #[doc = "1: VEREFP"]
    Ctl1RefspVerefp = 1,
}
impl From<Ctl1Refsp> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Refsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_REFSP` reader - This bit selects the DAC voltage reference source + input."]
pub type Ctl1RefspR = crate::BitReader<Ctl1Refsp>;
impl Ctl1RefspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Refsp {
        match self.bits {
            false => Ctl1Refsp::Ctl1RefspVdda,
            true => Ctl1Refsp::Ctl1RefspVerefp,
        }
    }
    #[doc = "VDDA"]
    #[inline(always)]
    pub fn is_ctl1_refsp_vdda(&self) -> bool {
        *self == Ctl1Refsp::Ctl1RefspVdda
    }
    #[doc = "VEREFP"]
    #[inline(always)]
    pub fn is_ctl1_refsp_verefp(&self) -> bool {
        *self == Ctl1Refsp::Ctl1RefspVerefp
    }
}
#[doc = "Field `CTL1_REFSP` writer - This bit selects the DAC voltage reference source + input."]
pub type Ctl1RefspW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Refsp>;
impl<'a, REG> Ctl1RefspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VDDA"]
    #[inline(always)]
    pub fn ctl1_refsp_vdda(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Refsp::Ctl1RefspVdda)
    }
    #[doc = "VEREFP"]
    #[inline(always)]
    pub fn ctl1_refsp_verefp(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Refsp::Ctl1RefspVerefp)
    }
}
#[doc = "This bit selects the DAC voltage reference source + input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Refsn {
    #[doc = "0: VEREFN"]
    Ctl1RefsnVerefn = 0,
    #[doc = "1: VSSA"]
    Ctl1RefsnVssa = 1,
}
impl From<Ctl1Refsn> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Refsn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_REFSN` reader - This bit selects the DAC voltage reference source + input."]
pub type Ctl1RefsnR = crate::BitReader<Ctl1Refsn>;
impl Ctl1RefsnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Refsn {
        match self.bits {
            false => Ctl1Refsn::Ctl1RefsnVerefn,
            true => Ctl1Refsn::Ctl1RefsnVssa,
        }
    }
    #[doc = "VEREFN"]
    #[inline(always)]
    pub fn is_ctl1_refsn_verefn(&self) -> bool {
        *self == Ctl1Refsn::Ctl1RefsnVerefn
    }
    #[doc = "VSSA"]
    #[inline(always)]
    pub fn is_ctl1_refsn_vssa(&self) -> bool {
        *self == Ctl1Refsn::Ctl1RefsnVssa
    }
}
#[doc = "Field `CTL1_REFSN` writer - This bit selects the DAC voltage reference source + input."]
pub type Ctl1RefsnW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Refsn>;
impl<'a, REG> Ctl1RefsnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VEREFN"]
    #[inline(always)]
    pub fn ctl1_refsn_verefn(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Refsn::Ctl1RefsnVerefn)
    }
    #[doc = "VSSA"]
    #[inline(always)]
    pub fn ctl1_refsn_vssa(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Refsn::Ctl1RefsnVssa)
    }
}
#[doc = "These bits select the DAC output on device pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Ops {
    #[doc = "0: NOC0"]
    Ctl1OpsNoc0 = 0,
    #[doc = "1: OUT0"]
    Ctl1OpsOut0 = 1,
}
impl From<Ctl1Ops> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Ops) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_OPS` reader - These bits select the DAC output on device pin."]
pub type Ctl1OpsR = crate::BitReader<Ctl1Ops>;
impl Ctl1OpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Ops {
        match self.bits {
            false => Ctl1Ops::Ctl1OpsNoc0,
            true => Ctl1Ops::Ctl1OpsOut0,
        }
    }
    #[doc = "NOC0"]
    #[inline(always)]
    pub fn is_ctl1_ops_noc0(&self) -> bool {
        *self == Ctl1Ops::Ctl1OpsNoc0
    }
    #[doc = "OUT0"]
    #[inline(always)]
    pub fn is_ctl1_ops_out0(&self) -> bool {
        *self == Ctl1Ops::Ctl1OpsOut0
    }
}
#[doc = "Field `CTL1_OPS` writer - These bits select the DAC output on device pin."]
pub type Ctl1OpsW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Ops>;
impl<'a, REG> Ctl1OpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NOC0"]
    #[inline(always)]
    pub fn ctl1_ops_noc0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ops::Ctl1OpsNoc0)
    }
    #[doc = "OUT0"]
    #[inline(always)]
    pub fn ctl1_ops_out0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ops::Ctl1OpsOut0)
    }
}
impl R {
    #[doc = "Bit 0 - AMP_EN - output amplifier enabled or disabled 0 : disabled 1 : enabled"]
    #[inline(always)]
    pub fn ctl1_ampen(&self) -> Ctl1AmpenR {
        Ctl1AmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AMPHIZ - amplifier output value 0 : amplifier output is high impedance 1 : amplifier output is pulled down to ground"]
    #[inline(always)]
    pub fn ctl1_amphiz(&self) -> Ctl1AmphizR {
        Ctl1AmphizR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit selects the DAC voltage reference source + input."]
    #[inline(always)]
    pub fn ctl1_refsp(&self) -> Ctl1RefspR {
        Ctl1RefspR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit selects the DAC voltage reference source + input."]
    #[inline(always)]
    pub fn ctl1_refsn(&self) -> Ctl1RefsnR {
        Ctl1RefsnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 24 - These bits select the DAC output on device pin."]
    #[inline(always)]
    pub fn ctl1_ops(&self) -> Ctl1OpsR {
        Ctl1OpsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AMP_EN - output amplifier enabled or disabled 0 : disabled 1 : enabled"]
    #[inline(always)]
    pub fn ctl1_ampen(&mut self) -> Ctl1AmpenW<Ctl1Spec> {
        Ctl1AmpenW::new(self, 0)
    }
    #[doc = "Bit 1 - AMPHIZ - amplifier output value 0 : amplifier output is high impedance 1 : amplifier output is pulled down to ground"]
    #[inline(always)]
    pub fn ctl1_amphiz(&mut self) -> Ctl1AmphizW<Ctl1Spec> {
        Ctl1AmphizW::new(self, 1)
    }
    #[doc = "Bit 8 - This bit selects the DAC voltage reference source + input."]
    #[inline(always)]
    pub fn ctl1_refsp(&mut self) -> Ctl1RefspW<Ctl1Spec> {
        Ctl1RefspW::new(self, 8)
    }
    #[doc = "Bit 9 - This bit selects the DAC voltage reference source + input."]
    #[inline(always)]
    pub fn ctl1_refsn(&mut self) -> Ctl1RefsnW<Ctl1Spec> {
        Ctl1RefsnW::new(self, 9)
    }
    #[doc = "Bit 24 - These bits select the DAC output on device pin."]
    #[inline(always)]
    pub fn ctl1_ops(&mut self) -> Ctl1OpsW<Ctl1Spec> {
        Ctl1OpsW::new(self, 24)
    }
}
#[doc = "Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}

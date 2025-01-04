#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Channel input selected for the positive terminal of the comparator if IPEN is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Ipsel {
    #[doc = "0: CH_0"]
    Ctl0IpselCh0 = 0,
    #[doc = "1: CH_1"]
    Ctl0IpselCh1 = 1,
    #[doc = "2: CH_2"]
    Ctl0IpselCh2 = 2,
    #[doc = "3: CH_3"]
    Ctl0IpselCh3 = 3,
    #[doc = "4: CH_4"]
    Ctl0IpselCh4 = 4,
    #[doc = "5: CH_5"]
    Ctl0IpselCh5 = 5,
    #[doc = "6: CH_6"]
    Ctl0IpselCh6 = 6,
    #[doc = "7: CH_7"]
    Ctl0IpselCh7 = 7,
}
impl From<Ctl0Ipsel> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Ipsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Ipsel {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Ipsel {}
#[doc = "Field `CTL0_IPSEL` reader - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
pub type Ctl0IpselR = crate::FieldReader<Ctl0Ipsel>;
impl Ctl0IpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Ipsel {
        match self.bits {
            0 => Ctl0Ipsel::Ctl0IpselCh0,
            1 => Ctl0Ipsel::Ctl0IpselCh1,
            2 => Ctl0Ipsel::Ctl0IpselCh2,
            3 => Ctl0Ipsel::Ctl0IpselCh3,
            4 => Ctl0Ipsel::Ctl0IpselCh4,
            5 => Ctl0Ipsel::Ctl0IpselCh5,
            6 => Ctl0Ipsel::Ctl0IpselCh6,
            7 => Ctl0Ipsel::Ctl0IpselCh7,
            _ => unreachable!(),
        }
    }
    #[doc = "CH_0"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_0(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh0
    }
    #[doc = "CH_1"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_1(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh1
    }
    #[doc = "CH_2"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_2(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh2
    }
    #[doc = "CH_3"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_3(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh3
    }
    #[doc = "CH_4"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_4(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh4
    }
    #[doc = "CH_5"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_5(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh5
    }
    #[doc = "CH_6"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_6(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh6
    }
    #[doc = "CH_7"]
    #[inline(always)]
    pub fn is_ctl0_ipsel_ch_7(&self) -> bool {
        *self == Ctl0Ipsel::Ctl0IpselCh7
    }
}
#[doc = "Field `CTL0_IPSEL` writer - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
pub type Ctl0IpselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl0Ipsel, crate::Safe>;
impl<'a, REG> Ctl0IpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH_0"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh0)
    }
    #[doc = "CH_1"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh1)
    }
    #[doc = "CH_2"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh2)
    }
    #[doc = "CH_3"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh3)
    }
    #[doc = "CH_4"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh4)
    }
    #[doc = "CH_5"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh5)
    }
    #[doc = "CH_6"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh6)
    }
    #[doc = "CH_7"]
    #[inline(always)]
    pub fn ctl0_ipsel_ch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipsel::Ctl0IpselCh7)
    }
}
#[doc = "Channel input enable for the positive terminal of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Ipen {
    #[doc = "0: DISABLE"]
    Ctl0IpenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0IpenEnable = 1,
}
impl From<Ctl0Ipen> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Ipen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_IPEN` reader - Channel input enable for the positive terminal of the comparator."]
pub type Ctl0IpenR = crate::BitReader<Ctl0Ipen>;
impl Ctl0IpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Ipen {
        match self.bits {
            false => Ctl0Ipen::Ctl0IpenDisable,
            true => Ctl0Ipen::Ctl0IpenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_ipen_disable(&self) -> bool {
        *self == Ctl0Ipen::Ctl0IpenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_ipen_enable(&self) -> bool {
        *self == Ctl0Ipen::Ctl0IpenEnable
    }
}
#[doc = "Field `CTL0_IPEN` writer - Channel input enable for the positive terminal of the comparator."]
pub type Ctl0IpenW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Ipen>;
impl<'a, REG> Ctl0IpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_ipen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipen::Ctl0IpenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_ipen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Ipen::Ctl0IpenEnable)
    }
}
#[doc = "Channel input selected for the negative terminal of the comparator if IMEN is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Imsel {
    #[doc = "0: CH_0"]
    Ctl0ImselCh0 = 0,
    #[doc = "1: CH_1"]
    Ctl0ImselCh1 = 1,
    #[doc = "2: CH_2"]
    Ctl0ImselCh2 = 2,
    #[doc = "3: CH_3"]
    Ctl0ImselCh3 = 3,
    #[doc = "4: CH_4"]
    Ctl0ImselCh4 = 4,
    #[doc = "5: CH_5"]
    Ctl0ImselCh5 = 5,
    #[doc = "6: CH_6"]
    Ctl0ImselCh6 = 6,
    #[doc = "7: CH_7"]
    Ctl0ImselCh7 = 7,
}
impl From<Ctl0Imsel> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Imsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Imsel {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Imsel {}
#[doc = "Field `CTL0_IMSEL` reader - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
pub type Ctl0ImselR = crate::FieldReader<Ctl0Imsel>;
impl Ctl0ImselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Imsel {
        match self.bits {
            0 => Ctl0Imsel::Ctl0ImselCh0,
            1 => Ctl0Imsel::Ctl0ImselCh1,
            2 => Ctl0Imsel::Ctl0ImselCh2,
            3 => Ctl0Imsel::Ctl0ImselCh3,
            4 => Ctl0Imsel::Ctl0ImselCh4,
            5 => Ctl0Imsel::Ctl0ImselCh5,
            6 => Ctl0Imsel::Ctl0ImselCh6,
            7 => Ctl0Imsel::Ctl0ImselCh7,
            _ => unreachable!(),
        }
    }
    #[doc = "CH_0"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_0(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh0
    }
    #[doc = "CH_1"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_1(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh1
    }
    #[doc = "CH_2"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_2(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh2
    }
    #[doc = "CH_3"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_3(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh3
    }
    #[doc = "CH_4"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_4(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh4
    }
    #[doc = "CH_5"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_5(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh5
    }
    #[doc = "CH_6"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_6(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh6
    }
    #[doc = "CH_7"]
    #[inline(always)]
    pub fn is_ctl0_imsel_ch_7(&self) -> bool {
        *self == Ctl0Imsel::Ctl0ImselCh7
    }
}
#[doc = "Field `CTL0_IMSEL` writer - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
pub type Ctl0ImselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl0Imsel, crate::Safe>;
impl<'a, REG> Ctl0ImselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH_0"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh0)
    }
    #[doc = "CH_1"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh1)
    }
    #[doc = "CH_2"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh2)
    }
    #[doc = "CH_3"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh3)
    }
    #[doc = "CH_4"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh4)
    }
    #[doc = "CH_5"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh5)
    }
    #[doc = "CH_6"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh6)
    }
    #[doc = "CH_7"]
    #[inline(always)]
    pub fn ctl0_imsel_ch_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imsel::Ctl0ImselCh7)
    }
}
#[doc = "Channel input enable for the negative terminal of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Imen {
    #[doc = "0: DISABLE"]
    Ctl0ImenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0ImenEnable = 1,
}
impl From<Ctl0Imen> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Imen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_IMEN` reader - Channel input enable for the negative terminal of the comparator."]
pub type Ctl0ImenR = crate::BitReader<Ctl0Imen>;
impl Ctl0ImenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Imen {
        match self.bits {
            false => Ctl0Imen::Ctl0ImenDisable,
            true => Ctl0Imen::Ctl0ImenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_imen_disable(&self) -> bool {
        *self == Ctl0Imen::Ctl0ImenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_imen_enable(&self) -> bool {
        *self == Ctl0Imen::Ctl0ImenEnable
    }
}
#[doc = "Field `CTL0_IMEN` writer - Channel input enable for the negative terminal of the comparator."]
pub type Ctl0ImenW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Imen>;
impl<'a, REG> Ctl0ImenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_imen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imen::Ctl0ImenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_imen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Imen::Ctl0ImenEnable)
    }
}
impl R {
    #[doc = "Bits 0:2 - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
    #[inline(always)]
    pub fn ctl0_ipsel(&self) -> Ctl0IpselR {
        Ctl0IpselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - Channel input enable for the positive terminal of the comparator."]
    #[inline(always)]
    pub fn ctl0_ipen(&self) -> Ctl0IpenR {
        Ctl0IpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
    #[inline(always)]
    pub fn ctl0_imsel(&self) -> Ctl0ImselR {
        Ctl0ImselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 31 - Channel input enable for the negative terminal of the comparator."]
    #[inline(always)]
    pub fn ctl0_imen(&self) -> Ctl0ImenR {
        Ctl0ImenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel input selected for the positive terminal of the comparator if IPEN is set to 1."]
    #[inline(always)]
    pub fn ctl0_ipsel(&mut self) -> Ctl0IpselW<Ctl0Spec> {
        Ctl0IpselW::new(self, 0)
    }
    #[doc = "Bit 15 - Channel input enable for the positive terminal of the comparator."]
    #[inline(always)]
    pub fn ctl0_ipen(&mut self) -> Ctl0IpenW<Ctl0Spec> {
        Ctl0IpenW::new(self, 15)
    }
    #[doc = "Bits 16:18 - Channel input selected for the negative terminal of the comparator if IMEN is set to 1."]
    #[inline(always)]
    pub fn ctl0_imsel(&mut self) -> Ctl0ImselW<Ctl0Spec> {
        Ctl0ImselW::new(self, 16)
    }
    #[doc = "Bit 31 - Channel input enable for the negative terminal of the comparator."]
    #[inline(always)]
    pub fn ctl0_imen(&mut self) -> Ctl0ImenW<Ctl0Spec> {
        Ctl0ImenW::new(self, 31)
    }
}
#[doc = "Control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}

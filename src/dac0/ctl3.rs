#[doc = "Register `CTL3` reader"]
pub type R = crate::R<Ctl3Spec>;
#[doc = "Register `CTL3` writer"]
pub type W = crate::W<Ctl3Spec>;
#[doc = "This bit enables the sample time generator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl3Stimen {
    #[doc = "0: CLR"]
    Ctl3StimenClr = 0,
    #[doc = "1: SET"]
    Ctl3StimenSet = 1,
}
impl From<Ctl3Stimen> for bool {
    #[inline(always)]
    fn from(variant: Ctl3Stimen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL3_STIMEN` reader - This bit enables the sample time generator."]
pub type Ctl3StimenR = crate::BitReader<Ctl3Stimen>;
impl Ctl3StimenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl3Stimen {
        match self.bits {
            false => Ctl3Stimen::Ctl3StimenClr,
            true => Ctl3Stimen::Ctl3StimenSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ctl3_stimen_clr(&self) -> bool {
        *self == Ctl3Stimen::Ctl3StimenClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ctl3_stimen_set(&self) -> bool {
        *self == Ctl3Stimen::Ctl3StimenSet
    }
}
#[doc = "Field `CTL3_STIMEN` writer - This bit enables the sample time generator."]
pub type Ctl3StimenW<'a, REG> = crate::BitWriter<'a, REG, Ctl3Stimen>;
impl<'a, REG> Ctl3StimenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn ctl3_stimen_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimen::Ctl3StimenClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn ctl3_stimen_set(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimen::Ctl3StimenSet)
    }
}
#[doc = "These bits are used to configure the trigger rate from the sample time generator. The STIMCONFIG values 10 to 15 are reserved and default to same effect as value 0 (500SPS).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl3Stimconfig {
    #[doc = "0: _500SPS"]
    Ctl3Stimconfig_500sps = 0,
    #[doc = "1: _1KSPS"]
    Ctl3Stimconfig_1ksps = 1,
    #[doc = "2: _2KSPS"]
    Ctl3Stimconfig_2ksps = 2,
    #[doc = "3: _4KSPS"]
    Ctl3Stimconfig_4ksps = 3,
    #[doc = "4: _8KSPS"]
    Ctl3Stimconfig_8ksps = 4,
    #[doc = "5: _16KSPS"]
    Ctl3Stimconfig_16ksps = 5,
    #[doc = "6: _100KSPS"]
    Ctl3Stimconfig_100ksps = 6,
    #[doc = "7: _200KSPS"]
    Ctl3Stimconfig_200ksps = 7,
    #[doc = "8: _500KSPS"]
    Ctl3Stimconfig_500ksps = 8,
    #[doc = "9: _1MSPS"]
    Ctl3Stimconfig_1msps = 9,
}
impl From<Ctl3Stimconfig> for u8 {
    #[inline(always)]
    fn from(variant: Ctl3Stimconfig) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl3Stimconfig {
    type Ux = u8;
}
impl crate::IsEnum for Ctl3Stimconfig {}
#[doc = "Field `CTL3_STIMCONFIG` reader - These bits are used to configure the trigger rate from the sample time generator. The STIMCONFIG values 10 to 15 are reserved and default to same effect as value 0 (500SPS)."]
pub type Ctl3StimconfigR = crate::FieldReader<Ctl3Stimconfig>;
impl Ctl3StimconfigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl3Stimconfig> {
        match self.bits {
            0 => Some(Ctl3Stimconfig::Ctl3Stimconfig_500sps),
            1 => Some(Ctl3Stimconfig::Ctl3Stimconfig_1ksps),
            2 => Some(Ctl3Stimconfig::Ctl3Stimconfig_2ksps),
            3 => Some(Ctl3Stimconfig::Ctl3Stimconfig_4ksps),
            4 => Some(Ctl3Stimconfig::Ctl3Stimconfig_8ksps),
            5 => Some(Ctl3Stimconfig::Ctl3Stimconfig_16ksps),
            6 => Some(Ctl3Stimconfig::Ctl3Stimconfig_100ksps),
            7 => Some(Ctl3Stimconfig::Ctl3Stimconfig_200ksps),
            8 => Some(Ctl3Stimconfig::Ctl3Stimconfig_500ksps),
            9 => Some(Ctl3Stimconfig::Ctl3Stimconfig_1msps),
            _ => None,
        }
    }
    #[doc = "_500SPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__500sps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_500sps
    }
    #[doc = "_1KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__1ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_1ksps
    }
    #[doc = "_2KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__2ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_2ksps
    }
    #[doc = "_4KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__4ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_4ksps
    }
    #[doc = "_8KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__8ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_8ksps
    }
    #[doc = "_16KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__16ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_16ksps
    }
    #[doc = "_100KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__100ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_100ksps
    }
    #[doc = "_200KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__200ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_200ksps
    }
    #[doc = "_500KSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__500ksps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_500ksps
    }
    #[doc = "_1MSPS"]
    #[inline(always)]
    pub fn is_ctl3_stimconfig__1msps(&self) -> bool {
        *self == Ctl3Stimconfig::Ctl3Stimconfig_1msps
    }
}
#[doc = "Field `CTL3_STIMCONFIG` writer - These bits are used to configure the trigger rate from the sample time generator. The STIMCONFIG values 10 to 15 are reserved and default to same effect as value 0 (500SPS)."]
pub type Ctl3StimconfigW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ctl3Stimconfig>;
impl<'a, REG> Ctl3StimconfigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_500SPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__500sps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_500sps)
    }
    #[doc = "_1KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__1ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_1ksps)
    }
    #[doc = "_2KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__2ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_2ksps)
    }
    #[doc = "_4KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__4ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_4ksps)
    }
    #[doc = "_8KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__8ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_8ksps)
    }
    #[doc = "_16KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__16ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_16ksps)
    }
    #[doc = "_100KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__100ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_100ksps)
    }
    #[doc = "_200KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__200ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_200ksps)
    }
    #[doc = "_500KSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__500ksps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_500ksps)
    }
    #[doc = "_1MSPS"]
    #[inline(always)]
    pub fn ctl3_stimconfig__1msps(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl3Stimconfig::Ctl3Stimconfig_1msps)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the sample time generator."]
    #[inline(always)]
    pub fn ctl3_stimen(&self) -> Ctl3StimenR {
        Ctl3StimenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - These bits are used to configure the trigger rate from the sample time generator. The STIMCONFIG values 10 to 15 are reserved and default to same effect as value 0 (500SPS)."]
    #[inline(always)]
    pub fn ctl3_stimconfig(&self) -> Ctl3StimconfigR {
        Ctl3StimconfigR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the sample time generator."]
    #[inline(always)]
    pub fn ctl3_stimen(&mut self) -> Ctl3StimenW<Ctl3Spec> {
        Ctl3StimenW::new(self, 0)
    }
    #[doc = "Bits 8:11 - These bits are used to configure the trigger rate from the sample time generator. The STIMCONFIG values 10 to 15 are reserved and default to same effect as value 0 (500SPS)."]
    #[inline(always)]
    pub fn ctl3_stimconfig(&mut self) -> Ctl3StimconfigW<Ctl3Spec> {
        Ctl3StimconfigW::new(self, 8)
    }
}
#[doc = "Control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl3Spec;
impl crate::RegisterSpec for Ctl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl3::R`](R) reader structure"]
impl crate::Readable for Ctl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl3::W`](W) writer structure"]
impl crate::Writable for Ctl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL3 to value 0"]
impl crate::Resettable for Ctl3Spec {
    const RESET_VALUE: u32 = 0;
}

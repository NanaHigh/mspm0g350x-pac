#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Enable conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Enc {
    #[doc = "0: OFF"]
    Ctl0EncOff = 0,
    #[doc = "1: ON"]
    Ctl0EncOn = 1,
}
impl From<Ctl0Enc> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Enc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_ENC` reader - Enable conversion"]
pub type Ctl0EncR = crate::BitReader<Ctl0Enc>;
impl Ctl0EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Enc {
        match self.bits {
            false => Ctl0Enc::Ctl0EncOff,
            true => Ctl0Enc::Ctl0EncOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_ctl0_enc_off(&self) -> bool {
        *self == Ctl0Enc::Ctl0EncOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_ctl0_enc_on(&self) -> bool {
        *self == Ctl0Enc::Ctl0EncOn
    }
}
#[doc = "Field `CTL0_ENC` writer - Enable conversion"]
pub type Ctl0EncW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Enc>;
impl<'a, REG> Ctl0EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn ctl0_enc_off(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enc::Ctl0EncOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn ctl0_enc_on(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Enc::Ctl0EncOn)
    }
}
#[doc = "Power down policy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Pwrdn {
    #[doc = "0: AUTO"]
    Ctl0PwrdnAuto = 0,
    #[doc = "1: MANUAL"]
    Ctl0PwrdnManual = 1,
}
impl From<Ctl0Pwrdn> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Pwrdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_PWRDN` reader - Power down policy"]
pub type Ctl0PwrdnR = crate::BitReader<Ctl0Pwrdn>;
impl Ctl0PwrdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Pwrdn {
        match self.bits {
            false => Ctl0Pwrdn::Ctl0PwrdnAuto,
            true => Ctl0Pwrdn::Ctl0PwrdnManual,
        }
    }
    #[doc = "AUTO"]
    #[inline(always)]
    pub fn is_ctl0_pwrdn_auto(&self) -> bool {
        *self == Ctl0Pwrdn::Ctl0PwrdnAuto
    }
    #[doc = "MANUAL"]
    #[inline(always)]
    pub fn is_ctl0_pwrdn_manual(&self) -> bool {
        *self == Ctl0Pwrdn::Ctl0PwrdnManual
    }
}
#[doc = "Field `CTL0_PWRDN` writer - Power down policy"]
pub type Ctl0PwrdnW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Pwrdn>;
impl<'a, REG> Ctl0PwrdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUTO"]
    #[inline(always)]
    pub fn ctl0_pwrdn_auto(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Pwrdn::Ctl0PwrdnAuto)
    }
    #[doc = "MANUAL"]
    #[inline(always)]
    pub fn ctl0_pwrdn_manual(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Pwrdn::Ctl0PwrdnManual)
    }
}
#[doc = "Sample clock divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Sclkdiv {
    #[doc = "0: DIV_BY_1"]
    Ctl0SclkdivDivBy1 = 0,
    #[doc = "1: DIV_BY_2"]
    Ctl0SclkdivDivBy2 = 1,
    #[doc = "2: DIV_BY_4"]
    Ctl0SclkdivDivBy4 = 2,
    #[doc = "3: DIV_BY_8"]
    Ctl0SclkdivDivBy8 = 3,
    #[doc = "4: DIV_BY_16"]
    Ctl0SclkdivDivBy16 = 4,
    #[doc = "5: DIV_BY_24"]
    Ctl0SclkdivDivBy24 = 5,
    #[doc = "6: DIV_BY_32"]
    Ctl0SclkdivDivBy32 = 6,
    #[doc = "7: DIV_BY_48"]
    Ctl0SclkdivDivBy48 = 7,
}
impl From<Ctl0Sclkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Sclkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Sclkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Sclkdiv {}
#[doc = "Field `CTL0_SCLKDIV` reader - Sample clock divider"]
pub type Ctl0SclkdivR = crate::FieldReader<Ctl0Sclkdiv>;
impl Ctl0SclkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Sclkdiv {
        match self.bits {
            0 => Ctl0Sclkdiv::Ctl0SclkdivDivBy1,
            1 => Ctl0Sclkdiv::Ctl0SclkdivDivBy2,
            2 => Ctl0Sclkdiv::Ctl0SclkdivDivBy4,
            3 => Ctl0Sclkdiv::Ctl0SclkdivDivBy8,
            4 => Ctl0Sclkdiv::Ctl0SclkdivDivBy16,
            5 => Ctl0Sclkdiv::Ctl0SclkdivDivBy24,
            6 => Ctl0Sclkdiv::Ctl0SclkdivDivBy32,
            7 => Ctl0Sclkdiv::Ctl0SclkdivDivBy48,
            _ => unreachable!(),
        }
    }
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_1(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy1
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_2(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy2
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_4(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy4
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_8(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy8
    }
    #[doc = "DIV_BY_16"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_16(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy16
    }
    #[doc = "DIV_BY_24"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_24(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy24
    }
    #[doc = "DIV_BY_32"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_32(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy32
    }
    #[doc = "DIV_BY_48"]
    #[inline(always)]
    pub fn is_ctl0_sclkdiv_div_by_48(&self) -> bool {
        *self == Ctl0Sclkdiv::Ctl0SclkdivDivBy48
    }
}
#[doc = "Field `CTL0_SCLKDIV` writer - Sample clock divider"]
pub type Ctl0SclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ctl0Sclkdiv, crate::Safe>;
impl<'a, REG> Ctl0SclkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV_BY_1"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy1)
    }
    #[doc = "DIV_BY_2"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy2)
    }
    #[doc = "DIV_BY_4"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy4)
    }
    #[doc = "DIV_BY_8"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy8)
    }
    #[doc = "DIV_BY_16"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy16)
    }
    #[doc = "DIV_BY_24"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_24(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy24)
    }
    #[doc = "DIV_BY_32"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy32)
    }
    #[doc = "DIV_BY_48"]
    #[inline(always)]
    pub fn ctl0_sclkdiv_div_by_48(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sclkdiv::Ctl0SclkdivDivBy48)
    }
}
impl R {
    #[doc = "Bit 0 - Enable conversion"]
    #[inline(always)]
    pub fn ctl0_enc(&self) -> Ctl0EncR {
        Ctl0EncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Power down policy"]
    #[inline(always)]
    pub fn ctl0_pwrdn(&self) -> Ctl0PwrdnR {
        Ctl0PwrdnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Sample clock divider"]
    #[inline(always)]
    pub fn ctl0_sclkdiv(&self) -> Ctl0SclkdivR {
        Ctl0SclkdivR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable conversion"]
    #[inline(always)]
    pub fn ctl0_enc(&mut self) -> Ctl0EncW<Ctl0Spec> {
        Ctl0EncW::new(self, 0)
    }
    #[doc = "Bit 16 - Power down policy"]
    #[inline(always)]
    pub fn ctl0_pwrdn(&mut self) -> Ctl0PwrdnW<Ctl0Spec> {
        Ctl0PwrdnW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Sample clock divider"]
    #[inline(always)]
    pub fn ctl0_sclkdiv(&mut self) -> Ctl0SclkdivW<Ctl0Spec> {
        Ctl0SclkdivW::new(self, 24)
    }
}
#[doc = "Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

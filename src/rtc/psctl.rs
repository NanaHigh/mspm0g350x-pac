#[doc = "Register `PSCTL` reader"]
pub type R = crate::R<PsctlSpec>;
#[doc = "Register `PSCTL` writer"]
pub type W = crate::W<PsctlSpec>;
#[doc = "Prescale timer 0 interrupt interval\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PsctlRt0ip {
    #[doc = "2: DIV8"]
    PsctlRt0ipDiv8 = 2,
    #[doc = "3: DIV16"]
    PsctlRt0ipDiv16 = 3,
    #[doc = "4: DIV32"]
    PsctlRt0ipDiv32 = 4,
    #[doc = "5: DIV64"]
    PsctlRt0ipDiv64 = 5,
    #[doc = "6: DIV128"]
    PsctlRt0ipDiv128 = 6,
    #[doc = "7: DIV256"]
    PsctlRt0ipDiv256 = 7,
}
impl From<PsctlRt0ip> for u8 {
    #[inline(always)]
    fn from(variant: PsctlRt0ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PsctlRt0ip {
    type Ux = u8;
}
impl crate::IsEnum for PsctlRt0ip {}
#[doc = "Field `PSCTL_RT0IP` reader - Prescale timer 0 interrupt interval"]
pub type PsctlRt0ipR = crate::FieldReader<PsctlRt0ip>;
impl PsctlRt0ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PsctlRt0ip> {
        match self.bits {
            2 => Some(PsctlRt0ip::PsctlRt0ipDiv8),
            3 => Some(PsctlRt0ip::PsctlRt0ipDiv16),
            4 => Some(PsctlRt0ip::PsctlRt0ipDiv32),
            5 => Some(PsctlRt0ip::PsctlRt0ipDiv64),
            6 => Some(PsctlRt0ip::PsctlRt0ipDiv128),
            7 => Some(PsctlRt0ip::PsctlRt0ipDiv256),
            _ => None,
        }
    }
    #[doc = "DIV8"]
    #[inline(always)]
    pub fn is_psctl_rt0ip_div8(&self) -> bool {
        *self == PsctlRt0ip::PsctlRt0ipDiv8
    }
    #[doc = "DIV16"]
    #[inline(always)]
    pub fn is_psctl_rt0ip_div16(&self) -> bool {
        *self == PsctlRt0ip::PsctlRt0ipDiv16
    }
    #[doc = "DIV32"]
    #[inline(always)]
    pub fn is_psctl_rt0ip_div32(&self) -> bool {
        *self == PsctlRt0ip::PsctlRt0ipDiv32
    }
    #[doc = "DIV64"]
    #[inline(always)]
    pub fn is_psctl_rt0ip_div64(&self) -> bool {
        *self == PsctlRt0ip::PsctlRt0ipDiv64
    }
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn is_psctl_rt0ip_div128(&self) -> bool {
        *self == PsctlRt0ip::PsctlRt0ipDiv128
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn is_psctl_rt0ip_div256(&self) -> bool {
        *self == PsctlRt0ip::PsctlRt0ipDiv256
    }
}
#[doc = "Field `PSCTL_RT0IP` writer - Prescale timer 0 interrupt interval"]
pub type PsctlRt0ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, PsctlRt0ip>;
impl<'a, REG> PsctlRt0ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV8"]
    #[inline(always)]
    pub fn psctl_rt0ip_div8(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt0ip::PsctlRt0ipDiv8)
    }
    #[doc = "DIV16"]
    #[inline(always)]
    pub fn psctl_rt0ip_div16(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt0ip::PsctlRt0ipDiv16)
    }
    #[doc = "DIV32"]
    #[inline(always)]
    pub fn psctl_rt0ip_div32(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt0ip::PsctlRt0ipDiv32)
    }
    #[doc = "DIV64"]
    #[inline(always)]
    pub fn psctl_rt0ip_div64(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt0ip::PsctlRt0ipDiv64)
    }
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn psctl_rt0ip_div128(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt0ip::PsctlRt0ipDiv128)
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn psctl_rt0ip_div256(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt0ip::PsctlRt0ipDiv256)
    }
}
#[doc = "Prescale timer 1 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PsctlRt1ip {
    #[doc = "0: DIV2"]
    PsctlRt1ipDiv2 = 0,
    #[doc = "1: DIV4"]
    PsctlRt1ipDiv4 = 1,
    #[doc = "2: DIV8"]
    PsctlRt1ipDiv8 = 2,
    #[doc = "3: DIV16"]
    PsctlRt1ipDiv16 = 3,
    #[doc = "4: DIV32"]
    PsctlRt1ipDiv32 = 4,
    #[doc = "5: DIV64"]
    PsctlRt1ipDiv64 = 5,
    #[doc = "6: DIV128"]
    PsctlRt1ipDiv128 = 6,
    #[doc = "7: DIV256"]
    PsctlRt1ipDiv256 = 7,
}
impl From<PsctlRt1ip> for u8 {
    #[inline(always)]
    fn from(variant: PsctlRt1ip) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PsctlRt1ip {
    type Ux = u8;
}
impl crate::IsEnum for PsctlRt1ip {}
#[doc = "Field `PSCTL_RT1IP` reader - Prescale timer 1 interrupt interval"]
pub type PsctlRt1ipR = crate::FieldReader<PsctlRt1ip>;
impl PsctlRt1ipR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PsctlRt1ip {
        match self.bits {
            0 => PsctlRt1ip::PsctlRt1ipDiv2,
            1 => PsctlRt1ip::PsctlRt1ipDiv4,
            2 => PsctlRt1ip::PsctlRt1ipDiv8,
            3 => PsctlRt1ip::PsctlRt1ipDiv16,
            4 => PsctlRt1ip::PsctlRt1ipDiv32,
            5 => PsctlRt1ip::PsctlRt1ipDiv64,
            6 => PsctlRt1ip::PsctlRt1ipDiv128,
            7 => PsctlRt1ip::PsctlRt1ipDiv256,
            _ => unreachable!(),
        }
    }
    #[doc = "DIV2"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div2(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv2
    }
    #[doc = "DIV4"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div4(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv4
    }
    #[doc = "DIV8"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div8(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv8
    }
    #[doc = "DIV16"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div16(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv16
    }
    #[doc = "DIV32"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div32(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv32
    }
    #[doc = "DIV64"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div64(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv64
    }
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div128(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv128
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn is_psctl_rt1ip_div256(&self) -> bool {
        *self == PsctlRt1ip::PsctlRt1ipDiv256
    }
}
#[doc = "Field `PSCTL_RT1IP` writer - Prescale timer 1 interrupt interval"]
pub type PsctlRt1ipW<'a, REG> = crate::FieldWriter<'a, REG, 3, PsctlRt1ip, crate::Safe>;
impl<'a, REG> PsctlRt1ipW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DIV2"]
    #[inline(always)]
    pub fn psctl_rt1ip_div2(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv2)
    }
    #[doc = "DIV4"]
    #[inline(always)]
    pub fn psctl_rt1ip_div4(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv4)
    }
    #[doc = "DIV8"]
    #[inline(always)]
    pub fn psctl_rt1ip_div8(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv8)
    }
    #[doc = "DIV16"]
    #[inline(always)]
    pub fn psctl_rt1ip_div16(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv16)
    }
    #[doc = "DIV32"]
    #[inline(always)]
    pub fn psctl_rt1ip_div32(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv32)
    }
    #[doc = "DIV64"]
    #[inline(always)]
    pub fn psctl_rt1ip_div64(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv64)
    }
    #[doc = "DIV128"]
    #[inline(always)]
    pub fn psctl_rt1ip_div128(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv128)
    }
    #[doc = "DIV256"]
    #[inline(always)]
    pub fn psctl_rt1ip_div256(self) -> &'a mut crate::W<REG> {
        self.variant(PsctlRt1ip::PsctlRt1ipDiv256)
    }
}
impl R {
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn psctl_rt0ip(&self) -> PsctlRt0ipR {
        PsctlRt0ipR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn psctl_rt1ip(&self) -> PsctlRt1ipR {
        PsctlRt1ipR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 2:4 - Prescale timer 0 interrupt interval"]
    #[inline(always)]
    pub fn psctl_rt0ip(&mut self) -> PsctlRt0ipW<PsctlSpec> {
        PsctlRt0ipW::new(self, 2)
    }
    #[doc = "Bits 18:20 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn psctl_rt1ip(&mut self) -> PsctlRt1ipW<PsctlSpec> {
        PsctlRt1ipW::new(self, 18)
    }
}
#[doc = "RTC Prescale Timer 0/1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsctlSpec;
impl crate::RegisterSpec for PsctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psctl::R`](R) reader structure"]
impl crate::Readable for PsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`psctl::W`](W) writer structure"]
impl crate::Writable for PsctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSCTL to value 0x08"]
impl crate::Resettable for PsctlSpec {
    const RESET_VALUE: u32 = 0x08;
}

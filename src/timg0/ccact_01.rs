#[doc = "Register `CCACT_01[%s]` reader"]
pub type R = crate::R<Ccact01Spec>;
#[doc = "Register `CCACT_01[%s]` writer"]
pub type W = crate::W<Ccact01Spec>;
#[doc = "CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Zact {
    #[doc = "0: DISABLED"]
    Ccact01ZactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01ZactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01ZactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact01ZactCcpToggle = 3,
}
impl From<Ccact01Zact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Zact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Zact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Zact {}
#[doc = "Field `CCACT_01_ZACT` reader - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
pub type Ccact01ZactR = crate::FieldReader<Ccact01Zact>;
impl Ccact01ZactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact01Zact {
        match self.bits {
            0 => Ccact01Zact::Ccact01ZactDisabled,
            1 => Ccact01Zact::Ccact01ZactCcpHigh,
            2 => Ccact01Zact::Ccact01ZactCcpLow,
            3 => Ccact01Zact::Ccact01ZactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_zact_disabled(&self) -> bool {
        *self == Ccact01Zact::Ccact01ZactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_zact_ccp_high(&self) -> bool {
        *self == Ccact01Zact::Ccact01ZactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_zact_ccp_low(&self) -> bool {
        *self == Ccact01Zact::Ccact01ZactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_01_zact_ccp_toggle(&self) -> bool {
        *self == Ccact01Zact::Ccact01ZactCcpToggle
    }
}
#[doc = "Field `CCACT_01_ZACT` writer - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
pub type Ccact01ZactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Zact, crate::Safe>;
impl<'a, REG> Ccact01ZactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_zact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Zact::Ccact01ZactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_zact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Zact::Ccact01ZactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_zact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Zact::Ccact01ZactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_01_zact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Zact::Ccact01ZactCcpToggle)
    }
}
#[doc = "CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Lact {
    #[doc = "0: DISABLED"]
    Ccact01LactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01LactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01LactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact01LactCcpToggle = 3,
}
impl From<Ccact01Lact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Lact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Lact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Lact {}
#[doc = "Field `CCACT_01_LACT` reader - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
pub type Ccact01LactR = crate::FieldReader<Ccact01Lact>;
impl Ccact01LactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact01Lact {
        match self.bits {
            0 => Ccact01Lact::Ccact01LactDisabled,
            1 => Ccact01Lact::Ccact01LactCcpHigh,
            2 => Ccact01Lact::Ccact01LactCcpLow,
            3 => Ccact01Lact::Ccact01LactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_lact_disabled(&self) -> bool {
        *self == Ccact01Lact::Ccact01LactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_lact_ccp_high(&self) -> bool {
        *self == Ccact01Lact::Ccact01LactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_lact_ccp_low(&self) -> bool {
        *self == Ccact01Lact::Ccact01LactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_01_lact_ccp_toggle(&self) -> bool {
        *self == Ccact01Lact::Ccact01LactCcpToggle
    }
}
#[doc = "Field `CCACT_01_LACT` writer - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
pub type Ccact01LactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Lact, crate::Safe>;
impl<'a, REG> Ccact01LactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_lact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Lact::Ccact01LactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_lact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Lact::Ccact01LactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_lact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Lact::Ccact01LactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_01_lact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Lact::Ccact01LactCcpToggle)
    }
}
#[doc = "CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Cdact {
    #[doc = "0: DISABLED"]
    Ccact01CdactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01CdactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01CdactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact01CdactCcpToggle = 3,
}
impl From<Ccact01Cdact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Cdact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Cdact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Cdact {}
#[doc = "Field `CCACT_01_CDACT` reader - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
pub type Ccact01CdactR = crate::FieldReader<Ccact01Cdact>;
impl Ccact01CdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact01Cdact {
        match self.bits {
            0 => Ccact01Cdact::Ccact01CdactDisabled,
            1 => Ccact01Cdact::Ccact01CdactCcpHigh,
            2 => Ccact01Cdact::Ccact01CdactCcpLow,
            3 => Ccact01Cdact::Ccact01CdactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_cdact_disabled(&self) -> bool {
        *self == Ccact01Cdact::Ccact01CdactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_cdact_ccp_high(&self) -> bool {
        *self == Ccact01Cdact::Ccact01CdactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_cdact_ccp_low(&self) -> bool {
        *self == Ccact01Cdact::Ccact01CdactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_01_cdact_ccp_toggle(&self) -> bool {
        *self == Ccact01Cdact::Ccact01CdactCcpToggle
    }
}
#[doc = "Field `CCACT_01_CDACT` writer - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
pub type Ccact01CdactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Cdact, crate::Safe>;
impl<'a, REG> Ccact01CdactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_cdact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cdact::Ccact01CdactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_cdact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cdact::Ccact01CdactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_cdact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cdact::Ccact01CdactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_01_cdact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cdact::Ccact01CdactCcpToggle)
    }
}
#[doc = "CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Cuact {
    #[doc = "0: DISABLED"]
    Ccact01CuactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01CuactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01CuactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact01CuactCcpToggle = 3,
}
impl From<Ccact01Cuact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Cuact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Cuact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Cuact {}
#[doc = "Field `CCACT_01_CUACT` reader - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
pub type Ccact01CuactR = crate::FieldReader<Ccact01Cuact>;
impl Ccact01CuactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact01Cuact {
        match self.bits {
            0 => Ccact01Cuact::Ccact01CuactDisabled,
            1 => Ccact01Cuact::Ccact01CuactCcpHigh,
            2 => Ccact01Cuact::Ccact01CuactCcpLow,
            3 => Ccact01Cuact::Ccact01CuactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_cuact_disabled(&self) -> bool {
        *self == Ccact01Cuact::Ccact01CuactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_cuact_ccp_high(&self) -> bool {
        *self == Ccact01Cuact::Ccact01CuactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_cuact_ccp_low(&self) -> bool {
        *self == Ccact01Cuact::Ccact01CuactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_01_cuact_ccp_toggle(&self) -> bool {
        *self == Ccact01Cuact::Ccact01CuactCcpToggle
    }
}
#[doc = "Field `CCACT_01_CUACT` writer - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
pub type Ccact01CuactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Cuact, crate::Safe>;
impl<'a, REG> Ccact01CuactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_cuact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cuact::Ccact01CuactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_cuact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cuact::Ccact01CuactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_cuact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cuact::Ccact01CuactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_01_cuact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cuact::Ccact01CuactCcpToggle)
    }
}
#[doc = "CCP Output Action on CC2D event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Cc2dact {
    #[doc = "0: DISABLED"]
    Ccact01Cc2dactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01Cc2dactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01Cc2dactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact01Cc2dactCcpToggle = 3,
}
impl From<Ccact01Cc2dact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Cc2dact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Cc2dact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Cc2dact {}
#[doc = "Field `CCACT_01_CC2DACT` reader - CCP Output Action on CC2D event."]
pub type Ccact01Cc2dactR = crate::FieldReader<Ccact01Cc2dact>;
impl Ccact01Cc2dactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact01Cc2dact {
        match self.bits {
            0 => Ccact01Cc2dact::Ccact01Cc2dactDisabled,
            1 => Ccact01Cc2dact::Ccact01Cc2dactCcpHigh,
            2 => Ccact01Cc2dact::Ccact01Cc2dactCcpLow,
            3 => Ccact01Cc2dact::Ccact01Cc2dactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_cc2dact_disabled(&self) -> bool {
        *self == Ccact01Cc2dact::Ccact01Cc2dactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_cc2dact_ccp_high(&self) -> bool {
        *self == Ccact01Cc2dact::Ccact01Cc2dactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_cc2dact_ccp_low(&self) -> bool {
        *self == Ccact01Cc2dact::Ccact01Cc2dactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_01_cc2dact_ccp_toggle(&self) -> bool {
        *self == Ccact01Cc2dact::Ccact01Cc2dactCcpToggle
    }
}
#[doc = "Field `CCACT_01_CC2DACT` writer - CCP Output Action on CC2D event."]
pub type Ccact01Cc2dactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Cc2dact, crate::Safe>;
impl<'a, REG> Ccact01Cc2dactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_cc2dact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2dact::Ccact01Cc2dactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_cc2dact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2dact::Ccact01Cc2dactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_cc2dact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2dact::Ccact01Cc2dactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_01_cc2dact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2dact::Ccact01Cc2dactCcpToggle)
    }
}
#[doc = "CCP Output Action on CC2U event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Cc2uact {
    #[doc = "0: DISABLED"]
    Ccact01Cc2uactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01Cc2uactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01Cc2uactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact01Cc2uactCcpToggle = 3,
}
impl From<Ccact01Cc2uact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Cc2uact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Cc2uact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Cc2uact {}
#[doc = "Field `CCACT_01_CC2UACT` reader - CCP Output Action on CC2U event."]
pub type Ccact01Cc2uactR = crate::FieldReader<Ccact01Cc2uact>;
impl Ccact01Cc2uactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact01Cc2uact {
        match self.bits {
            0 => Ccact01Cc2uact::Ccact01Cc2uactDisabled,
            1 => Ccact01Cc2uact::Ccact01Cc2uactCcpHigh,
            2 => Ccact01Cc2uact::Ccact01Cc2uactCcpLow,
            3 => Ccact01Cc2uact::Ccact01Cc2uactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_cc2uact_disabled(&self) -> bool {
        *self == Ccact01Cc2uact::Ccact01Cc2uactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_cc2uact_ccp_high(&self) -> bool {
        *self == Ccact01Cc2uact::Ccact01Cc2uactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_cc2uact_ccp_low(&self) -> bool {
        *self == Ccact01Cc2uact::Ccact01Cc2uactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_01_cc2uact_ccp_toggle(&self) -> bool {
        *self == Ccact01Cc2uact::Ccact01Cc2uactCcpToggle
    }
}
#[doc = "Field `CCACT_01_CC2UACT` writer - CCP Output Action on CC2U event."]
pub type Ccact01Cc2uactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Cc2uact, crate::Safe>;
impl<'a, REG> Ccact01Cc2uactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_cc2uact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2uact::Ccact01Cc2uactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_cc2uact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2uact::Ccact01Cc2uactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_cc2uact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2uact::Ccact01Cc2uactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_01_cc2uact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Cc2uact::Ccact01Cc2uactCcpToggle)
    }
}
#[doc = "CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact01Swfrcact {
    #[doc = "0: DISABLED"]
    Ccact01SwfrcactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact01SwfrcactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact01SwfrcactCcpLow = 2,
}
impl From<Ccact01Swfrcact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact01Swfrcact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact01Swfrcact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact01Swfrcact {}
#[doc = "Field `CCACT_01_SWFRCACT` reader - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type Ccact01SwfrcactR = crate::FieldReader<Ccact01Swfrcact>;
impl Ccact01SwfrcactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccact01Swfrcact> {
        match self.bits {
            0 => Some(Ccact01Swfrcact::Ccact01SwfrcactDisabled),
            1 => Some(Ccact01Swfrcact::Ccact01SwfrcactCcpHigh),
            2 => Some(Ccact01Swfrcact::Ccact01SwfrcactCcpLow),
            _ => None,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_01_swfrcact_disabled(&self) -> bool {
        *self == Ccact01Swfrcact::Ccact01SwfrcactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_01_swfrcact_ccp_high(&self) -> bool {
        *self == Ccact01Swfrcact::Ccact01SwfrcactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_01_swfrcact_ccp_low(&self) -> bool {
        *self == Ccact01Swfrcact::Ccact01SwfrcactCcpLow
    }
}
#[doc = "Field `CCACT_01_SWFRCACT` writer - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type Ccact01SwfrcactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact01Swfrcact>;
impl<'a, REG> Ccact01SwfrcactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_01_swfrcact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Swfrcact::Ccact01SwfrcactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_01_swfrcact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Swfrcact::Ccact01SwfrcactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_01_swfrcact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact01Swfrcact::Ccact01SwfrcactCcpLow)
    }
}
impl R {
    #[doc = "Bits 0:1 - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
    #[inline(always)]
    pub fn ccact_01_zact(&self) -> Ccact01ZactR {
        Ccact01ZactR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
    #[inline(always)]
    pub fn ccact_01_lact(&self) -> Ccact01LactR {
        Ccact01LactR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
    #[inline(always)]
    pub fn ccact_01_cdact(&self) -> Ccact01CdactR {
        Ccact01CdactR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:10 - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
    #[inline(always)]
    pub fn ccact_01_cuact(&self) -> Ccact01CuactR {
        Ccact01CuactR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CCP Output Action on CC2D event."]
    #[inline(always)]
    pub fn ccact_01_cc2dact(&self) -> Ccact01Cc2dactR {
        Ccact01Cc2dactR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 15:16 - CCP Output Action on CC2U event."]
    #[inline(always)]
    pub fn ccact_01_cc2uact(&self) -> Ccact01Cc2uactR {
        Ccact01Cc2uactR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 28:29 - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn ccact_01_swfrcact(&self) -> Ccact01SwfrcactR {
        Ccact01SwfrcactR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
    #[inline(always)]
    pub fn ccact_01_zact(&mut self) -> Ccact01ZactW<Ccact01Spec> {
        Ccact01ZactW::new(self, 0)
    }
    #[doc = "Bits 3:4 - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
    #[inline(always)]
    pub fn ccact_01_lact(&mut self) -> Ccact01LactW<Ccact01Spec> {
        Ccact01LactW::new(self, 3)
    }
    #[doc = "Bits 6:7 - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
    #[inline(always)]
    pub fn ccact_01_cdact(&mut self) -> Ccact01CdactW<Ccact01Spec> {
        Ccact01CdactW::new(self, 6)
    }
    #[doc = "Bits 9:10 - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
    #[inline(always)]
    pub fn ccact_01_cuact(&mut self) -> Ccact01CuactW<Ccact01Spec> {
        Ccact01CuactW::new(self, 9)
    }
    #[doc = "Bits 12:13 - CCP Output Action on CC2D event."]
    #[inline(always)]
    pub fn ccact_01_cc2dact(&mut self) -> Ccact01Cc2dactW<Ccact01Spec> {
        Ccact01Cc2dactW::new(self, 12)
    }
    #[doc = "Bits 15:16 - CCP Output Action on CC2U event."]
    #[inline(always)]
    pub fn ccact_01_cc2uact(&mut self) -> Ccact01Cc2uactW<Ccact01Spec> {
        Ccact01Cc2uactW::new(self, 15)
    }
    #[doc = "Bits 28:29 - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn ccact_01_swfrcact(&mut self) -> Ccact01SwfrcactW<Ccact01Spec> {
        Ccact01SwfrcactW::new(self, 28)
    }
}
#[doc = "Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccact_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccact_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccact01Spec;
impl crate::RegisterSpec for Ccact01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccact_01::R`](R) reader structure"]
impl crate::Readable for Ccact01Spec {}
#[doc = "`write(|w| ..)` method takes [`ccact_01::W`](W) writer structure"]
impl crate::Writable for Ccact01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCACT_01[%s]
to value 0"]
impl crate::Resettable for Ccact01Spec {
    const RESET_VALUE: u32 = 0;
}

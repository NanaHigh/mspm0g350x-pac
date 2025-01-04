#[doc = "Register `CCACT_23[%s]` reader"]
pub type R = crate::R<Ccact23Spec>;
#[doc = "Register `CCACT_23[%s]` writer"]
pub type W = crate::W<Ccact23Spec>;
#[doc = "CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Zact {
    #[doc = "0: DISABLED"]
    Ccact23ZactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23ZactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23ZactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23ZactCcpToggle = 3,
}
impl From<Ccact23Zact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Zact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Zact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Zact {}
#[doc = "Field `CCACT_23_ZACT` reader - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
pub type Ccact23ZactR = crate::FieldReader<Ccact23Zact>;
impl Ccact23ZactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact23Zact {
        match self.bits {
            0 => Ccact23Zact::Ccact23ZactDisabled,
            1 => Ccact23Zact::Ccact23ZactCcpHigh,
            2 => Ccact23Zact::Ccact23ZactCcpLow,
            3 => Ccact23Zact::Ccact23ZactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_zact_disabled(&self) -> bool {
        *self == Ccact23Zact::Ccact23ZactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_zact_ccp_high(&self) -> bool {
        *self == Ccact23Zact::Ccact23ZactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_zact_ccp_low(&self) -> bool {
        *self == Ccact23Zact::Ccact23ZactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_zact_ccp_toggle(&self) -> bool {
        *self == Ccact23Zact::Ccact23ZactCcpToggle
    }
}
#[doc = "Field `CCACT_23_ZACT` writer - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
pub type Ccact23ZactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Zact, crate::Safe>;
impl<'a, REG> Ccact23ZactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_zact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Zact::Ccact23ZactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_zact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Zact::Ccact23ZactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_zact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Zact::Ccact23ZactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_zact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Zact::Ccact23ZactCcpToggle)
    }
}
#[doc = "CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Lact {
    #[doc = "0: DISABLED"]
    Ccact23LactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23LactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23LactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23LactCcpToggle = 3,
}
impl From<Ccact23Lact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Lact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Lact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Lact {}
#[doc = "Field `CCACT_23_LACT` reader - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
pub type Ccact23LactR = crate::FieldReader<Ccact23Lact>;
impl Ccact23LactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact23Lact {
        match self.bits {
            0 => Ccact23Lact::Ccact23LactDisabled,
            1 => Ccact23Lact::Ccact23LactCcpHigh,
            2 => Ccact23Lact::Ccact23LactCcpLow,
            3 => Ccact23Lact::Ccact23LactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_lact_disabled(&self) -> bool {
        *self == Ccact23Lact::Ccact23LactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_lact_ccp_high(&self) -> bool {
        *self == Ccact23Lact::Ccact23LactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_lact_ccp_low(&self) -> bool {
        *self == Ccact23Lact::Ccact23LactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_lact_ccp_toggle(&self) -> bool {
        *self == Ccact23Lact::Ccact23LactCcpToggle
    }
}
#[doc = "Field `CCACT_23_LACT` writer - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
pub type Ccact23LactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Lact, crate::Safe>;
impl<'a, REG> Ccact23LactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_lact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Lact::Ccact23LactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_lact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Lact::Ccact23LactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_lact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Lact::Ccact23LactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_lact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Lact::Ccact23LactCcpToggle)
    }
}
#[doc = "CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Cdact {
    #[doc = "0: DISABLED"]
    Ccact23CdactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23CdactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23CdactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23CdactCcpToggle = 3,
}
impl From<Ccact23Cdact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Cdact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Cdact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Cdact {}
#[doc = "Field `CCACT_23_CDACT` reader - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
pub type Ccact23CdactR = crate::FieldReader<Ccact23Cdact>;
impl Ccact23CdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact23Cdact {
        match self.bits {
            0 => Ccact23Cdact::Ccact23CdactDisabled,
            1 => Ccact23Cdact::Ccact23CdactCcpHigh,
            2 => Ccact23Cdact::Ccact23CdactCcpLow,
            3 => Ccact23Cdact::Ccact23CdactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_cdact_disabled(&self) -> bool {
        *self == Ccact23Cdact::Ccact23CdactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_cdact_ccp_high(&self) -> bool {
        *self == Ccact23Cdact::Ccact23CdactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_cdact_ccp_low(&self) -> bool {
        *self == Ccact23Cdact::Ccact23CdactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_cdact_ccp_toggle(&self) -> bool {
        *self == Ccact23Cdact::Ccact23CdactCcpToggle
    }
}
#[doc = "Field `CCACT_23_CDACT` writer - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
pub type Ccact23CdactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Cdact, crate::Safe>;
impl<'a, REG> Ccact23CdactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_cdact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cdact::Ccact23CdactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_cdact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cdact::Ccact23CdactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_cdact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cdact::Ccact23CdactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_cdact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cdact::Ccact23CdactCcpToggle)
    }
}
#[doc = "CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Cuact {
    #[doc = "0: DISABLED"]
    Ccact23CuactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23CuactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23CuactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23CuactCcpToggle = 3,
}
impl From<Ccact23Cuact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Cuact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Cuact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Cuact {}
#[doc = "Field `CCACT_23_CUACT` reader - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
pub type Ccact23CuactR = crate::FieldReader<Ccact23Cuact>;
impl Ccact23CuactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact23Cuact {
        match self.bits {
            0 => Ccact23Cuact::Ccact23CuactDisabled,
            1 => Ccact23Cuact::Ccact23CuactCcpHigh,
            2 => Ccact23Cuact::Ccact23CuactCcpLow,
            3 => Ccact23Cuact::Ccact23CuactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_cuact_disabled(&self) -> bool {
        *self == Ccact23Cuact::Ccact23CuactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_cuact_ccp_high(&self) -> bool {
        *self == Ccact23Cuact::Ccact23CuactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_cuact_ccp_low(&self) -> bool {
        *self == Ccact23Cuact::Ccact23CuactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_cuact_ccp_toggle(&self) -> bool {
        *self == Ccact23Cuact::Ccact23CuactCcpToggle
    }
}
#[doc = "Field `CCACT_23_CUACT` writer - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
pub type Ccact23CuactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Cuact, crate::Safe>;
impl<'a, REG> Ccact23CuactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_cuact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cuact::Ccact23CuactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_cuact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cuact::Ccact23CuactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_cuact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cuact::Ccact23CuactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_cuact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cuact::Ccact23CuactCcpToggle)
    }
}
#[doc = "CCP Output Action on CC2D event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Cc2dact {
    #[doc = "0: DISABLED"]
    Ccact23Cc2dactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23Cc2dactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23Cc2dactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23Cc2dactCcpToggle = 3,
}
impl From<Ccact23Cc2dact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Cc2dact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Cc2dact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Cc2dact {}
#[doc = "Field `CCACT_23_CC2DACT` reader - CCP Output Action on CC2D event."]
pub type Ccact23Cc2dactR = crate::FieldReader<Ccact23Cc2dact>;
impl Ccact23Cc2dactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact23Cc2dact {
        match self.bits {
            0 => Ccact23Cc2dact::Ccact23Cc2dactDisabled,
            1 => Ccact23Cc2dact::Ccact23Cc2dactCcpHigh,
            2 => Ccact23Cc2dact::Ccact23Cc2dactCcpLow,
            3 => Ccact23Cc2dact::Ccact23Cc2dactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_cc2dact_disabled(&self) -> bool {
        *self == Ccact23Cc2dact::Ccact23Cc2dactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_cc2dact_ccp_high(&self) -> bool {
        *self == Ccact23Cc2dact::Ccact23Cc2dactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_cc2dact_ccp_low(&self) -> bool {
        *self == Ccact23Cc2dact::Ccact23Cc2dactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_cc2dact_ccp_toggle(&self) -> bool {
        *self == Ccact23Cc2dact::Ccact23Cc2dactCcpToggle
    }
}
#[doc = "Field `CCACT_23_CC2DACT` writer - CCP Output Action on CC2D event."]
pub type Ccact23Cc2dactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Cc2dact, crate::Safe>;
impl<'a, REG> Ccact23Cc2dactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_cc2dact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2dact::Ccact23Cc2dactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_cc2dact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2dact::Ccact23Cc2dactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_cc2dact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2dact::Ccact23Cc2dactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_cc2dact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2dact::Ccact23Cc2dactCcpToggle)
    }
}
#[doc = "CCP Output Action on CC2U event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Cc2uact {
    #[doc = "0: DISABLED"]
    Ccact23Cc2uactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23Cc2uactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23Cc2uactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23Cc2uactCcpToggle = 3,
}
impl From<Ccact23Cc2uact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Cc2uact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Cc2uact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Cc2uact {}
#[doc = "Field `CCACT_23_CC2UACT` reader - CCP Output Action on CC2U event."]
pub type Ccact23Cc2uactR = crate::FieldReader<Ccact23Cc2uact>;
impl Ccact23Cc2uactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccact23Cc2uact {
        match self.bits {
            0 => Ccact23Cc2uact::Ccact23Cc2uactDisabled,
            1 => Ccact23Cc2uact::Ccact23Cc2uactCcpHigh,
            2 => Ccact23Cc2uact::Ccact23Cc2uactCcpLow,
            3 => Ccact23Cc2uact::Ccact23Cc2uactCcpToggle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_cc2uact_disabled(&self) -> bool {
        *self == Ccact23Cc2uact::Ccact23Cc2uactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_cc2uact_ccp_high(&self) -> bool {
        *self == Ccact23Cc2uact::Ccact23Cc2uactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_cc2uact_ccp_low(&self) -> bool {
        *self == Ccact23Cc2uact::Ccact23Cc2uactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_cc2uact_ccp_toggle(&self) -> bool {
        *self == Ccact23Cc2uact::Ccact23Cc2uactCcpToggle
    }
}
#[doc = "Field `CCACT_23_CC2UACT` writer - CCP Output Action on CC2U event."]
pub type Ccact23Cc2uactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Cc2uact, crate::Safe>;
impl<'a, REG> Ccact23Cc2uactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_cc2uact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2uact::Ccact23Cc2uactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_cc2uact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2uact::Ccact23Cc2uactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_cc2uact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2uact::Ccact23Cc2uactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_cc2uact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Cc2uact::Ccact23Cc2uactCcpToggle)
    }
}
#[doc = "CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Fenact {
    #[doc = "0: DISABLED"]
    Ccact23FenactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23FenactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23FenactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23FenactCcpToggle = 3,
    #[doc = "4: CCP_HIGHZ"]
    Ccact23FenactCcpHighz = 4,
}
impl From<Ccact23Fenact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Fenact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Fenact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Fenact {}
#[doc = "Field `CCACT_23_FENACT` reader - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
pub type Ccact23FenactR = crate::FieldReader<Ccact23Fenact>;
impl Ccact23FenactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccact23Fenact> {
        match self.bits {
            0 => Some(Ccact23Fenact::Ccact23FenactDisabled),
            1 => Some(Ccact23Fenact::Ccact23FenactCcpHigh),
            2 => Some(Ccact23Fenact::Ccact23FenactCcpLow),
            3 => Some(Ccact23Fenact::Ccact23FenactCcpToggle),
            4 => Some(Ccact23Fenact::Ccact23FenactCcpHighz),
            _ => None,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_fenact_disabled(&self) -> bool {
        *self == Ccact23Fenact::Ccact23FenactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_fenact_ccp_high(&self) -> bool {
        *self == Ccact23Fenact::Ccact23FenactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_fenact_ccp_low(&self) -> bool {
        *self == Ccact23Fenact::Ccact23FenactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_fenact_ccp_toggle(&self) -> bool {
        *self == Ccact23Fenact::Ccact23FenactCcpToggle
    }
    #[doc = "CCP_HIGHZ"]
    #[inline(always)]
    pub fn is_ccact_23_fenact_ccp_highz(&self) -> bool {
        *self == Ccact23Fenact::Ccact23FenactCcpHighz
    }
}
#[doc = "Field `CCACT_23_FENACT` writer - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
pub type Ccact23FenactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccact23Fenact>;
impl<'a, REG> Ccact23FenactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_fenact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fenact::Ccact23FenactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_fenact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fenact::Ccact23FenactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_fenact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fenact::Ccact23FenactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_fenact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fenact::Ccact23FenactCcpToggle)
    }
    #[doc = "CCP_HIGHZ"]
    #[inline(always)]
    pub fn ccact_23_fenact_ccp_highz(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fenact::Ccact23FenactCcpHighz)
    }
}
#[doc = "CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Fexact {
    #[doc = "0: DISABLED"]
    Ccact23FexactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23FexactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23FexactCcpLow = 2,
    #[doc = "3: CCP_TOGGLE"]
    Ccact23FexactCcpToggle = 3,
    #[doc = "4: CCP_HIGHZ"]
    Ccact23FexactCcpHighz = 4,
}
impl From<Ccact23Fexact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Fexact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Fexact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Fexact {}
#[doc = "Field `CCACT_23_FEXACT` reader - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
pub type Ccact23FexactR = crate::FieldReader<Ccact23Fexact>;
impl Ccact23FexactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccact23Fexact> {
        match self.bits {
            0 => Some(Ccact23Fexact::Ccact23FexactDisabled),
            1 => Some(Ccact23Fexact::Ccact23FexactCcpHigh),
            2 => Some(Ccact23Fexact::Ccact23FexactCcpLow),
            3 => Some(Ccact23Fexact::Ccact23FexactCcpToggle),
            4 => Some(Ccact23Fexact::Ccact23FexactCcpHighz),
            _ => None,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_fexact_disabled(&self) -> bool {
        *self == Ccact23Fexact::Ccact23FexactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_fexact_ccp_high(&self) -> bool {
        *self == Ccact23Fexact::Ccact23FexactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_fexact_ccp_low(&self) -> bool {
        *self == Ccact23Fexact::Ccact23FexactCcpLow
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn is_ccact_23_fexact_ccp_toggle(&self) -> bool {
        *self == Ccact23Fexact::Ccact23FexactCcpToggle
    }
    #[doc = "CCP_HIGHZ"]
    #[inline(always)]
    pub fn is_ccact_23_fexact_ccp_highz(&self) -> bool {
        *self == Ccact23Fexact::Ccact23FexactCcpHighz
    }
}
#[doc = "Field `CCACT_23_FEXACT` writer - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
pub type Ccact23FexactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccact23Fexact>;
impl<'a, REG> Ccact23FexactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_fexact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fexact::Ccact23FexactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_fexact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fexact::Ccact23FexactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_fexact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fexact::Ccact23FexactCcpLow)
    }
    #[doc = "CCP_TOGGLE"]
    #[inline(always)]
    pub fn ccact_23_fexact_ccp_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fexact::Ccact23FexactCcpToggle)
    }
    #[doc = "CCP_HIGHZ"]
    #[inline(always)]
    pub fn ccact_23_fexact_ccp_highz(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Fexact::Ccact23FexactCcpHighz)
    }
}
#[doc = "CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23Swfrcact {
    #[doc = "0: DISABLED"]
    Ccact23SwfrcactDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23SwfrcactCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23SwfrcactCcpLow = 2,
}
impl From<Ccact23Swfrcact> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23Swfrcact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23Swfrcact {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23Swfrcact {}
#[doc = "Field `CCACT_23_SWFRCACT` reader - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type Ccact23SwfrcactR = crate::FieldReader<Ccact23Swfrcact>;
impl Ccact23SwfrcactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccact23Swfrcact> {
        match self.bits {
            0 => Some(Ccact23Swfrcact::Ccact23SwfrcactDisabled),
            1 => Some(Ccact23Swfrcact::Ccact23SwfrcactCcpHigh),
            2 => Some(Ccact23Swfrcact::Ccact23SwfrcactCcpLow),
            _ => None,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_swfrcact_disabled(&self) -> bool {
        *self == Ccact23Swfrcact::Ccact23SwfrcactDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_swfrcact_ccp_high(&self) -> bool {
        *self == Ccact23Swfrcact::Ccact23SwfrcactCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_swfrcact_ccp_low(&self) -> bool {
        *self == Ccact23Swfrcact::Ccact23SwfrcactCcpLow
    }
}
#[doc = "Field `CCACT_23_SWFRCACT` writer - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type Ccact23SwfrcactW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23Swfrcact>;
impl<'a, REG> Ccact23SwfrcactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_swfrcact_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Swfrcact::Ccact23SwfrcactDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_swfrcact_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Swfrcact::Ccact23SwfrcactCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_swfrcact_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23Swfrcact::Ccact23SwfrcactCcpLow)
    }
}
#[doc = "CCP_CMPL Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccact23SwfrcactCmpl {
    #[doc = "0: DISABLED"]
    Ccact23SwfrcactCmplDisabled = 0,
    #[doc = "1: CCP_HIGH"]
    Ccact23SwfrcactCmplCcpHigh = 1,
    #[doc = "2: CCP_LOW"]
    Ccact23SwfrcactCmplCcpLow = 2,
}
impl From<Ccact23SwfrcactCmpl> for u8 {
    #[inline(always)]
    fn from(variant: Ccact23SwfrcactCmpl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccact23SwfrcactCmpl {
    type Ux = u8;
}
impl crate::IsEnum for Ccact23SwfrcactCmpl {}
#[doc = "Field `CCACT_23_SWFRCACT_CMPL` reader - CCP_CMPL Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type Ccact23SwfrcactCmplR = crate::FieldReader<Ccact23SwfrcactCmpl>;
impl Ccact23SwfrcactCmplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccact23SwfrcactCmpl> {
        match self.bits {
            0 => Some(Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplDisabled),
            1 => Some(Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplCcpHigh),
            2 => Some(Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplCcpLow),
            _ => None,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccact_23_swfrcact_cmpl_disabled(&self) -> bool {
        *self == Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplDisabled
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn is_ccact_23_swfrcact_cmpl_ccp_high(&self) -> bool {
        *self == Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplCcpHigh
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn is_ccact_23_swfrcact_cmpl_ccp_low(&self) -> bool {
        *self == Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplCcpLow
    }
}
#[doc = "Field `CCACT_23_SWFRCACT_CMPL` writer - CCP_CMPL Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
pub type Ccact23SwfrcactCmplW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ccact23SwfrcactCmpl>;
impl<'a, REG> Ccact23SwfrcactCmplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccact_23_swfrcact_cmpl_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplDisabled)
    }
    #[doc = "CCP_HIGH"]
    #[inline(always)]
    pub fn ccact_23_swfrcact_cmpl_ccp_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplCcpHigh)
    }
    #[doc = "CCP_LOW"]
    #[inline(always)]
    pub fn ccact_23_swfrcact_cmpl_ccp_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ccact23SwfrcactCmpl::Ccact23SwfrcactCmplCcpLow)
    }
}
impl R {
    #[doc = "Bits 0:1 - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
    #[inline(always)]
    pub fn ccact_23_zact(&self) -> Ccact23ZactR {
        Ccact23ZactR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
    #[inline(always)]
    pub fn ccact_23_lact(&self) -> Ccact23LactR {
        Ccact23LactR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
    #[inline(always)]
    pub fn ccact_23_cdact(&self) -> Ccact23CdactR {
        Ccact23CdactR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 9:10 - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
    #[inline(always)]
    pub fn ccact_23_cuact(&self) -> Ccact23CuactR {
        Ccact23CuactR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - CCP Output Action on CC2D event."]
    #[inline(always)]
    pub fn ccact_23_cc2dact(&self) -> Ccact23Cc2dactR {
        Ccact23Cc2dactR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 15:16 - CCP Output Action on CC2U event."]
    #[inline(always)]
    pub fn ccact_23_cc2uact(&self) -> Ccact23Cc2uactR {
        Ccact23Cc2uactR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 22:24 - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
    #[inline(always)]
    pub fn ccact_23_fenact(&self) -> Ccact23FenactR {
        Ccact23FenactR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
    #[inline(always)]
    pub fn ccact_23_fexact(&self) -> Ccact23FexactR {
        Ccact23FexactR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn ccact_23_swfrcact(&self) -> Ccact23SwfrcactR {
        Ccact23SwfrcactR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - CCP_CMPL Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn ccact_23_swfrcact_cmpl(&self) -> Ccact23SwfrcactCmplR {
        Ccact23SwfrcactCmplR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CCP Output Action on Zero Specifies what changes occur to CCP output as the result of a zero event."]
    #[inline(always)]
    pub fn ccact_23_zact(&mut self) -> Ccact23ZactW<Ccact23Spec> {
        Ccact23ZactW::new(self, 0)
    }
    #[doc = "Bits 3:4 - CCP Output Action on Load Specifies what changes occur to CCP output as the result of a load event."]
    #[inline(always)]
    pub fn ccact_23_lact(&mut self) -> Ccact23LactW<Ccact23Spec> {
        Ccact23LactW::new(self, 3)
    }
    #[doc = "Bits 6:7 - CCP Output Action on Compare (Down) This field describes the resulting action of the signal generator upon detecting a compare event while counting down."]
    #[inline(always)]
    pub fn ccact_23_cdact(&mut self) -> Ccact23CdactW<Ccact23Spec> {
        Ccact23CdactW::new(self, 6)
    }
    #[doc = "Bits 9:10 - CCP Output Action on Compare (Up) This field describes the resulting action of the signal generator upon detecting a compare event while counting up."]
    #[inline(always)]
    pub fn ccact_23_cuact(&mut self) -> Ccact23CuactW<Ccact23Spec> {
        Ccact23CuactW::new(self, 9)
    }
    #[doc = "Bits 12:13 - CCP Output Action on CC2D event."]
    #[inline(always)]
    pub fn ccact_23_cc2dact(&mut self) -> Ccact23Cc2dactW<Ccact23Spec> {
        Ccact23Cc2dactW::new(self, 12)
    }
    #[doc = "Bits 15:16 - CCP Output Action on CC2U event."]
    #[inline(always)]
    pub fn ccact_23_cc2uact(&mut self) -> Ccact23Cc2uactW<Ccact23Spec> {
        Ccact23Cc2uactW::new(self, 15)
    }
    #[doc = "Bits 22:24 - CCP Output Action on Fault Entry This field describes the resulting action of the signal generator upon detecting a fault."]
    #[inline(always)]
    pub fn ccact_23_fenact(&mut self) -> Ccact23FenactW<Ccact23Spec> {
        Ccact23FenactW::new(self, 22)
    }
    #[doc = "Bits 25:27 - CCP Output Action on Fault Exit This field describes the resulting action of the signal generator upon exiting the fault condition."]
    #[inline(always)]
    pub fn ccact_23_fexact(&mut self) -> Ccact23FexactW<Ccact23Spec> {
        Ccact23FexactW::new(self, 25)
    }
    #[doc = "Bits 28:29 - CCP Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn ccact_23_swfrcact(&mut self) -> Ccact23SwfrcactW<Ccact23Spec> {
        Ccact23SwfrcactW::new(self, 28)
    }
    #[doc = "Bits 30:31 - CCP_CMPL Output Action on Software Froce Output This field describes the resulting action of software force. This action has a shadow register, which will be updated under specific condition. So that this register cannot take into effect immediately."]
    #[inline(always)]
    pub fn ccact_23_swfrcact_cmpl(&mut self) -> Ccact23SwfrcactCmplW<Ccact23Spec> {
        Ccact23SwfrcactCmplW::new(self, 30)
    }
}
#[doc = "Capture or Compare Action Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccact_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccact_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccact23Spec;
impl crate::RegisterSpec for Ccact23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccact_23::R`](R) reader structure"]
impl crate::Readable for Ccact23Spec {}
#[doc = "`write(|w| ..)` method takes [`ccact_23::W`](W) writer structure"]
impl crate::Writable for Ccact23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCACT_23[%s]
to value 0"]
impl crate::Resettable for Ccact23Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CCCTL_23[%s]` reader"]
pub type R = crate::R<Ccctl23Spec>;
#[doc = "Register `CCCTL_23[%s]` writer"]
pub type W = crate::W<Ccctl23Spec>;
#[doc = "Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Ccond {
    #[doc = "0: NOCAPTURE"]
    Ccctl23CcondNocapture = 0,
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl23CcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl23CcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl23CcondCcTrigEdge = 3,
}
impl From<Ccctl23Ccond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Ccond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Ccond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Ccond {}
#[doc = "Field `CCCTL_23_CCOND` reader - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
pub type Ccctl23CcondR = crate::FieldReader<Ccctl23Ccond>;
impl Ccctl23CcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Ccond> {
        match self.bits {
            0 => Some(Ccctl23Ccond::Ccctl23CcondNocapture),
            1 => Some(Ccctl23Ccond::Ccctl23CcondCcTrigRise),
            2 => Some(Ccctl23Ccond::Ccctl23CcondCcTrigFall),
            3 => Some(Ccctl23Ccond::Ccctl23CcondCcTrigEdge),
            _ => None,
        }
    }
    #[doc = "NOCAPTURE"]
    #[inline(always)]
    pub fn is_ccctl_23_ccond_nocapture(&self) -> bool {
        *self == Ccctl23Ccond::Ccctl23CcondNocapture
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_23_ccond_cc_trig_rise(&self) -> bool {
        *self == Ccctl23Ccond::Ccctl23CcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_23_ccond_cc_trig_fall(&self) -> bool {
        *self == Ccctl23Ccond::Ccctl23CcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_23_ccond_cc_trig_edge(&self) -> bool {
        *self == Ccctl23Ccond::Ccctl23CcondCcTrigEdge
    }
}
#[doc = "Field `CCCTL_23_CCOND` writer - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
pub type Ccctl23CcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Ccond>;
impl<'a, REG> Ccctl23CcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOCAPTURE"]
    #[inline(always)]
    pub fn ccctl_23_ccond_nocapture(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccond::Ccctl23CcondNocapture)
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_23_ccond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccond::Ccctl23CcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_23_ccond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccond::Ccctl23CcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_23_ccond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccond::Ccctl23CcondCcTrigEdge)
    }
}
#[doc = "Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Acond {
    #[doc = "0: TIMCLK"]
    Ccctl23AcondTimclk = 0,
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl23AcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl23AcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl23AcondCcTrigEdge = 3,
    #[doc = "5: CC_TRIG_HIGH"]
    Ccctl23AcondCcTrigHigh = 5,
}
impl From<Ccctl23Acond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Acond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Acond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Acond {}
#[doc = "Field `CCCTL_23_ACOND` reader - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
pub type Ccctl23AcondR = crate::FieldReader<Ccctl23Acond>;
impl Ccctl23AcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Acond> {
        match self.bits {
            0 => Some(Ccctl23Acond::Ccctl23AcondTimclk),
            1 => Some(Ccctl23Acond::Ccctl23AcondCcTrigRise),
            2 => Some(Ccctl23Acond::Ccctl23AcondCcTrigFall),
            3 => Some(Ccctl23Acond::Ccctl23AcondCcTrigEdge),
            5 => Some(Ccctl23Acond::Ccctl23AcondCcTrigHigh),
            _ => None,
        }
    }
    #[doc = "TIMCLK"]
    #[inline(always)]
    pub fn is_ccctl_23_acond_timclk(&self) -> bool {
        *self == Ccctl23Acond::Ccctl23AcondTimclk
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_23_acond_cc_trig_rise(&self) -> bool {
        *self == Ccctl23Acond::Ccctl23AcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_23_acond_cc_trig_fall(&self) -> bool {
        *self == Ccctl23Acond::Ccctl23AcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_23_acond_cc_trig_edge(&self) -> bool {
        *self == Ccctl23Acond::Ccctl23AcondCcTrigEdge
    }
    #[doc = "CC_TRIG_HIGH"]
    #[inline(always)]
    pub fn is_ccctl_23_acond_cc_trig_high(&self) -> bool {
        *self == Ccctl23Acond::Ccctl23AcondCcTrigHigh
    }
}
#[doc = "Field `CCCTL_23_ACOND` writer - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
pub type Ccctl23AcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Acond>;
impl<'a, REG> Ccctl23AcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIMCLK"]
    #[inline(always)]
    pub fn ccctl_23_acond_timclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Acond::Ccctl23AcondTimclk)
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_23_acond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Acond::Ccctl23AcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_23_acond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Acond::Ccctl23AcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_23_acond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Acond::Ccctl23AcondCcTrigEdge)
    }
    #[doc = "CC_TRIG_HIGH"]
    #[inline(always)]
    pub fn ccctl_23_acond_cc_trig_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Acond::Ccctl23AcondCcTrigHigh)
    }
}
#[doc = "Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Lcond {
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl23LcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl23LcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl23LcondCcTrigEdge = 3,
}
impl From<Ccctl23Lcond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Lcond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Lcond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Lcond {}
#[doc = "Field `CCCTL_23_LCOND` reader - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
pub type Ccctl23LcondR = crate::FieldReader<Ccctl23Lcond>;
impl Ccctl23LcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Lcond> {
        match self.bits {
            1 => Some(Ccctl23Lcond::Ccctl23LcondCcTrigRise),
            2 => Some(Ccctl23Lcond::Ccctl23LcondCcTrigFall),
            3 => Some(Ccctl23Lcond::Ccctl23LcondCcTrigEdge),
            _ => None,
        }
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_23_lcond_cc_trig_rise(&self) -> bool {
        *self == Ccctl23Lcond::Ccctl23LcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_23_lcond_cc_trig_fall(&self) -> bool {
        *self == Ccctl23Lcond::Ccctl23LcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_23_lcond_cc_trig_edge(&self) -> bool {
        *self == Ccctl23Lcond::Ccctl23LcondCcTrigEdge
    }
}
#[doc = "Field `CCCTL_23_LCOND` writer - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
pub type Ccctl23LcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Lcond>;
impl<'a, REG> Ccctl23LcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_23_lcond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Lcond::Ccctl23LcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_23_lcond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Lcond::Ccctl23LcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_23_lcond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Lcond::Ccctl23LcondCcTrigEdge)
    }
}
#[doc = "Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Zcond {
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl23ZcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl23ZcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl23ZcondCcTrigEdge = 3,
}
impl From<Ccctl23Zcond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Zcond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Zcond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Zcond {}
#[doc = "Field `CCCTL_23_ZCOND` reader - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
pub type Ccctl23ZcondR = crate::FieldReader<Ccctl23Zcond>;
impl Ccctl23ZcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Zcond> {
        match self.bits {
            1 => Some(Ccctl23Zcond::Ccctl23ZcondCcTrigRise),
            2 => Some(Ccctl23Zcond::Ccctl23ZcondCcTrigFall),
            3 => Some(Ccctl23Zcond::Ccctl23ZcondCcTrigEdge),
            _ => None,
        }
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_23_zcond_cc_trig_rise(&self) -> bool {
        *self == Ccctl23Zcond::Ccctl23ZcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_23_zcond_cc_trig_fall(&self) -> bool {
        *self == Ccctl23Zcond::Ccctl23ZcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_23_zcond_cc_trig_edge(&self) -> bool {
        *self == Ccctl23Zcond::Ccctl23ZcondCcTrigEdge
    }
}
#[doc = "Field `CCCTL_23_ZCOND` writer - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
pub type Ccctl23ZcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Zcond>;
impl<'a, REG> Ccctl23ZcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_23_zcond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Zcond::Ccctl23ZcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_23_zcond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Zcond::Ccctl23ZcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_23_zcond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Zcond::Ccctl23ZcondCcTrigEdge)
    }
}
#[doc = "Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccctl23Coc {
    #[doc = "0: COMPARE"]
    Ccctl23CocCompare = 0,
    #[doc = "1: CAPTURE"]
    Ccctl23CocCapture = 1,
}
impl From<Ccctl23Coc> for bool {
    #[inline(always)]
    fn from(variant: Ccctl23Coc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCTL_23_COC` reader - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
pub type Ccctl23CocR = crate::BitReader<Ccctl23Coc>;
impl Ccctl23CocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccctl23Coc {
        match self.bits {
            false => Ccctl23Coc::Ccctl23CocCompare,
            true => Ccctl23Coc::Ccctl23CocCapture,
        }
    }
    #[doc = "COMPARE"]
    #[inline(always)]
    pub fn is_ccctl_23_coc_compare(&self) -> bool {
        *self == Ccctl23Coc::Ccctl23CocCompare
    }
    #[doc = "CAPTURE"]
    #[inline(always)]
    pub fn is_ccctl_23_coc_capture(&self) -> bool {
        *self == Ccctl23Coc::Ccctl23CocCapture
    }
}
#[doc = "Field `CCCTL_23_COC` writer - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
pub type Ccctl23CocW<'a, REG> = crate::BitWriter<'a, REG, Ccctl23Coc>;
impl<'a, REG> Ccctl23CocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMPARE"]
    #[inline(always)]
    pub fn ccctl_23_coc_compare(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Coc::Ccctl23CocCompare)
    }
    #[doc = "CAPTURE"]
    #[inline(always)]
    pub fn ccctl_23_coc_capture(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Coc::Ccctl23CocCapture)
    }
}
#[doc = "Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Ccupd {
    #[doc = "0: IMMEDIATELY"]
    Ccctl23CcupdImmediately = 0,
    #[doc = "1: ZERO_EVT"]
    Ccctl23CcupdZeroEvt = 1,
    #[doc = "2: COMPARE_DOWN_EVT"]
    Ccctl23CcupdCompareDownEvt = 2,
    #[doc = "3: COMPARE_UP_EVT"]
    Ccctl23CcupdCompareUpEvt = 3,
    #[doc = "4: ZERO_LOAD_EVT"]
    Ccctl23CcupdZeroLoadEvt = 4,
    #[doc = "5: ZERO_RC_ZERO_EVT"]
    Ccctl23CcupdZeroRcZeroEvt = 5,
    #[doc = "6: TRIG"]
    Ccctl23CcupdTrig = 6,
}
impl From<Ccctl23Ccupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Ccupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Ccupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Ccupd {}
#[doc = "Field `CCCTL_23_CCUPD` reader - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type Ccctl23CcupdR = crate::FieldReader<Ccctl23Ccupd>;
impl Ccctl23CcupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Ccupd> {
        match self.bits {
            0 => Some(Ccctl23Ccupd::Ccctl23CcupdImmediately),
            1 => Some(Ccctl23Ccupd::Ccctl23CcupdZeroEvt),
            2 => Some(Ccctl23Ccupd::Ccctl23CcupdCompareDownEvt),
            3 => Some(Ccctl23Ccupd::Ccctl23CcupdCompareUpEvt),
            4 => Some(Ccctl23Ccupd::Ccctl23CcupdZeroLoadEvt),
            5 => Some(Ccctl23Ccupd::Ccctl23CcupdZeroRcZeroEvt),
            6 => Some(Ccctl23Ccupd::Ccctl23CcupdTrig),
            _ => None,
        }
    }
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_immediately(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdImmediately
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_zero_evt(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdZeroEvt
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_compare_down_evt(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdCompareDownEvt
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_compare_up_evt(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdCompareUpEvt
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_zero_load_evt(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdZeroLoadEvt
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_zero_rc_zero_evt(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdZeroRcZeroEvt
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn is_ccctl_23_ccupd_trig(&self) -> bool {
        *self == Ccctl23Ccupd::Ccctl23CcupdTrig
    }
}
#[doc = "Field `CCCTL_23_CCUPD` writer - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type Ccctl23CcupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Ccupd>;
impl<'a, REG> Ccctl23CcupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdImmediately)
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdZeroEvt)
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdCompareDownEvt)
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdCompareUpEvt)
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdZeroLoadEvt)
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdZeroRcZeroEvt)
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn ccctl_23_ccupd_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccupd::Ccctl23CcupdTrig)
    }
}
#[doc = "Selects the source second CCU event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Cc2selu {
    #[doc = "0: SEL_CCU0"]
    Ccctl23Cc2seluSelCcu0 = 0,
    #[doc = "1: SEL_CCU1"]
    Ccctl23Cc2seluSelCcu1 = 1,
    #[doc = "2: SEL_CCU2"]
    Ccctl23Cc2seluSelCcu2 = 2,
    #[doc = "3: SEL_CCU3"]
    Ccctl23Cc2seluSelCcu3 = 3,
    #[doc = "4: SEL_CCU4"]
    Ccctl23Cc2seluSelCcu4 = 4,
    #[doc = "5: SEL_CCU5"]
    Ccctl23Cc2seluSelCcu5 = 5,
}
impl From<Ccctl23Cc2selu> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Cc2selu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Cc2selu {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Cc2selu {}
#[doc = "Field `CCCTL_23_CC2SELU` reader - Selects the source second CCU event."]
pub type Ccctl23Cc2seluR = crate::FieldReader<Ccctl23Cc2selu>;
impl Ccctl23Cc2seluR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Cc2selu> {
        match self.bits {
            0 => Some(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu0),
            1 => Some(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu1),
            2 => Some(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu2),
            3 => Some(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu3),
            4 => Some(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu4),
            5 => Some(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu5),
            _ => None,
        }
    }
    #[doc = "SEL_CCU0"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2selu_sel_ccu0(&self) -> bool {
        *self == Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu0
    }
    #[doc = "SEL_CCU1"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2selu_sel_ccu1(&self) -> bool {
        *self == Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu1
    }
    #[doc = "SEL_CCU2"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2selu_sel_ccu2(&self) -> bool {
        *self == Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu2
    }
    #[doc = "SEL_CCU3"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2selu_sel_ccu3(&self) -> bool {
        *self == Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu3
    }
    #[doc = "SEL_CCU4"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2selu_sel_ccu4(&self) -> bool {
        *self == Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu4
    }
    #[doc = "SEL_CCU5"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2selu_sel_ccu5(&self) -> bool {
        *self == Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu5
    }
}
#[doc = "Field `CCCTL_23_CC2SELU` writer - Selects the source second CCU event."]
pub type Ccctl23Cc2seluW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Cc2selu>;
impl<'a, REG> Ccctl23Cc2seluW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SEL_CCU0"]
    #[inline(always)]
    pub fn ccctl_23_cc2selu_sel_ccu0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu0)
    }
    #[doc = "SEL_CCU1"]
    #[inline(always)]
    pub fn ccctl_23_cc2selu_sel_ccu1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu1)
    }
    #[doc = "SEL_CCU2"]
    #[inline(always)]
    pub fn ccctl_23_cc2selu_sel_ccu2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu2)
    }
    #[doc = "SEL_CCU3"]
    #[inline(always)]
    pub fn ccctl_23_cc2selu_sel_ccu3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu3)
    }
    #[doc = "SEL_CCU4"]
    #[inline(always)]
    pub fn ccctl_23_cc2selu_sel_ccu4(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu4)
    }
    #[doc = "SEL_CCU5"]
    #[inline(always)]
    pub fn ccctl_23_cc2selu_sel_ccu5(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2selu::Ccctl23Cc2seluSelCcu5)
    }
}
#[doc = "Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RCn) value is not 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccctl23Scercnez {
    #[doc = "0: DISABLED"]
    Ccctl23ScercnezDisabled = 0,
    #[doc = "1: ENABLED"]
    Ccctl23ScercnezEnabled = 1,
}
impl From<Ccctl23Scercnez> for bool {
    #[inline(always)]
    fn from(variant: Ccctl23Scercnez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCTL_23_SCERCNEZ` reader - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RCn) value is not 0."]
pub type Ccctl23ScercnezR = crate::BitReader<Ccctl23Scercnez>;
impl Ccctl23ScercnezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccctl23Scercnez {
        match self.bits {
            false => Ccctl23Scercnez::Ccctl23ScercnezDisabled,
            true => Ccctl23Scercnez::Ccctl23ScercnezEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccctl_23_scercnez_disabled(&self) -> bool {
        *self == Ccctl23Scercnez::Ccctl23ScercnezDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ccctl_23_scercnez_enabled(&self) -> bool {
        *self == Ccctl23Scercnez::Ccctl23ScercnezEnabled
    }
}
#[doc = "Field `CCCTL_23_SCERCNEZ` writer - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RCn) value is not 0."]
pub type Ccctl23ScercnezW<'a, REG> = crate::BitWriter<'a, REG, Ccctl23Scercnez>;
impl<'a, REG> Ccctl23ScercnezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccctl_23_scercnez_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Scercnez::Ccctl23ScercnezDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ccctl_23_scercnez_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Scercnez::Ccctl23ScercnezEnabled)
    }
}
#[doc = "CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Ccactupd {
    #[doc = "0: IMMEDIATELY"]
    Ccctl23CcactupdImmediately = 0,
    #[doc = "1: ZERO_EVT"]
    Ccctl23CcactupdZeroEvt = 1,
    #[doc = "2: COMPARE_DOWN_EVT"]
    Ccctl23CcactupdCompareDownEvt = 2,
    #[doc = "3: COMPARE_UP_EVT"]
    Ccctl23CcactupdCompareUpEvt = 3,
    #[doc = "4: ZERO_LOAD_EVT"]
    Ccctl23CcactupdZeroLoadEvt = 4,
    #[doc = "5: ZERO_RC_ZERO_EVT"]
    Ccctl23CcactupdZeroRcZeroEvt = 5,
    #[doc = "6: TRIG"]
    Ccctl23CcactupdTrig = 6,
}
impl From<Ccctl23Ccactupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Ccactupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Ccactupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Ccactupd {}
#[doc = "Field `CCCTL_23_CCACTUPD` reader - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
pub type Ccctl23CcactupdR = crate::FieldReader<Ccctl23Ccactupd>;
impl Ccctl23CcactupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Ccactupd> {
        match self.bits {
            0 => Some(Ccctl23Ccactupd::Ccctl23CcactupdImmediately),
            1 => Some(Ccctl23Ccactupd::Ccctl23CcactupdZeroEvt),
            2 => Some(Ccctl23Ccactupd::Ccctl23CcactupdCompareDownEvt),
            3 => Some(Ccctl23Ccactupd::Ccctl23CcactupdCompareUpEvt),
            4 => Some(Ccctl23Ccactupd::Ccctl23CcactupdZeroLoadEvt),
            5 => Some(Ccctl23Ccactupd::Ccctl23CcactupdZeroRcZeroEvt),
            6 => Some(Ccctl23Ccactupd::Ccctl23CcactupdTrig),
            _ => None,
        }
    }
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_immediately(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdImmediately
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_zero_evt(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdZeroEvt
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_compare_down_evt(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdCompareDownEvt
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_compare_up_evt(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdCompareUpEvt
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_zero_load_evt(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdZeroLoadEvt
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_zero_rc_zero_evt(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdZeroRcZeroEvt
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn is_ccctl_23_ccactupd_trig(&self) -> bool {
        *self == Ccctl23Ccactupd::Ccctl23CcactupdTrig
    }
}
#[doc = "Field `CCCTL_23_CCACTUPD` writer - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
pub type Ccctl23CcactupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Ccactupd>;
impl<'a, REG> Ccctl23CcactupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdImmediately)
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdZeroEvt)
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdCompareDownEvt)
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdCompareUpEvt)
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdZeroLoadEvt)
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdZeroRcZeroEvt)
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Ccactupd::Ccctl23CcactupdTrig)
    }
}
#[doc = "Selects the source second CCD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl23Cc2seld {
    #[doc = "0: SEL_CCD0"]
    Ccctl23Cc2seldSelCcd0 = 0,
    #[doc = "1: SEL_CCD1"]
    Ccctl23Cc2seldSelCcd1 = 1,
    #[doc = "2: SEL_CCD2"]
    Ccctl23Cc2seldSelCcd2 = 2,
    #[doc = "3: SEL_CCD3"]
    Ccctl23Cc2seldSelCcd3 = 3,
    #[doc = "4: SEL_CCD4"]
    Ccctl23Cc2seldSelCcd4 = 4,
    #[doc = "5: SEL_CCD5"]
    Ccctl23Cc2seldSelCcd5 = 5,
}
impl From<Ccctl23Cc2seld> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl23Cc2seld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl23Cc2seld {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl23Cc2seld {}
#[doc = "Field `CCCTL_23_CC2SELD` reader - Selects the source second CCD event."]
pub type Ccctl23Cc2seldR = crate::FieldReader<Ccctl23Cc2seld>;
impl Ccctl23Cc2seldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl23Cc2seld> {
        match self.bits {
            0 => Some(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd0),
            1 => Some(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd1),
            2 => Some(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd2),
            3 => Some(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd3),
            4 => Some(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd4),
            5 => Some(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd5),
            _ => None,
        }
    }
    #[doc = "SEL_CCD0"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2seld_sel_ccd0(&self) -> bool {
        *self == Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd0
    }
    #[doc = "SEL_CCD1"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2seld_sel_ccd1(&self) -> bool {
        *self == Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd1
    }
    #[doc = "SEL_CCD2"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2seld_sel_ccd2(&self) -> bool {
        *self == Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd2
    }
    #[doc = "SEL_CCD3"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2seld_sel_ccd3(&self) -> bool {
        *self == Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd3
    }
    #[doc = "SEL_CCD4"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2seld_sel_ccd4(&self) -> bool {
        *self == Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd4
    }
    #[doc = "SEL_CCD5"]
    #[inline(always)]
    pub fn is_ccctl_23_cc2seld_sel_ccd5(&self) -> bool {
        *self == Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd5
    }
}
#[doc = "Field `CCCTL_23_CC2SELD` writer - Selects the source second CCD event."]
pub type Ccctl23Cc2seldW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl23Cc2seld>;
impl<'a, REG> Ccctl23Cc2seldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SEL_CCD0"]
    #[inline(always)]
    pub fn ccctl_23_cc2seld_sel_ccd0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd0)
    }
    #[doc = "SEL_CCD1"]
    #[inline(always)]
    pub fn ccctl_23_cc2seld_sel_ccd1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd1)
    }
    #[doc = "SEL_CCD2"]
    #[inline(always)]
    pub fn ccctl_23_cc2seld_sel_ccd2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd2)
    }
    #[doc = "SEL_CCD3"]
    #[inline(always)]
    pub fn ccctl_23_cc2seld_sel_ccd3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd3)
    }
    #[doc = "SEL_CCD4"]
    #[inline(always)]
    pub fn ccctl_23_cc2seld_sel_ccd4(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd4)
    }
    #[doc = "SEL_CCD5"]
    #[inline(always)]
    pub fn ccctl_23_cc2seld_sel_ccd5(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl23Cc2seld::Ccctl23Cc2seldSelCcd5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_ccond(&self) -> Ccctl23CcondR {
        Ccctl23CcondR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_acond(&self) -> Ccctl23AcondR {
        Ccctl23AcondR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_lcond(&self) -> Ccctl23LcondR {
        Ccctl23LcondR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_zcond(&self) -> Ccctl23ZcondR {
        Ccctl23ZcondR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
    #[inline(always)]
    pub fn ccctl_23_coc(&self) -> Ccctl23CocR {
        Ccctl23CocR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccctl_23_ccupd(&self) -> Ccctl23CcupdR {
        Ccctl23CcupdR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Selects the source second CCU event."]
    #[inline(always)]
    pub fn ccctl_23_cc2selu(&self) -> Ccctl23Cc2seluR {
        Ccctl23Cc2seluR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RCn) value is not 0."]
    #[inline(always)]
    pub fn ccctl_23_scercnez(&self) -> Ccctl23ScercnezR {
        Ccctl23ScercnezR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd(&self) -> Ccctl23CcactupdR {
        Ccctl23CcactupdR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Selects the source second CCD event."]
    #[inline(always)]
    pub fn ccctl_23_cc2seld(&self) -> Ccctl23Cc2seldR {
        Ccctl23Cc2seldR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_ccond(&mut self) -> Ccctl23CcondW<Ccctl23Spec> {
        Ccctl23CcondW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_acond(&mut self) -> Ccctl23AcondW<Ccctl23Spec> {
        Ccctl23AcondW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_lcond(&mut self) -> Ccctl23LcondW<Ccctl23Spec> {
        Ccctl23LcondW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_23_zcond(&mut self) -> Ccctl23ZcondW<Ccctl23Spec> {
        Ccctl23ZcondW::new(self, 12)
    }
    #[doc = "Bit 17 - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
    #[inline(always)]
    pub fn ccctl_23_coc(&mut self) -> Ccctl23CocW<Ccctl23Spec> {
        Ccctl23CocW::new(self, 17)
    }
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccctl_23_ccupd(&mut self) -> Ccctl23CcupdW<Ccctl23Spec> {
        Ccctl23CcupdW::new(self, 18)
    }
    #[doc = "Bits 22:24 - Selects the source second CCU event."]
    #[inline(always)]
    pub fn ccctl_23_cc2selu(&mut self) -> Ccctl23Cc2seluW<Ccctl23Spec> {
        Ccctl23Cc2seluW::new(self, 22)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RCn) value is not 0."]
    #[inline(always)]
    pub fn ccctl_23_scercnez(&mut self) -> Ccctl23ScercnezW<Ccctl23Spec> {
        Ccctl23ScercnezW::new(self, 25)
    }
    #[doc = "Bits 26:28 - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
    #[inline(always)]
    pub fn ccctl_23_ccactupd(&mut self) -> Ccctl23CcactupdW<Ccctl23Spec> {
        Ccctl23CcactupdW::new(self, 26)
    }
    #[doc = "Bits 29:31 - Selects the source second CCD event."]
    #[inline(always)]
    pub fn ccctl_23_cc2seld(&mut self) -> Ccctl23Cc2seldW<Ccctl23Spec> {
        Ccctl23Cc2seldW::new(self, 29)
    }
}
#[doc = "Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccctl23Spec;
impl crate::RegisterSpec for Ccctl23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccctl_23::R`](R) reader structure"]
impl crate::Readable for Ccctl23Spec {}
#[doc = "`write(|w| ..)` method takes [`ccctl_23::W`](W) writer structure"]
impl crate::Writable for Ccctl23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCTL_23[%s]
to value 0"]
impl crate::Resettable for Ccctl23Spec {
    const RESET_VALUE: u32 = 0;
}

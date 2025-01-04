#[doc = "Register `CCCTL_01[%s]` reader"]
pub type R = crate::R<Ccctl01Spec>;
#[doc = "Register `CCCTL_01[%s]` writer"]
pub type W = crate::W<Ccctl01Spec>;
#[doc = "Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Ccond {
    #[doc = "0: NOCAPTURE"]
    Ccctl01CcondNocapture = 0,
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl01CcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl01CcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl01CcondCcTrigEdge = 3,
}
impl From<Ccctl01Ccond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Ccond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Ccond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Ccond {}
#[doc = "Field `CCCTL_01_CCOND` reader - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
pub type Ccctl01CcondR = crate::FieldReader<Ccctl01Ccond>;
impl Ccctl01CcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Ccond> {
        match self.bits {
            0 => Some(Ccctl01Ccond::Ccctl01CcondNocapture),
            1 => Some(Ccctl01Ccond::Ccctl01CcondCcTrigRise),
            2 => Some(Ccctl01Ccond::Ccctl01CcondCcTrigFall),
            3 => Some(Ccctl01Ccond::Ccctl01CcondCcTrigEdge),
            _ => None,
        }
    }
    #[doc = "NOCAPTURE"]
    #[inline(always)]
    pub fn is_ccctl_01_ccond_nocapture(&self) -> bool {
        *self == Ccctl01Ccond::Ccctl01CcondNocapture
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_01_ccond_cc_trig_rise(&self) -> bool {
        *self == Ccctl01Ccond::Ccctl01CcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_01_ccond_cc_trig_fall(&self) -> bool {
        *self == Ccctl01Ccond::Ccctl01CcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_01_ccond_cc_trig_edge(&self) -> bool {
        *self == Ccctl01Ccond::Ccctl01CcondCcTrigEdge
    }
}
#[doc = "Field `CCCTL_01_CCOND` writer - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
pub type Ccctl01CcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Ccond>;
impl<'a, REG> Ccctl01CcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOCAPTURE"]
    #[inline(always)]
    pub fn ccctl_01_ccond_nocapture(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccond::Ccctl01CcondNocapture)
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_01_ccond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccond::Ccctl01CcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_01_ccond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccond::Ccctl01CcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_01_ccond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccond::Ccctl01CcondCcTrigEdge)
    }
}
#[doc = "Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Acond {
    #[doc = "0: TIMCLK"]
    Ccctl01AcondTimclk = 0,
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl01AcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl01AcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl01AcondCcTrigEdge = 3,
    #[doc = "5: CC_TRIG_HIGH"]
    Ccctl01AcondCcTrigHigh = 5,
}
impl From<Ccctl01Acond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Acond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Acond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Acond {}
#[doc = "Field `CCCTL_01_ACOND` reader - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
pub type Ccctl01AcondR = crate::FieldReader<Ccctl01Acond>;
impl Ccctl01AcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Acond> {
        match self.bits {
            0 => Some(Ccctl01Acond::Ccctl01AcondTimclk),
            1 => Some(Ccctl01Acond::Ccctl01AcondCcTrigRise),
            2 => Some(Ccctl01Acond::Ccctl01AcondCcTrigFall),
            3 => Some(Ccctl01Acond::Ccctl01AcondCcTrigEdge),
            5 => Some(Ccctl01Acond::Ccctl01AcondCcTrigHigh),
            _ => None,
        }
    }
    #[doc = "TIMCLK"]
    #[inline(always)]
    pub fn is_ccctl_01_acond_timclk(&self) -> bool {
        *self == Ccctl01Acond::Ccctl01AcondTimclk
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_01_acond_cc_trig_rise(&self) -> bool {
        *self == Ccctl01Acond::Ccctl01AcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_01_acond_cc_trig_fall(&self) -> bool {
        *self == Ccctl01Acond::Ccctl01AcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_01_acond_cc_trig_edge(&self) -> bool {
        *self == Ccctl01Acond::Ccctl01AcondCcTrigEdge
    }
    #[doc = "CC_TRIG_HIGH"]
    #[inline(always)]
    pub fn is_ccctl_01_acond_cc_trig_high(&self) -> bool {
        *self == Ccctl01Acond::Ccctl01AcondCcTrigHigh
    }
}
#[doc = "Field `CCCTL_01_ACOND` writer - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
pub type Ccctl01AcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Acond>;
impl<'a, REG> Ccctl01AcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIMCLK"]
    #[inline(always)]
    pub fn ccctl_01_acond_timclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Acond::Ccctl01AcondTimclk)
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_01_acond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Acond::Ccctl01AcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_01_acond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Acond::Ccctl01AcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_01_acond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Acond::Ccctl01AcondCcTrigEdge)
    }
    #[doc = "CC_TRIG_HIGH"]
    #[inline(always)]
    pub fn ccctl_01_acond_cc_trig_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Acond::Ccctl01AcondCcTrigHigh)
    }
}
#[doc = "Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Lcond {
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl01LcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl01LcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl01LcondCcTrigEdge = 3,
}
impl From<Ccctl01Lcond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Lcond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Lcond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Lcond {}
#[doc = "Field `CCCTL_01_LCOND` reader - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
pub type Ccctl01LcondR = crate::FieldReader<Ccctl01Lcond>;
impl Ccctl01LcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Lcond> {
        match self.bits {
            1 => Some(Ccctl01Lcond::Ccctl01LcondCcTrigRise),
            2 => Some(Ccctl01Lcond::Ccctl01LcondCcTrigFall),
            3 => Some(Ccctl01Lcond::Ccctl01LcondCcTrigEdge),
            _ => None,
        }
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_01_lcond_cc_trig_rise(&self) -> bool {
        *self == Ccctl01Lcond::Ccctl01LcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_01_lcond_cc_trig_fall(&self) -> bool {
        *self == Ccctl01Lcond::Ccctl01LcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_01_lcond_cc_trig_edge(&self) -> bool {
        *self == Ccctl01Lcond::Ccctl01LcondCcTrigEdge
    }
}
#[doc = "Field `CCCTL_01_LCOND` writer - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
pub type Ccctl01LcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Lcond>;
impl<'a, REG> Ccctl01LcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_01_lcond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Lcond::Ccctl01LcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_01_lcond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Lcond::Ccctl01LcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_01_lcond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Lcond::Ccctl01LcondCcTrigEdge)
    }
}
#[doc = "Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Zcond {
    #[doc = "1: CC_TRIG_RISE"]
    Ccctl01ZcondCcTrigRise = 1,
    #[doc = "2: CC_TRIG_FALL"]
    Ccctl01ZcondCcTrigFall = 2,
    #[doc = "3: CC_TRIG_EDGE"]
    Ccctl01ZcondCcTrigEdge = 3,
}
impl From<Ccctl01Zcond> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Zcond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Zcond {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Zcond {}
#[doc = "Field `CCCTL_01_ZCOND` reader - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
pub type Ccctl01ZcondR = crate::FieldReader<Ccctl01Zcond>;
impl Ccctl01ZcondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Zcond> {
        match self.bits {
            1 => Some(Ccctl01Zcond::Ccctl01ZcondCcTrigRise),
            2 => Some(Ccctl01Zcond::Ccctl01ZcondCcTrigFall),
            3 => Some(Ccctl01Zcond::Ccctl01ZcondCcTrigEdge),
            _ => None,
        }
    }
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn is_ccctl_01_zcond_cc_trig_rise(&self) -> bool {
        *self == Ccctl01Zcond::Ccctl01ZcondCcTrigRise
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn is_ccctl_01_zcond_cc_trig_fall(&self) -> bool {
        *self == Ccctl01Zcond::Ccctl01ZcondCcTrigFall
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn is_ccctl_01_zcond_cc_trig_edge(&self) -> bool {
        *self == Ccctl01Zcond::Ccctl01ZcondCcTrigEdge
    }
}
#[doc = "Field `CCCTL_01_ZCOND` writer - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
pub type Ccctl01ZcondW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Zcond>;
impl<'a, REG> Ccctl01ZcondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CC_TRIG_RISE"]
    #[inline(always)]
    pub fn ccctl_01_zcond_cc_trig_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Zcond::Ccctl01ZcondCcTrigRise)
    }
    #[doc = "CC_TRIG_FALL"]
    #[inline(always)]
    pub fn ccctl_01_zcond_cc_trig_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Zcond::Ccctl01ZcondCcTrigFall)
    }
    #[doc = "CC_TRIG_EDGE"]
    #[inline(always)]
    pub fn ccctl_01_zcond_cc_trig_edge(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Zcond::Ccctl01ZcondCcTrigEdge)
    }
}
#[doc = "Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccctl01Coc {
    #[doc = "0: COMPARE"]
    Ccctl01CocCompare = 0,
    #[doc = "1: CAPTURE"]
    Ccctl01CocCapture = 1,
}
impl From<Ccctl01Coc> for bool {
    #[inline(always)]
    fn from(variant: Ccctl01Coc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCTL_01_COC` reader - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
pub type Ccctl01CocR = crate::BitReader<Ccctl01Coc>;
impl Ccctl01CocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccctl01Coc {
        match self.bits {
            false => Ccctl01Coc::Ccctl01CocCompare,
            true => Ccctl01Coc::Ccctl01CocCapture,
        }
    }
    #[doc = "COMPARE"]
    #[inline(always)]
    pub fn is_ccctl_01_coc_compare(&self) -> bool {
        *self == Ccctl01Coc::Ccctl01CocCompare
    }
    #[doc = "CAPTURE"]
    #[inline(always)]
    pub fn is_ccctl_01_coc_capture(&self) -> bool {
        *self == Ccctl01Coc::Ccctl01CocCapture
    }
}
#[doc = "Field `CCCTL_01_COC` writer - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
pub type Ccctl01CocW<'a, REG> = crate::BitWriter<'a, REG, Ccctl01Coc>;
impl<'a, REG> Ccctl01CocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COMPARE"]
    #[inline(always)]
    pub fn ccctl_01_coc_compare(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Coc::Ccctl01CocCompare)
    }
    #[doc = "CAPTURE"]
    #[inline(always)]
    pub fn ccctl_01_coc_capture(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Coc::Ccctl01CocCapture)
    }
}
#[doc = "Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Ccupd {
    #[doc = "0: IMMEDIATELY"]
    Ccctl01CcupdImmediately = 0,
    #[doc = "1: ZERO_EVT"]
    Ccctl01CcupdZeroEvt = 1,
    #[doc = "2: COMPARE_DOWN_EVT"]
    Ccctl01CcupdCompareDownEvt = 2,
    #[doc = "3: COMPARE_UP_EVT"]
    Ccctl01CcupdCompareUpEvt = 3,
    #[doc = "4: ZERO_LOAD_EVT"]
    Ccctl01CcupdZeroLoadEvt = 4,
    #[doc = "5: ZERO_RC_ZERO_EVT"]
    Ccctl01CcupdZeroRcZeroEvt = 5,
    #[doc = "6: TRIG"]
    Ccctl01CcupdTrig = 6,
}
impl From<Ccctl01Ccupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Ccupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Ccupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Ccupd {}
#[doc = "Field `CCCTL_01_CCUPD` reader - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type Ccctl01CcupdR = crate::FieldReader<Ccctl01Ccupd>;
impl Ccctl01CcupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Ccupd> {
        match self.bits {
            0 => Some(Ccctl01Ccupd::Ccctl01CcupdImmediately),
            1 => Some(Ccctl01Ccupd::Ccctl01CcupdZeroEvt),
            2 => Some(Ccctl01Ccupd::Ccctl01CcupdCompareDownEvt),
            3 => Some(Ccctl01Ccupd::Ccctl01CcupdCompareUpEvt),
            4 => Some(Ccctl01Ccupd::Ccctl01CcupdZeroLoadEvt),
            5 => Some(Ccctl01Ccupd::Ccctl01CcupdZeroRcZeroEvt),
            6 => Some(Ccctl01Ccupd::Ccctl01CcupdTrig),
            _ => None,
        }
    }
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_immediately(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdImmediately
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_zero_evt(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdZeroEvt
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_compare_down_evt(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdCompareDownEvt
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_compare_up_evt(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdCompareUpEvt
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_zero_load_evt(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdZeroLoadEvt
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_zero_rc_zero_evt(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdZeroRcZeroEvt
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn is_ccctl_01_ccupd_trig(&self) -> bool {
        *self == Ccctl01Ccupd::Ccctl01CcupdTrig
    }
}
#[doc = "Field `CCCTL_01_CCUPD` writer - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type Ccctl01CcupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Ccupd>;
impl<'a, REG> Ccctl01CcupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdImmediately)
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdZeroEvt)
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdCompareDownEvt)
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdCompareUpEvt)
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdZeroLoadEvt)
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdZeroRcZeroEvt)
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn ccctl_01_ccupd_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccupd::Ccctl01CcupdTrig)
    }
}
#[doc = "Selects the source second CCU event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Cc2selu {
    #[doc = "0: SEL_CCU0"]
    Ccctl01Cc2seluSelCcu0 = 0,
    #[doc = "1: SEL_CCU1"]
    Ccctl01Cc2seluSelCcu1 = 1,
    #[doc = "2: SEL_CCU2"]
    Ccctl01Cc2seluSelCcu2 = 2,
    #[doc = "3: SEL_CCU3"]
    Ccctl01Cc2seluSelCcu3 = 3,
    #[doc = "4: SEL_CCU4"]
    Ccctl01Cc2seluSelCcu4 = 4,
    #[doc = "5: SEL_CCU5"]
    Ccctl01Cc2seluSelCcu5 = 5,
}
impl From<Ccctl01Cc2selu> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Cc2selu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Cc2selu {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Cc2selu {}
#[doc = "Field `CCCTL_01_CC2SELU` reader - Selects the source second CCU event."]
pub type Ccctl01Cc2seluR = crate::FieldReader<Ccctl01Cc2selu>;
impl Ccctl01Cc2seluR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Cc2selu> {
        match self.bits {
            0 => Some(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu0),
            1 => Some(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu1),
            2 => Some(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu2),
            3 => Some(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu3),
            4 => Some(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu4),
            5 => Some(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu5),
            _ => None,
        }
    }
    #[doc = "SEL_CCU0"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2selu_sel_ccu0(&self) -> bool {
        *self == Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu0
    }
    #[doc = "SEL_CCU1"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2selu_sel_ccu1(&self) -> bool {
        *self == Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu1
    }
    #[doc = "SEL_CCU2"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2selu_sel_ccu2(&self) -> bool {
        *self == Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu2
    }
    #[doc = "SEL_CCU3"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2selu_sel_ccu3(&self) -> bool {
        *self == Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu3
    }
    #[doc = "SEL_CCU4"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2selu_sel_ccu4(&self) -> bool {
        *self == Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu4
    }
    #[doc = "SEL_CCU5"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2selu_sel_ccu5(&self) -> bool {
        *self == Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu5
    }
}
#[doc = "Field `CCCTL_01_CC2SELU` writer - Selects the source second CCU event."]
pub type Ccctl01Cc2seluW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Cc2selu>;
impl<'a, REG> Ccctl01Cc2seluW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SEL_CCU0"]
    #[inline(always)]
    pub fn ccctl_01_cc2selu_sel_ccu0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu0)
    }
    #[doc = "SEL_CCU1"]
    #[inline(always)]
    pub fn ccctl_01_cc2selu_sel_ccu1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu1)
    }
    #[doc = "SEL_CCU2"]
    #[inline(always)]
    pub fn ccctl_01_cc2selu_sel_ccu2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu2)
    }
    #[doc = "SEL_CCU3"]
    #[inline(always)]
    pub fn ccctl_01_cc2selu_sel_ccu3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu3)
    }
    #[doc = "SEL_CCU4"]
    #[inline(always)]
    pub fn ccctl_01_cc2selu_sel_ccu4(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu4)
    }
    #[doc = "SEL_CCU5"]
    #[inline(always)]
    pub fn ccctl_01_cc2selu_sel_ccu5(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2selu::Ccctl01Cc2seluSelCcu5)
    }
}
#[doc = "Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccctl01Scercnez {
    #[doc = "0: DISABLED"]
    Ccctl01ScercnezDisabled = 0,
    #[doc = "1: ENABLED"]
    Ccctl01ScercnezEnabled = 1,
}
impl From<Ccctl01Scercnez> for bool {
    #[inline(always)]
    fn from(variant: Ccctl01Scercnez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCTL_01_SCERCNEZ` reader - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
pub type Ccctl01ScercnezR = crate::BitReader<Ccctl01Scercnez>;
impl Ccctl01ScercnezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccctl01Scercnez {
        match self.bits {
            false => Ccctl01Scercnez::Ccctl01ScercnezDisabled,
            true => Ccctl01Scercnez::Ccctl01ScercnezEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccctl_01_scercnez_disabled(&self) -> bool {
        *self == Ccctl01Scercnez::Ccctl01ScercnezDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ccctl_01_scercnez_enabled(&self) -> bool {
        *self == Ccctl01Scercnez::Ccctl01ScercnezEnabled
    }
}
#[doc = "Field `CCCTL_01_SCERCNEZ` writer - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
pub type Ccctl01ScercnezW<'a, REG> = crate::BitWriter<'a, REG, Ccctl01Scercnez>;
impl<'a, REG> Ccctl01ScercnezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccctl_01_scercnez_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Scercnez::Ccctl01ScercnezDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ccctl_01_scercnez_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Scercnez::Ccctl01ScercnezEnabled)
    }
}
#[doc = "CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Ccactupd {
    #[doc = "0: IMMEDIATELY"]
    Ccctl01CcactupdImmediately = 0,
    #[doc = "1: ZERO_EVT"]
    Ccctl01CcactupdZeroEvt = 1,
    #[doc = "2: COMPARE_DOWN_EVT"]
    Ccctl01CcactupdCompareDownEvt = 2,
    #[doc = "3: COMPARE_UP_EVT"]
    Ccctl01CcactupdCompareUpEvt = 3,
    #[doc = "4: ZERO_LOAD_EVT"]
    Ccctl01CcactupdZeroLoadEvt = 4,
    #[doc = "5: ZERO_RC_ZERO_EVT"]
    Ccctl01CcactupdZeroRcZeroEvt = 5,
    #[doc = "6: TRIG"]
    Ccctl01CcactupdTrig = 6,
}
impl From<Ccctl01Ccactupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Ccactupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Ccactupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Ccactupd {}
#[doc = "Field `CCCTL_01_CCACTUPD` reader - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
pub type Ccctl01CcactupdR = crate::FieldReader<Ccctl01Ccactupd>;
impl Ccctl01CcactupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Ccactupd> {
        match self.bits {
            0 => Some(Ccctl01Ccactupd::Ccctl01CcactupdImmediately),
            1 => Some(Ccctl01Ccactupd::Ccctl01CcactupdZeroEvt),
            2 => Some(Ccctl01Ccactupd::Ccctl01CcactupdCompareDownEvt),
            3 => Some(Ccctl01Ccactupd::Ccctl01CcactupdCompareUpEvt),
            4 => Some(Ccctl01Ccactupd::Ccctl01CcactupdZeroLoadEvt),
            5 => Some(Ccctl01Ccactupd::Ccctl01CcactupdZeroRcZeroEvt),
            6 => Some(Ccctl01Ccactupd::Ccctl01CcactupdTrig),
            _ => None,
        }
    }
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_immediately(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdImmediately
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_zero_evt(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdZeroEvt
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_compare_down_evt(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdCompareDownEvt
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_compare_up_evt(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdCompareUpEvt
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_zero_load_evt(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdZeroLoadEvt
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_zero_rc_zero_evt(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdZeroRcZeroEvt
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn is_ccctl_01_ccactupd_trig(&self) -> bool {
        *self == Ccctl01Ccactupd::Ccctl01CcactupdTrig
    }
}
#[doc = "Field `CCCTL_01_CCACTUPD` writer - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
pub type Ccctl01CcactupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Ccactupd>;
impl<'a, REG> Ccctl01CcactupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdImmediately)
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdZeroEvt)
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdCompareDownEvt)
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdCompareUpEvt)
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdZeroLoadEvt)
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdZeroRcZeroEvt)
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Ccactupd::Ccctl01CcactupdTrig)
    }
}
#[doc = "Selects the source second CCD event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl01Cc2seld {
    #[doc = "0: SEL_CCD0"]
    Ccctl01Cc2seldSelCcd0 = 0,
    #[doc = "1: SEL_CCD1"]
    Ccctl01Cc2seldSelCcd1 = 1,
    #[doc = "2: SEL_CCD2"]
    Ccctl01Cc2seldSelCcd2 = 2,
    #[doc = "3: SEL_CCD3"]
    Ccctl01Cc2seldSelCcd3 = 3,
    #[doc = "4: SEL_CCD4"]
    Ccctl01Cc2seldSelCcd4 = 4,
    #[doc = "5: SEL_CCD5"]
    Ccctl01Cc2seldSelCcd5 = 5,
}
impl From<Ccctl01Cc2seld> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl01Cc2seld) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl01Cc2seld {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl01Cc2seld {}
#[doc = "Field `CCCTL_01_CC2SELD` reader - Selects the source second CCD event."]
pub type Ccctl01Cc2seldR = crate::FieldReader<Ccctl01Cc2seld>;
impl Ccctl01Cc2seldR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl01Cc2seld> {
        match self.bits {
            0 => Some(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd0),
            1 => Some(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd1),
            2 => Some(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd2),
            3 => Some(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd3),
            4 => Some(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd4),
            5 => Some(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd5),
            _ => None,
        }
    }
    #[doc = "SEL_CCD0"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2seld_sel_ccd0(&self) -> bool {
        *self == Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd0
    }
    #[doc = "SEL_CCD1"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2seld_sel_ccd1(&self) -> bool {
        *self == Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd1
    }
    #[doc = "SEL_CCD2"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2seld_sel_ccd2(&self) -> bool {
        *self == Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd2
    }
    #[doc = "SEL_CCD3"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2seld_sel_ccd3(&self) -> bool {
        *self == Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd3
    }
    #[doc = "SEL_CCD4"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2seld_sel_ccd4(&self) -> bool {
        *self == Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd4
    }
    #[doc = "SEL_CCD5"]
    #[inline(always)]
    pub fn is_ccctl_01_cc2seld_sel_ccd5(&self) -> bool {
        *self == Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd5
    }
}
#[doc = "Field `CCCTL_01_CC2SELD` writer - Selects the source second CCD event."]
pub type Ccctl01Cc2seldW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl01Cc2seld>;
impl<'a, REG> Ccctl01Cc2seldW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SEL_CCD0"]
    #[inline(always)]
    pub fn ccctl_01_cc2seld_sel_ccd0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd0)
    }
    #[doc = "SEL_CCD1"]
    #[inline(always)]
    pub fn ccctl_01_cc2seld_sel_ccd1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd1)
    }
    #[doc = "SEL_CCD2"]
    #[inline(always)]
    pub fn ccctl_01_cc2seld_sel_ccd2(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd2)
    }
    #[doc = "SEL_CCD3"]
    #[inline(always)]
    pub fn ccctl_01_cc2seld_sel_ccd3(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd3)
    }
    #[doc = "SEL_CCD4"]
    #[inline(always)]
    pub fn ccctl_01_cc2seld_sel_ccd4(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd4)
    }
    #[doc = "SEL_CCD5"]
    #[inline(always)]
    pub fn ccctl_01_cc2seld_sel_ccd5(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl01Cc2seld::Ccctl01Cc2seldSelCcd5)
    }
}
impl R {
    #[doc = "Bits 0:2 - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_ccond(&self) -> Ccctl01CcondR {
        Ccctl01CcondR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_acond(&self) -> Ccctl01AcondR {
        Ccctl01AcondR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_lcond(&self) -> Ccctl01LcondR {
        Ccctl01LcondR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_zcond(&self) -> Ccctl01ZcondR {
        Ccctl01ZcondR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 17 - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
    #[inline(always)]
    pub fn ccctl_01_coc(&self) -> Ccctl01CocR {
        Ccctl01CocR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccctl_01_ccupd(&self) -> Ccctl01CcupdR {
        Ccctl01CcupdR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Selects the source second CCU event."]
    #[inline(always)]
    pub fn ccctl_01_cc2selu(&self) -> Ccctl01Cc2seluR {
        Ccctl01Cc2seluR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn ccctl_01_scercnez(&self) -> Ccctl01ScercnezR {
        Ccctl01ScercnezR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd(&self) -> Ccctl01CcactupdR {
        Ccctl01CcactupdR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Selects the source second CCD event."]
    #[inline(always)]
    pub fn ccctl_01_cc2seld(&self) -> Ccctl01Cc2seldR {
        Ccctl01Cc2seldR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Capture Condition. #br# Specifies the condition that generates a capture pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_ccond(&mut self) -> Ccctl01CcondW<Ccctl01Spec> {
        Ccctl01CcondW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Advance Condition. #br# Specifies the condition that generates an advance pulse. 6h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_acond(&mut self) -> Ccctl01AcondW<Ccctl01Spec> {
        Ccctl01AcondW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Load Condition. #br# Specifies the condition that generates a load pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_lcond(&mut self) -> Ccctl01LcondW<Ccctl01Spec> {
        Ccctl01LcondW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Zero Condition. #br# This field specifies the condition that generates a zero pulse. 4h-Fh = Reserved"]
    #[inline(always)]
    pub fn ccctl_01_zcond(&mut self) -> Ccctl01ZcondW<Ccctl01Spec> {
        Ccctl01ZcondW::new(self, 12)
    }
    #[doc = "Bit 17 - Capture or Compare. #br# Specifies whether the corresponding CC register is used as a capture register or a compare register (never both)."]
    #[inline(always)]
    pub fn ccctl_01_coc(&mut self) -> Ccctl01CocW<Ccctl01Spec> {
        Ccctl01CocW::new(self, 17)
    }
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccctl_01_ccupd(&mut self) -> Ccctl01CcupdW<Ccctl01Spec> {
        Ccctl01CcupdW::new(self, 18)
    }
    #[doc = "Bits 22:24 - Selects the source second CCU event."]
    #[inline(always)]
    pub fn ccctl_01_cc2selu(&mut self) -> Ccctl01Cc2seluW<Ccctl01Spec> {
        Ccctl01Cc2seluW::new(self, 22)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn ccctl_01_scercnez(&mut self) -> Ccctl01ScercnezW<Ccctl01Spec> {
        Ccctl01ScercnezW::new(self, 25)
    }
    #[doc = "Bits 26:28 - CCACT shadow register Update Method This field controls how updates to the CCCACT shadow register are performed"]
    #[inline(always)]
    pub fn ccctl_01_ccactupd(&mut self) -> Ccctl01CcactupdW<Ccctl01Spec> {
        Ccctl01CcactupdW::new(self, 26)
    }
    #[doc = "Bits 29:31 - Selects the source second CCD event."]
    #[inline(always)]
    pub fn ccctl_01_cc2seld(&mut self) -> Ccctl01Cc2seldW<Ccctl01Spec> {
        Ccctl01Cc2seldW::new(self, 29)
    }
}
#[doc = "Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccctl01Spec;
impl crate::RegisterSpec for Ccctl01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccctl_01::R`](R) reader structure"]
impl crate::Readable for Ccctl01Spec {}
#[doc = "`write(|w| ..)` method takes [`ccctl_01::W`](W) writer structure"]
impl crate::Writable for Ccctl01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCTL_01[%s]
to value 0"]
impl crate::Resettable for Ccctl01Spec {
    const RESET_VALUE: u32 = 0;
}

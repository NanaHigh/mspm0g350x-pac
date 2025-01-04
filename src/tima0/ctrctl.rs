#[doc = "Register `CTRCTL` reader"]
pub type R = crate::R<CtrctlSpec>;
#[doc = "Register `CTRCTL` writer"]
pub type W = crate::W<CtrctlSpec>;
#[doc = "Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrctlEn {
    #[doc = "0: DISABLED"]
    CtrctlEnDisabled = 0,
    #[doc = "1: ENABLED"]
    CtrctlEnEnabled = 1,
}
impl From<CtrctlEn> for bool {
    #[inline(always)]
    fn from(variant: CtrctlEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRCTL_EN` reader - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
pub type CtrctlEnR = crate::BitReader<CtrctlEn>;
impl CtrctlEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrctlEn {
        match self.bits {
            false => CtrctlEn::CtrctlEnDisabled,
            true => CtrctlEn::CtrctlEnEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ctrctl_en_disabled(&self) -> bool {
        *self == CtrctlEn::CtrctlEnDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ctrctl_en_enabled(&self) -> bool {
        *self == CtrctlEn::CtrctlEnEnabled
    }
}
#[doc = "Field `CTRCTL_EN` writer - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
pub type CtrctlEnW<'a, REG> = crate::BitWriter<'a, REG, CtrctlEn>;
impl<'a, REG> CtrctlEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ctrctl_en_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlEn::CtrctlEnDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ctrctl_en_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlEn::CtrctlEnEnabled)
    }
}
#[doc = "Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrctlRepeat {
    #[doc = "0: REPEAT_0"]
    CtrctlRepeatRepeat0 = 0,
    #[doc = "1: REPEAT_1"]
    CtrctlRepeatRepeat1 = 1,
    #[doc = "2: REPEAT_2"]
    CtrctlRepeatRepeat2 = 2,
    #[doc = "3: REPEAT_3"]
    CtrctlRepeatRepeat3 = 3,
    #[doc = "4: REPEAT_4"]
    CtrctlRepeatRepeat4 = 4,
}
impl From<CtrctlRepeat> for u8 {
    #[inline(always)]
    fn from(variant: CtrctlRepeat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrctlRepeat {
    type Ux = u8;
}
impl crate::IsEnum for CtrctlRepeat {}
#[doc = "Field `CTRCTL_REPEAT` reader - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
pub type CtrctlRepeatR = crate::FieldReader<CtrctlRepeat>;
impl CtrctlRepeatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtrctlRepeat> {
        match self.bits {
            0 => Some(CtrctlRepeat::CtrctlRepeatRepeat0),
            1 => Some(CtrctlRepeat::CtrctlRepeatRepeat1),
            2 => Some(CtrctlRepeat::CtrctlRepeatRepeat2),
            3 => Some(CtrctlRepeat::CtrctlRepeatRepeat3),
            4 => Some(CtrctlRepeat::CtrctlRepeatRepeat4),
            _ => None,
        }
    }
    #[doc = "REPEAT_0"]
    #[inline(always)]
    pub fn is_ctrctl_repeat_repeat_0(&self) -> bool {
        *self == CtrctlRepeat::CtrctlRepeatRepeat0
    }
    #[doc = "REPEAT_1"]
    #[inline(always)]
    pub fn is_ctrctl_repeat_repeat_1(&self) -> bool {
        *self == CtrctlRepeat::CtrctlRepeatRepeat1
    }
    #[doc = "REPEAT_2"]
    #[inline(always)]
    pub fn is_ctrctl_repeat_repeat_2(&self) -> bool {
        *self == CtrctlRepeat::CtrctlRepeatRepeat2
    }
    #[doc = "REPEAT_3"]
    #[inline(always)]
    pub fn is_ctrctl_repeat_repeat_3(&self) -> bool {
        *self == CtrctlRepeat::CtrctlRepeatRepeat3
    }
    #[doc = "REPEAT_4"]
    #[inline(always)]
    pub fn is_ctrctl_repeat_repeat_4(&self) -> bool {
        *self == CtrctlRepeat::CtrctlRepeatRepeat4
    }
}
#[doc = "Field `CTRCTL_REPEAT` writer - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
pub type CtrctlRepeatW<'a, REG> = crate::FieldWriter<'a, REG, 3, CtrctlRepeat>;
impl<'a, REG> CtrctlRepeatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "REPEAT_0"]
    #[inline(always)]
    pub fn ctrctl_repeat_repeat_0(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlRepeat::CtrctlRepeatRepeat0)
    }
    #[doc = "REPEAT_1"]
    #[inline(always)]
    pub fn ctrctl_repeat_repeat_1(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlRepeat::CtrctlRepeatRepeat1)
    }
    #[doc = "REPEAT_2"]
    #[inline(always)]
    pub fn ctrctl_repeat_repeat_2(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlRepeat::CtrctlRepeatRepeat2)
    }
    #[doc = "REPEAT_3"]
    #[inline(always)]
    pub fn ctrctl_repeat_repeat_3(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlRepeat::CtrctlRepeatRepeat3)
    }
    #[doc = "REPEAT_4"]
    #[inline(always)]
    pub fn ctrctl_repeat_repeat_4(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlRepeat::CtrctlRepeatRepeat4)
    }
}
#[doc = "Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrctlCm {
    #[doc = "0: DOWN"]
    CtrctlCmDown = 0,
    #[doc = "1: UP_DOWN"]
    CtrctlCmUpDown = 1,
    #[doc = "2: UP"]
    CtrctlCmUp = 2,
}
impl From<CtrctlCm> for u8 {
    #[inline(always)]
    fn from(variant: CtrctlCm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrctlCm {
    type Ux = u8;
}
impl crate::IsEnum for CtrctlCm {}
#[doc = "Field `CTRCTL_CM` reader - Count Mode"]
pub type CtrctlCmR = crate::FieldReader<CtrctlCm>;
impl CtrctlCmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtrctlCm> {
        match self.bits {
            0 => Some(CtrctlCm::CtrctlCmDown),
            1 => Some(CtrctlCm::CtrctlCmUpDown),
            2 => Some(CtrctlCm::CtrctlCmUp),
            _ => None,
        }
    }
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn is_ctrctl_cm_down(&self) -> bool {
        *self == CtrctlCm::CtrctlCmDown
    }
    #[doc = "UP_DOWN"]
    #[inline(always)]
    pub fn is_ctrctl_cm_up_down(&self) -> bool {
        *self == CtrctlCm::CtrctlCmUpDown
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn is_ctrctl_cm_up(&self) -> bool {
        *self == CtrctlCm::CtrctlCmUp
    }
}
#[doc = "Field `CTRCTL_CM` writer - Count Mode"]
pub type CtrctlCmW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtrctlCm>;
impl<'a, REG> CtrctlCmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn ctrctl_cm_down(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCm::CtrctlCmDown)
    }
    #[doc = "UP_DOWN"]
    #[inline(always)]
    pub fn ctrctl_cm_up_down(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCm::CtrctlCmUpDown)
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn ctrctl_cm_up(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCm::CtrctlCmUp)
    }
}
#[doc = "Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrctlClc {
    #[doc = "0: CCCTL0_LCOND"]
    CtrctlClcCcctl0Lcond = 0,
    #[doc = "1: CCCTL1_LCOND"]
    CtrctlClcCcctl1Lcond = 1,
    #[doc = "2: CCCTL2_LCOND"]
    CtrctlClcCcctl2Lcond = 2,
    #[doc = "3: CCCTL3_LCOND"]
    CtrctlClcCcctl3Lcond = 3,
    #[doc = "4: QEI_2INP"]
    CtrctlClcQei2inp = 4,
    #[doc = "5: QEI_3INP"]
    CtrctlClcQei3inp = 5,
}
impl From<CtrctlClc> for u8 {
    #[inline(always)]
    fn from(variant: CtrctlClc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrctlClc {
    type Ux = u8;
}
impl crate::IsEnum for CtrctlClc {}
#[doc = "Field `CTRCTL_CLC` reader - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CtrctlClcR = crate::FieldReader<CtrctlClc>;
impl CtrctlClcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtrctlClc> {
        match self.bits {
            0 => Some(CtrctlClc::CtrctlClcCcctl0Lcond),
            1 => Some(CtrctlClc::CtrctlClcCcctl1Lcond),
            2 => Some(CtrctlClc::CtrctlClcCcctl2Lcond),
            3 => Some(CtrctlClc::CtrctlClcCcctl3Lcond),
            4 => Some(CtrctlClc::CtrctlClcQei2inp),
            5 => Some(CtrctlClc::CtrctlClcQei3inp),
            _ => None,
        }
    }
    #[doc = "CCCTL0_LCOND"]
    #[inline(always)]
    pub fn is_ctrctl_clc_ccctl0_lcond(&self) -> bool {
        *self == CtrctlClc::CtrctlClcCcctl0Lcond
    }
    #[doc = "CCCTL1_LCOND"]
    #[inline(always)]
    pub fn is_ctrctl_clc_ccctl1_lcond(&self) -> bool {
        *self == CtrctlClc::CtrctlClcCcctl1Lcond
    }
    #[doc = "CCCTL2_LCOND"]
    #[inline(always)]
    pub fn is_ctrctl_clc_ccctl2_lcond(&self) -> bool {
        *self == CtrctlClc::CtrctlClcCcctl2Lcond
    }
    #[doc = "CCCTL3_LCOND"]
    #[inline(always)]
    pub fn is_ctrctl_clc_ccctl3_lcond(&self) -> bool {
        *self == CtrctlClc::CtrctlClcCcctl3Lcond
    }
    #[doc = "QEI_2INP"]
    #[inline(always)]
    pub fn is_ctrctl_clc_qei_2inp(&self) -> bool {
        *self == CtrctlClc::CtrctlClcQei2inp
    }
    #[doc = "QEI_3INP"]
    #[inline(always)]
    pub fn is_ctrctl_clc_qei_3inp(&self) -> bool {
        *self == CtrctlClc::CtrctlClcQei3inp
    }
}
#[doc = "Field `CTRCTL_CLC` writer - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CtrctlClcW<'a, REG> = crate::FieldWriter<'a, REG, 3, CtrctlClc>;
impl<'a, REG> CtrctlClcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCCTL0_LCOND"]
    #[inline(always)]
    pub fn ctrctl_clc_ccctl0_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlClc::CtrctlClcCcctl0Lcond)
    }
    #[doc = "CCCTL1_LCOND"]
    #[inline(always)]
    pub fn ctrctl_clc_ccctl1_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlClc::CtrctlClcCcctl1Lcond)
    }
    #[doc = "CCCTL2_LCOND"]
    #[inline(always)]
    pub fn ctrctl_clc_ccctl2_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlClc::CtrctlClcCcctl2Lcond)
    }
    #[doc = "CCCTL3_LCOND"]
    #[inline(always)]
    pub fn ctrctl_clc_ccctl3_lcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlClc::CtrctlClcCcctl3Lcond)
    }
    #[doc = "QEI_2INP"]
    #[inline(always)]
    pub fn ctrctl_clc_qei_2inp(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlClc::CtrctlClcQei2inp)
    }
    #[doc = "QEI_3INP"]
    #[inline(always)]
    pub fn ctrctl_clc_qei_3inp(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlClc::CtrctlClcQei3inp)
    }
}
#[doc = "Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrctlCac {
    #[doc = "0: CCCTL0_ACOND"]
    CtrctlCacCcctl0Acond = 0,
    #[doc = "1: CCCTL1_ACOND"]
    CtrctlCacCcctl1Acond = 1,
    #[doc = "2: CCCTL2_ACOND"]
    CtrctlCacCcctl2Acond = 2,
    #[doc = "3: CCCTL3_ACOND"]
    CtrctlCacCcctl3Acond = 3,
    #[doc = "4: QEI_2INP"]
    CtrctlCacQei2inp = 4,
    #[doc = "5: QEI_3INP"]
    CtrctlCacQei3inp = 5,
}
impl From<CtrctlCac> for u8 {
    #[inline(always)]
    fn from(variant: CtrctlCac) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrctlCac {
    type Ux = u8;
}
impl crate::IsEnum for CtrctlCac {}
#[doc = "Field `CTRCTL_CAC` reader - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CtrctlCacR = crate::FieldReader<CtrctlCac>;
impl CtrctlCacR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtrctlCac> {
        match self.bits {
            0 => Some(CtrctlCac::CtrctlCacCcctl0Acond),
            1 => Some(CtrctlCac::CtrctlCacCcctl1Acond),
            2 => Some(CtrctlCac::CtrctlCacCcctl2Acond),
            3 => Some(CtrctlCac::CtrctlCacCcctl3Acond),
            4 => Some(CtrctlCac::CtrctlCacQei2inp),
            5 => Some(CtrctlCac::CtrctlCacQei3inp),
            _ => None,
        }
    }
    #[doc = "CCCTL0_ACOND"]
    #[inline(always)]
    pub fn is_ctrctl_cac_ccctl0_acond(&self) -> bool {
        *self == CtrctlCac::CtrctlCacCcctl0Acond
    }
    #[doc = "CCCTL1_ACOND"]
    #[inline(always)]
    pub fn is_ctrctl_cac_ccctl1_acond(&self) -> bool {
        *self == CtrctlCac::CtrctlCacCcctl1Acond
    }
    #[doc = "CCCTL2_ACOND"]
    #[inline(always)]
    pub fn is_ctrctl_cac_ccctl2_acond(&self) -> bool {
        *self == CtrctlCac::CtrctlCacCcctl2Acond
    }
    #[doc = "CCCTL3_ACOND"]
    #[inline(always)]
    pub fn is_ctrctl_cac_ccctl3_acond(&self) -> bool {
        *self == CtrctlCac::CtrctlCacCcctl3Acond
    }
    #[doc = "QEI_2INP"]
    #[inline(always)]
    pub fn is_ctrctl_cac_qei_2inp(&self) -> bool {
        *self == CtrctlCac::CtrctlCacQei2inp
    }
    #[doc = "QEI_3INP"]
    #[inline(always)]
    pub fn is_ctrctl_cac_qei_3inp(&self) -> bool {
        *self == CtrctlCac::CtrctlCacQei3inp
    }
}
#[doc = "Field `CTRCTL_CAC` writer - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CtrctlCacW<'a, REG> = crate::FieldWriter<'a, REG, 3, CtrctlCac>;
impl<'a, REG> CtrctlCacW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCCTL0_ACOND"]
    #[inline(always)]
    pub fn ctrctl_cac_ccctl0_acond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCac::CtrctlCacCcctl0Acond)
    }
    #[doc = "CCCTL1_ACOND"]
    #[inline(always)]
    pub fn ctrctl_cac_ccctl1_acond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCac::CtrctlCacCcctl1Acond)
    }
    #[doc = "CCCTL2_ACOND"]
    #[inline(always)]
    pub fn ctrctl_cac_ccctl2_acond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCac::CtrctlCacCcctl2Acond)
    }
    #[doc = "CCCTL3_ACOND"]
    #[inline(always)]
    pub fn ctrctl_cac_ccctl3_acond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCac::CtrctlCacCcctl3Acond)
    }
    #[doc = "QEI_2INP"]
    #[inline(always)]
    pub fn ctrctl_cac_qei_2inp(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCac::CtrctlCacQei2inp)
    }
    #[doc = "QEI_3INP"]
    #[inline(always)]
    pub fn ctrctl_cac_qei_3inp(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCac::CtrctlCacQei3inp)
    }
}
#[doc = "Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrctlCzc {
    #[doc = "0: CCCTL0_ZCOND"]
    CtrctlCzcCcctl0Zcond = 0,
    #[doc = "1: CCCTL1_ZCOND"]
    CtrctlCzcCcctl1Zcond = 1,
    #[doc = "2: CCCTL2_ZCOND"]
    CtrctlCzcCcctl2Zcond = 2,
    #[doc = "3: CCCTL3_ZCOND"]
    CtrctlCzcCcctl3Zcond = 3,
    #[doc = "4: QEI_2INP"]
    CtrctlCzcQei2inp = 4,
    #[doc = "5: QEI_3INP"]
    CtrctlCzcQei3inp = 5,
}
impl From<CtrctlCzc> for u8 {
    #[inline(always)]
    fn from(variant: CtrctlCzc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrctlCzc {
    type Ux = u8;
}
impl crate::IsEnum for CtrctlCzc {}
#[doc = "Field `CTRCTL_CZC` reader - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CtrctlCzcR = crate::FieldReader<CtrctlCzc>;
impl CtrctlCzcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtrctlCzc> {
        match self.bits {
            0 => Some(CtrctlCzc::CtrctlCzcCcctl0Zcond),
            1 => Some(CtrctlCzc::CtrctlCzcCcctl1Zcond),
            2 => Some(CtrctlCzc::CtrctlCzcCcctl2Zcond),
            3 => Some(CtrctlCzc::CtrctlCzcCcctl3Zcond),
            4 => Some(CtrctlCzc::CtrctlCzcQei2inp),
            5 => Some(CtrctlCzc::CtrctlCzcQei3inp),
            _ => None,
        }
    }
    #[doc = "CCCTL0_ZCOND"]
    #[inline(always)]
    pub fn is_ctrctl_czc_ccctl0_zcond(&self) -> bool {
        *self == CtrctlCzc::CtrctlCzcCcctl0Zcond
    }
    #[doc = "CCCTL1_ZCOND"]
    #[inline(always)]
    pub fn is_ctrctl_czc_ccctl1_zcond(&self) -> bool {
        *self == CtrctlCzc::CtrctlCzcCcctl1Zcond
    }
    #[doc = "CCCTL2_ZCOND"]
    #[inline(always)]
    pub fn is_ctrctl_czc_ccctl2_zcond(&self) -> bool {
        *self == CtrctlCzc::CtrctlCzcCcctl2Zcond
    }
    #[doc = "CCCTL3_ZCOND"]
    #[inline(always)]
    pub fn is_ctrctl_czc_ccctl3_zcond(&self) -> bool {
        *self == CtrctlCzc::CtrctlCzcCcctl3Zcond
    }
    #[doc = "QEI_2INP"]
    #[inline(always)]
    pub fn is_ctrctl_czc_qei_2inp(&self) -> bool {
        *self == CtrctlCzc::CtrctlCzcQei2inp
    }
    #[doc = "QEI_3INP"]
    #[inline(always)]
    pub fn is_ctrctl_czc_qei_3inp(&self) -> bool {
        *self == CtrctlCzc::CtrctlCzcQei3inp
    }
}
#[doc = "Field `CTRCTL_CZC` writer - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
pub type CtrctlCzcW<'a, REG> = crate::FieldWriter<'a, REG, 3, CtrctlCzc>;
impl<'a, REG> CtrctlCzcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CCCTL0_ZCOND"]
    #[inline(always)]
    pub fn ctrctl_czc_ccctl0_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCzc::CtrctlCzcCcctl0Zcond)
    }
    #[doc = "CCCTL1_ZCOND"]
    #[inline(always)]
    pub fn ctrctl_czc_ccctl1_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCzc::CtrctlCzcCcctl1Zcond)
    }
    #[doc = "CCCTL2_ZCOND"]
    #[inline(always)]
    pub fn ctrctl_czc_ccctl2_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCzc::CtrctlCzcCcctl2Zcond)
    }
    #[doc = "CCCTL3_ZCOND"]
    #[inline(always)]
    pub fn ctrctl_czc_ccctl3_zcond(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCzc::CtrctlCzcCcctl3Zcond)
    }
    #[doc = "QEI_2INP"]
    #[inline(always)]
    pub fn ctrctl_czc_qei_2inp(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCzc::CtrctlCzcQei2inp)
    }
    #[doc = "QEI_3INP"]
    #[inline(always)]
    pub fn ctrctl_czc_qei_3inp(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCzc::CtrctlCzcQei3inp)
    }
}
#[doc = "Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrctlDrb {
    #[doc = "0: RESUME"]
    CtrctlDrbResume = 0,
    #[doc = "1: CVAE_ACTION"]
    CtrctlDrbCvaeAction = 1,
}
impl From<CtrctlDrb> for bool {
    #[inline(always)]
    fn from(variant: CtrctlDrb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRCTL_DRB` reader - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
pub type CtrctlDrbR = crate::BitReader<CtrctlDrb>;
impl CtrctlDrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrctlDrb {
        match self.bits {
            false => CtrctlDrb::CtrctlDrbResume,
            true => CtrctlDrb::CtrctlDrbCvaeAction,
        }
    }
    #[doc = "RESUME"]
    #[inline(always)]
    pub fn is_ctrctl_drb_resume(&self) -> bool {
        *self == CtrctlDrb::CtrctlDrbResume
    }
    #[doc = "CVAE_ACTION"]
    #[inline(always)]
    pub fn is_ctrctl_drb_cvae_action(&self) -> bool {
        *self == CtrctlDrb::CtrctlDrbCvaeAction
    }
}
#[doc = "Field `CTRCTL_DRB` writer - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
pub type CtrctlDrbW<'a, REG> = crate::BitWriter<'a, REG, CtrctlDrb>;
impl<'a, REG> CtrctlDrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESUME"]
    #[inline(always)]
    pub fn ctrctl_drb_resume(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlDrb::CtrctlDrbResume)
    }
    #[doc = "CVAE_ACTION"]
    #[inline(always)]
    pub fn ctrctl_drb_cvae_action(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlDrb::CtrctlDrbCvaeAction)
    }
}
#[doc = "Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrctlFb {
    #[doc = "0: CONT_COUNT"]
    CtrctlFbContCount = 0,
    #[doc = "1: SUSP_COUNT"]
    CtrctlFbSuspCount = 1,
}
impl From<CtrctlFb> for bool {
    #[inline(always)]
    fn from(variant: CtrctlFb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRCTL_FB` reader - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
pub type CtrctlFbR = crate::BitReader<CtrctlFb>;
impl CtrctlFbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrctlFb {
        match self.bits {
            false => CtrctlFb::CtrctlFbContCount,
            true => CtrctlFb::CtrctlFbSuspCount,
        }
    }
    #[doc = "CONT_COUNT"]
    #[inline(always)]
    pub fn is_ctrctl_fb_cont_count(&self) -> bool {
        *self == CtrctlFb::CtrctlFbContCount
    }
    #[doc = "SUSP_COUNT"]
    #[inline(always)]
    pub fn is_ctrctl_fb_susp_count(&self) -> bool {
        *self == CtrctlFb::CtrctlFbSuspCount
    }
}
#[doc = "Field `CTRCTL_FB` writer - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
pub type CtrctlFbW<'a, REG> = crate::BitWriter<'a, REG, CtrctlFb>;
impl<'a, REG> CtrctlFbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CONT_COUNT"]
    #[inline(always)]
    pub fn ctrctl_fb_cont_count(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlFb::CtrctlFbContCount)
    }
    #[doc = "SUSP_COUNT"]
    #[inline(always)]
    pub fn ctrctl_fb_susp_count(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlFb::CtrctlFbSuspCount)
    }
}
#[doc = "Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrctlFrb {
    #[doc = "0: RESUME"]
    CtrctlFrbResume = 0,
    #[doc = "1: CVAE_ACTION"]
    CtrctlFrbCvaeAction = 1,
}
impl From<CtrctlFrb> for bool {
    #[inline(always)]
    fn from(variant: CtrctlFrb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRCTL_FRB` reader - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
pub type CtrctlFrbR = crate::BitReader<CtrctlFrb>;
impl CtrctlFrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrctlFrb {
        match self.bits {
            false => CtrctlFrb::CtrctlFrbResume,
            true => CtrctlFrb::CtrctlFrbCvaeAction,
        }
    }
    #[doc = "RESUME"]
    #[inline(always)]
    pub fn is_ctrctl_frb_resume(&self) -> bool {
        *self == CtrctlFrb::CtrctlFrbResume
    }
    #[doc = "CVAE_ACTION"]
    #[inline(always)]
    pub fn is_ctrctl_frb_cvae_action(&self) -> bool {
        *self == CtrctlFrb::CtrctlFrbCvaeAction
    }
}
#[doc = "Field `CTRCTL_FRB` writer - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
pub type CtrctlFrbW<'a, REG> = crate::BitWriter<'a, REG, CtrctlFrb>;
impl<'a, REG> CtrctlFrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RESUME"]
    #[inline(always)]
    pub fn ctrctl_frb_resume(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlFrb::CtrctlFrbResume)
    }
    #[doc = "CVAE_ACTION"]
    #[inline(always)]
    pub fn ctrctl_frb_cvae_action(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlFrb::CtrctlFrbCvaeAction)
    }
}
#[doc = "Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrctlSlzercnez {
    #[doc = "0: DISABLED"]
    CtrctlSlzercnezDisabled = 0,
    #[doc = "1: ENABLED"]
    CtrctlSlzercnezEnabled = 1,
}
impl From<CtrctlSlzercnez> for bool {
    #[inline(always)]
    fn from(variant: CtrctlSlzercnez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRCTL_SLZERCNEZ` reader - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
pub type CtrctlSlzercnezR = crate::BitReader<CtrctlSlzercnez>;
impl CtrctlSlzercnezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrctlSlzercnez {
        match self.bits {
            false => CtrctlSlzercnez::CtrctlSlzercnezDisabled,
            true => CtrctlSlzercnez::CtrctlSlzercnezEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ctrctl_slzercnez_disabled(&self) -> bool {
        *self == CtrctlSlzercnez::CtrctlSlzercnezDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ctrctl_slzercnez_enabled(&self) -> bool {
        *self == CtrctlSlzercnez::CtrctlSlzercnezEnabled
    }
}
#[doc = "Field `CTRCTL_SLZERCNEZ` writer - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
pub type CtrctlSlzercnezW<'a, REG> = crate::BitWriter<'a, REG, CtrctlSlzercnez>;
impl<'a, REG> CtrctlSlzercnezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ctrctl_slzercnez_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlSlzercnez::CtrctlSlzercnezDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ctrctl_slzercnez_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlSlzercnez::CtrctlSlzercnezEnabled)
    }
}
#[doc = "Phase Load Enable. This bit allows the timer to have phase load feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrctlPlen {
    #[doc = "0: DISABLED"]
    CtrctlPlenDisabled = 0,
    #[doc = "1: ENABLED"]
    CtrctlPlenEnabled = 1,
}
impl From<CtrctlPlen> for bool {
    #[inline(always)]
    fn from(variant: CtrctlPlen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRCTL_PLEN` reader - Phase Load Enable. This bit allows the timer to have phase load feature."]
pub type CtrctlPlenR = crate::BitReader<CtrctlPlen>;
impl CtrctlPlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrctlPlen {
        match self.bits {
            false => CtrctlPlen::CtrctlPlenDisabled,
            true => CtrctlPlen::CtrctlPlenEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ctrctl_plen_disabled(&self) -> bool {
        *self == CtrctlPlen::CtrctlPlenDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ctrctl_plen_enabled(&self) -> bool {
        *self == CtrctlPlen::CtrctlPlenEnabled
    }
}
#[doc = "Field `CTRCTL_PLEN` writer - Phase Load Enable. This bit allows the timer to have phase load feature."]
pub type CtrctlPlenW<'a, REG> = crate::BitWriter<'a, REG, CtrctlPlen>;
impl<'a, REG> CtrctlPlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ctrctl_plen_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlPlen::CtrctlPlenDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ctrctl_plen_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlPlen::CtrctlPlenEnabled)
    }
}
#[doc = "Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtrctlCvae {
    #[doc = "0: LDVAL"]
    CtrctlCvaeLdval = 0,
    #[doc = "1: NOCHANGE"]
    CtrctlCvaeNochange = 1,
    #[doc = "2: ZEROVAL"]
    CtrctlCvaeZeroval = 2,
}
impl From<CtrctlCvae> for u8 {
    #[inline(always)]
    fn from(variant: CtrctlCvae) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtrctlCvae {
    type Ux = u8;
}
impl crate::IsEnum for CtrctlCvae {}
#[doc = "Field `CTRCTL_CVAE` reader - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
pub type CtrctlCvaeR = crate::FieldReader<CtrctlCvae>;
impl CtrctlCvaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtrctlCvae> {
        match self.bits {
            0 => Some(CtrctlCvae::CtrctlCvaeLdval),
            1 => Some(CtrctlCvae::CtrctlCvaeNochange),
            2 => Some(CtrctlCvae::CtrctlCvaeZeroval),
            _ => None,
        }
    }
    #[doc = "LDVAL"]
    #[inline(always)]
    pub fn is_ctrctl_cvae_ldval(&self) -> bool {
        *self == CtrctlCvae::CtrctlCvaeLdval
    }
    #[doc = "NOCHANGE"]
    #[inline(always)]
    pub fn is_ctrctl_cvae_nochange(&self) -> bool {
        *self == CtrctlCvae::CtrctlCvaeNochange
    }
    #[doc = "ZEROVAL"]
    #[inline(always)]
    pub fn is_ctrctl_cvae_zeroval(&self) -> bool {
        *self == CtrctlCvae::CtrctlCvaeZeroval
    }
}
#[doc = "Field `CTRCTL_CVAE` writer - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
pub type CtrctlCvaeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtrctlCvae>;
impl<'a, REG> CtrctlCvaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LDVAL"]
    #[inline(always)]
    pub fn ctrctl_cvae_ldval(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCvae::CtrctlCvaeLdval)
    }
    #[doc = "NOCHANGE"]
    #[inline(always)]
    pub fn ctrctl_cvae_nochange(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCvae::CtrctlCvaeNochange)
    }
    #[doc = "ZEROVAL"]
    #[inline(always)]
    pub fn ctrctl_cvae_zeroval(self) -> &'a mut crate::W<REG> {
        self.variant(CtrctlCvae::CtrctlCvaeZeroval)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
    #[inline(always)]
    pub fn ctrctl_en(&self) -> CtrctlEnR {
        CtrctlEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
    #[inline(always)]
    pub fn ctrctl_repeat(&self) -> CtrctlRepeatR {
        CtrctlRepeatR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Count Mode"]
    #[inline(always)]
    pub fn ctrctl_cm(&self) -> CtrctlCmR {
        CtrctlCmR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 7:9 - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn ctrctl_clc(&self) -> CtrctlClcR {
        CtrctlClcR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn ctrctl_cac(&self) -> CtrctlCacR {
        CtrctlCacR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn ctrctl_czc(&self) -> CtrctlCzcR {
        CtrctlCzcR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 17 - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
    #[inline(always)]
    pub fn ctrctl_drb(&self) -> CtrctlDrbR {
        CtrctlDrbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
    #[inline(always)]
    pub fn ctrctl_fb(&self) -> CtrctlFbR {
        CtrctlFbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
    #[inline(always)]
    pub fn ctrctl_frb(&self) -> CtrctlFrbR {
        CtrctlFrbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn ctrctl_slzercnez(&self) -> CtrctlSlzercnezR {
        CtrctlSlzercnezR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Phase Load Enable. This bit allows the timer to have phase load feature."]
    #[inline(always)]
    pub fn ctrctl_plen(&self) -> CtrctlPlenR {
        CtrctlPlenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
    #[inline(always)]
    pub fn ctrctl_cvae(&self) -> CtrctlCvaeR {
        CtrctlCvaeR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable. This bit allows the timer to advance This bit is automatically cleared if REPEAT=0 (do not automatically reload) and the counter value equals zero. CPU Write: A register write that sets the EN bit, the counter value is set per the CVAE value. Hardware: This bit may also be set as the result of an LCOND or ZCOND condition being met and the counter value changed to the load value or zero value, respectively."]
    #[inline(always)]
    pub fn ctrctl_en(&mut self) -> CtrctlEnW<CtrctlSpec> {
        CtrctlEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Repeat. The repeat bit controls whether the counter continues to advance following a zero event, or the exiting of a debug or fault condition. If counting down, a zero event is followed by a load at the next advance condition. If counting up-down, a zero event is followed by an advance event (+1). The intent of encoding 3 is that if the debug condition is in effect, the generation of the load pulse is deferred until the debug condition is over. This allows the counter to reach zero before counting is suspended."]
    #[inline(always)]
    pub fn ctrctl_repeat(&mut self) -> CtrctlRepeatW<CtrctlSpec> {
        CtrctlRepeatW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Count Mode"]
    #[inline(always)]
    pub fn ctrctl_cm(&mut self) -> CtrctlCmW<CtrctlSpec> {
        CtrctlCmW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Counter Load Control. This field specifies what controls the counter operation with respect to setting the counter to the LD register value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn ctrctl_clc(&mut self) -> CtrctlClcW<CtrctlSpec> {
        CtrctlClcW::new(self, 7)
    }
    #[doc = "Bits 10:12 - Counter Advance Control. This field specifies what controls the counter operation with respect to advancing (incrementing or decrementing) the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn ctrctl_cac(&mut self) -> CtrctlCacW<CtrctlSpec> {
        CtrctlCacW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Counter Zero Control This field specifies what controls the counter operation with respect to zeroing the counter value. Encodings 1-3 are present based on the CCPC parameter value. Bits 4-5 are present based on the HQEI parameter value. Any encodings not provided are documented as reserved."]
    #[inline(always)]
    pub fn ctrctl_czc(&mut self) -> CtrctlCzcW<CtrctlSpec> {
        CtrctlCzcW::new(self, 13)
    }
    #[doc = "Bit 17 - Debug Resume Behavior This bit specifies what the device does following the release/exit of debug mode."]
    #[inline(always)]
    pub fn ctrctl_drb(&mut self) -> CtrctlDrbW<CtrctlSpec> {
        CtrctlDrbW::new(self, 17)
    }
    #[doc = "Bit 18 - Fault Behavior This bit specifies whether the counter continues running or suspends during a fault mode. There is a separate control under REPEAT to indicate whether counting is to suspend at next Counter==0"]
    #[inline(always)]
    pub fn ctrctl_fb(&mut self) -> CtrctlFbW<CtrctlSpec> {
        CtrctlFbW::new(self, 18)
    }
    #[doc = "Bit 19 - Fault Resume Behavior This bit specifies what the device does following the release/exit of fault condition."]
    #[inline(always)]
    pub fn ctrctl_frb(&mut self) -> CtrctlFrbW<CtrctlSpec> {
        CtrctlFrbW::new(self, 19)
    }
    #[doc = "Bit 23 - Suppress Load and Zero Events if Repeat Counter is Not Equal to Zero. This bit suppresses the generation of the Z (zero) and L (load) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn ctrctl_slzercnez(&mut self) -> CtrctlSlzercnezW<CtrctlSpec> {
        CtrctlSlzercnezW::new(self, 23)
    }
    #[doc = "Bit 24 - Phase Load Enable. This bit allows the timer to have phase load feature."]
    #[inline(always)]
    pub fn ctrctl_plen(&mut self) -> CtrctlPlenW<CtrctlSpec> {
        CtrctlPlenW::new(self, 24)
    }
    #[doc = "Bits 28:29 - Counter Value After Enable. This field specifies the initialization condition of the counter when the EN bit is changed from 0 to 1 by a write to the CTRCTL register. Note that an external event can also cause the EN bit to go active."]
    #[inline(always)]
    pub fn ctrctl_cvae(&mut self) -> CtrctlCvaeW<CtrctlSpec> {
        CtrctlCvaeW::new(self, 28)
    }
}
#[doc = "Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrctlSpec;
impl crate::RegisterSpec for CtrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrctl::R`](R) reader structure"]
impl crate::Readable for CtrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrctl::W`](W) writer structure"]
impl crate::Writable for CtrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRCTL to value 0xff80"]
impl crate::Resettable for CtrctlSpec {
    const RESET_VALUE: u32 = 0xff80;
}

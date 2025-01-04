#[doc = "Register `CCCTL_45[%s]` reader"]
pub type R = crate::R<Ccctl45Spec>;
#[doc = "Register `CCCTL_45[%s]` writer"]
pub type W = crate::W<Ccctl45Spec>;
#[doc = "Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ccctl45Ccupd {
    #[doc = "0: IMMEDIATELY"]
    Ccctl45CcupdImmediately = 0,
    #[doc = "1: ZERO_EVT"]
    Ccctl45CcupdZeroEvt = 1,
    #[doc = "2: COMPARE_DOWN_EVT"]
    Ccctl45CcupdCompareDownEvt = 2,
    #[doc = "3: COMPARE_UP_EVT"]
    Ccctl45CcupdCompareUpEvt = 3,
    #[doc = "4: ZERO_LOAD_EVT"]
    Ccctl45CcupdZeroLoadEvt = 4,
    #[doc = "5: ZERO_RC_ZERO_EVT"]
    Ccctl45CcupdZeroRcZeroEvt = 5,
    #[doc = "6: TRIG"]
    Ccctl45CcupdTrig = 6,
}
impl From<Ccctl45Ccupd> for u8 {
    #[inline(always)]
    fn from(variant: Ccctl45Ccupd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ccctl45Ccupd {
    type Ux = u8;
}
impl crate::IsEnum for Ccctl45Ccupd {}
#[doc = "Field `CCCTL_45_CCUPD` reader - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type Ccctl45CcupdR = crate::FieldReader<Ccctl45Ccupd>;
impl Ccctl45CcupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ccctl45Ccupd> {
        match self.bits {
            0 => Some(Ccctl45Ccupd::Ccctl45CcupdImmediately),
            1 => Some(Ccctl45Ccupd::Ccctl45CcupdZeroEvt),
            2 => Some(Ccctl45Ccupd::Ccctl45CcupdCompareDownEvt),
            3 => Some(Ccctl45Ccupd::Ccctl45CcupdCompareUpEvt),
            4 => Some(Ccctl45Ccupd::Ccctl45CcupdZeroLoadEvt),
            5 => Some(Ccctl45Ccupd::Ccctl45CcupdZeroRcZeroEvt),
            6 => Some(Ccctl45Ccupd::Ccctl45CcupdTrig),
            _ => None,
        }
    }
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_immediately(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdImmediately
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_zero_evt(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdZeroEvt
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_compare_down_evt(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdCompareDownEvt
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_compare_up_evt(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdCompareUpEvt
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_zero_load_evt(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdZeroLoadEvt
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_zero_rc_zero_evt(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdZeroRcZeroEvt
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn is_ccctl_45_ccupd_trig(&self) -> bool {
        *self == Ccctl45Ccupd::Ccctl45CcupdTrig
    }
}
#[doc = "Field `CCCTL_45_CCUPD` writer - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
pub type Ccctl45CcupdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ccctl45Ccupd>;
impl<'a, REG> Ccctl45CcupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IMMEDIATELY"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_immediately(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdImmediately)
    }
    #[doc = "ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdZeroEvt)
    }
    #[doc = "COMPARE_DOWN_EVT"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_compare_down_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdCompareDownEvt)
    }
    #[doc = "COMPARE_UP_EVT"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_compare_up_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdCompareUpEvt)
    }
    #[doc = "ZERO_LOAD_EVT"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_zero_load_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdZeroLoadEvt)
    }
    #[doc = "ZERO_RC_ZERO_EVT"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_zero_rc_zero_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdZeroRcZeroEvt)
    }
    #[doc = "TRIG"]
    #[inline(always)]
    pub fn ccctl_45_ccupd_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Ccupd::Ccctl45CcupdTrig)
    }
}
#[doc = "Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccctl45Scercnez {
    #[doc = "0: DISABLED"]
    Ccctl45ScercnezDisabled = 0,
    #[doc = "1: ENABLED"]
    Ccctl45ScercnezEnabled = 1,
}
impl From<Ccctl45Scercnez> for bool {
    #[inline(always)]
    fn from(variant: Ccctl45Scercnez) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCTL_45_SCERCNEZ` reader - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
pub type Ccctl45ScercnezR = crate::BitReader<Ccctl45Scercnez>;
impl Ccctl45ScercnezR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccctl45Scercnez {
        match self.bits {
            false => Ccctl45Scercnez::Ccctl45ScercnezDisabled,
            true => Ccctl45Scercnez::Ccctl45ScercnezEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ccctl_45_scercnez_disabled(&self) -> bool {
        *self == Ccctl45Scercnez::Ccctl45ScercnezDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ccctl_45_scercnez_enabled(&self) -> bool {
        *self == Ccctl45Scercnez::Ccctl45ScercnezEnabled
    }
}
#[doc = "Field `CCCTL_45_SCERCNEZ` writer - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
pub type Ccctl45ScercnezW<'a, REG> = crate::BitWriter<'a, REG, Ccctl45Scercnez>;
impl<'a, REG> Ccctl45ScercnezW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ccctl_45_scercnez_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Scercnez::Ccctl45ScercnezDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ccctl_45_scercnez_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccctl45Scercnez::Ccctl45ScercnezEnabled)
    }
}
impl R {
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccctl_45_ccupd(&self) -> Ccctl45CcupdR {
        Ccctl45CcupdR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn ccctl_45_scercnez(&self) -> Ccctl45ScercnezR {
        Ccctl45ScercnezR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:20 - Capture and Compare Update Method This field controls how updates to the pipelined capture and compare register are performed (when operating in compare mode, COC=0)."]
    #[inline(always)]
    pub fn ccctl_45_ccupd(&mut self) -> Ccctl45CcupdW<Ccctl45Spec> {
        Ccctl45CcupdW::new(self, 18)
    }
    #[doc = "Bit 25 - Suppress Compare Event if Repeat Counter is Not Equal to Zero This bit suppresses the generation of the compare (CCD, CCU and RC) events from the counter when the repeat counter (RC) value is not 0."]
    #[inline(always)]
    pub fn ccctl_45_scercnez(&mut self) -> Ccctl45ScercnezW<Ccctl45Spec> {
        Ccctl45ScercnezW::new(self, 25)
    }
}
#[doc = "Capture or Compare Control Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`ccctl_45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccctl_45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccctl45Spec;
impl crate::RegisterSpec for Ccctl45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccctl_45::R`](R) reader structure"]
impl crate::Readable for Ccctl45Spec {}
#[doc = "`write(|w| ..)` method takes [`ccctl_45::W`](W) writer structure"]
impl crate::Writable for Ccctl45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCCTL_45[%s]
to value 0"]
impl crate::Resettable for Ccctl45Spec {
    const RESET_VALUE: u32 = 0;
}

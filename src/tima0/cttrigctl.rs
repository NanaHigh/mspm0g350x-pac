#[doc = "Register `CTTRIGCTL` reader"]
pub type R = crate::R<CttrigctlSpec>;
#[doc = "Register `CTTRIGCTL` writer"]
pub type W = crate::W<CttrigctlSpec>;
#[doc = "Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CttrigctlCten {
    #[doc = "0: DISABLED"]
    CttrigctlCtenDisabled = 0,
    #[doc = "1: ENABLE"]
    CttrigctlCtenEnable = 1,
}
impl From<CttrigctlCten> for bool {
    #[inline(always)]
    fn from(variant: CttrigctlCten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTRIGCTL_CTEN` reader - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
pub type CttrigctlCtenR = crate::BitReader<CttrigctlCten>;
impl CttrigctlCtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CttrigctlCten {
        match self.bits {
            false => CttrigctlCten::CttrigctlCtenDisabled,
            true => CttrigctlCten::CttrigctlCtenEnable,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_cttrigctl_cten_disabled(&self) -> bool {
        *self == CttrigctlCten::CttrigctlCtenDisabled
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_cttrigctl_cten_enable(&self) -> bool {
        *self == CttrigctlCten::CttrigctlCtenEnable
    }
}
#[doc = "Field `CTTRIGCTL_CTEN` writer - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
pub type CttrigctlCtenW<'a, REG> = crate::BitWriter<'a, REG, CttrigctlCten>;
impl<'a, REG> CttrigctlCtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn cttrigctl_cten_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlCten::CttrigctlCtenDisabled)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn cttrigctl_cten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlCten::CttrigctlCtenEnable)
    }
}
#[doc = "Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers. Refer Figure 8 Cross Trigger Generation Path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CttrigctlEvtcten {
    #[doc = "0: DISABLED"]
    CttrigctlEvtctenDisabled = 0,
    #[doc = "1: ENABLE"]
    CttrigctlEvtctenEnable = 1,
}
impl From<CttrigctlEvtcten> for bool {
    #[inline(always)]
    fn from(variant: CttrigctlEvtcten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTTRIGCTL_EVTCTEN` reader - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers. Refer Figure 8 Cross Trigger Generation Path"]
pub type CttrigctlEvtctenR = crate::BitReader<CttrigctlEvtcten>;
impl CttrigctlEvtctenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CttrigctlEvtcten {
        match self.bits {
            false => CttrigctlEvtcten::CttrigctlEvtctenDisabled,
            true => CttrigctlEvtcten::CttrigctlEvtctenEnable,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcten_disabled(&self) -> bool {
        *self == CttrigctlEvtcten::CttrigctlEvtctenDisabled
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcten_enable(&self) -> bool {
        *self == CttrigctlEvtcten::CttrigctlEvtctenEnable
    }
}
#[doc = "Field `CTTRIGCTL_EVTCTEN` writer - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers. Refer Figure 8 Cross Trigger Generation Path"]
pub type CttrigctlEvtctenW<'a, REG> = crate::BitWriter<'a, REG, CttrigctlEvtcten>;
impl<'a, REG> CttrigctlEvtctenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn cttrigctl_evtcten_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcten::CttrigctlEvtctenDisabled)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn cttrigctl_evtcten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcten::CttrigctlEvtctenEnable)
    }
}
#[doc = "Used to Select the subscriber port that should be used for input cross trigger. Refer Figure 8 Cross Trigger Generation Path\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CttrigctlEvtcttrigsel {
    #[doc = "0: FSUB0"]
    CttrigctlEvtcttrigselFsub0 = 0,
    #[doc = "1: FSUB1"]
    CttrigctlEvtcttrigselFsub1 = 1,
    #[doc = "2: Z"]
    CttrigctlEvtcttrigselZ = 2,
    #[doc = "3: L"]
    CttrigctlEvtcttrigselL = 3,
    #[doc = "4: CCD0"]
    CttrigctlEvtcttrigselCcd0 = 4,
    #[doc = "5: CCD1"]
    CttrigctlEvtcttrigselCcd1 = 5,
    #[doc = "6: CCD2"]
    CttrigctlEvtcttrigselCcd2 = 6,
    #[doc = "7: CCD3"]
    CttrigctlEvtcttrigselCcd3 = 7,
    #[doc = "8: CCU0"]
    CttrigctlEvtcttrigselCcu0 = 8,
    #[doc = "9: CCU1"]
    CttrigctlEvtcttrigselCcu1 = 9,
    #[doc = "10: CCU2"]
    CttrigctlEvtcttrigselCcu2 = 10,
    #[doc = "11: CCU3"]
    CttrigctlEvtcttrigselCcu3 = 11,
}
impl From<CttrigctlEvtcttrigsel> for u8 {
    #[inline(always)]
    fn from(variant: CttrigctlEvtcttrigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CttrigctlEvtcttrigsel {
    type Ux = u8;
}
impl crate::IsEnum for CttrigctlEvtcttrigsel {}
#[doc = "Field `CTTRIGCTL_EVTCTTRIGSEL` reader - Used to Select the subscriber port that should be used for input cross trigger. Refer Figure 8 Cross Trigger Generation Path"]
pub type CttrigctlEvtcttrigselR = crate::FieldReader<CttrigctlEvtcttrigsel>;
impl CttrigctlEvtcttrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CttrigctlEvtcttrigsel> {
        match self.bits {
            0 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselFsub0),
            1 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselFsub1),
            2 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselZ),
            3 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselL),
            4 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd0),
            5 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd1),
            6 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd2),
            7 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd3),
            8 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu0),
            9 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu1),
            10 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu2),
            11 => Some(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu3),
            _ => None,
        }
    }
    #[doc = "FSUB0"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_fsub0(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselFsub0
    }
    #[doc = "FSUB1"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_fsub1(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselFsub1
    }
    #[doc = "Z"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_z(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselZ
    }
    #[doc = "L"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_l(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselL
    }
    #[doc = "CCD0"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccd0(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd0
    }
    #[doc = "CCD1"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccd1(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd1
    }
    #[doc = "CCD2"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccd2(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd2
    }
    #[doc = "CCD3"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccd3(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd3
    }
    #[doc = "CCU0"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccu0(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu0
    }
    #[doc = "CCU1"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccu1(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu1
    }
    #[doc = "CCU2"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccu2(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu2
    }
    #[doc = "CCU3"]
    #[inline(always)]
    pub fn is_cttrigctl_evtcttrigsel_ccu3(&self) -> bool {
        *self == CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu3
    }
}
#[doc = "Field `CTTRIGCTL_EVTCTTRIGSEL` writer - Used to Select the subscriber port that should be used for input cross trigger. Refer Figure 8 Cross Trigger Generation Path"]
pub type CttrigctlEvtcttrigselW<'a, REG> = crate::FieldWriter<'a, REG, 4, CttrigctlEvtcttrigsel>;
impl<'a, REG> CttrigctlEvtcttrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FSUB0"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_fsub0(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselFsub0)
    }
    #[doc = "FSUB1"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_fsub1(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselFsub1)
    }
    #[doc = "Z"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_z(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselZ)
    }
    #[doc = "L"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_l(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselL)
    }
    #[doc = "CCD0"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccd0(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd0)
    }
    #[doc = "CCD1"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccd1(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd1)
    }
    #[doc = "CCD2"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccd2(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd2)
    }
    #[doc = "CCD3"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccd3(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcd3)
    }
    #[doc = "CCU0"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccu0(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu0)
    }
    #[doc = "CCU1"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccu1(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu1)
    }
    #[doc = "CCU2"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccu2(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu2)
    }
    #[doc = "CCU3"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel_ccu3(self) -> &'a mut crate::W<REG> {
        self.variant(CttrigctlEvtcttrigsel::CttrigctlEvtcttrigselCcu3)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
    #[inline(always)]
    pub fn cttrigctl_cten(&self) -> CttrigctlCtenR {
        CttrigctlCtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers. Refer Figure 8 Cross Trigger Generation Path"]
    #[inline(always)]
    pub fn cttrigctl_evtcten(&self) -> CttrigctlEvtctenR {
        CttrigctlEvtctenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Used to Select the subscriber port that should be used for input cross trigger. Refer Figure 8 Cross Trigger Generation Path"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel(&self) -> CttrigctlEvtcttrigselR {
        CttrigctlEvtcttrigselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Cross trigger enable. This field is used to enable whether the SW or HW logic can generate a timer cross trigger event in the system. These cross triggers are connected to the respective timer trigger in of the other timer IPs in the SOC power domain. The timer cross trigger is essentially the combined logic of the HW and SW conditions controlling EN bit in the CTRCTL register."]
    #[inline(always)]
    pub fn cttrigctl_cten(&mut self) -> CttrigctlCtenW<CttrigctlSpec> {
        CttrigctlCtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the Input Trigger Conditions to the Timer module as a condition for Cross Triggers. Refer Figure 8 Cross Trigger Generation Path"]
    #[inline(always)]
    pub fn cttrigctl_evtcten(&mut self) -> CttrigctlEvtctenW<CttrigctlSpec> {
        CttrigctlEvtctenW::new(self, 1)
    }
    #[doc = "Bits 16:19 - Used to Select the subscriber port that should be used for input cross trigger. Refer Figure 8 Cross Trigger Generation Path"]
    #[inline(always)]
    pub fn cttrigctl_evtcttrigsel(&mut self) -> CttrigctlEvtcttrigselW<CttrigctlSpec> {
        CttrigctlEvtcttrigselW::new(self, 16)
    }
}
#[doc = "Timer Cross Trigger Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cttrigctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cttrigctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CttrigctlSpec;
impl crate::RegisterSpec for CttrigctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cttrigctl::R`](R) reader structure"]
impl crate::Readable for CttrigctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cttrigctl::W`](W) writer structure"]
impl crate::Writable for CttrigctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTTRIGCTL to value 0"]
impl crate::Resettable for CttrigctlSpec {
    const RESET_VALUE: u32 = 0;
}

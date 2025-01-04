#[doc = "Register `SYSOSCCFG` reader"]
pub type R = crate::R<SysosccfgSpec>;
#[doc = "Register `SYSOSCCFG` writer"]
pub type W = crate::W<SysosccfgSpec>;
#[doc = "Target operating frequency for the system oscillator (SYSOSC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysosccfgFreq {
    #[doc = "0: SYSOSCBASE"]
    SysosccfgFreqSysoscbase = 0,
    #[doc = "1: SYSOSC4M"]
    SysosccfgFreqSysosc4m = 1,
    #[doc = "2: SYSOSCUSER"]
    SysosccfgFreqSysoscuser = 2,
}
impl From<SysosccfgFreq> for u8 {
    #[inline(always)]
    fn from(variant: SysosccfgFreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysosccfgFreq {
    type Ux = u8;
}
impl crate::IsEnum for SysosccfgFreq {}
#[doc = "Field `SYSOSCCFG_FREQ` reader - Target operating frequency for the system oscillator (SYSOSC)"]
pub type SysosccfgFreqR = crate::FieldReader<SysosccfgFreq>;
impl SysosccfgFreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SysosccfgFreq> {
        match self.bits {
            0 => Some(SysosccfgFreq::SysosccfgFreqSysoscbase),
            1 => Some(SysosccfgFreq::SysosccfgFreqSysosc4m),
            2 => Some(SysosccfgFreq::SysosccfgFreqSysoscuser),
            _ => None,
        }
    }
    #[doc = "SYSOSCBASE"]
    #[inline(always)]
    pub fn is_sysosccfg_freq_sysoscbase(&self) -> bool {
        *self == SysosccfgFreq::SysosccfgFreqSysoscbase
    }
    #[doc = "SYSOSC4M"]
    #[inline(always)]
    pub fn is_sysosccfg_freq_sysosc4m(&self) -> bool {
        *self == SysosccfgFreq::SysosccfgFreqSysosc4m
    }
    #[doc = "SYSOSCUSER"]
    #[inline(always)]
    pub fn is_sysosccfg_freq_sysoscuser(&self) -> bool {
        *self == SysosccfgFreq::SysosccfgFreqSysoscuser
    }
}
#[doc = "Field `SYSOSCCFG_FREQ` writer - Target operating frequency for the system oscillator (SYSOSC)"]
pub type SysosccfgFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysosccfgFreq>;
impl<'a, REG> SysosccfgFreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSOSCBASE"]
    #[inline(always)]
    pub fn sysosccfg_freq_sysoscbase(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgFreq::SysosccfgFreqSysoscbase)
    }
    #[doc = "SYSOSC4M"]
    #[inline(always)]
    pub fn sysosccfg_freq_sysosc4m(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgFreq::SysosccfgFreqSysosc4m)
    }
    #[doc = "SYSOSCUSER"]
    #[inline(always)]
    pub fn sysosccfg_freq_sysoscuser(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgFreq::SysosccfgFreqSysoscuser)
    }
}
#[doc = "USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysosccfgUse4mhzstop {
    #[doc = "0: DISABLE"]
    SysosccfgUse4mhzstopDisable = 0,
    #[doc = "1: ENABLE"]
    SysosccfgUse4mhzstopEnable = 1,
}
impl From<SysosccfgUse4mhzstop> for bool {
    #[inline(always)]
    fn from(variant: SysosccfgUse4mhzstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCCFG_USE4MHZSTOP` reader - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
pub type SysosccfgUse4mhzstopR = crate::BitReader<SysosccfgUse4mhzstop>;
impl SysosccfgUse4mhzstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysosccfgUse4mhzstop {
        match self.bits {
            false => SysosccfgUse4mhzstop::SysosccfgUse4mhzstopDisable,
            true => SysosccfgUse4mhzstop::SysosccfgUse4mhzstopEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_use4mhzstop_disable(&self) -> bool {
        *self == SysosccfgUse4mhzstop::SysosccfgUse4mhzstopDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_use4mhzstop_enable(&self) -> bool {
        *self == SysosccfgUse4mhzstop::SysosccfgUse4mhzstopEnable
    }
}
#[doc = "Field `SYSOSCCFG_USE4MHZSTOP` writer - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
pub type SysosccfgUse4mhzstopW<'a, REG> = crate::BitWriter<'a, REG, SysosccfgUse4mhzstop>;
impl<'a, REG> SysosccfgUse4mhzstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sysosccfg_use4mhzstop_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgUse4mhzstop::SysosccfgUse4mhzstopDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sysosccfg_use4mhzstop_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgUse4mhzstop::SysosccfgUse4mhzstopEnable)
    }
}
#[doc = "DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysosccfgDisablestop {
    #[doc = "0: DISABLE"]
    SysosccfgDisablestopDisable = 0,
    #[doc = "1: ENABLE"]
    SysosccfgDisablestopEnable = 1,
}
impl From<SysosccfgDisablestop> for bool {
    #[inline(always)]
    fn from(variant: SysosccfgDisablestop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCCFG_DISABLESTOP` reader - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
pub type SysosccfgDisablestopR = crate::BitReader<SysosccfgDisablestop>;
impl SysosccfgDisablestopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysosccfgDisablestop {
        match self.bits {
            false => SysosccfgDisablestop::SysosccfgDisablestopDisable,
            true => SysosccfgDisablestop::SysosccfgDisablestopEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_disablestop_disable(&self) -> bool {
        *self == SysosccfgDisablestop::SysosccfgDisablestopDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_disablestop_enable(&self) -> bool {
        *self == SysosccfgDisablestop::SysosccfgDisablestopEnable
    }
}
#[doc = "Field `SYSOSCCFG_DISABLESTOP` writer - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
pub type SysosccfgDisablestopW<'a, REG> = crate::BitWriter<'a, REG, SysosccfgDisablestop>;
impl<'a, REG> SysosccfgDisablestopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sysosccfg_disablestop_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgDisablestop::SysosccfgDisablestopDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sysosccfg_disablestop_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgDisablestop::SysosccfgDisablestopEnable)
    }
}
#[doc = "DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysosccfgDisable {
    #[doc = "0: DISABLE"]
    SysosccfgDisableDisable = 0,
    #[doc = "1: ENABLE"]
    SysosccfgDisableEnable = 1,
}
impl From<SysosccfgDisable> for bool {
    #[inline(always)]
    fn from(variant: SysosccfgDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCCFG_DISABLE` reader - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
pub type SysosccfgDisableR = crate::BitReader<SysosccfgDisable>;
impl SysosccfgDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysosccfgDisable {
        match self.bits {
            false => SysosccfgDisable::SysosccfgDisableDisable,
            true => SysosccfgDisable::SysosccfgDisableEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_disable_disable(&self) -> bool {
        *self == SysosccfgDisable::SysosccfgDisableDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_disable_enable(&self) -> bool {
        *self == SysosccfgDisable::SysosccfgDisableEnable
    }
}
#[doc = "Field `SYSOSCCFG_DISABLE` writer - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
pub type SysosccfgDisableW<'a, REG> = crate::BitWriter<'a, REG, SysosccfgDisable>;
impl<'a, REG> SysosccfgDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sysosccfg_disable_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgDisable::SysosccfgDisableDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sysosccfg_disable_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgDisable::SysosccfgDisableEnable)
    }
}
#[doc = "BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysosccfgBlockasyncall {
    #[doc = "0: DISABLE"]
    SysosccfgBlockasyncallDisable = 0,
    #[doc = "1: ENABLE"]
    SysosccfgBlockasyncallEnable = 1,
}
impl From<SysosccfgBlockasyncall> for bool {
    #[inline(always)]
    fn from(variant: SysosccfgBlockasyncall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCCFG_BLOCKASYNCALL` reader - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
pub type SysosccfgBlockasyncallR = crate::BitReader<SysosccfgBlockasyncall>;
impl SysosccfgBlockasyncallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysosccfgBlockasyncall {
        match self.bits {
            false => SysosccfgBlockasyncall::SysosccfgBlockasyncallDisable,
            true => SysosccfgBlockasyncall::SysosccfgBlockasyncallEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_blockasyncall_disable(&self) -> bool {
        *self == SysosccfgBlockasyncall::SysosccfgBlockasyncallDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_blockasyncall_enable(&self) -> bool {
        *self == SysosccfgBlockasyncall::SysosccfgBlockasyncallEnable
    }
}
#[doc = "Field `SYSOSCCFG_BLOCKASYNCALL` writer - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
pub type SysosccfgBlockasyncallW<'a, REG> = crate::BitWriter<'a, REG, SysosccfgBlockasyncall>;
impl<'a, REG> SysosccfgBlockasyncallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sysosccfg_blockasyncall_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgBlockasyncall::SysosccfgBlockasyncallDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sysosccfg_blockasyncall_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgBlockasyncall::SysosccfgBlockasyncallEnable)
    }
}
#[doc = "FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysosccfgFastcpuevent {
    #[doc = "0: DISABLE"]
    SysosccfgFastcpueventDisable = 0,
    #[doc = "1: ENABLE"]
    SysosccfgFastcpueventEnable = 1,
}
impl From<SysosccfgFastcpuevent> for bool {
    #[inline(always)]
    fn from(variant: SysosccfgFastcpuevent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCCFG_FASTCPUEVENT` reader - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
pub type SysosccfgFastcpueventR = crate::BitReader<SysosccfgFastcpuevent>;
impl SysosccfgFastcpueventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SysosccfgFastcpuevent {
        match self.bits {
            false => SysosccfgFastcpuevent::SysosccfgFastcpueventDisable,
            true => SysosccfgFastcpuevent::SysosccfgFastcpueventEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_fastcpuevent_disable(&self) -> bool {
        *self == SysosccfgFastcpuevent::SysosccfgFastcpueventDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sysosccfg_fastcpuevent_enable(&self) -> bool {
        *self == SysosccfgFastcpuevent::SysosccfgFastcpueventEnable
    }
}
#[doc = "Field `SYSOSCCFG_FASTCPUEVENT` writer - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
pub type SysosccfgFastcpueventW<'a, REG> = crate::BitWriter<'a, REG, SysosccfgFastcpuevent>;
impl<'a, REG> SysosccfgFastcpueventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sysosccfg_fastcpuevent_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgFastcpuevent::SysosccfgFastcpueventDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sysosccfg_fastcpuevent_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SysosccfgFastcpuevent::SysosccfgFastcpueventEnable)
    }
}
impl R {
    #[doc = "Bits 0:1 - Target operating frequency for the system oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn sysosccfg_freq(&self) -> SysosccfgFreqR {
        SysosccfgFreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
    #[inline(always)]
    pub fn sysosccfg_use4mhzstop(&self) -> SysosccfgUse4mhzstopR {
        SysosccfgUse4mhzstopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
    #[inline(always)]
    pub fn sysosccfg_disablestop(&self) -> SysosccfgDisablestopR {
        SysosccfgDisablestopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
    #[inline(always)]
    pub fn sysosccfg_disable(&self) -> SysosccfgDisableR {
        SysosccfgDisableR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
    #[inline(always)]
    pub fn sysosccfg_blockasyncall(&self) -> SysosccfgBlockasyncallR {
        SysosccfgBlockasyncallR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
    #[inline(always)]
    pub fn sysosccfg_fastcpuevent(&self) -> SysosccfgFastcpueventR {
        SysosccfgFastcpueventR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Target operating frequency for the system oscillator (SYSOSC)"]
    #[inline(always)]
    pub fn sysosccfg_freq(&mut self) -> SysosccfgFreqW<SysosccfgSpec> {
        SysosccfgFreqW::new(self, 0)
    }
    #[doc = "Bit 8 - USE4MHZSTOP sets the SYSOSC stop mode frequency policy. When entering STOP mode, the SYSOSC frequency may be automatically switched to 4MHz to reduce SYSOSC power consumption."]
    #[inline(always)]
    pub fn sysosccfg_use4mhzstop(&mut self) -> SysosccfgUse4mhzstopW<SysosccfgSpec> {
        SysosccfgUse4mhzstopW::new(self, 8)
    }
    #[doc = "Bit 9 - DISABLESTOP sets the SYSOSC stop mode enable/disable policy. When operating in STOP mode, the SYSOSC may be automatically disabled. When set, ULPCLK will run from LFCLK in STOP mode and SYSOSC will be disabled to reduce power consumption."]
    #[inline(always)]
    pub fn sysosccfg_disablestop(&mut self) -> SysosccfgDisablestopW<SysosccfgSpec> {
        SysosccfgDisablestopW::new(self, 9)
    }
    #[doc = "Bit 10 - DISABLE sets the SYSOSC enable/disable policy. SYSOSC may be powered off in RUN, SLEEP, and STOP modes to reduce power consumption. When SYSOSC is disabled, MCLK and ULPCLK are sourced from LFCLK."]
    #[inline(always)]
    pub fn sysosccfg_disable(&mut self) -> SysosccfgDisableW<SysosccfgSpec> {
        SysosccfgDisableW::new(self, 10)
    }
    #[doc = "Bit 16 - BLOCKASYNCALL may be used to mask block all asynchronous fast clock requests, preventing hardware from dynamically changing the active clock configuration when operating in a given mode."]
    #[inline(always)]
    pub fn sysosccfg_blockasyncall(&mut self) -> SysosccfgBlockasyncallW<SysosccfgSpec> {
        SysosccfgBlockasyncallW::new(self, 16)
    }
    #[doc = "Bit 17 - FASTCPUEVENT may be used to assert a fast clock request when an interrupt is asserted to the CPU, reducing interrupt latency."]
    #[inline(always)]
    pub fn sysosccfg_fastcpuevent(&mut self) -> SysosccfgFastcpueventW<SysosccfgSpec> {
        SysosccfgFastcpueventW::new(self, 17)
    }
}
#[doc = "SYSOSC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sysosccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysosccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysosccfgSpec;
impl crate::RegisterSpec for SysosccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysosccfg::R`](R) reader structure"]
impl crate::Readable for SysosccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysosccfg::W`](W) writer structure"]
impl crate::Writable for SysosccfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSOSCCFG to value 0x0002_0000"]
impl crate::Resettable for SysosccfgSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}

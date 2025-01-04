#[doc = "Register `SYSOSCFCLCTL` writer"]
pub type W = crate::W<SysoscfclctlSpec>;
#[doc = "Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysoscfclctlSetusefcl {
    #[doc = "1: TRUE"]
    SysoscfclctlSetusefclTrue = 1,
}
impl From<SysoscfclctlSetusefcl> for bool {
    #[inline(always)]
    fn from(variant: SysoscfclctlSetusefcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCFCLCTL_SETUSEFCL` writer - Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST."]
pub type SysoscfclctlSetusefclW<'a, REG> = crate::BitWriter<'a, REG, SysoscfclctlSetusefcl>;
impl<'a, REG> SysoscfclctlSetusefclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn sysoscfclctl_setusefcl_true(self) -> &'a mut crate::W<REG> {
        self.variant(SysoscfclctlSetusefcl::SysoscfclctlSetusefclTrue)
    }
}
#[doc = "Set SETUSEEXRES to specify that an external resistor will be used for the FCL. An appropriate resistor must be populated on the ROSC pin. This state is locked until the next BOOTRST.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysoscfclctlSetuseexres {
    #[doc = "1: TRUE"]
    SysoscfclctlSetuseexresTrue = 1,
}
impl From<SysoscfclctlSetuseexres> for bool {
    #[inline(always)]
    fn from(variant: SysoscfclctlSetuseexres) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSCFCLCTL_SETUSEEXRES` writer - Set SETUSEEXRES to specify that an external resistor will be used for the FCL. An appropriate resistor must be populated on the ROSC pin. This state is locked until the next BOOTRST."]
pub type SysoscfclctlSetuseexresW<'a, REG> = crate::BitWriter<'a, REG, SysoscfclctlSetuseexres>;
impl<'a, REG> SysoscfclctlSetuseexresW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn sysoscfclctl_setuseexres_true(self) -> &'a mut crate::W<REG> {
        self.variant(SysoscfclctlSetuseexres::SysoscfclctlSetuseexresTrue)
    }
}
#[doc = "The key value of 2Ah (42) must be written to KEY together with SETUSEFCL to enable the FCL.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysoscfclctlKey {
    #[doc = "42: VALUE"]
    SysoscfclctlKeyValue = 42,
}
impl From<SysoscfclctlKey> for u8 {
    #[inline(always)]
    fn from(variant: SysoscfclctlKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysoscfclctlKey {
    type Ux = u8;
}
impl crate::IsEnum for SysoscfclctlKey {}
#[doc = "Field `SYSOSCFCLCTL_KEY` writer - The key value of 2Ah (42) must be written to KEY together with SETUSEFCL to enable the FCL."]
pub type SysoscfclctlKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, SysoscfclctlKey>;
impl<'a, REG> SysoscfclctlKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn sysoscfclctl_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(SysoscfclctlKey::SysoscfclctlKeyValue)
    }
}
impl W {
    #[doc = "Bit 0 - Set SETUSEFCL to enable the frequency correction loop in SYSOSC. Once enabled, this state is locked until the next BOOTRST."]
    #[inline(always)]
    pub fn sysoscfclctl_setusefcl(&mut self) -> SysoscfclctlSetusefclW<SysoscfclctlSpec> {
        SysoscfclctlSetusefclW::new(self, 0)
    }
    #[doc = "Bit 1 - Set SETUSEEXRES to specify that an external resistor will be used for the FCL. An appropriate resistor must be populated on the ROSC pin. This state is locked until the next BOOTRST."]
    #[inline(always)]
    pub fn sysoscfclctl_setuseexres(&mut self) -> SysoscfclctlSetuseexresW<SysoscfclctlSpec> {
        SysoscfclctlSetuseexresW::new(self, 1)
    }
    #[doc = "Bits 24:31 - The key value of 2Ah (42) must be written to KEY together with SETUSEFCL to enable the FCL."]
    #[inline(always)]
    pub fn sysoscfclctl_key(&mut self) -> SysoscfclctlKeyW<SysoscfclctlSpec> {
        SysoscfclctlKeyW::new(self, 24)
    }
}
#[doc = "SYSOSC frequency correction loop (FCL) ROSC enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysoscfclctl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysoscfclctlSpec;
impl crate::RegisterSpec for SysoscfclctlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sysoscfclctl::W`](W) writer structure"]
impl crate::Writable for SysoscfclctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSOSCFCLCTL to value 0"]
impl crate::Resettable for SysoscfclctlSpec {
    const RESET_VALUE: u32 = 0;
}

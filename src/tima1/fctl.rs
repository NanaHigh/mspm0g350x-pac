#[doc = "Register `FCTL` reader"]
pub type R = crate::R<FctlSpec>;
#[doc = "Register `FCTL` writer"]
pub type W = crate::W<FctlSpec>;
#[doc = "Fault Input Enable This bit enables the input for fault detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFien {
    #[doc = "0: DISABLED"]
    FctlFienDisabled = 0,
    #[doc = "1: ENABLED"]
    FctlFienEnabled = 1,
}
impl From<FctlFien> for bool {
    #[inline(always)]
    fn from(variant: FctlFien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FIEN` reader - Fault Input Enable This bit enables the input for fault detection."]
pub type FctlFienR = crate::BitReader<FctlFien>;
impl FctlFienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFien {
        match self.bits {
            false => FctlFien::FctlFienDisabled,
            true => FctlFien::FctlFienEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_fctl_fien_disabled(&self) -> bool {
        *self == FctlFien::FctlFienDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_fctl_fien_enabled(&self) -> bool {
        *self == FctlFien::FctlFienEnabled
    }
}
#[doc = "Field `FCTL_FIEN` writer - Fault Input Enable This bit enables the input for fault detection."]
pub type FctlFienW<'a, REG> = crate::BitWriter<'a, REG, FctlFien>;
impl<'a, REG> FctlFienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn fctl_fien_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFien::FctlFienDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn fctl_fien_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFien::FctlFienEnabled)
    }
}
#[doc = "Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFi {
    #[doc = "0: INDEPENDENT"]
    FctlFiIndependent = 0,
    #[doc = "1: DEPENDENT"]
    FctlFiDependent = 1,
}
impl From<FctlFi> for bool {
    #[inline(always)]
    fn from(variant: FctlFi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FI` reader - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
pub type FctlFiR = crate::BitReader<FctlFi>;
impl FctlFiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFi {
        match self.bits {
            false => FctlFi::FctlFiIndependent,
            true => FctlFi::FctlFiDependent,
        }
    }
    #[doc = "INDEPENDENT"]
    #[inline(always)]
    pub fn is_fctl_fi_independent(&self) -> bool {
        *self == FctlFi::FctlFiIndependent
    }
    #[doc = "DEPENDENT"]
    #[inline(always)]
    pub fn is_fctl_fi_dependent(&self) -> bool {
        *self == FctlFi::FctlFiDependent
    }
}
#[doc = "Field `FCTL_FI` writer - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
pub type FctlFiW<'a, REG> = crate::BitWriter<'a, REG, FctlFi>;
impl<'a, REG> FctlFiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INDEPENDENT"]
    #[inline(always)]
    pub fn fctl_fi_independent(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFi::FctlFiIndependent)
    }
    #[doc = "DEPENDENT"]
    #[inline(always)]
    pub fn fctl_fi_dependent(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFi::FctlFiDependent)
    }
}
#[doc = "Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FctlFl {
    #[doc = "0: NO_LATCH"]
    FctlFlNoLatch = 0,
    #[doc = "1: LATCH_SW_CLR"]
    FctlFlLatchSwClr = 1,
    #[doc = "2: LATCH_Z_CLR"]
    FctlFlLatchZClr = 2,
    #[doc = "3: LATCH_LD_CLR"]
    FctlFlLatchLdClr = 3,
}
impl From<FctlFl> for u8 {
    #[inline(always)]
    fn from(variant: FctlFl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FctlFl {
    type Ux = u8;
}
impl crate::IsEnum for FctlFl {}
#[doc = "Field `FCTL_FL` reader - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
pub type FctlFlR = crate::FieldReader<FctlFl>;
impl FctlFlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFl {
        match self.bits {
            0 => FctlFl::FctlFlNoLatch,
            1 => FctlFl::FctlFlLatchSwClr,
            2 => FctlFl::FctlFlLatchZClr,
            3 => FctlFl::FctlFlLatchLdClr,
            _ => unreachable!(),
        }
    }
    #[doc = "NO_LATCH"]
    #[inline(always)]
    pub fn is_fctl_fl_no_latch(&self) -> bool {
        *self == FctlFl::FctlFlNoLatch
    }
    #[doc = "LATCH_SW_CLR"]
    #[inline(always)]
    pub fn is_fctl_fl_latch_sw_clr(&self) -> bool {
        *self == FctlFl::FctlFlLatchSwClr
    }
    #[doc = "LATCH_Z_CLR"]
    #[inline(always)]
    pub fn is_fctl_fl_latch_z_clr(&self) -> bool {
        *self == FctlFl::FctlFlLatchZClr
    }
    #[doc = "LATCH_LD_CLR"]
    #[inline(always)]
    pub fn is_fctl_fl_latch_ld_clr(&self) -> bool {
        *self == FctlFl::FctlFlLatchLdClr
    }
}
#[doc = "Field `FCTL_FL` writer - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
pub type FctlFlW<'a, REG> = crate::FieldWriter<'a, REG, 2, FctlFl, crate::Safe>;
impl<'a, REG> FctlFlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NO_LATCH"]
    #[inline(always)]
    pub fn fctl_fl_no_latch(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFl::FctlFlNoLatch)
    }
    #[doc = "LATCH_SW_CLR"]
    #[inline(always)]
    pub fn fctl_fl_latch_sw_clr(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFl::FctlFlLatchSwClr)
    }
    #[doc = "LATCH_Z_CLR"]
    #[inline(always)]
    pub fn fctl_fl_latch_z_clr(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFl::FctlFlLatchZClr)
    }
    #[doc = "LATCH_LD_CLR"]
    #[inline(always)]
    pub fn fctl_fl_latch_ld_clr(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFl::FctlFlLatchLdClr)
    }
}
#[doc = "Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlTfim {
    #[doc = "0: DISABLED"]
    FctlTfimDisabled = 0,
    #[doc = "1: ENABLED"]
    FctlTfimEnabled = 1,
}
impl From<FctlTfim> for bool {
    #[inline(always)]
    fn from(variant: FctlTfim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_TFIM` reader - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
pub type FctlTfimR = crate::BitReader<FctlTfim>;
impl FctlTfimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlTfim {
        match self.bits {
            false => FctlTfim::FctlTfimDisabled,
            true => FctlTfim::FctlTfimEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_fctl_tfim_disabled(&self) -> bool {
        *self == FctlTfim::FctlTfimDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_fctl_tfim_enabled(&self) -> bool {
        *self == FctlTfim::FctlTfimEnabled
    }
}
#[doc = "Field `FCTL_TFIM` writer - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
pub type FctlTfimW<'a, REG> = crate::BitWriter<'a, REG, FctlTfim>;
impl<'a, REG> FctlTfimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn fctl_tfim_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FctlTfim::FctlTfimDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn fctl_tfim_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FctlTfim::FctlTfimEnabled)
    }
}
#[doc = "Specifies whether the analog comparator0 fault sense is high or low active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFsenac0 {
    #[doc = "0: LOWCTIVE"]
    FctlFsenac0Lowctive = 0,
    #[doc = "1: HIGHACTIVE"]
    FctlFsenac0Highactive = 1,
}
impl From<FctlFsenac0> for bool {
    #[inline(always)]
    fn from(variant: FctlFsenac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FSENAC0` reader - Specifies whether the analog comparator0 fault sense is high or low active"]
pub type FctlFsenac0R = crate::BitReader<FctlFsenac0>;
impl FctlFsenac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFsenac0 {
        match self.bits {
            false => FctlFsenac0::FctlFsenac0Lowctive,
            true => FctlFsenac0::FctlFsenac0Highactive,
        }
    }
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenac0_lowctive(&self) -> bool {
        *self == FctlFsenac0::FctlFsenac0Lowctive
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenac0_highactive(&self) -> bool {
        *self == FctlFsenac0::FctlFsenac0Highactive
    }
}
#[doc = "Field `FCTL_FSENAC0` writer - Specifies whether the analog comparator0 fault sense is high or low active"]
pub type FctlFsenac0W<'a, REG> = crate::BitWriter<'a, REG, FctlFsenac0>;
impl<'a, REG> FctlFsenac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn fctl_fsenac0_lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenac0::FctlFsenac0Lowctive)
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn fctl_fsenac0_highactive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenac0::FctlFsenac0Highactive)
    }
}
#[doc = "Specifies whether the analog comparator1 fault sense is high or low active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFsenac1 {
    #[doc = "0: LOWCTIVE"]
    FctlFsenac1Lowctive = 0,
    #[doc = "1: HIGHACTIVE"]
    FctlFsenac1Highactive = 1,
}
impl From<FctlFsenac1> for bool {
    #[inline(always)]
    fn from(variant: FctlFsenac1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FSENAC1` reader - Specifies whether the analog comparator1 fault sense is high or low active"]
pub type FctlFsenac1R = crate::BitReader<FctlFsenac1>;
impl FctlFsenac1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFsenac1 {
        match self.bits {
            false => FctlFsenac1::FctlFsenac1Lowctive,
            true => FctlFsenac1::FctlFsenac1Highactive,
        }
    }
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenac1_lowctive(&self) -> bool {
        *self == FctlFsenac1::FctlFsenac1Lowctive
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenac1_highactive(&self) -> bool {
        *self == FctlFsenac1::FctlFsenac1Highactive
    }
}
#[doc = "Field `FCTL_FSENAC1` writer - Specifies whether the analog comparator1 fault sense is high or low active"]
pub type FctlFsenac1W<'a, REG> = crate::BitWriter<'a, REG, FctlFsenac1>;
impl<'a, REG> FctlFsenac1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn fctl_fsenac1_lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenac1::FctlFsenac1Lowctive)
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn fctl_fsenac1_highactive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenac1::FctlFsenac1Highactive)
    }
}
#[doc = "Specifies whether the analog comparator2 fault sense is high or low active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFsenac2 {
    #[doc = "0: LOWCTIVE"]
    FctlFsenac2Lowctive = 0,
    #[doc = "1: HIGHACTIVE"]
    FctlFsenac2Highactive = 1,
}
impl From<FctlFsenac2> for bool {
    #[inline(always)]
    fn from(variant: FctlFsenac2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FSENAC2` reader - Specifies whether the analog comparator2 fault sense is high or low active"]
pub type FctlFsenac2R = crate::BitReader<FctlFsenac2>;
impl FctlFsenac2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFsenac2 {
        match self.bits {
            false => FctlFsenac2::FctlFsenac2Lowctive,
            true => FctlFsenac2::FctlFsenac2Highactive,
        }
    }
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenac2_lowctive(&self) -> bool {
        *self == FctlFsenac2::FctlFsenac2Lowctive
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenac2_highactive(&self) -> bool {
        *self == FctlFsenac2::FctlFsenac2Highactive
    }
}
#[doc = "Field `FCTL_FSENAC2` writer - Specifies whether the analog comparator2 fault sense is high or low active"]
pub type FctlFsenac2W<'a, REG> = crate::BitWriter<'a, REG, FctlFsenac2>;
impl<'a, REG> FctlFsenac2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn fctl_fsenac2_lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenac2::FctlFsenac2Lowctive)
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn fctl_fsenac2_highactive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenac2::FctlFsenac2Highactive)
    }
}
#[doc = "Specifies whether the external fault pin0 sense is high or low active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFsenext0 {
    #[doc = "0: LOWCTIVE"]
    FctlFsenext0Lowctive = 0,
    #[doc = "1: HIGHACTIVE"]
    FctlFsenext0Highactive = 1,
}
impl From<FctlFsenext0> for bool {
    #[inline(always)]
    fn from(variant: FctlFsenext0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FSENEXT0` reader - Specifies whether the external fault pin0 sense is high or low active"]
pub type FctlFsenext0R = crate::BitReader<FctlFsenext0>;
impl FctlFsenext0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFsenext0 {
        match self.bits {
            false => FctlFsenext0::FctlFsenext0Lowctive,
            true => FctlFsenext0::FctlFsenext0Highactive,
        }
    }
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenext0_lowctive(&self) -> bool {
        *self == FctlFsenext0::FctlFsenext0Lowctive
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenext0_highactive(&self) -> bool {
        *self == FctlFsenext0::FctlFsenext0Highactive
    }
}
#[doc = "Field `FCTL_FSENEXT0` writer - Specifies whether the external fault pin0 sense is high or low active"]
pub type FctlFsenext0W<'a, REG> = crate::BitWriter<'a, REG, FctlFsenext0>;
impl<'a, REG> FctlFsenext0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn fctl_fsenext0_lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenext0::FctlFsenext0Lowctive)
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn fctl_fsenext0_highactive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenext0::FctlFsenext0Highactive)
    }
}
#[doc = "Specifies whether the external fault pin1 fault sense is high or low active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFsenext1 {
    #[doc = "0: LOWCTIVE"]
    FctlFsenext1Lowctive = 0,
    #[doc = "1: HIGHACTIVE"]
    FctlFsenext1Highactive = 1,
}
impl From<FctlFsenext1> for bool {
    #[inline(always)]
    fn from(variant: FctlFsenext1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FSENEXT1` reader - Specifies whether the external fault pin1 fault sense is high or low active"]
pub type FctlFsenext1R = crate::BitReader<FctlFsenext1>;
impl FctlFsenext1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFsenext1 {
        match self.bits {
            false => FctlFsenext1::FctlFsenext1Lowctive,
            true => FctlFsenext1::FctlFsenext1Highactive,
        }
    }
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenext1_lowctive(&self) -> bool {
        *self == FctlFsenext1::FctlFsenext1Lowctive
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenext1_highactive(&self) -> bool {
        *self == FctlFsenext1::FctlFsenext1Highactive
    }
}
#[doc = "Field `FCTL_FSENEXT1` writer - Specifies whether the external fault pin1 fault sense is high or low active"]
pub type FctlFsenext1W<'a, REG> = crate::BitWriter<'a, REG, FctlFsenext1>;
impl<'a, REG> FctlFsenext1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn fctl_fsenext1_lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenext1::FctlFsenext1Lowctive)
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn fctl_fsenext1_highactive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenext1::FctlFsenext1Highactive)
    }
}
#[doc = "Specifies whether the external fault pin3 fault sense is high or low active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FctlFsenext2 {
    #[doc = "0: LOWCTIVE"]
    FctlFsenext2Lowctive = 0,
    #[doc = "1: HIGHACTIVE"]
    FctlFsenext2Highactive = 1,
}
impl From<FctlFsenext2> for bool {
    #[inline(always)]
    fn from(variant: FctlFsenext2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCTL_FSENEXT2` reader - Specifies whether the external fault pin3 fault sense is high or low active"]
pub type FctlFsenext2R = crate::BitReader<FctlFsenext2>;
impl FctlFsenext2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FctlFsenext2 {
        match self.bits {
            false => FctlFsenext2::FctlFsenext2Lowctive,
            true => FctlFsenext2::FctlFsenext2Highactive,
        }
    }
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenext2_lowctive(&self) -> bool {
        *self == FctlFsenext2::FctlFsenext2Lowctive
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn is_fctl_fsenext2_highactive(&self) -> bool {
        *self == FctlFsenext2::FctlFsenext2Highactive
    }
}
#[doc = "Field `FCTL_FSENEXT2` writer - Specifies whether the external fault pin3 fault sense is high or low active"]
pub type FctlFsenext2W<'a, REG> = crate::BitWriter<'a, REG, FctlFsenext2>;
impl<'a, REG> FctlFsenext2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOWCTIVE"]
    #[inline(always)]
    pub fn fctl_fsenext2_lowctive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenext2::FctlFsenext2Lowctive)
    }
    #[doc = "HIGHACTIVE"]
    #[inline(always)]
    pub fn fctl_fsenext2_highactive(self) -> &'a mut crate::W<REG> {
        self.variant(FctlFsenext2::FctlFsenext2Highactive)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Input Enable This bit enables the input for fault detection."]
    #[inline(always)]
    pub fn fctl_fien(&self) -> FctlFienR {
        FctlFienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
    #[inline(always)]
    pub fn fctl_fi(&self) -> FctlFiR {
        FctlFiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
    #[inline(always)]
    pub fn fctl_fl(&self) -> FctlFlR {
        FctlFlR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 7 - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
    #[inline(always)]
    pub fn fctl_tfim(&self) -> FctlTfimR {
        FctlTfimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the analog comparator0 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenac0(&self) -> FctlFsenac0R {
        FctlFsenac0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies whether the analog comparator1 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenac1(&self) -> FctlFsenac1R {
        FctlFsenac1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Specifies whether the analog comparator2 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenac2(&self) -> FctlFsenac2R {
        FctlFsenac2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Specifies whether the external fault pin0 sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenext0(&self) -> FctlFsenext0R {
        FctlFsenext0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies whether the external fault pin1 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenext1(&self) -> FctlFsenext1R {
        FctlFsenext1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Specifies whether the external fault pin3 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenext2(&self) -> FctlFsenext2R {
        FctlFsenext2R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Input Enable This bit enables the input for fault detection."]
    #[inline(always)]
    pub fn fctl_fien(&mut self) -> FctlFienW<FctlSpec> {
        FctlFienW::new(self, 0)
    }
    #[doc = "Bit 2 - Fault Input Specifies whether the overall fault condition is dependent on the sensed fault pin."]
    #[inline(always)]
    pub fn fctl_fi(&mut self) -> FctlFiW<FctlSpec> {
        FctlFiW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Fault Latch mode Specifies whether the fault condition is latched and configures the latch clear conditions."]
    #[inline(always)]
    pub fn fctl_fl(&mut self) -> FctlFlW<FctlSpec> {
        FctlFlW::new(self, 3)
    }
    #[doc = "Bit 7 - Trigger Fault Input Mask Specifies whether the selected trigger participates as a fault input. If enabled and the trigger asserts, the trigger is treated as a fault."]
    #[inline(always)]
    pub fn fctl_tfim(&mut self) -> FctlTfimW<FctlSpec> {
        FctlTfimW::new(self, 7)
    }
    #[doc = "Bit 8 - Specifies whether the analog comparator0 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenac0(&mut self) -> FctlFsenac0W<FctlSpec> {
        FctlFsenac0W::new(self, 8)
    }
    #[doc = "Bit 9 - Specifies whether the analog comparator1 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenac1(&mut self) -> FctlFsenac1W<FctlSpec> {
        FctlFsenac1W::new(self, 9)
    }
    #[doc = "Bit 10 - Specifies whether the analog comparator2 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenac2(&mut self) -> FctlFsenac2W<FctlSpec> {
        FctlFsenac2W::new(self, 10)
    }
    #[doc = "Bit 11 - Specifies whether the external fault pin0 sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenext0(&mut self) -> FctlFsenext0W<FctlSpec> {
        FctlFsenext0W::new(self, 11)
    }
    #[doc = "Bit 12 - Specifies whether the external fault pin1 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenext1(&mut self) -> FctlFsenext1W<FctlSpec> {
        FctlFsenext1W::new(self, 12)
    }
    #[doc = "Bit 13 - Specifies whether the external fault pin3 fault sense is high or low active"]
    #[inline(always)]
    pub fn fctl_fsenext2(&mut self) -> FctlFsenext2W<FctlSpec> {
        FctlFsenext2W::new(self, 13)
    }
}
#[doc = "Fault Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctlSpec;
impl crate::RegisterSpec for FctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctl::R`](R) reader structure"]
impl crate::Readable for FctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctl::W`](W) writer structure"]
impl crate::Writable for FctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTL to value 0"]
impl crate::Resettable for FctlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTL1` reader"]
pub type R = crate::R<Ctl1Spec>;
#[doc = "Register `CTL1` writer"]
pub type W = crate::W<Ctl1Spec>;
#[doc = "This bit turns on the comparator. When the comparator is turned off it consumes no power.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Enable {
    #[doc = "0: OFF"]
    Ctl1EnableOff = 0,
    #[doc = "1: ON"]
    Ctl1EnableOn = 1,
}
impl From<Ctl1Enable> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_ENABLE` reader - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
pub type Ctl1EnableR = crate::BitReader<Ctl1Enable>;
impl Ctl1EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Enable {
        match self.bits {
            false => Ctl1Enable::Ctl1EnableOff,
            true => Ctl1Enable::Ctl1EnableOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_ctl1_enable_off(&self) -> bool {
        *self == Ctl1Enable::Ctl1EnableOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_ctl1_enable_on(&self) -> bool {
        *self == Ctl1Enable::Ctl1EnableOn
    }
}
#[doc = "Field `CTL1_ENABLE` writer - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
pub type Ctl1EnableW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Enable>;
impl<'a, REG> Ctl1EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn ctl1_enable_off(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Enable::Ctl1EnableOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn ctl1_enable_on(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Enable::Ctl1EnableOn)
    }
}
#[doc = "This bit selects the comparator operating mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Mode {
    #[doc = "0: FAST"]
    Ctl1ModeFast = 0,
    #[doc = "1: ULP"]
    Ctl1ModeUlp = 1,
}
impl From<Ctl1Mode> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_MODE` reader - This bit selects the comparator operating mode."]
pub type Ctl1ModeR = crate::BitReader<Ctl1Mode>;
impl Ctl1ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Mode {
        match self.bits {
            false => Ctl1Mode::Ctl1ModeFast,
            true => Ctl1Mode::Ctl1ModeUlp,
        }
    }
    #[doc = "FAST"]
    #[inline(always)]
    pub fn is_ctl1_mode_fast(&self) -> bool {
        *self == Ctl1Mode::Ctl1ModeFast
    }
    #[doc = "ULP"]
    #[inline(always)]
    pub fn is_ctl1_mode_ulp(&self) -> bool {
        *self == Ctl1Mode::Ctl1ModeUlp
    }
}
#[doc = "Field `CTL1_MODE` writer - This bit selects the comparator operating mode."]
pub type Ctl1ModeW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Mode>;
impl<'a, REG> Ctl1ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FAST"]
    #[inline(always)]
    pub fn ctl1_mode_fast(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Mode::Ctl1ModeFast)
    }
    #[doc = "ULP"]
    #[inline(always)]
    pub fn ctl1_mode_ulp(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Mode::Ctl1ModeUlp)
    }
}
#[doc = "This bit exchanges the comparator inputs and inverts the comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Exch {
    #[doc = "0: NO_EXC"]
    Ctl1ExchNoExc = 0,
    #[doc = "1: EXC"]
    Ctl1ExchExc = 1,
}
impl From<Ctl1Exch> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Exch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_EXCH` reader - This bit exchanges the comparator inputs and inverts the comparator output."]
pub type Ctl1ExchR = crate::BitReader<Ctl1Exch>;
impl Ctl1ExchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Exch {
        match self.bits {
            false => Ctl1Exch::Ctl1ExchNoExc,
            true => Ctl1Exch::Ctl1ExchExc,
        }
    }
    #[doc = "NO_EXC"]
    #[inline(always)]
    pub fn is_ctl1_exch_no_exc(&self) -> bool {
        *self == Ctl1Exch::Ctl1ExchNoExc
    }
    #[doc = "EXC"]
    #[inline(always)]
    pub fn is_ctl1_exch_exc(&self) -> bool {
        *self == Ctl1Exch::Ctl1ExchExc
    }
}
#[doc = "Field `CTL1_EXCH` writer - This bit exchanges the comparator inputs and inverts the comparator output."]
pub type Ctl1ExchW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Exch>;
impl<'a, REG> Ctl1ExchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EXC"]
    #[inline(always)]
    pub fn ctl1_exch_no_exc(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Exch::Ctl1ExchNoExc)
    }
    #[doc = "EXC"]
    #[inline(always)]
    pub fn ctl1_exch_exc(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Exch::Ctl1ExchExc)
    }
}
#[doc = "This bit shorts the positive and negative input terminals of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Short {
    #[doc = "0: NO_SHT"]
    Ctl1ShortNoSht = 0,
    #[doc = "1: SHT"]
    Ctl1ShortSht = 1,
}
impl From<Ctl1Short> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Short) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_SHORT` reader - This bit shorts the positive and negative input terminals of the comparator."]
pub type Ctl1ShortR = crate::BitReader<Ctl1Short>;
impl Ctl1ShortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Short {
        match self.bits {
            false => Ctl1Short::Ctl1ShortNoSht,
            true => Ctl1Short::Ctl1ShortSht,
        }
    }
    #[doc = "NO_SHT"]
    #[inline(always)]
    pub fn is_ctl1_short_no_sht(&self) -> bool {
        *self == Ctl1Short::Ctl1ShortNoSht
    }
    #[doc = "SHT"]
    #[inline(always)]
    pub fn is_ctl1_short_sht(&self) -> bool {
        *self == Ctl1Short::Ctl1ShortSht
    }
}
#[doc = "Field `CTL1_SHORT` writer - This bit shorts the positive and negative input terminals of the comparator."]
pub type Ctl1ShortW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Short>;
impl<'a, REG> Ctl1ShortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_SHT"]
    #[inline(always)]
    pub fn ctl1_short_no_sht(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Short::Ctl1ShortNoSht)
    }
    #[doc = "SHT"]
    #[inline(always)]
    pub fn ctl1_short_sht(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Short::Ctl1ShortSht)
    }
}
#[doc = "This bit selected the interrupt edge for COMPIFG and COMPINVIFG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Ies {
    #[doc = "0: RISING"]
    Ctl1IesRising = 0,
    #[doc = "1: FALLING"]
    Ctl1IesFalling = 1,
}
impl From<Ctl1Ies> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Ies) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_IES` reader - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
pub type Ctl1IesR = crate::BitReader<Ctl1Ies>;
impl Ctl1IesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Ies {
        match self.bits {
            false => Ctl1Ies::Ctl1IesRising,
            true => Ctl1Ies::Ctl1IesFalling,
        }
    }
    #[doc = "RISING"]
    #[inline(always)]
    pub fn is_ctl1_ies_rising(&self) -> bool {
        *self == Ctl1Ies::Ctl1IesRising
    }
    #[doc = "FALLING"]
    #[inline(always)]
    pub fn is_ctl1_ies_falling(&self) -> bool {
        *self == Ctl1Ies::Ctl1IesFalling
    }
}
#[doc = "Field `CTL1_IES` writer - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
pub type Ctl1IesW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Ies>;
impl<'a, REG> Ctl1IesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RISING"]
    #[inline(always)]
    pub fn ctl1_ies_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ies::Ctl1IesRising)
    }
    #[doc = "FALLING"]
    #[inline(always)]
    pub fn ctl1_ies_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Ies::Ctl1IesFalling)
    }
}
#[doc = "These bits select the hysteresis setting of the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Hyst {
    #[doc = "0: NO_HYS"]
    Ctl1HystNoHys = 0,
    #[doc = "1: LOW_HYS"]
    Ctl1HystLowHys = 1,
    #[doc = "2: MED_HYS"]
    Ctl1HystMedHys = 2,
    #[doc = "3: HIGH_HYS"]
    Ctl1HystHighHys = 3,
}
impl From<Ctl1Hyst> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Hyst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Hyst {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Hyst {}
#[doc = "Field `CTL1_HYST` reader - These bits select the hysteresis setting of the comparator."]
pub type Ctl1HystR = crate::FieldReader<Ctl1Hyst>;
impl Ctl1HystR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Hyst {
        match self.bits {
            0 => Ctl1Hyst::Ctl1HystNoHys,
            1 => Ctl1Hyst::Ctl1HystLowHys,
            2 => Ctl1Hyst::Ctl1HystMedHys,
            3 => Ctl1Hyst::Ctl1HystHighHys,
            _ => unreachable!(),
        }
    }
    #[doc = "NO_HYS"]
    #[inline(always)]
    pub fn is_ctl1_hyst_no_hys(&self) -> bool {
        *self == Ctl1Hyst::Ctl1HystNoHys
    }
    #[doc = "LOW_HYS"]
    #[inline(always)]
    pub fn is_ctl1_hyst_low_hys(&self) -> bool {
        *self == Ctl1Hyst::Ctl1HystLowHys
    }
    #[doc = "MED_HYS"]
    #[inline(always)]
    pub fn is_ctl1_hyst_med_hys(&self) -> bool {
        *self == Ctl1Hyst::Ctl1HystMedHys
    }
    #[doc = "HIGH_HYS"]
    #[inline(always)]
    pub fn is_ctl1_hyst_high_hys(&self) -> bool {
        *self == Ctl1Hyst::Ctl1HystHighHys
    }
}
#[doc = "Field `CTL1_HYST` writer - These bits select the hysteresis setting of the comparator."]
pub type Ctl1HystW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl1Hyst, crate::Safe>;
impl<'a, REG> Ctl1HystW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NO_HYS"]
    #[inline(always)]
    pub fn ctl1_hyst_no_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Hyst::Ctl1HystNoHys)
    }
    #[doc = "LOW_HYS"]
    #[inline(always)]
    pub fn ctl1_hyst_low_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Hyst::Ctl1HystLowHys)
    }
    #[doc = "MED_HYS"]
    #[inline(always)]
    pub fn ctl1_hyst_med_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Hyst::Ctl1HystMedHys)
    }
    #[doc = "HIGH_HYS"]
    #[inline(always)]
    pub fn ctl1_hyst_high_hys(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Hyst::Ctl1HystHighHys)
    }
}
#[doc = "This bit selects the comparator output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Outpol {
    #[doc = "0: NON_INV"]
    Ctl1OutpolNonInv = 0,
    #[doc = "1: INV"]
    Ctl1OutpolInv = 1,
}
impl From<Ctl1Outpol> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Outpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_OUTPOL` reader - This bit selects the comparator output polarity."]
pub type Ctl1OutpolR = crate::BitReader<Ctl1Outpol>;
impl Ctl1OutpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Outpol {
        match self.bits {
            false => Ctl1Outpol::Ctl1OutpolNonInv,
            true => Ctl1Outpol::Ctl1OutpolInv,
        }
    }
    #[doc = "NON_INV"]
    #[inline(always)]
    pub fn is_ctl1_outpol_non_inv(&self) -> bool {
        *self == Ctl1Outpol::Ctl1OutpolNonInv
    }
    #[doc = "INV"]
    #[inline(always)]
    pub fn is_ctl1_outpol_inv(&self) -> bool {
        *self == Ctl1Outpol::Ctl1OutpolInv
    }
}
#[doc = "Field `CTL1_OUTPOL` writer - This bit selects the comparator output polarity."]
pub type Ctl1OutpolW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Outpol>;
impl<'a, REG> Ctl1OutpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NON_INV"]
    #[inline(always)]
    pub fn ctl1_outpol_non_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Outpol::Ctl1OutpolNonInv)
    }
    #[doc = "INV"]
    #[inline(always)]
    pub fn ctl1_outpol_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Outpol::Ctl1OutpolInv)
    }
}
#[doc = "This bit enables the analog filter at comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Flten {
    #[doc = "0: DISABLE"]
    Ctl1FltenDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl1FltenEnable = 1,
}
impl From<Ctl1Flten> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Flten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_FLTEN` reader - This bit enables the analog filter at comparator output."]
pub type Ctl1FltenR = crate::BitReader<Ctl1Flten>;
impl Ctl1FltenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Flten {
        match self.bits {
            false => Ctl1Flten::Ctl1FltenDisable,
            true => Ctl1Flten::Ctl1FltenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl1_flten_disable(&self) -> bool {
        *self == Ctl1Flten::Ctl1FltenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl1_flten_enable(&self) -> bool {
        *self == Ctl1Flten::Ctl1FltenEnable
    }
}
#[doc = "Field `CTL1_FLTEN` writer - This bit enables the analog filter at comparator output."]
pub type Ctl1FltenW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Flten>;
impl<'a, REG> Ctl1FltenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl1_flten_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Flten::Ctl1FltenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl1_flten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Flten::Ctl1FltenEnable)
    }
}
#[doc = "These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl1Fltdly {
    #[doc = "0: DLY_0"]
    Ctl1FltdlyDly0 = 0,
    #[doc = "1: DLY_1"]
    Ctl1FltdlyDly1 = 1,
    #[doc = "2: DLY_2"]
    Ctl1FltdlyDly2 = 2,
    #[doc = "3: DLY_3"]
    Ctl1FltdlyDly3 = 3,
}
impl From<Ctl1Fltdly> for u8 {
    #[inline(always)]
    fn from(variant: Ctl1Fltdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl1Fltdly {
    type Ux = u8;
}
impl crate::IsEnum for Ctl1Fltdly {}
#[doc = "Field `CTL1_FLTDLY` reader - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
pub type Ctl1FltdlyR = crate::FieldReader<Ctl1Fltdly>;
impl Ctl1FltdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Fltdly {
        match self.bits {
            0 => Ctl1Fltdly::Ctl1FltdlyDly0,
            1 => Ctl1Fltdly::Ctl1FltdlyDly1,
            2 => Ctl1Fltdly::Ctl1FltdlyDly2,
            3 => Ctl1Fltdly::Ctl1FltdlyDly3,
            _ => unreachable!(),
        }
    }
    #[doc = "DLY_0"]
    #[inline(always)]
    pub fn is_ctl1_fltdly_dly_0(&self) -> bool {
        *self == Ctl1Fltdly::Ctl1FltdlyDly0
    }
    #[doc = "DLY_1"]
    #[inline(always)]
    pub fn is_ctl1_fltdly_dly_1(&self) -> bool {
        *self == Ctl1Fltdly::Ctl1FltdlyDly1
    }
    #[doc = "DLY_2"]
    #[inline(always)]
    pub fn is_ctl1_fltdly_dly_2(&self) -> bool {
        *self == Ctl1Fltdly::Ctl1FltdlyDly2
    }
    #[doc = "DLY_3"]
    #[inline(always)]
    pub fn is_ctl1_fltdly_dly_3(&self) -> bool {
        *self == Ctl1Fltdly::Ctl1FltdlyDly3
    }
}
#[doc = "Field `CTL1_FLTDLY` writer - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
pub type Ctl1FltdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl1Fltdly, crate::Safe>;
impl<'a, REG> Ctl1FltdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DLY_0"]
    #[inline(always)]
    pub fn ctl1_fltdly_dly_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Fltdly::Ctl1FltdlyDly0)
    }
    #[doc = "DLY_1"]
    #[inline(always)]
    pub fn ctl1_fltdly_dly_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Fltdly::Ctl1FltdlyDly1)
    }
    #[doc = "DLY_2"]
    #[inline(always)]
    pub fn ctl1_fltdly_dly_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Fltdly::Ctl1FltdlyDly2)
    }
    #[doc = "DLY_3"]
    #[inline(always)]
    pub fn ctl1_fltdly_dly_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Fltdly::Ctl1FltdlyDly3)
    }
}
#[doc = "This bit enables window comparator operation of comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl1Wincompen {
    #[doc = "0: OFF"]
    Ctl1WincompenOff = 0,
    #[doc = "1: ON"]
    Ctl1WincompenOn = 1,
}
impl From<Ctl1Wincompen> for bool {
    #[inline(always)]
    fn from(variant: Ctl1Wincompen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL1_WINCOMPEN` reader - This bit enables window comparator operation of comparator."]
pub type Ctl1WincompenR = crate::BitReader<Ctl1Wincompen>;
impl Ctl1WincompenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl1Wincompen {
        match self.bits {
            false => Ctl1Wincompen::Ctl1WincompenOff,
            true => Ctl1Wincompen::Ctl1WincompenOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_ctl1_wincompen_off(&self) -> bool {
        *self == Ctl1Wincompen::Ctl1WincompenOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_ctl1_wincompen_on(&self) -> bool {
        *self == Ctl1Wincompen::Ctl1WincompenOn
    }
}
#[doc = "Field `CTL1_WINCOMPEN` writer - This bit enables window comparator operation of comparator."]
pub type Ctl1WincompenW<'a, REG> = crate::BitWriter<'a, REG, Ctl1Wincompen>;
impl<'a, REG> Ctl1WincompenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn ctl1_wincompen_off(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Wincompen::Ctl1WincompenOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn ctl1_wincompen_on(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl1Wincompen::Ctl1WincompenOn)
    }
}
impl R {
    #[doc = "Bit 0 - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
    #[inline(always)]
    pub fn ctl1_enable(&self) -> Ctl1EnableR {
        Ctl1EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit selects the comparator operating mode."]
    #[inline(always)]
    pub fn ctl1_mode(&self) -> Ctl1ModeR {
        Ctl1ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit exchanges the comparator inputs and inverts the comparator output."]
    #[inline(always)]
    pub fn ctl1_exch(&self) -> Ctl1ExchR {
        Ctl1ExchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit shorts the positive and negative input terminals of the comparator."]
    #[inline(always)]
    pub fn ctl1_short(&self) -> Ctl1ShortR {
        Ctl1ShortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
    #[inline(always)]
    pub fn ctl1_ies(&self) -> Ctl1IesR {
        Ctl1IesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - These bits select the hysteresis setting of the comparator."]
    #[inline(always)]
    pub fn ctl1_hyst(&self) -> Ctl1HystR {
        Ctl1HystR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - This bit selects the comparator output polarity."]
    #[inline(always)]
    pub fn ctl1_outpol(&self) -> Ctl1OutpolR {
        Ctl1OutpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit enables the analog filter at comparator output."]
    #[inline(always)]
    pub fn ctl1_flten(&self) -> Ctl1FltenR {
        Ctl1FltenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
    #[inline(always)]
    pub fn ctl1_fltdly(&self) -> Ctl1FltdlyR {
        Ctl1FltdlyR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - This bit enables window comparator operation of comparator."]
    #[inline(always)]
    pub fn ctl1_wincompen(&self) -> Ctl1WincompenR {
        Ctl1WincompenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit turns on the comparator. When the comparator is turned off it consumes no power."]
    #[inline(always)]
    pub fn ctl1_enable(&mut self) -> Ctl1EnableW<Ctl1Spec> {
        Ctl1EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit selects the comparator operating mode."]
    #[inline(always)]
    pub fn ctl1_mode(&mut self) -> Ctl1ModeW<Ctl1Spec> {
        Ctl1ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit exchanges the comparator inputs and inverts the comparator output."]
    #[inline(always)]
    pub fn ctl1_exch(&mut self) -> Ctl1ExchW<Ctl1Spec> {
        Ctl1ExchW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit shorts the positive and negative input terminals of the comparator."]
    #[inline(always)]
    pub fn ctl1_short(&mut self) -> Ctl1ShortW<Ctl1Spec> {
        Ctl1ShortW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit selected the interrupt edge for COMPIFG and COMPINVIFG."]
    #[inline(always)]
    pub fn ctl1_ies(&mut self) -> Ctl1IesW<Ctl1Spec> {
        Ctl1IesW::new(self, 4)
    }
    #[doc = "Bits 5:6 - These bits select the hysteresis setting of the comparator."]
    #[inline(always)]
    pub fn ctl1_hyst(&mut self) -> Ctl1HystW<Ctl1Spec> {
        Ctl1HystW::new(self, 5)
    }
    #[doc = "Bit 7 - This bit selects the comparator output polarity."]
    #[inline(always)]
    pub fn ctl1_outpol(&mut self) -> Ctl1OutpolW<Ctl1Spec> {
        Ctl1OutpolW::new(self, 7)
    }
    #[doc = "Bit 8 - This bit enables the analog filter at comparator output."]
    #[inline(always)]
    pub fn ctl1_flten(&mut self) -> Ctl1FltenW<Ctl1Spec> {
        Ctl1FltenW::new(self, 8)
    }
    #[doc = "Bits 9:10 - These bits select the comparator output filter delay. See the device-specific data sheet for specific values on comparator propagation delay for different filter delay settings."]
    #[inline(always)]
    pub fn ctl1_fltdly(&mut self) -> Ctl1FltdlyW<Ctl1Spec> {
        Ctl1FltdlyW::new(self, 9)
    }
    #[doc = "Bit 12 - This bit enables window comparator operation of comparator."]
    #[inline(always)]
    pub fn ctl1_wincompen(&mut self) -> Ctl1WincompenW<Ctl1Spec> {
        Ctl1WincompenW::new(self, 12)
    }
}
#[doc = "Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl1Spec;
impl crate::RegisterSpec for Ctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl1::R`](R) reader structure"]
impl crate::Readable for Ctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl1::W`](W) writer structure"]
impl crate::Writable for Ctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for Ctl1Spec {
    const RESET_VALUE: u32 = 0;
}

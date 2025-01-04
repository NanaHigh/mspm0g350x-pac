#[doc = "Register `WWDTCTL0` reader"]
pub type R = crate::R<Wwdtctl0Spec>;
#[doc = "Register `WWDTCTL0` writer"]
pub type W = crate::W<Wwdtctl0Spec>;
#[doc = "Field `WWDTCTL0_CLKDIV` reader - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
pub type Wwdtctl0ClkdivR = crate::FieldReader;
#[doc = "Field `WWDTCTL0_CLKDIV` writer - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
pub type Wwdtctl0ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Timer Period of the WWDT. These bits select the total watchdog timer count.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wwdtctl0Per {
    #[doc = "0: EN_25"]
    Wwdtctl0PerEn25 = 0,
    #[doc = "1: EN_21"]
    Wwdtctl0PerEn21 = 1,
    #[doc = "2: EN_18"]
    Wwdtctl0PerEn18 = 2,
    #[doc = "3: EN_15"]
    Wwdtctl0PerEn15 = 3,
    #[doc = "4: EN_12"]
    Wwdtctl0PerEn12 = 4,
    #[doc = "5: EN_10"]
    Wwdtctl0PerEn10 = 5,
    #[doc = "6: EN_8"]
    Wwdtctl0PerEn8 = 6,
    #[doc = "7: EN_6"]
    Wwdtctl0PerEn6 = 7,
}
impl From<Wwdtctl0Per> for u8 {
    #[inline(always)]
    fn from(variant: Wwdtctl0Per) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wwdtctl0Per {
    type Ux = u8;
}
impl crate::IsEnum for Wwdtctl0Per {}
#[doc = "Field `WWDTCTL0_PER` reader - Timer Period of the WWDT. These bits select the total watchdog timer count."]
pub type Wwdtctl0PerR = crate::FieldReader<Wwdtctl0Per>;
impl Wwdtctl0PerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtctl0Per {
        match self.bits {
            0 => Wwdtctl0Per::Wwdtctl0PerEn25,
            1 => Wwdtctl0Per::Wwdtctl0PerEn21,
            2 => Wwdtctl0Per::Wwdtctl0PerEn18,
            3 => Wwdtctl0Per::Wwdtctl0PerEn15,
            4 => Wwdtctl0Per::Wwdtctl0PerEn12,
            5 => Wwdtctl0Per::Wwdtctl0PerEn10,
            6 => Wwdtctl0Per::Wwdtctl0PerEn8,
            7 => Wwdtctl0Per::Wwdtctl0PerEn6,
            _ => unreachable!(),
        }
    }
    #[doc = "EN_25"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_25(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn25
    }
    #[doc = "EN_21"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_21(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn21
    }
    #[doc = "EN_18"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_18(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn18
    }
    #[doc = "EN_15"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_15(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn15
    }
    #[doc = "EN_12"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_12(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn12
    }
    #[doc = "EN_10"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_10(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn10
    }
    #[doc = "EN_8"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_8(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn8
    }
    #[doc = "EN_6"]
    #[inline(always)]
    pub fn is_wwdtctl0_per_en_6(&self) -> bool {
        *self == Wwdtctl0Per::Wwdtctl0PerEn6
    }
}
#[doc = "Field `WWDTCTL0_PER` writer - Timer Period of the WWDT. These bits select the total watchdog timer count."]
pub type Wwdtctl0PerW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wwdtctl0Per, crate::Safe>;
impl<'a, REG> Wwdtctl0PerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EN_25"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_25(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn25)
    }
    #[doc = "EN_21"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_21(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn21)
    }
    #[doc = "EN_18"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_18(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn18)
    }
    #[doc = "EN_15"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_15(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn15)
    }
    #[doc = "EN_12"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_12(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn12)
    }
    #[doc = "EN_10"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_10(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn10)
    }
    #[doc = "EN_8"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_8(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn8)
    }
    #[doc = "EN_6"]
    #[inline(always)]
    pub fn wwdtctl0_per_en_6(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Per::Wwdtctl0PerEn6)
    }
}
#[doc = "Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wwdtctl0Window0 {
    #[doc = "0: SIZE_0"]
    Wwdtctl0Window0Size0 = 0,
    #[doc = "1: SIZE_12"]
    Wwdtctl0Window0Size12 = 1,
    #[doc = "2: SIZE_18"]
    Wwdtctl0Window0Size18 = 2,
    #[doc = "3: SIZE_25"]
    Wwdtctl0Window0Size25 = 3,
    #[doc = "4: SIZE_50"]
    Wwdtctl0Window0Size50 = 4,
    #[doc = "5: SIZE_75"]
    Wwdtctl0Window0Size75 = 5,
    #[doc = "6: SIZE_81"]
    Wwdtctl0Window0Size81 = 6,
    #[doc = "7: SIZE_87"]
    Wwdtctl0Window0Size87 = 7,
}
impl From<Wwdtctl0Window0> for u8 {
    #[inline(always)]
    fn from(variant: Wwdtctl0Window0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wwdtctl0Window0 {
    type Ux = u8;
}
impl crate::IsEnum for Wwdtctl0Window0 {}
#[doc = "Field `WWDTCTL0_WINDOW0` reader - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Wwdtctl0Window0R = crate::FieldReader<Wwdtctl0Window0>;
impl Wwdtctl0Window0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtctl0Window0 {
        match self.bits {
            0 => Wwdtctl0Window0::Wwdtctl0Window0Size0,
            1 => Wwdtctl0Window0::Wwdtctl0Window0Size12,
            2 => Wwdtctl0Window0::Wwdtctl0Window0Size18,
            3 => Wwdtctl0Window0::Wwdtctl0Window0Size25,
            4 => Wwdtctl0Window0::Wwdtctl0Window0Size50,
            5 => Wwdtctl0Window0::Wwdtctl0Window0Size75,
            6 => Wwdtctl0Window0::Wwdtctl0Window0Size81,
            7 => Wwdtctl0Window0::Wwdtctl0Window0Size87,
            _ => unreachable!(),
        }
    }
    #[doc = "SIZE_0"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_0(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size0
    }
    #[doc = "SIZE_12"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_12(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size12
    }
    #[doc = "SIZE_18"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_18(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size18
    }
    #[doc = "SIZE_25"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_25(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size25
    }
    #[doc = "SIZE_50"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_50(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size50
    }
    #[doc = "SIZE_75"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_75(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size75
    }
    #[doc = "SIZE_81"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_81(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size81
    }
    #[doc = "SIZE_87"]
    #[inline(always)]
    pub fn is_wwdtctl0_window0_size_87(&self) -> bool {
        *self == Wwdtctl0Window0::Wwdtctl0Window0Size87
    }
}
#[doc = "Field `WWDTCTL0_WINDOW0` writer - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Wwdtctl0Window0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Wwdtctl0Window0, crate::Safe>;
impl<'a, REG> Wwdtctl0Window0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SIZE_0"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size0)
    }
    #[doc = "SIZE_12"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_12(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size12)
    }
    #[doc = "SIZE_18"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_18(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size18)
    }
    #[doc = "SIZE_25"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_25(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size25)
    }
    #[doc = "SIZE_50"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_50(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size50)
    }
    #[doc = "SIZE_75"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_75(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size75)
    }
    #[doc = "SIZE_81"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_81(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size81)
    }
    #[doc = "SIZE_87"]
    #[inline(always)]
    pub fn wwdtctl0_window0_size_87(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window0::Wwdtctl0Window0Size87)
    }
}
#[doc = "Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wwdtctl0Window1 {
    #[doc = "0: SIZE_0"]
    Wwdtctl0Window1Size0 = 0,
    #[doc = "1: SIZE_12"]
    Wwdtctl0Window1Size12 = 1,
    #[doc = "2: SIZE_18"]
    Wwdtctl0Window1Size18 = 2,
    #[doc = "3: SIZE_25"]
    Wwdtctl0Window1Size25 = 3,
    #[doc = "4: SIZE_50"]
    Wwdtctl0Window1Size50 = 4,
    #[doc = "5: SIZE_75"]
    Wwdtctl0Window1Size75 = 5,
    #[doc = "6: SIZE_81"]
    Wwdtctl0Window1Size81 = 6,
    #[doc = "7: SIZE_87"]
    Wwdtctl0Window1Size87 = 7,
}
impl From<Wwdtctl0Window1> for u8 {
    #[inline(always)]
    fn from(variant: Wwdtctl0Window1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wwdtctl0Window1 {
    type Ux = u8;
}
impl crate::IsEnum for Wwdtctl0Window1 {}
#[doc = "Field `WWDTCTL0_WINDOW1` reader - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Wwdtctl0Window1R = crate::FieldReader<Wwdtctl0Window1>;
impl Wwdtctl0Window1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtctl0Window1 {
        match self.bits {
            0 => Wwdtctl0Window1::Wwdtctl0Window1Size0,
            1 => Wwdtctl0Window1::Wwdtctl0Window1Size12,
            2 => Wwdtctl0Window1::Wwdtctl0Window1Size18,
            3 => Wwdtctl0Window1::Wwdtctl0Window1Size25,
            4 => Wwdtctl0Window1::Wwdtctl0Window1Size50,
            5 => Wwdtctl0Window1::Wwdtctl0Window1Size75,
            6 => Wwdtctl0Window1::Wwdtctl0Window1Size81,
            7 => Wwdtctl0Window1::Wwdtctl0Window1Size87,
            _ => unreachable!(),
        }
    }
    #[doc = "SIZE_0"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_0(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size0
    }
    #[doc = "SIZE_12"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_12(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size12
    }
    #[doc = "SIZE_18"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_18(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size18
    }
    #[doc = "SIZE_25"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_25(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size25
    }
    #[doc = "SIZE_50"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_50(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size50
    }
    #[doc = "SIZE_75"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_75(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size75
    }
    #[doc = "SIZE_81"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_81(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size81
    }
    #[doc = "SIZE_87"]
    #[inline(always)]
    pub fn is_wwdtctl0_window1_size_87(&self) -> bool {
        *self == Wwdtctl0Window1::Wwdtctl0Window1Size87
    }
}
#[doc = "Field `WWDTCTL0_WINDOW1` writer - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
pub type Wwdtctl0Window1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Wwdtctl0Window1, crate::Safe>;
impl<'a, REG> Wwdtctl0Window1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SIZE_0"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_0(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size0)
    }
    #[doc = "SIZE_12"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_12(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size12)
    }
    #[doc = "SIZE_18"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_18(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size18)
    }
    #[doc = "SIZE_25"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_25(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size25)
    }
    #[doc = "SIZE_50"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_50(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size50)
    }
    #[doc = "SIZE_75"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_75(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size75)
    }
    #[doc = "SIZE_81"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_81(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size81)
    }
    #[doc = "SIZE_87"]
    #[inline(always)]
    pub fn wwdtctl0_window1_size_87(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Window1::Wwdtctl0Window1Size87)
    }
}
#[doc = "Window Watchdog Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdtctl0Mode {
    #[doc = "0: WINDOW"]
    Wwdtctl0ModeWindow = 0,
    #[doc = "1: INTERVAL"]
    Wwdtctl0ModeInterval = 1,
}
impl From<Wwdtctl0Mode> for bool {
    #[inline(always)]
    fn from(variant: Wwdtctl0Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTCTL0_MODE` reader - Window Watchdog Timer Mode"]
pub type Wwdtctl0ModeR = crate::BitReader<Wwdtctl0Mode>;
impl Wwdtctl0ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtctl0Mode {
        match self.bits {
            false => Wwdtctl0Mode::Wwdtctl0ModeWindow,
            true => Wwdtctl0Mode::Wwdtctl0ModeInterval,
        }
    }
    #[doc = "WINDOW"]
    #[inline(always)]
    pub fn is_wwdtctl0_mode_window(&self) -> bool {
        *self == Wwdtctl0Mode::Wwdtctl0ModeWindow
    }
    #[doc = "INTERVAL"]
    #[inline(always)]
    pub fn is_wwdtctl0_mode_interval(&self) -> bool {
        *self == Wwdtctl0Mode::Wwdtctl0ModeInterval
    }
}
#[doc = "Field `WWDTCTL0_MODE` writer - Window Watchdog Timer Mode"]
pub type Wwdtctl0ModeW<'a, REG> = crate::BitWriter<'a, REG, Wwdtctl0Mode>;
impl<'a, REG> Wwdtctl0ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "WINDOW"]
    #[inline(always)]
    pub fn wwdtctl0_mode_window(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Mode::Wwdtctl0ModeWindow)
    }
    #[doc = "INTERVAL"]
    #[inline(always)]
    pub fn wwdtctl0_mode_interval(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Mode::Wwdtctl0ModeInterval)
    }
}
#[doc = "Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wwdtctl0Stism {
    #[doc = "0: CONT"]
    Wwdtctl0StismCont = 0,
    #[doc = "1: STOP"]
    Wwdtctl0StismStop = 1,
}
impl From<Wwdtctl0Stism> for bool {
    #[inline(always)]
    fn from(variant: Wwdtctl0Stism) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTCTL0_STISM` reader - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
pub type Wwdtctl0StismR = crate::BitReader<Wwdtctl0Stism>;
impl Wwdtctl0StismR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wwdtctl0Stism {
        match self.bits {
            false => Wwdtctl0Stism::Wwdtctl0StismCont,
            true => Wwdtctl0Stism::Wwdtctl0StismStop,
        }
    }
    #[doc = "CONT"]
    #[inline(always)]
    pub fn is_wwdtctl0_stism_cont(&self) -> bool {
        *self == Wwdtctl0Stism::Wwdtctl0StismCont
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn is_wwdtctl0_stism_stop(&self) -> bool {
        *self == Wwdtctl0Stism::Wwdtctl0StismStop
    }
}
#[doc = "Field `WWDTCTL0_STISM` writer - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
pub type Wwdtctl0StismW<'a, REG> = crate::BitWriter<'a, REG, Wwdtctl0Stism>;
impl<'a, REG> Wwdtctl0StismW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CONT"]
    #[inline(always)]
    pub fn wwdtctl0_stism_cont(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Stism::Wwdtctl0StismCont)
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn wwdtctl0_stism_stop(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Stism::Wwdtctl0StismStop)
    }
}
#[doc = "KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wwdtctl0Key {
    #[doc = "201: _TO_UNLOCK_W_"]
    Wwdtctl0KeyUnlockW = 201,
}
impl From<Wwdtctl0Key> for u8 {
    #[inline(always)]
    fn from(variant: Wwdtctl0Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wwdtctl0Key {
    type Ux = u8;
}
impl crate::IsEnum for Wwdtctl0Key {}
#[doc = "Field `WWDTCTL0_KEY` writer - KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0."]
pub type Wwdtctl0KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Wwdtctl0Key>;
impl<'a, REG> Wwdtctl0KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_TO_UNLOCK_W_"]
    #[inline(always)]
    pub fn wwdtctl0_key_unlock_w(self) -> &'a mut crate::W<REG> {
        self.variant(Wwdtctl0Key::Wwdtctl0KeyUnlockW)
    }
}
impl R {
    #[doc = "Bits 0:2 - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
    #[inline(always)]
    pub fn wwdtctl0_clkdiv(&self) -> Wwdtctl0ClkdivR {
        Wwdtctl0ClkdivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Timer Period of the WWDT. These bits select the total watchdog timer count."]
    #[inline(always)]
    pub fn wwdtctl0_per(&self) -> Wwdtctl0PerR {
        Wwdtctl0PerR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn wwdtctl0_window0(&self) -> Wwdtctl0Window0R {
        Wwdtctl0Window0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn wwdtctl0_window1(&self) -> Wwdtctl0Window1R {
        Wwdtctl0Window1R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Window Watchdog Timer Mode"]
    #[inline(always)]
    pub fn wwdtctl0_mode(&self) -> Wwdtctl0ModeR {
        Wwdtctl0ModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
    #[inline(always)]
    pub fn wwdtctl0_stism(&self) -> Wwdtctl0StismR {
        Wwdtctl0StismR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Module Clock Divider, Divide the clock source by CLKDIV+1. Divider values from /1 to /8 are possible. The clock divider is currently 4 bits. Bit 4 has no effect and should always be written with 0."]
    #[inline(always)]
    pub fn wwdtctl0_clkdiv(&mut self) -> Wwdtctl0ClkdivW<Wwdtctl0Spec> {
        Wwdtctl0ClkdivW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timer Period of the WWDT. These bits select the total watchdog timer count."]
    #[inline(always)]
    pub fn wwdtctl0_per(&mut self) -> Wwdtctl0PerW<Wwdtctl0Spec> {
        Wwdtctl0PerW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn wwdtctl0_window0(&mut self) -> Wwdtctl0Window0W<Wwdtctl0Spec> {
        Wwdtctl0Window0W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Closed window period in percentage of the timer interval. WWDTCTL1.WINSEL determines the active window setting (WWDTCTL0.WINDOW0 or WWDTCTL0.WINDOW1)."]
    #[inline(always)]
    pub fn wwdtctl0_window1(&mut self) -> Wwdtctl0Window1W<Wwdtctl0Spec> {
        Wwdtctl0Window1W::new(self, 12)
    }
    #[doc = "Bit 16 - Window Watchdog Timer Mode"]
    #[inline(always)]
    pub fn wwdtctl0_mode(&mut self) -> Wwdtctl0ModeW<Wwdtctl0Spec> {
        Wwdtctl0ModeW::new(self, 16)
    }
    #[doc = "Bit 17 - Stop In Sleep Mode. The functionality of this bit requires that POLICY.HWCEN = 0. If POLICY.HWCEN = 1 the WWDT resets during sleep and needs re-configuration. Note: This bit has no effect for the global Window Watchdog as Sleep Mode is not supported."]
    #[inline(always)]
    pub fn wwdtctl0_stism(&mut self) -> Wwdtctl0StismW<Wwdtctl0Spec> {
        Wwdtctl0StismW::new(self, 17)
    }
    #[doc = "Bits 24:31 - KEY to allow write access to this register. Writing to this register with an incorrect key activates the WWDT error signal to the ESM. Read as 0."]
    #[inline(always)]
    pub fn wwdtctl0_key(&mut self) -> Wwdtctl0KeyW<Wwdtctl0Spec> {
        Wwdtctl0KeyW::new(self, 24)
    }
}
#[doc = "Window Watchdog Timer Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wwdtctl0Spec;
impl crate::RegisterSpec for Wwdtctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdtctl0::R`](R) reader structure"]
impl crate::Readable for Wwdtctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`wwdtctl0::W`](W) writer structure"]
impl crate::Writable for Wwdtctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDTCTL0 to value 0x43"]
impl crate::Resettable for Wwdtctl0Spec {
    const RESET_VALUE: u32 = 0x43;
}

#[doc = "Register `SYSPLLCFG0` reader"]
pub type R = crate::R<Syspllcfg0Spec>;
#[doc = "Register `SYSPLLCFG0` writer"]
pub type W = crate::W<Syspllcfg0Spec>;
#[doc = "SYSPLLREF selects the system PLL (SYSPLL) reference clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllcfg0Syspllref {
    #[doc = "0: SYSOSC"]
    Syspllcfg0SyspllrefSysosc = 0,
    #[doc = "1: HFCLK"]
    Syspllcfg0SyspllrefHfclk = 1,
}
impl From<Syspllcfg0Syspllref> for bool {
    #[inline(always)]
    fn from(variant: Syspllcfg0Syspllref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLCFG0_SYSPLLREF` reader - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
pub type Syspllcfg0SyspllrefR = crate::BitReader<Syspllcfg0Syspllref>;
impl Syspllcfg0SyspllrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Syspllref {
        match self.bits {
            false => Syspllcfg0Syspllref::Syspllcfg0SyspllrefSysosc,
            true => Syspllcfg0Syspllref::Syspllcfg0SyspllrefHfclk,
        }
    }
    #[doc = "SYSOSC"]
    #[inline(always)]
    pub fn is_syspllcfg0_syspllref_sysosc(&self) -> bool {
        *self == Syspllcfg0Syspllref::Syspllcfg0SyspllrefSysosc
    }
    #[doc = "HFCLK"]
    #[inline(always)]
    pub fn is_syspllcfg0_syspllref_hfclk(&self) -> bool {
        *self == Syspllcfg0Syspllref::Syspllcfg0SyspllrefHfclk
    }
}
#[doc = "Field `SYSPLLCFG0_SYSPLLREF` writer - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
pub type Syspllcfg0SyspllrefW<'a, REG> = crate::BitWriter<'a, REG, Syspllcfg0Syspllref>;
impl<'a, REG> Syspllcfg0SyspllrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSOSC"]
    #[inline(always)]
    pub fn syspllcfg0_syspllref_sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Syspllref::Syspllcfg0SyspllrefSysosc)
    }
    #[doc = "HFCLK"]
    #[inline(always)]
    pub fn syspllcfg0_syspllref_hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Syspllref::Syspllcfg0SyspllrefHfclk)
    }
}
#[doc = "MCLK2XVCO selects the SYSPLL output which is sent to the HSCLK mux for use by MCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllcfg0Mclk2xvco {
    #[doc = "0: DISABLE"]
    Syspllcfg0Mclk2xvcoDisable = 0,
    #[doc = "1: ENABLE"]
    Syspllcfg0Mclk2xvcoEnable = 1,
}
impl From<Syspllcfg0Mclk2xvco> for bool {
    #[inline(always)]
    fn from(variant: Syspllcfg0Mclk2xvco) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLCFG0_MCLK2XVCO` reader - MCLK2XVCO selects the SYSPLL output which is sent to the HSCLK mux for use by MCLK."]
pub type Syspllcfg0Mclk2xvcoR = crate::BitReader<Syspllcfg0Mclk2xvco>;
impl Syspllcfg0Mclk2xvcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Mclk2xvco {
        match self.bits {
            false => Syspllcfg0Mclk2xvco::Syspllcfg0Mclk2xvcoDisable,
            true => Syspllcfg0Mclk2xvco::Syspllcfg0Mclk2xvcoEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_mclk2xvco_disable(&self) -> bool {
        *self == Syspllcfg0Mclk2xvco::Syspllcfg0Mclk2xvcoDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_mclk2xvco_enable(&self) -> bool {
        *self == Syspllcfg0Mclk2xvco::Syspllcfg0Mclk2xvcoEnable
    }
}
#[doc = "Field `SYSPLLCFG0_MCLK2XVCO` writer - MCLK2XVCO selects the SYSPLL output which is sent to the HSCLK mux for use by MCLK."]
pub type Syspllcfg0Mclk2xvcoW<'a, REG> = crate::BitWriter<'a, REG, Syspllcfg0Mclk2xvco>;
impl<'a, REG> Syspllcfg0Mclk2xvcoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn syspllcfg0_mclk2xvco_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Mclk2xvco::Syspllcfg0Mclk2xvcoDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn syspllcfg0_mclk2xvco_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Mclk2xvco::Syspllcfg0Mclk2xvcoEnable)
    }
}
#[doc = "ENABLECLK0 enables or disables the SYSPLLCLK0 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllcfg0Enableclk0 {
    #[doc = "0: DISABLE"]
    Syspllcfg0Enableclk0Disable = 0,
    #[doc = "1: ENABLE"]
    Syspllcfg0Enableclk0Enable = 1,
}
impl From<Syspllcfg0Enableclk0> for bool {
    #[inline(always)]
    fn from(variant: Syspllcfg0Enableclk0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLCFG0_ENABLECLK0` reader - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
pub type Syspllcfg0Enableclk0R = crate::BitReader<Syspllcfg0Enableclk0>;
impl Syspllcfg0Enableclk0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Enableclk0 {
        match self.bits {
            false => Syspllcfg0Enableclk0::Syspllcfg0Enableclk0Disable,
            true => Syspllcfg0Enableclk0::Syspllcfg0Enableclk0Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_enableclk0_disable(&self) -> bool {
        *self == Syspllcfg0Enableclk0::Syspllcfg0Enableclk0Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_enableclk0_enable(&self) -> bool {
        *self == Syspllcfg0Enableclk0::Syspllcfg0Enableclk0Enable
    }
}
#[doc = "Field `SYSPLLCFG0_ENABLECLK0` writer - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
pub type Syspllcfg0Enableclk0W<'a, REG> = crate::BitWriter<'a, REG, Syspllcfg0Enableclk0>;
impl<'a, REG> Syspllcfg0Enableclk0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn syspllcfg0_enableclk0_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Enableclk0::Syspllcfg0Enableclk0Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn syspllcfg0_enableclk0_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Enableclk0::Syspllcfg0Enableclk0Enable)
    }
}
#[doc = "ENABLECLK1 enables or disables the SYSPLLCLK1 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllcfg0Enableclk1 {
    #[doc = "0: DISABLE"]
    Syspllcfg0Enableclk1Disable = 0,
    #[doc = "1: ENABLE"]
    Syspllcfg0Enableclk1Enable = 1,
}
impl From<Syspllcfg0Enableclk1> for bool {
    #[inline(always)]
    fn from(variant: Syspllcfg0Enableclk1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLCFG0_ENABLECLK1` reader - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
pub type Syspllcfg0Enableclk1R = crate::BitReader<Syspllcfg0Enableclk1>;
impl Syspllcfg0Enableclk1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Enableclk1 {
        match self.bits {
            false => Syspllcfg0Enableclk1::Syspllcfg0Enableclk1Disable,
            true => Syspllcfg0Enableclk1::Syspllcfg0Enableclk1Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_enableclk1_disable(&self) -> bool {
        *self == Syspllcfg0Enableclk1::Syspllcfg0Enableclk1Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_enableclk1_enable(&self) -> bool {
        *self == Syspllcfg0Enableclk1::Syspllcfg0Enableclk1Enable
    }
}
#[doc = "Field `SYSPLLCFG0_ENABLECLK1` writer - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
pub type Syspllcfg0Enableclk1W<'a, REG> = crate::BitWriter<'a, REG, Syspllcfg0Enableclk1>;
impl<'a, REG> Syspllcfg0Enableclk1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn syspllcfg0_enableclk1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Enableclk1::Syspllcfg0Enableclk1Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn syspllcfg0_enableclk1_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Enableclk1::Syspllcfg0Enableclk1Enable)
    }
}
#[doc = "ENABLECLK2X enables or disables the SYSPLLCLK2X output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Syspllcfg0Enableclk2x {
    #[doc = "0: DISABLE"]
    Syspllcfg0Enableclk2xDisable = 0,
    #[doc = "1: ENABLE"]
    Syspllcfg0Enableclk2xEnable = 1,
}
impl From<Syspllcfg0Enableclk2x> for bool {
    #[inline(always)]
    fn from(variant: Syspllcfg0Enableclk2x) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLCFG0_ENABLECLK2X` reader - ENABLECLK2X enables or disables the SYSPLLCLK2X output."]
pub type Syspllcfg0Enableclk2xR = crate::BitReader<Syspllcfg0Enableclk2x>;
impl Syspllcfg0Enableclk2xR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Enableclk2x {
        match self.bits {
            false => Syspllcfg0Enableclk2x::Syspllcfg0Enableclk2xDisable,
            true => Syspllcfg0Enableclk2x::Syspllcfg0Enableclk2xEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_enableclk2x_disable(&self) -> bool {
        *self == Syspllcfg0Enableclk2x::Syspllcfg0Enableclk2xDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_syspllcfg0_enableclk2x_enable(&self) -> bool {
        *self == Syspllcfg0Enableclk2x::Syspllcfg0Enableclk2xEnable
    }
}
#[doc = "Field `SYSPLLCFG0_ENABLECLK2X` writer - ENABLECLK2X enables or disables the SYSPLLCLK2X output."]
pub type Syspllcfg0Enableclk2xW<'a, REG> = crate::BitWriter<'a, REG, Syspllcfg0Enableclk2x>;
impl<'a, REG> Syspllcfg0Enableclk2xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn syspllcfg0_enableclk2x_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Enableclk2x::Syspllcfg0Enableclk2xDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn syspllcfg0_enableclk2x_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Enableclk2x::Syspllcfg0Enableclk2xEnable)
    }
}
#[doc = "RDIVCLK0 sets the final divider for the SYSPLLCLK0 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syspllcfg0Rdivclk0 {
    #[doc = "0: CLK0DIV2"]
    Syspllcfg0Rdivclk0Clk0div2 = 0,
    #[doc = "1: CLK0DIV4"]
    Syspllcfg0Rdivclk0Clk0div4 = 1,
    #[doc = "2: CLK0DIV6"]
    Syspllcfg0Rdivclk0Clk0div6 = 2,
    #[doc = "3: CLK0DIV8"]
    Syspllcfg0Rdivclk0Clk0div8 = 3,
    #[doc = "4: CLK0DIV10"]
    Syspllcfg0Rdivclk0Clk0div10 = 4,
    #[doc = "5: CLK0DIV12"]
    Syspllcfg0Rdivclk0Clk0div12 = 5,
    #[doc = "6: CLK0DIV14"]
    Syspllcfg0Rdivclk0Clk0div14 = 6,
    #[doc = "7: CLK0DIV16"]
    Syspllcfg0Rdivclk0Clk0div16 = 7,
    #[doc = "8: CLK0DIV18"]
    Syspllcfg0Rdivclk0Clk0div18 = 8,
    #[doc = "9: CLK0DIV20"]
    Syspllcfg0Rdivclk0Clk0div20 = 9,
    #[doc = "10: CLK0DIV22"]
    Syspllcfg0Rdivclk0Clk0div22 = 10,
    #[doc = "11: CLK0DIV24"]
    Syspllcfg0Rdivclk0Clk0div24 = 11,
    #[doc = "12: CLK0DIV26"]
    Syspllcfg0Rdivclk0Clk0div26 = 12,
    #[doc = "13: CLK0DIV28"]
    Syspllcfg0Rdivclk0Clk0div28 = 13,
    #[doc = "14: CLK0DIV30"]
    Syspllcfg0Rdivclk0Clk0div30 = 14,
    #[doc = "15: CLK0DIV32"]
    Syspllcfg0Rdivclk0Clk0div32 = 15,
}
impl From<Syspllcfg0Rdivclk0> for u8 {
    #[inline(always)]
    fn from(variant: Syspllcfg0Rdivclk0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syspllcfg0Rdivclk0 {
    type Ux = u8;
}
impl crate::IsEnum for Syspllcfg0Rdivclk0 {}
#[doc = "Field `SYSPLLCFG0_RDIVCLK0` reader - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
pub type Syspllcfg0Rdivclk0R = crate::FieldReader<Syspllcfg0Rdivclk0>;
impl Syspllcfg0Rdivclk0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Rdivclk0 {
        match self.bits {
            0 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div2,
            1 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div4,
            2 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div6,
            3 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div8,
            4 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div10,
            5 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div12,
            6 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div14,
            7 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div16,
            8 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div18,
            9 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div20,
            10 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div22,
            11 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div24,
            12 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div26,
            13 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div28,
            14 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div30,
            15 => Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div32,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK0DIV2"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div2(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div2
    }
    #[doc = "CLK0DIV4"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div4(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div4
    }
    #[doc = "CLK0DIV6"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div6(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div6
    }
    #[doc = "CLK0DIV8"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div8(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div8
    }
    #[doc = "CLK0DIV10"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div10(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div10
    }
    #[doc = "CLK0DIV12"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div12(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div12
    }
    #[doc = "CLK0DIV14"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div14(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div14
    }
    #[doc = "CLK0DIV16"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div16(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div16
    }
    #[doc = "CLK0DIV18"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div18(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div18
    }
    #[doc = "CLK0DIV20"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div20(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div20
    }
    #[doc = "CLK0DIV22"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div22(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div22
    }
    #[doc = "CLK0DIV24"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div24(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div24
    }
    #[doc = "CLK0DIV26"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div26(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div26
    }
    #[doc = "CLK0DIV28"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div28(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div28
    }
    #[doc = "CLK0DIV30"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div30(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div30
    }
    #[doc = "CLK0DIV32"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk0_clk0div32(&self) -> bool {
        *self == Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div32
    }
}
#[doc = "Field `SYSPLLCFG0_RDIVCLK0` writer - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
pub type Syspllcfg0Rdivclk0W<'a, REG> =
    crate::FieldWriter<'a, REG, 4, Syspllcfg0Rdivclk0, crate::Safe>;
impl<'a, REG> Syspllcfg0Rdivclk0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK0DIV2"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div2(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div2)
    }
    #[doc = "CLK0DIV4"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div4(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div4)
    }
    #[doc = "CLK0DIV6"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div6(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div6)
    }
    #[doc = "CLK0DIV8"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div8(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div8)
    }
    #[doc = "CLK0DIV10"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div10(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div10)
    }
    #[doc = "CLK0DIV12"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div12(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div12)
    }
    #[doc = "CLK0DIV14"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div14(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div14)
    }
    #[doc = "CLK0DIV16"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div16(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div16)
    }
    #[doc = "CLK0DIV18"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div18(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div18)
    }
    #[doc = "CLK0DIV20"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div20(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div20)
    }
    #[doc = "CLK0DIV22"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div22(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div22)
    }
    #[doc = "CLK0DIV24"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div24(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div24)
    }
    #[doc = "CLK0DIV26"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div26(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div26)
    }
    #[doc = "CLK0DIV28"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div28(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div28)
    }
    #[doc = "CLK0DIV30"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div30(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div30)
    }
    #[doc = "CLK0DIV32"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0_clk0div32(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk0::Syspllcfg0Rdivclk0Clk0div32)
    }
}
#[doc = "RDIVCLK1 sets the final divider for the SYSPLLCLK1 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syspllcfg0Rdivclk1 {
    #[doc = "0: CLK1DIV2"]
    Syspllcfg0Rdivclk1Clk1div2 = 0,
    #[doc = "1: CLK1DIV4"]
    Syspllcfg0Rdivclk1Clk1div4 = 1,
    #[doc = "2: CLK1DIV6"]
    Syspllcfg0Rdivclk1Clk1div6 = 2,
    #[doc = "3: CLK1DIV8"]
    Syspllcfg0Rdivclk1Clk1div8 = 3,
    #[doc = "4: CLK1DIV10"]
    Syspllcfg0Rdivclk1Clk1div10 = 4,
    #[doc = "5: CLK1DIV12"]
    Syspllcfg0Rdivclk1Clk1div12 = 5,
    #[doc = "6: CLK1DIV14"]
    Syspllcfg0Rdivclk1Clk1div14 = 6,
    #[doc = "7: CLK1DIV16"]
    Syspllcfg0Rdivclk1Clk1div16 = 7,
    #[doc = "8: CLK1DIV18"]
    Syspllcfg0Rdivclk1Clk1div18 = 8,
    #[doc = "9: CLK1DIV20"]
    Syspllcfg0Rdivclk1Clk1div20 = 9,
    #[doc = "10: CLK1DIV22"]
    Syspllcfg0Rdivclk1Clk1div22 = 10,
    #[doc = "11: CLK1DIV24"]
    Syspllcfg0Rdivclk1Clk1div24 = 11,
    #[doc = "12: CLK1DIV26"]
    Syspllcfg0Rdivclk1Clk1div26 = 12,
    #[doc = "13: CLK1DIV28"]
    Syspllcfg0Rdivclk1Clk1div28 = 13,
    #[doc = "14: CLK1DIV30"]
    Syspllcfg0Rdivclk1Clk1div30 = 14,
    #[doc = "15: CLK1DIV32"]
    Syspllcfg0Rdivclk1Clk1div32 = 15,
}
impl From<Syspllcfg0Rdivclk1> for u8 {
    #[inline(always)]
    fn from(variant: Syspllcfg0Rdivclk1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syspllcfg0Rdivclk1 {
    type Ux = u8;
}
impl crate::IsEnum for Syspllcfg0Rdivclk1 {}
#[doc = "Field `SYSPLLCFG0_RDIVCLK1` reader - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
pub type Syspllcfg0Rdivclk1R = crate::FieldReader<Syspllcfg0Rdivclk1>;
impl Syspllcfg0Rdivclk1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Rdivclk1 {
        match self.bits {
            0 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div2,
            1 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div4,
            2 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div6,
            3 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div8,
            4 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div10,
            5 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div12,
            6 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div14,
            7 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div16,
            8 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div18,
            9 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div20,
            10 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div22,
            11 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div24,
            12 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div26,
            13 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div28,
            14 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div30,
            15 => Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div32,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK1DIV2"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div2(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div2
    }
    #[doc = "CLK1DIV4"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div4(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div4
    }
    #[doc = "CLK1DIV6"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div6(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div6
    }
    #[doc = "CLK1DIV8"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div8(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div8
    }
    #[doc = "CLK1DIV10"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div10(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div10
    }
    #[doc = "CLK1DIV12"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div12(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div12
    }
    #[doc = "CLK1DIV14"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div14(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div14
    }
    #[doc = "CLK1DIV16"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div16(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div16
    }
    #[doc = "CLK1DIV18"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div18(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div18
    }
    #[doc = "CLK1DIV20"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div20(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div20
    }
    #[doc = "CLK1DIV22"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div22(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div22
    }
    #[doc = "CLK1DIV24"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div24(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div24
    }
    #[doc = "CLK1DIV26"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div26(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div26
    }
    #[doc = "CLK1DIV28"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div28(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div28
    }
    #[doc = "CLK1DIV30"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div30(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div30
    }
    #[doc = "CLK1DIV32"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk1_clk1div32(&self) -> bool {
        *self == Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div32
    }
}
#[doc = "Field `SYSPLLCFG0_RDIVCLK1` writer - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
pub type Syspllcfg0Rdivclk1W<'a, REG> =
    crate::FieldWriter<'a, REG, 4, Syspllcfg0Rdivclk1, crate::Safe>;
impl<'a, REG> Syspllcfg0Rdivclk1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK1DIV2"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div2(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div2)
    }
    #[doc = "CLK1DIV4"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div4(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div4)
    }
    #[doc = "CLK1DIV6"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div6(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div6)
    }
    #[doc = "CLK1DIV8"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div8(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div8)
    }
    #[doc = "CLK1DIV10"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div10(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div10)
    }
    #[doc = "CLK1DIV12"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div12(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div12)
    }
    #[doc = "CLK1DIV14"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div14(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div14)
    }
    #[doc = "CLK1DIV16"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div16(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div16)
    }
    #[doc = "CLK1DIV18"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div18(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div18)
    }
    #[doc = "CLK1DIV20"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div20(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div20)
    }
    #[doc = "CLK1DIV22"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div22(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div22)
    }
    #[doc = "CLK1DIV24"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div24(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div24)
    }
    #[doc = "CLK1DIV26"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div26(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div26)
    }
    #[doc = "CLK1DIV28"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div28(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div28)
    }
    #[doc = "CLK1DIV30"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div30(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div30)
    }
    #[doc = "CLK1DIV32"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1_clk1div32(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk1::Syspllcfg0Rdivclk1Clk1div32)
    }
}
#[doc = "RDIVCLK2X sets the final divider for the SYSPLLCLK2X output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syspllcfg0Rdivclk2x {
    #[doc = "0: CLK2XDIV1"]
    Syspllcfg0Rdivclk2xClk2xdiv1 = 0,
    #[doc = "1: CLK2XDIV2"]
    Syspllcfg0Rdivclk2xClk2xdiv2 = 1,
    #[doc = "2: CLK2XDIV3"]
    Syspllcfg0Rdivclk2xClk2xdiv3 = 2,
    #[doc = "3: CLK2XDIV4"]
    Syspllcfg0Rdivclk2xClk2xdiv4 = 3,
    #[doc = "4: CLK2XDIV5"]
    Syspllcfg0Rdivclk2xClk2xdiv5 = 4,
    #[doc = "5: CLK2XDIV6"]
    Syspllcfg0Rdivclk2xClk2xdiv6 = 5,
    #[doc = "6: CLK2XDIV7"]
    Syspllcfg0Rdivclk2xClk2xdiv7 = 6,
    #[doc = "7: CLK2XDIV8"]
    Syspllcfg0Rdivclk2xClk2xdiv8 = 7,
    #[doc = "8: CLK2XDIV9"]
    Syspllcfg0Rdivclk2xClk2xdiv9 = 8,
    #[doc = "9: CLK2XDIV10"]
    Syspllcfg0Rdivclk2xClk2xdiv10 = 9,
    #[doc = "10: CLK2XDIV11"]
    Syspllcfg0Rdivclk2xClk2xdiv11 = 10,
    #[doc = "11: CLK2XDIV12"]
    Syspllcfg0Rdivclk2xClk2xdiv12 = 11,
    #[doc = "12: CLK2XDIV13"]
    Syspllcfg0Rdivclk2xClk2xdiv13 = 12,
    #[doc = "13: CLK2XDIV14"]
    Syspllcfg0Rdivclk2xClk2xdiv14 = 13,
    #[doc = "14: CLK2XDIV15"]
    Syspllcfg0Rdivclk2xClk2xdiv15 = 14,
    #[doc = "15: CLK2XDIV16"]
    Syspllcfg0Rdivclk2xClk2xdiv16 = 15,
}
impl From<Syspllcfg0Rdivclk2x> for u8 {
    #[inline(always)]
    fn from(variant: Syspllcfg0Rdivclk2x) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syspllcfg0Rdivclk2x {
    type Ux = u8;
}
impl crate::IsEnum for Syspllcfg0Rdivclk2x {}
#[doc = "Field `SYSPLLCFG0_RDIVCLK2X` reader - RDIVCLK2X sets the final divider for the SYSPLLCLK2X output."]
pub type Syspllcfg0Rdivclk2xR = crate::FieldReader<Syspllcfg0Rdivclk2x>;
impl Syspllcfg0Rdivclk2xR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Syspllcfg0Rdivclk2x {
        match self.bits {
            0 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv1,
            1 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv2,
            2 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv3,
            3 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv4,
            4 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv5,
            5 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv6,
            6 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv7,
            7 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv8,
            8 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv9,
            9 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv10,
            10 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv11,
            11 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv12,
            12 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv13,
            13 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv14,
            14 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv15,
            15 => Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv16,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK2XDIV1"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv1(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv1
    }
    #[doc = "CLK2XDIV2"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv2(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv2
    }
    #[doc = "CLK2XDIV3"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv3(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv3
    }
    #[doc = "CLK2XDIV4"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv4(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv4
    }
    #[doc = "CLK2XDIV5"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv5(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv5
    }
    #[doc = "CLK2XDIV6"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv6(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv6
    }
    #[doc = "CLK2XDIV7"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv7(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv7
    }
    #[doc = "CLK2XDIV8"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv8(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv8
    }
    #[doc = "CLK2XDIV9"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv9(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv9
    }
    #[doc = "CLK2XDIV10"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv10(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv10
    }
    #[doc = "CLK2XDIV11"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv11(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv11
    }
    #[doc = "CLK2XDIV12"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv12(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv12
    }
    #[doc = "CLK2XDIV13"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv13(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv13
    }
    #[doc = "CLK2XDIV14"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv14(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv14
    }
    #[doc = "CLK2XDIV15"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv15(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv15
    }
    #[doc = "CLK2XDIV16"]
    #[inline(always)]
    pub fn is_syspllcfg0_rdivclk2x_clk2xdiv16(&self) -> bool {
        *self == Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv16
    }
}
#[doc = "Field `SYSPLLCFG0_RDIVCLK2X` writer - RDIVCLK2X sets the final divider for the SYSPLLCLK2X output."]
pub type Syspllcfg0Rdivclk2xW<'a, REG> =
    crate::FieldWriter<'a, REG, 4, Syspllcfg0Rdivclk2x, crate::Safe>;
impl<'a, REG> Syspllcfg0Rdivclk2xW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK2XDIV1"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv1(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv1)
    }
    #[doc = "CLK2XDIV2"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv2)
    }
    #[doc = "CLK2XDIV3"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv3(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv3)
    }
    #[doc = "CLK2XDIV4"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv4(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv4)
    }
    #[doc = "CLK2XDIV5"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv5(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv5)
    }
    #[doc = "CLK2XDIV6"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv6(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv6)
    }
    #[doc = "CLK2XDIV7"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv7(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv7)
    }
    #[doc = "CLK2XDIV8"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv8(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv8)
    }
    #[doc = "CLK2XDIV9"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv9(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv9)
    }
    #[doc = "CLK2XDIV10"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv10(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv10)
    }
    #[doc = "CLK2XDIV11"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv11(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv11)
    }
    #[doc = "CLK2XDIV12"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv12(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv12)
    }
    #[doc = "CLK2XDIV13"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv13(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv13)
    }
    #[doc = "CLK2XDIV14"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv14(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv14)
    }
    #[doc = "CLK2XDIV15"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv15(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv15)
    }
    #[doc = "CLK2XDIV16"]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x_clk2xdiv16(self) -> &'a mut crate::W<REG> {
        self.variant(Syspllcfg0Rdivclk2x::Syspllcfg0Rdivclk2xClk2xdiv16)
    }
}
impl R {
    #[doc = "Bit 0 - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
    #[inline(always)]
    pub fn syspllcfg0_syspllref(&self) -> Syspllcfg0SyspllrefR {
        Syspllcfg0SyspllrefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCLK2XVCO selects the SYSPLL output which is sent to the HSCLK mux for use by MCLK."]
    #[inline(always)]
    pub fn syspllcfg0_mclk2xvco(&self) -> Syspllcfg0Mclk2xvcoR {
        Syspllcfg0Mclk2xvcoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn syspllcfg0_enableclk0(&self) -> Syspllcfg0Enableclk0R {
        Syspllcfg0Enableclk0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn syspllcfg0_enableclk1(&self) -> Syspllcfg0Enableclk1R {
        Syspllcfg0Enableclk1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ENABLECLK2X enables or disables the SYSPLLCLK2X output."]
    #[inline(always)]
    pub fn syspllcfg0_enableclk2x(&self) -> Syspllcfg0Enableclk2xR {
        Syspllcfg0Enableclk2xR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0(&self) -> Syspllcfg0Rdivclk0R {
        Syspllcfg0Rdivclk0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1(&self) -> Syspllcfg0Rdivclk1R {
        Syspllcfg0Rdivclk1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RDIVCLK2X sets the final divider for the SYSPLLCLK2X output."]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x(&self) -> Syspllcfg0Rdivclk2xR {
        Syspllcfg0Rdivclk2xR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SYSPLLREF selects the system PLL (SYSPLL) reference clock source."]
    #[inline(always)]
    pub fn syspllcfg0_syspllref(&mut self) -> Syspllcfg0SyspllrefW<Syspllcfg0Spec> {
        Syspllcfg0SyspllrefW::new(self, 0)
    }
    #[doc = "Bit 1 - MCLK2XVCO selects the SYSPLL output which is sent to the HSCLK mux for use by MCLK."]
    #[inline(always)]
    pub fn syspllcfg0_mclk2xvco(&mut self) -> Syspllcfg0Mclk2xvcoW<Syspllcfg0Spec> {
        Syspllcfg0Mclk2xvcoW::new(self, 1)
    }
    #[doc = "Bit 4 - ENABLECLK0 enables or disables the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn syspllcfg0_enableclk0(&mut self) -> Syspllcfg0Enableclk0W<Syspllcfg0Spec> {
        Syspllcfg0Enableclk0W::new(self, 4)
    }
    #[doc = "Bit 5 - ENABLECLK1 enables or disables the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn syspllcfg0_enableclk1(&mut self) -> Syspllcfg0Enableclk1W<Syspllcfg0Spec> {
        Syspllcfg0Enableclk1W::new(self, 5)
    }
    #[doc = "Bit 6 - ENABLECLK2X enables or disables the SYSPLLCLK2X output."]
    #[inline(always)]
    pub fn syspllcfg0_enableclk2x(&mut self) -> Syspllcfg0Enableclk2xW<Syspllcfg0Spec> {
        Syspllcfg0Enableclk2xW::new(self, 6)
    }
    #[doc = "Bits 8:11 - RDIVCLK0 sets the final divider for the SYSPLLCLK0 output."]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk0(&mut self) -> Syspllcfg0Rdivclk0W<Syspllcfg0Spec> {
        Syspllcfg0Rdivclk0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - RDIVCLK1 sets the final divider for the SYSPLLCLK1 output."]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk1(&mut self) -> Syspllcfg0Rdivclk1W<Syspllcfg0Spec> {
        Syspllcfg0Rdivclk1W::new(self, 12)
    }
    #[doc = "Bits 16:19 - RDIVCLK2X sets the final divider for the SYSPLLCLK2X output."]
    #[inline(always)]
    pub fn syspllcfg0_rdivclk2x(&mut self) -> Syspllcfg0Rdivclk2xW<Syspllcfg0Spec> {
        Syspllcfg0Rdivclk2xW::new(self, 16)
    }
}
#[doc = "SYSPLL reference and output configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`syspllcfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syspllcfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syspllcfg0Spec;
impl crate::RegisterSpec for Syspllcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syspllcfg0::R`](R) reader structure"]
impl crate::Readable for Syspllcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`syspllcfg0::W`](W) writer structure"]
impl crate::Writable for Syspllcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSPLLCFG0 to value 0"]
impl crate::Resettable for Syspllcfg0Spec {
    const RESET_VALUE: u32 = 0;
}

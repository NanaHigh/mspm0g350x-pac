#[doc = "Register `CTL0` reader"]
pub type R = crate::R<Ctl0Spec>;
#[doc = "Register `CTL0` writer"]
pub type W = crate::W<Ctl0Spec>;
#[doc = "Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Dss {
    #[doc = "3: DSS_4"]
    Ctl0DssDss4 = 3,
    #[doc = "4: DSS_5"]
    Ctl0DssDss5 = 4,
    #[doc = "5: DSS_6"]
    Ctl0DssDss6 = 5,
    #[doc = "6: DSS_7"]
    Ctl0DssDss7 = 6,
    #[doc = "7: DSS_8"]
    Ctl0DssDss8 = 7,
    #[doc = "8: DSS_9"]
    Ctl0DssDss9 = 8,
    #[doc = "9: DSS_10"]
    Ctl0DssDss10 = 9,
    #[doc = "10: DSS_11"]
    Ctl0DssDss11 = 10,
    #[doc = "11: DSS_12"]
    Ctl0DssDss12 = 11,
    #[doc = "12: DSS_13"]
    Ctl0DssDss13 = 12,
    #[doc = "13: DSS_14"]
    Ctl0DssDss14 = 13,
    #[doc = "14: DSS_15"]
    Ctl0DssDss15 = 14,
    #[doc = "15: DSS_16"]
    Ctl0DssDss16 = 15,
    #[doc = "31: DSS_32"]
    Ctl0DssDss32 = 31,
}
impl From<Ctl0Dss> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Dss {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Dss {}
#[doc = "Field `CTL0_DSS` reader - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
pub type Ctl0DssR = crate::FieldReader<Ctl0Dss>;
impl Ctl0DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ctl0Dss> {
        match self.bits {
            3 => Some(Ctl0Dss::Ctl0DssDss4),
            4 => Some(Ctl0Dss::Ctl0DssDss5),
            5 => Some(Ctl0Dss::Ctl0DssDss6),
            6 => Some(Ctl0Dss::Ctl0DssDss7),
            7 => Some(Ctl0Dss::Ctl0DssDss8),
            8 => Some(Ctl0Dss::Ctl0DssDss9),
            9 => Some(Ctl0Dss::Ctl0DssDss10),
            10 => Some(Ctl0Dss::Ctl0DssDss11),
            11 => Some(Ctl0Dss::Ctl0DssDss12),
            12 => Some(Ctl0Dss::Ctl0DssDss13),
            13 => Some(Ctl0Dss::Ctl0DssDss14),
            14 => Some(Ctl0Dss::Ctl0DssDss15),
            15 => Some(Ctl0Dss::Ctl0DssDss16),
            31 => Some(Ctl0Dss::Ctl0DssDss32),
            _ => None,
        }
    }
    #[doc = "DSS_4"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_4(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss4
    }
    #[doc = "DSS_5"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_5(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss5
    }
    #[doc = "DSS_6"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_6(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss6
    }
    #[doc = "DSS_7"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_7(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss7
    }
    #[doc = "DSS_8"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_8(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss8
    }
    #[doc = "DSS_9"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_9(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss9
    }
    #[doc = "DSS_10"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_10(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss10
    }
    #[doc = "DSS_11"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_11(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss11
    }
    #[doc = "DSS_12"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_12(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss12
    }
    #[doc = "DSS_13"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_13(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss13
    }
    #[doc = "DSS_14"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_14(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss14
    }
    #[doc = "DSS_15"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_15(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss15
    }
    #[doc = "DSS_16"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_16(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss16
    }
    #[doc = "DSS_32"]
    #[inline(always)]
    pub fn is_ctl0_dss_dss_32(&self) -> bool {
        *self == Ctl0Dss::Ctl0DssDss32
    }
}
#[doc = "Field `CTL0_DSS` writer - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
pub type Ctl0DssW<'a, REG> = crate::FieldWriter<'a, REG, 5, Ctl0Dss>;
impl<'a, REG> Ctl0DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSS_4"]
    #[inline(always)]
    pub fn ctl0_dss_dss_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss4)
    }
    #[doc = "DSS_5"]
    #[inline(always)]
    pub fn ctl0_dss_dss_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss5)
    }
    #[doc = "DSS_6"]
    #[inline(always)]
    pub fn ctl0_dss_dss_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss6)
    }
    #[doc = "DSS_7"]
    #[inline(always)]
    pub fn ctl0_dss_dss_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss7)
    }
    #[doc = "DSS_8"]
    #[inline(always)]
    pub fn ctl0_dss_dss_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss8)
    }
    #[doc = "DSS_9"]
    #[inline(always)]
    pub fn ctl0_dss_dss_9(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss9)
    }
    #[doc = "DSS_10"]
    #[inline(always)]
    pub fn ctl0_dss_dss_10(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss10)
    }
    #[doc = "DSS_11"]
    #[inline(always)]
    pub fn ctl0_dss_dss_11(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss11)
    }
    #[doc = "DSS_12"]
    #[inline(always)]
    pub fn ctl0_dss_dss_12(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss12)
    }
    #[doc = "DSS_13"]
    #[inline(always)]
    pub fn ctl0_dss_dss_13(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss13)
    }
    #[doc = "DSS_14"]
    #[inline(always)]
    pub fn ctl0_dss_dss_14(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss14)
    }
    #[doc = "DSS_15"]
    #[inline(always)]
    pub fn ctl0_dss_dss_15(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss15)
    }
    #[doc = "DSS_16"]
    #[inline(always)]
    pub fn ctl0_dss_dss_16(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss16)
    }
    #[doc = "DSS_32"]
    #[inline(always)]
    pub fn ctl0_dss_dss_32(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Dss::Ctl0DssDss32)
    }
}
#[doc = "Frame format Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Frf {
    #[doc = "0: MOTOROLA_3WIRE"]
    Ctl0FrfMotorola3wire = 0,
    #[doc = "1: MOTOROLA_4WIRE"]
    Ctl0FrfMotorola4wire = 1,
    #[doc = "2: TI_SYNC"]
    Ctl0FrfTiSync = 2,
    #[doc = "3: MIRCOWIRE"]
    Ctl0FrfMircowire = 3,
}
impl From<Ctl0Frf> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Frf {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Frf {}
#[doc = "Field `CTL0_FRF` reader - Frame format Select"]
pub type Ctl0FrfR = crate::FieldReader<Ctl0Frf>;
impl Ctl0FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Frf {
        match self.bits {
            0 => Ctl0Frf::Ctl0FrfMotorola3wire,
            1 => Ctl0Frf::Ctl0FrfMotorola4wire,
            2 => Ctl0Frf::Ctl0FrfTiSync,
            3 => Ctl0Frf::Ctl0FrfMircowire,
            _ => unreachable!(),
        }
    }
    #[doc = "MOTOROLA_3WIRE"]
    #[inline(always)]
    pub fn is_ctl0_frf_motorola_3wire(&self) -> bool {
        *self == Ctl0Frf::Ctl0FrfMotorola3wire
    }
    #[doc = "MOTOROLA_4WIRE"]
    #[inline(always)]
    pub fn is_ctl0_frf_motorola_4wire(&self) -> bool {
        *self == Ctl0Frf::Ctl0FrfMotorola4wire
    }
    #[doc = "TI_SYNC"]
    #[inline(always)]
    pub fn is_ctl0_frf_ti_sync(&self) -> bool {
        *self == Ctl0Frf::Ctl0FrfTiSync
    }
    #[doc = "MIRCOWIRE"]
    #[inline(always)]
    pub fn is_ctl0_frf_mircowire(&self) -> bool {
        *self == Ctl0Frf::Ctl0FrfMircowire
    }
}
#[doc = "Field `CTL0_FRF` writer - Frame format Select"]
pub type Ctl0FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl0Frf, crate::Safe>;
impl<'a, REG> Ctl0FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MOTOROLA_3WIRE"]
    #[inline(always)]
    pub fn ctl0_frf_motorola_3wire(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Frf::Ctl0FrfMotorola3wire)
    }
    #[doc = "MOTOROLA_4WIRE"]
    #[inline(always)]
    pub fn ctl0_frf_motorola_4wire(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Frf::Ctl0FrfMotorola4wire)
    }
    #[doc = "TI_SYNC"]
    #[inline(always)]
    pub fn ctl0_frf_ti_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Frf::Ctl0FrfTiSync)
    }
    #[doc = "MIRCOWIRE"]
    #[inline(always)]
    pub fn ctl0_frf_mircowire(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Frf::Ctl0FrfMircowire)
    }
}
#[doc = "Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Packen {
    #[doc = "0: DISABLED"]
    Ctl0PackenDisabled = 0,
    #[doc = "1: ENABLED"]
    Ctl0PackenEnabled = 1,
}
impl From<Ctl0Packen> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Packen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_PACKEN` reader - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
pub type Ctl0PackenR = crate::BitReader<Ctl0Packen>;
impl Ctl0PackenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Packen {
        match self.bits {
            false => Ctl0Packen::Ctl0PackenDisabled,
            true => Ctl0Packen::Ctl0PackenEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_ctl0_packen_disabled(&self) -> bool {
        *self == Ctl0Packen::Ctl0PackenDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_ctl0_packen_enabled(&self) -> bool {
        *self == Ctl0Packen::Ctl0PackenEnabled
    }
}
#[doc = "Field `CTL0_PACKEN` writer - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
pub type Ctl0PackenW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Packen>;
impl<'a, REG> Ctl0PackenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn ctl0_packen_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Packen::Ctl0PackenDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn ctl0_packen_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Packen::Ctl0PackenEnabled)
    }
}
#[doc = "CLKOUT polarity (Motorola SPI frame format only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Spo {
    #[doc = "0: LOW"]
    Ctl0SpoLow = 0,
    #[doc = "1: HIGH"]
    Ctl0SpoHigh = 1,
}
impl From<Ctl0Spo> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Spo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_SPO` reader - CLKOUT polarity (Motorola SPI frame format only)"]
pub type Ctl0SpoR = crate::BitReader<Ctl0Spo>;
impl Ctl0SpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Spo {
        match self.bits {
            false => Ctl0Spo::Ctl0SpoLow,
            true => Ctl0Spo::Ctl0SpoHigh,
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_ctl0_spo_low(&self) -> bool {
        *self == Ctl0Spo::Ctl0SpoLow
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_ctl0_spo_high(&self) -> bool {
        *self == Ctl0Spo::Ctl0SpoHigh
    }
}
#[doc = "Field `CTL0_SPO` writer - CLKOUT polarity (Motorola SPI frame format only)"]
pub type Ctl0SpoW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Spo>;
impl<'a, REG> Ctl0SpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOW"]
    #[inline(always)]
    pub fn ctl0_spo_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Spo::Ctl0SpoLow)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn ctl0_spo_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Spo::Ctl0SpoHigh)
    }
}
#[doc = "CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Sph {
    #[doc = "0: FIRST"]
    Ctl0SphFirst = 0,
    #[doc = "1: SECOND"]
    Ctl0SphSecond = 1,
}
impl From<Ctl0Sph> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Sph) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_SPH` reader - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type Ctl0SphR = crate::BitReader<Ctl0Sph>;
impl Ctl0SphR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Sph {
        match self.bits {
            false => Ctl0Sph::Ctl0SphFirst,
            true => Ctl0Sph::Ctl0SphSecond,
        }
    }
    #[doc = "FIRST"]
    #[inline(always)]
    pub fn is_ctl0_sph_first(&self) -> bool {
        *self == Ctl0Sph::Ctl0SphFirst
    }
    #[doc = "SECOND"]
    #[inline(always)]
    pub fn is_ctl0_sph_second(&self) -> bool {
        *self == Ctl0Sph::Ctl0SphSecond
    }
}
#[doc = "Field `CTL0_SPH` writer - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
pub type Ctl0SphW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Sph>;
impl<'a, REG> Ctl0SphW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FIRST"]
    #[inline(always)]
    pub fn ctl0_sph_first(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sph::Ctl0SphFirst)
    }
    #[doc = "SECOND"]
    #[inline(always)]
    pub fn ctl0_sph_second(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Sph::Ctl0SphSecond)
    }
}
#[doc = "Select the CS line to control on data transfer This bit is for controller mode only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl0Cssel {
    #[doc = "0: CSSEL_0"]
    Ctl0CsselCssel0 = 0,
    #[doc = "1: CSSEL_1"]
    Ctl0CsselCssel1 = 1,
    #[doc = "2: CSSEL_2"]
    Ctl0CsselCssel2 = 2,
    #[doc = "3: CSSEL_3"]
    Ctl0CsselCssel3 = 3,
}
impl From<Ctl0Cssel> for u8 {
    #[inline(always)]
    fn from(variant: Ctl0Cssel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl0Cssel {
    type Ux = u8;
}
impl crate::IsEnum for Ctl0Cssel {}
#[doc = "Field `CTL0_CSSEL` reader - Select the CS line to control on data transfer This bit is for controller mode only."]
pub type Ctl0CsselR = crate::FieldReader<Ctl0Cssel>;
impl Ctl0CsselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Cssel {
        match self.bits {
            0 => Ctl0Cssel::Ctl0CsselCssel0,
            1 => Ctl0Cssel::Ctl0CsselCssel1,
            2 => Ctl0Cssel::Ctl0CsselCssel2,
            3 => Ctl0Cssel::Ctl0CsselCssel3,
            _ => unreachable!(),
        }
    }
    #[doc = "CSSEL_0"]
    #[inline(always)]
    pub fn is_ctl0_cssel_cssel_0(&self) -> bool {
        *self == Ctl0Cssel::Ctl0CsselCssel0
    }
    #[doc = "CSSEL_1"]
    #[inline(always)]
    pub fn is_ctl0_cssel_cssel_1(&self) -> bool {
        *self == Ctl0Cssel::Ctl0CsselCssel1
    }
    #[doc = "CSSEL_2"]
    #[inline(always)]
    pub fn is_ctl0_cssel_cssel_2(&self) -> bool {
        *self == Ctl0Cssel::Ctl0CsselCssel2
    }
    #[doc = "CSSEL_3"]
    #[inline(always)]
    pub fn is_ctl0_cssel_cssel_3(&self) -> bool {
        *self == Ctl0Cssel::Ctl0CsselCssel3
    }
}
#[doc = "Field `CTL0_CSSEL` writer - Select the CS line to control on data transfer This bit is for controller mode only."]
pub type Ctl0CsselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl0Cssel, crate::Safe>;
impl<'a, REG> Ctl0CsselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSSEL_0"]
    #[inline(always)]
    pub fn ctl0_cssel_cssel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Cssel::Ctl0CsselCssel0)
    }
    #[doc = "CSSEL_1"]
    #[inline(always)]
    pub fn ctl0_cssel_cssel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Cssel::Ctl0CsselCssel1)
    }
    #[doc = "CSSEL_2"]
    #[inline(always)]
    pub fn ctl0_cssel_cssel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Cssel::Ctl0CsselCssel2)
    }
    #[doc = "CSSEL_3"]
    #[inline(always)]
    pub fn ctl0_cssel_cssel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Cssel::Ctl0CsselCssel3)
    }
}
#[doc = "Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.MS=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl0Csclr {
    #[doc = "0: DISABLE"]
    Ctl0CsclrDisable = 0,
    #[doc = "1: ENABLE"]
    Ctl0CsclrEnable = 1,
}
impl From<Ctl0Csclr> for bool {
    #[inline(always)]
    fn from(variant: Ctl0Csclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL0_CSCLR` reader - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.MS=0."]
pub type Ctl0CsclrR = crate::BitReader<Ctl0Csclr>;
impl Ctl0CsclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl0Csclr {
        match self.bits {
            false => Ctl0Csclr::Ctl0CsclrDisable,
            true => Ctl0Csclr::Ctl0CsclrEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl0_csclr_disable(&self) -> bool {
        *self == Ctl0Csclr::Ctl0CsclrDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl0_csclr_enable(&self) -> bool {
        *self == Ctl0Csclr::Ctl0CsclrEnable
    }
}
#[doc = "Field `CTL0_CSCLR` writer - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.MS=0."]
pub type Ctl0CsclrW<'a, REG> = crate::BitWriter<'a, REG, Ctl0Csclr>;
impl<'a, REG> Ctl0CsclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl0_csclr_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Csclr::Ctl0CsclrDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl0_csclr_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl0Csclr::Ctl0CsclrEnable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
    #[inline(always)]
    pub fn ctl0_dss(&self) -> Ctl0DssR {
        Ctl0DssR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Frame format Select"]
    #[inline(always)]
    pub fn ctl0_frf(&self) -> Ctl0FrfR {
        Ctl0FrfR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
    #[inline(always)]
    pub fn ctl0_packen(&self) -> Ctl0PackenR {
        Ctl0PackenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn ctl0_spo(&self) -> Ctl0SpoR {
        Ctl0SpoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn ctl0_sph(&self) -> Ctl0SphR {
        Ctl0SphR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Select the CS line to control on data transfer This bit is for controller mode only."]
    #[inline(always)]
    pub fn ctl0_cssel(&self) -> Ctl0CsselR {
        Ctl0CsselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.MS=0."]
    #[inline(always)]
    pub fn ctl0_csclr(&self) -> Ctl0CsclrR {
        Ctl0CsclrR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Data Size Select. Values 0 - 2 are reserved and shall not be used. 3h = 4_BIT : 4-bit data SPI allows only values up to 16 Bit"]
    #[inline(always)]
    pub fn ctl0_dss(&mut self) -> Ctl0DssW<Ctl0Spec> {
        Ctl0DssW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Frame format Select"]
    #[inline(always)]
    pub fn ctl0_frf(&mut self) -> Ctl0FrfW<Ctl0Spec> {
        Ctl0FrfW::new(self, 5)
    }
    #[doc = "Bit 7 - Packing Enable. When 1, packing feature is enabled inside the IP When 0, packing feature is disabled inside the IP"]
    #[inline(always)]
    pub fn ctl0_packen(&mut self) -> Ctl0PackenW<Ctl0Spec> {
        Ctl0PackenW::new(self, 7)
    }
    #[doc = "Bit 8 - CLKOUT polarity (Motorola SPI frame format only)"]
    #[inline(always)]
    pub fn ctl0_spo(&mut self) -> Ctl0SpoW<Ctl0Spec> {
        Ctl0SpoW::new(self, 8)
    }
    #[doc = "Bit 9 - CLKOUT phase (Motorola SPI frame format only) This bit selects the clock edge that captures data and enables it to change state. It has the most impact on the first bit transmitted by either permitting or not permitting a clock transition before the first data capture edge."]
    #[inline(always)]
    pub fn ctl0_sph(&mut self) -> Ctl0SphW<Ctl0Spec> {
        Ctl0SphW::new(self, 9)
    }
    #[doc = "Bits 12:13 - Select the CS line to control on data transfer This bit is for controller mode only."]
    #[inline(always)]
    pub fn ctl0_cssel(&mut self) -> Ctl0CsselW<Ctl0Spec> {
        Ctl0CsselW::new(self, 12)
    }
    #[doc = "Bit 14 - Clear shift register counter on CS inactive This bit is relevant only in the peripheral, CTL1.MS=0."]
    #[inline(always)]
    pub fn ctl0_csclr(&mut self) -> Ctl0CsclrW<Ctl0Spec> {
        Ctl0CsclrW::new(self, 14)
    }
}
#[doc = "SPI control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl0Spec;
impl crate::RegisterSpec for Ctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl0::R`](R) reader structure"]
impl crate::Readable for Ctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl0::W`](W) writer structure"]
impl crate::Writable for Ctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0"]
impl crate::Resettable for Ctl0Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FILTEREN15_0` reader"]
pub type R = crate::R<Filteren15_0Spec>;
#[doc = "Register `FILTEREN15_0` writer"]
pub type W = crate::W<Filteren15_0Spec>;
#[doc = "Programmable counter length of digital glitch filter for DIN0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din0 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din0Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din0OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din0ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din0EightCycle = 3,
}
impl From<Filteren15_0Din0> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din0 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din0 {}
#[doc = "Field `FILTEREN15_0_DIN0` reader - Programmable counter length of digital glitch filter for DIN0"]
pub type Filteren15_0Din0R = crate::FieldReader<Filteren15_0Din0>;
impl Filteren15_0Din0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din0 {
        match self.bits {
            0 => Filteren15_0Din0::Filteren15_0Din0Disable,
            1 => Filteren15_0Din0::Filteren15_0Din0OneCycle,
            2 => Filteren15_0Din0::Filteren15_0Din0ThreeCycle,
            3 => Filteren15_0Din0::Filteren15_0Din0EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din0_disable(&self) -> bool {
        *self == Filteren15_0Din0::Filteren15_0Din0Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din0_one_cycle(&self) -> bool {
        *self == Filteren15_0Din0::Filteren15_0Din0OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din0_three_cycle(&self) -> bool {
        *self == Filteren15_0Din0::Filteren15_0Din0ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din0_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din0::Filteren15_0Din0EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN0` writer - Programmable counter length of digital glitch filter for DIN0"]
pub type Filteren15_0Din0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din0, crate::Safe>;
impl<'a, REG> Filteren15_0Din0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din0_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din0::Filteren15_0Din0Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din0_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din0::Filteren15_0Din0OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din0_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din0::Filteren15_0Din0ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din0_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din0::Filteren15_0Din0EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din1 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din1Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din1OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din1ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din1EightCycle = 3,
}
impl From<Filteren15_0Din1> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din1 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din1 {}
#[doc = "Field `FILTEREN15_0_DIN1` reader - Programmable counter length of digital glitch filter for DIN1"]
pub type Filteren15_0Din1R = crate::FieldReader<Filteren15_0Din1>;
impl Filteren15_0Din1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din1 {
        match self.bits {
            0 => Filteren15_0Din1::Filteren15_0Din1Disable,
            1 => Filteren15_0Din1::Filteren15_0Din1OneCycle,
            2 => Filteren15_0Din1::Filteren15_0Din1ThreeCycle,
            3 => Filteren15_0Din1::Filteren15_0Din1EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din1_disable(&self) -> bool {
        *self == Filteren15_0Din1::Filteren15_0Din1Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din1_one_cycle(&self) -> bool {
        *self == Filteren15_0Din1::Filteren15_0Din1OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din1_three_cycle(&self) -> bool {
        *self == Filteren15_0Din1::Filteren15_0Din1ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din1_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din1::Filteren15_0Din1EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN1` writer - Programmable counter length of digital glitch filter for DIN1"]
pub type Filteren15_0Din1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din1, crate::Safe>;
impl<'a, REG> Filteren15_0Din1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din1::Filteren15_0Din1Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din1_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din1::Filteren15_0Din1OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din1_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din1::Filteren15_0Din1ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din1_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din1::Filteren15_0Din1EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din2 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din2Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din2OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din2ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din2EightCycle = 3,
}
impl From<Filteren15_0Din2> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din2 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din2 {}
#[doc = "Field `FILTEREN15_0_DIN2` reader - Programmable counter length of digital glitch filter for DIN2"]
pub type Filteren15_0Din2R = crate::FieldReader<Filteren15_0Din2>;
impl Filteren15_0Din2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din2 {
        match self.bits {
            0 => Filteren15_0Din2::Filteren15_0Din2Disable,
            1 => Filteren15_0Din2::Filteren15_0Din2OneCycle,
            2 => Filteren15_0Din2::Filteren15_0Din2ThreeCycle,
            3 => Filteren15_0Din2::Filteren15_0Din2EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din2_disable(&self) -> bool {
        *self == Filteren15_0Din2::Filteren15_0Din2Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din2_one_cycle(&self) -> bool {
        *self == Filteren15_0Din2::Filteren15_0Din2OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din2_three_cycle(&self) -> bool {
        *self == Filteren15_0Din2::Filteren15_0Din2ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din2_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din2::Filteren15_0Din2EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN2` writer - Programmable counter length of digital glitch filter for DIN2"]
pub type Filteren15_0Din2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din2, crate::Safe>;
impl<'a, REG> Filteren15_0Din2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din2_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din2::Filteren15_0Din2Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din2_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din2::Filteren15_0Din2OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din2_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din2::Filteren15_0Din2ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din2_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din2::Filteren15_0Din2EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din3 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din3Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din3OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din3ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din3EightCycle = 3,
}
impl From<Filteren15_0Din3> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din3 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din3 {}
#[doc = "Field `FILTEREN15_0_DIN3` reader - Programmable counter length of digital glitch filter for DIN3"]
pub type Filteren15_0Din3R = crate::FieldReader<Filteren15_0Din3>;
impl Filteren15_0Din3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din3 {
        match self.bits {
            0 => Filteren15_0Din3::Filteren15_0Din3Disable,
            1 => Filteren15_0Din3::Filteren15_0Din3OneCycle,
            2 => Filteren15_0Din3::Filteren15_0Din3ThreeCycle,
            3 => Filteren15_0Din3::Filteren15_0Din3EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din3_disable(&self) -> bool {
        *self == Filteren15_0Din3::Filteren15_0Din3Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din3_one_cycle(&self) -> bool {
        *self == Filteren15_0Din3::Filteren15_0Din3OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din3_three_cycle(&self) -> bool {
        *self == Filteren15_0Din3::Filteren15_0Din3ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din3_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din3::Filteren15_0Din3EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN3` writer - Programmable counter length of digital glitch filter for DIN3"]
pub type Filteren15_0Din3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din3, crate::Safe>;
impl<'a, REG> Filteren15_0Din3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din3_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din3::Filteren15_0Din3Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din3_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din3::Filteren15_0Din3OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din3_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din3::Filteren15_0Din3ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din3_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din3::Filteren15_0Din3EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din4 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din4Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din4OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din4ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din4EightCycle = 3,
}
impl From<Filteren15_0Din4> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din4 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din4 {}
#[doc = "Field `FILTEREN15_0_DIN4` reader - Programmable counter length of digital glitch filter for DIN4"]
pub type Filteren15_0Din4R = crate::FieldReader<Filteren15_0Din4>;
impl Filteren15_0Din4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din4 {
        match self.bits {
            0 => Filteren15_0Din4::Filteren15_0Din4Disable,
            1 => Filteren15_0Din4::Filteren15_0Din4OneCycle,
            2 => Filteren15_0Din4::Filteren15_0Din4ThreeCycle,
            3 => Filteren15_0Din4::Filteren15_0Din4EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din4_disable(&self) -> bool {
        *self == Filteren15_0Din4::Filteren15_0Din4Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din4_one_cycle(&self) -> bool {
        *self == Filteren15_0Din4::Filteren15_0Din4OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din4_three_cycle(&self) -> bool {
        *self == Filteren15_0Din4::Filteren15_0Din4ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din4_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din4::Filteren15_0Din4EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN4` writer - Programmable counter length of digital glitch filter for DIN4"]
pub type Filteren15_0Din4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din4, crate::Safe>;
impl<'a, REG> Filteren15_0Din4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din4_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din4::Filteren15_0Din4Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din4_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din4::Filteren15_0Din4OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din4_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din4::Filteren15_0Din4ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din4_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din4::Filteren15_0Din4EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din5 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din5Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din5OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din5ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din5EightCycle = 3,
}
impl From<Filteren15_0Din5> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din5 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din5 {}
#[doc = "Field `FILTEREN15_0_DIN5` reader - Programmable counter length of digital glitch filter for DIN5"]
pub type Filteren15_0Din5R = crate::FieldReader<Filteren15_0Din5>;
impl Filteren15_0Din5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din5 {
        match self.bits {
            0 => Filteren15_0Din5::Filteren15_0Din5Disable,
            1 => Filteren15_0Din5::Filteren15_0Din5OneCycle,
            2 => Filteren15_0Din5::Filteren15_0Din5ThreeCycle,
            3 => Filteren15_0Din5::Filteren15_0Din5EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din5_disable(&self) -> bool {
        *self == Filteren15_0Din5::Filteren15_0Din5Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din5_one_cycle(&self) -> bool {
        *self == Filteren15_0Din5::Filteren15_0Din5OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din5_three_cycle(&self) -> bool {
        *self == Filteren15_0Din5::Filteren15_0Din5ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din5_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din5::Filteren15_0Din5EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN5` writer - Programmable counter length of digital glitch filter for DIN5"]
pub type Filteren15_0Din5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din5, crate::Safe>;
impl<'a, REG> Filteren15_0Din5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din5_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din5::Filteren15_0Din5Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din5_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din5::Filteren15_0Din5OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din5_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din5::Filteren15_0Din5ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din5_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din5::Filteren15_0Din5EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din6 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din6Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din6OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din6ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din6EightCycle = 3,
}
impl From<Filteren15_0Din6> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din6 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din6 {}
#[doc = "Field `FILTEREN15_0_DIN6` reader - Programmable counter length of digital glitch filter for DIN6"]
pub type Filteren15_0Din6R = crate::FieldReader<Filteren15_0Din6>;
impl Filteren15_0Din6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din6 {
        match self.bits {
            0 => Filteren15_0Din6::Filteren15_0Din6Disable,
            1 => Filteren15_0Din6::Filteren15_0Din6OneCycle,
            2 => Filteren15_0Din6::Filteren15_0Din6ThreeCycle,
            3 => Filteren15_0Din6::Filteren15_0Din6EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din6_disable(&self) -> bool {
        *self == Filteren15_0Din6::Filteren15_0Din6Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din6_one_cycle(&self) -> bool {
        *self == Filteren15_0Din6::Filteren15_0Din6OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din6_three_cycle(&self) -> bool {
        *self == Filteren15_0Din6::Filteren15_0Din6ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din6_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din6::Filteren15_0Din6EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN6` writer - Programmable counter length of digital glitch filter for DIN6"]
pub type Filteren15_0Din6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din6, crate::Safe>;
impl<'a, REG> Filteren15_0Din6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din6_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din6::Filteren15_0Din6Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din6_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din6::Filteren15_0Din6OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din6_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din6::Filteren15_0Din6ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din6_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din6::Filteren15_0Din6EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din7 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din7Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din7OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din7ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din7EightCycle = 3,
}
impl From<Filteren15_0Din7> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din7 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din7 {}
#[doc = "Field `FILTEREN15_0_DIN7` reader - Programmable counter length of digital glitch filter for DIN7"]
pub type Filteren15_0Din7R = crate::FieldReader<Filteren15_0Din7>;
impl Filteren15_0Din7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din7 {
        match self.bits {
            0 => Filteren15_0Din7::Filteren15_0Din7Disable,
            1 => Filteren15_0Din7::Filteren15_0Din7OneCycle,
            2 => Filteren15_0Din7::Filteren15_0Din7ThreeCycle,
            3 => Filteren15_0Din7::Filteren15_0Din7EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din7_disable(&self) -> bool {
        *self == Filteren15_0Din7::Filteren15_0Din7Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din7_one_cycle(&self) -> bool {
        *self == Filteren15_0Din7::Filteren15_0Din7OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din7_three_cycle(&self) -> bool {
        *self == Filteren15_0Din7::Filteren15_0Din7ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din7_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din7::Filteren15_0Din7EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN7` writer - Programmable counter length of digital glitch filter for DIN7"]
pub type Filteren15_0Din7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din7, crate::Safe>;
impl<'a, REG> Filteren15_0Din7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din7_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din7::Filteren15_0Din7Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din7_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din7::Filteren15_0Din7OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din7_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din7::Filteren15_0Din7ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din7_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din7::Filteren15_0Din7EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din8 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din8Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din8OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din8ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din8EightCycle = 3,
}
impl From<Filteren15_0Din8> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din8 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din8 {}
#[doc = "Field `FILTEREN15_0_DIN8` reader - Programmable counter length of digital glitch filter for DIN8"]
pub type Filteren15_0Din8R = crate::FieldReader<Filteren15_0Din8>;
impl Filteren15_0Din8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din8 {
        match self.bits {
            0 => Filteren15_0Din8::Filteren15_0Din8Disable,
            1 => Filteren15_0Din8::Filteren15_0Din8OneCycle,
            2 => Filteren15_0Din8::Filteren15_0Din8ThreeCycle,
            3 => Filteren15_0Din8::Filteren15_0Din8EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din8_disable(&self) -> bool {
        *self == Filteren15_0Din8::Filteren15_0Din8Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din8_one_cycle(&self) -> bool {
        *self == Filteren15_0Din8::Filteren15_0Din8OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din8_three_cycle(&self) -> bool {
        *self == Filteren15_0Din8::Filteren15_0Din8ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din8_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din8::Filteren15_0Din8EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN8` writer - Programmable counter length of digital glitch filter for DIN8"]
pub type Filteren15_0Din8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din8, crate::Safe>;
impl<'a, REG> Filteren15_0Din8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din8_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din8::Filteren15_0Din8Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din8_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din8::Filteren15_0Din8OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din8_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din8::Filteren15_0Din8ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din8_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din8::Filteren15_0Din8EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din9 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din9Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din9OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din9ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din9EightCycle = 3,
}
impl From<Filteren15_0Din9> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din9 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din9 {}
#[doc = "Field `FILTEREN15_0_DIN9` reader - Programmable counter length of digital glitch filter for DIN9"]
pub type Filteren15_0Din9R = crate::FieldReader<Filteren15_0Din9>;
impl Filteren15_0Din9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din9 {
        match self.bits {
            0 => Filteren15_0Din9::Filteren15_0Din9Disable,
            1 => Filteren15_0Din9::Filteren15_0Din9OneCycle,
            2 => Filteren15_0Din9::Filteren15_0Din9ThreeCycle,
            3 => Filteren15_0Din9::Filteren15_0Din9EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din9_disable(&self) -> bool {
        *self == Filteren15_0Din9::Filteren15_0Din9Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din9_one_cycle(&self) -> bool {
        *self == Filteren15_0Din9::Filteren15_0Din9OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din9_three_cycle(&self) -> bool {
        *self == Filteren15_0Din9::Filteren15_0Din9ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din9_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din9::Filteren15_0Din9EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN9` writer - Programmable counter length of digital glitch filter for DIN9"]
pub type Filteren15_0Din9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Filteren15_0Din9, crate::Safe>;
impl<'a, REG> Filteren15_0Din9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din9_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din9::Filteren15_0Din9Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din9_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din9::Filteren15_0Din9OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din9_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din9::Filteren15_0Din9ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din9_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din9::Filteren15_0Din9EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din10 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din10Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din10OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din10ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din10EightCycle = 3,
}
impl From<Filteren15_0Din10> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din10 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din10 {}
#[doc = "Field `FILTEREN15_0_DIN10` reader - Programmable counter length of digital glitch filter for DIN10"]
pub type Filteren15_0Din10R = crate::FieldReader<Filteren15_0Din10>;
impl Filteren15_0Din10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din10 {
        match self.bits {
            0 => Filteren15_0Din10::Filteren15_0Din10Disable,
            1 => Filteren15_0Din10::Filteren15_0Din10OneCycle,
            2 => Filteren15_0Din10::Filteren15_0Din10ThreeCycle,
            3 => Filteren15_0Din10::Filteren15_0Din10EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din10_disable(&self) -> bool {
        *self == Filteren15_0Din10::Filteren15_0Din10Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din10_one_cycle(&self) -> bool {
        *self == Filteren15_0Din10::Filteren15_0Din10OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din10_three_cycle(&self) -> bool {
        *self == Filteren15_0Din10::Filteren15_0Din10ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din10_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din10::Filteren15_0Din10EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN10` writer - Programmable counter length of digital glitch filter for DIN10"]
pub type Filteren15_0Din10W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren15_0Din10, crate::Safe>;
impl<'a, REG> Filteren15_0Din10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din10_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din10::Filteren15_0Din10Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din10_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din10::Filteren15_0Din10OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din10_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din10::Filteren15_0Din10ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din10_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din10::Filteren15_0Din10EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din11 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din11Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din11OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din11ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din11EightCycle = 3,
}
impl From<Filteren15_0Din11> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din11 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din11 {}
#[doc = "Field `FILTEREN15_0_DIN11` reader - Programmable counter length of digital glitch filter for DIN11"]
pub type Filteren15_0Din11R = crate::FieldReader<Filteren15_0Din11>;
impl Filteren15_0Din11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din11 {
        match self.bits {
            0 => Filteren15_0Din11::Filteren15_0Din11Disable,
            1 => Filteren15_0Din11::Filteren15_0Din11OneCycle,
            2 => Filteren15_0Din11::Filteren15_0Din11ThreeCycle,
            3 => Filteren15_0Din11::Filteren15_0Din11EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din11_disable(&self) -> bool {
        *self == Filteren15_0Din11::Filteren15_0Din11Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din11_one_cycle(&self) -> bool {
        *self == Filteren15_0Din11::Filteren15_0Din11OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din11_three_cycle(&self) -> bool {
        *self == Filteren15_0Din11::Filteren15_0Din11ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din11_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din11::Filteren15_0Din11EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN11` writer - Programmable counter length of digital glitch filter for DIN11"]
pub type Filteren15_0Din11W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren15_0Din11, crate::Safe>;
impl<'a, REG> Filteren15_0Din11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din11_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din11::Filteren15_0Din11Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din11_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din11::Filteren15_0Din11OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din11_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din11::Filteren15_0Din11ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din11_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din11::Filteren15_0Din11EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din12 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din12Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din12OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din12ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din12EightCycle = 3,
}
impl From<Filteren15_0Din12> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din12 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din12 {}
#[doc = "Field `FILTEREN15_0_DIN12` reader - Programmable counter length of digital glitch filter for DIN12"]
pub type Filteren15_0Din12R = crate::FieldReader<Filteren15_0Din12>;
impl Filteren15_0Din12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din12 {
        match self.bits {
            0 => Filteren15_0Din12::Filteren15_0Din12Disable,
            1 => Filteren15_0Din12::Filteren15_0Din12OneCycle,
            2 => Filteren15_0Din12::Filteren15_0Din12ThreeCycle,
            3 => Filteren15_0Din12::Filteren15_0Din12EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din12_disable(&self) -> bool {
        *self == Filteren15_0Din12::Filteren15_0Din12Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din12_one_cycle(&self) -> bool {
        *self == Filteren15_0Din12::Filteren15_0Din12OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din12_three_cycle(&self) -> bool {
        *self == Filteren15_0Din12::Filteren15_0Din12ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din12_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din12::Filteren15_0Din12EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN12` writer - Programmable counter length of digital glitch filter for DIN12"]
pub type Filteren15_0Din12W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren15_0Din12, crate::Safe>;
impl<'a, REG> Filteren15_0Din12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din12_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din12::Filteren15_0Din12Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din12_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din12::Filteren15_0Din12OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din12_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din12::Filteren15_0Din12ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din12_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din12::Filteren15_0Din12EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din13 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din13Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din13OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din13ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din13EightCycle = 3,
}
impl From<Filteren15_0Din13> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din13 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din13 {}
#[doc = "Field `FILTEREN15_0_DIN13` reader - Programmable counter length of digital glitch filter for DIN13"]
pub type Filteren15_0Din13R = crate::FieldReader<Filteren15_0Din13>;
impl Filteren15_0Din13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din13 {
        match self.bits {
            0 => Filteren15_0Din13::Filteren15_0Din13Disable,
            1 => Filteren15_0Din13::Filteren15_0Din13OneCycle,
            2 => Filteren15_0Din13::Filteren15_0Din13ThreeCycle,
            3 => Filteren15_0Din13::Filteren15_0Din13EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din13_disable(&self) -> bool {
        *self == Filteren15_0Din13::Filteren15_0Din13Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din13_one_cycle(&self) -> bool {
        *self == Filteren15_0Din13::Filteren15_0Din13OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din13_three_cycle(&self) -> bool {
        *self == Filteren15_0Din13::Filteren15_0Din13ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din13_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din13::Filteren15_0Din13EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN13` writer - Programmable counter length of digital glitch filter for DIN13"]
pub type Filteren15_0Din13W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren15_0Din13, crate::Safe>;
impl<'a, REG> Filteren15_0Din13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din13_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din13::Filteren15_0Din13Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din13_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din13::Filteren15_0Din13OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din13_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din13::Filteren15_0Din13ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din13_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din13::Filteren15_0Din13EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din14 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din14Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din14OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din14ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din14EightCycle = 3,
}
impl From<Filteren15_0Din14> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din14 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din14 {}
#[doc = "Field `FILTEREN15_0_DIN14` reader - Programmable counter length of digital glitch filter for DIN14"]
pub type Filteren15_0Din14R = crate::FieldReader<Filteren15_0Din14>;
impl Filteren15_0Din14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din14 {
        match self.bits {
            0 => Filteren15_0Din14::Filteren15_0Din14Disable,
            1 => Filteren15_0Din14::Filteren15_0Din14OneCycle,
            2 => Filteren15_0Din14::Filteren15_0Din14ThreeCycle,
            3 => Filteren15_0Din14::Filteren15_0Din14EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din14_disable(&self) -> bool {
        *self == Filteren15_0Din14::Filteren15_0Din14Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din14_one_cycle(&self) -> bool {
        *self == Filteren15_0Din14::Filteren15_0Din14OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din14_three_cycle(&self) -> bool {
        *self == Filteren15_0Din14::Filteren15_0Din14ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din14_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din14::Filteren15_0Din14EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN14` writer - Programmable counter length of digital glitch filter for DIN14"]
pub type Filteren15_0Din14W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren15_0Din14, crate::Safe>;
impl<'a, REG> Filteren15_0Din14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din14_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din14::Filteren15_0Din14Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din14_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din14::Filteren15_0Din14OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din14_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din14::Filteren15_0Din14ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din14_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din14::Filteren15_0Din14EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren15_0Din15 {
    #[doc = "0: DISABLE"]
    Filteren15_0Din15Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren15_0Din15OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren15_0Din15ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren15_0Din15EightCycle = 3,
}
impl From<Filteren15_0Din15> for u8 {
    #[inline(always)]
    fn from(variant: Filteren15_0Din15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren15_0Din15 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren15_0Din15 {}
#[doc = "Field `FILTEREN15_0_DIN15` reader - Programmable counter length of digital glitch filter for DIN15"]
pub type Filteren15_0Din15R = crate::FieldReader<Filteren15_0Din15>;
impl Filteren15_0Din15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren15_0Din15 {
        match self.bits {
            0 => Filteren15_0Din15::Filteren15_0Din15Disable,
            1 => Filteren15_0Din15::Filteren15_0Din15OneCycle,
            2 => Filteren15_0Din15::Filteren15_0Din15ThreeCycle,
            3 => Filteren15_0Din15::Filteren15_0Din15EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din15_disable(&self) -> bool {
        *self == Filteren15_0Din15::Filteren15_0Din15Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din15_one_cycle(&self) -> bool {
        *self == Filteren15_0Din15::Filteren15_0Din15OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din15_three_cycle(&self) -> bool {
        *self == Filteren15_0Din15::Filteren15_0Din15ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren15_0_din15_eight_cycle(&self) -> bool {
        *self == Filteren15_0Din15::Filteren15_0Din15EightCycle
    }
}
#[doc = "Field `FILTEREN15_0_DIN15` writer - Programmable counter length of digital glitch filter for DIN15"]
pub type Filteren15_0Din15W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren15_0Din15, crate::Safe>;
impl<'a, REG> Filteren15_0Din15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren15_0_din15_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din15::Filteren15_0Din15Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din15_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din15::Filteren15_0Din15OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din15_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din15::Filteren15_0Din15ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren15_0_din15_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren15_0Din15::Filteren15_0Din15EightCycle)
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN0"]
    #[inline(always)]
    pub fn filteren15_0_din0(&self) -> Filteren15_0Din0R {
        Filteren15_0Din0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN1"]
    #[inline(always)]
    pub fn filteren15_0_din1(&self) -> Filteren15_0Din1R {
        Filteren15_0Din1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN2"]
    #[inline(always)]
    pub fn filteren15_0_din2(&self) -> Filteren15_0Din2R {
        Filteren15_0Din2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN3"]
    #[inline(always)]
    pub fn filteren15_0_din3(&self) -> Filteren15_0Din3R {
        Filteren15_0Din3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN4"]
    #[inline(always)]
    pub fn filteren15_0_din4(&self) -> Filteren15_0Din4R {
        Filteren15_0Din4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN5"]
    #[inline(always)]
    pub fn filteren15_0_din5(&self) -> Filteren15_0Din5R {
        Filteren15_0Din5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN6"]
    #[inline(always)]
    pub fn filteren15_0_din6(&self) -> Filteren15_0Din6R {
        Filteren15_0Din6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN7"]
    #[inline(always)]
    pub fn filteren15_0_din7(&self) -> Filteren15_0Din7R {
        Filteren15_0Din7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN8"]
    #[inline(always)]
    pub fn filteren15_0_din8(&self) -> Filteren15_0Din8R {
        Filteren15_0Din8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN9"]
    #[inline(always)]
    pub fn filteren15_0_din9(&self) -> Filteren15_0Din9R {
        Filteren15_0Din9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN10"]
    #[inline(always)]
    pub fn filteren15_0_din10(&self) -> Filteren15_0Din10R {
        Filteren15_0Din10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN11"]
    #[inline(always)]
    pub fn filteren15_0_din11(&self) -> Filteren15_0Din11R {
        Filteren15_0Din11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN12"]
    #[inline(always)]
    pub fn filteren15_0_din12(&self) -> Filteren15_0Din12R {
        Filteren15_0Din12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN13"]
    #[inline(always)]
    pub fn filteren15_0_din13(&self) -> Filteren15_0Din13R {
        Filteren15_0Din13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN14"]
    #[inline(always)]
    pub fn filteren15_0_din14(&self) -> Filteren15_0Din14R {
        Filteren15_0Din14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN15"]
    #[inline(always)]
    pub fn filteren15_0_din15(&self) -> Filteren15_0Din15R {
        Filteren15_0Din15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN0"]
    #[inline(always)]
    pub fn filteren15_0_din0(&mut self) -> Filteren15_0Din0W<Filteren15_0Spec> {
        Filteren15_0Din0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN1"]
    #[inline(always)]
    pub fn filteren15_0_din1(&mut self) -> Filteren15_0Din1W<Filteren15_0Spec> {
        Filteren15_0Din1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN2"]
    #[inline(always)]
    pub fn filteren15_0_din2(&mut self) -> Filteren15_0Din2W<Filteren15_0Spec> {
        Filteren15_0Din2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN3"]
    #[inline(always)]
    pub fn filteren15_0_din3(&mut self) -> Filteren15_0Din3W<Filteren15_0Spec> {
        Filteren15_0Din3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN4"]
    #[inline(always)]
    pub fn filteren15_0_din4(&mut self) -> Filteren15_0Din4W<Filteren15_0Spec> {
        Filteren15_0Din4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN5"]
    #[inline(always)]
    pub fn filteren15_0_din5(&mut self) -> Filteren15_0Din5W<Filteren15_0Spec> {
        Filteren15_0Din5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN6"]
    #[inline(always)]
    pub fn filteren15_0_din6(&mut self) -> Filteren15_0Din6W<Filteren15_0Spec> {
        Filteren15_0Din6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN7"]
    #[inline(always)]
    pub fn filteren15_0_din7(&mut self) -> Filteren15_0Din7W<Filteren15_0Spec> {
        Filteren15_0Din7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN8"]
    #[inline(always)]
    pub fn filteren15_0_din8(&mut self) -> Filteren15_0Din8W<Filteren15_0Spec> {
        Filteren15_0Din8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN9"]
    #[inline(always)]
    pub fn filteren15_0_din9(&mut self) -> Filteren15_0Din9W<Filteren15_0Spec> {
        Filteren15_0Din9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN10"]
    #[inline(always)]
    pub fn filteren15_0_din10(&mut self) -> Filteren15_0Din10W<Filteren15_0Spec> {
        Filteren15_0Din10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN11"]
    #[inline(always)]
    pub fn filteren15_0_din11(&mut self) -> Filteren15_0Din11W<Filteren15_0Spec> {
        Filteren15_0Din11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN12"]
    #[inline(always)]
    pub fn filteren15_0_din12(&mut self) -> Filteren15_0Din12W<Filteren15_0Spec> {
        Filteren15_0Din12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN13"]
    #[inline(always)]
    pub fn filteren15_0_din13(&mut self) -> Filteren15_0Din13W<Filteren15_0Spec> {
        Filteren15_0Din13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN14"]
    #[inline(always)]
    pub fn filteren15_0_din14(&mut self) -> Filteren15_0Din14W<Filteren15_0Spec> {
        Filteren15_0Din14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN15"]
    #[inline(always)]
    pub fn filteren15_0_din15(&mut self) -> Filteren15_0Din15W<Filteren15_0Spec> {
        Filteren15_0Din15W::new(self, 30)
    }
}
#[doc = "Filter Enable 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`filteren15_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filteren15_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filteren15_0Spec;
impl crate::RegisterSpec for Filteren15_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filteren15_0::R`](R) reader structure"]
impl crate::Readable for Filteren15_0Spec {}
#[doc = "`write(|w| ..)` method takes [`filteren15_0::W`](W) writer structure"]
impl crate::Writable for Filteren15_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTEREN15_0 to value 0"]
impl crate::Resettable for Filteren15_0Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `FILTEREN31_16` reader"]
pub type R = crate::R<Filteren31_16Spec>;
#[doc = "Register `FILTEREN31_16` writer"]
pub type W = crate::W<Filteren31_16Spec>;
#[doc = "Programmable counter length of digital glitch filter for DIN16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din16 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din16Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din16OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din16ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din16EightCycle = 3,
}
impl From<Filteren31_16Din16> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din16 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din16 {}
#[doc = "Field `FILTEREN31_16_DIN16` reader - Programmable counter length of digital glitch filter for DIN16"]
pub type Filteren31_16Din16R = crate::FieldReader<Filteren31_16Din16>;
impl Filteren31_16Din16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din16 {
        match self.bits {
            0 => Filteren31_16Din16::Filteren31_16Din16Disable,
            1 => Filteren31_16Din16::Filteren31_16Din16OneCycle,
            2 => Filteren31_16Din16::Filteren31_16Din16ThreeCycle,
            3 => Filteren31_16Din16::Filteren31_16Din16EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din16_disable(&self) -> bool {
        *self == Filteren31_16Din16::Filteren31_16Din16Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din16_one_cycle(&self) -> bool {
        *self == Filteren31_16Din16::Filteren31_16Din16OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din16_three_cycle(&self) -> bool {
        *self == Filteren31_16Din16::Filteren31_16Din16ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din16_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din16::Filteren31_16Din16EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN16` writer - Programmable counter length of digital glitch filter for DIN16"]
pub type Filteren31_16Din16W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din16, crate::Safe>;
impl<'a, REG> Filteren31_16Din16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din16_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din16::Filteren31_16Din16Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din16_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din16::Filteren31_16Din16OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din16_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din16::Filteren31_16Din16ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din16_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din16::Filteren31_16Din16EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din17 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din17Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din17OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din17ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din17EightCycle = 3,
}
impl From<Filteren31_16Din17> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din17 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din17 {}
#[doc = "Field `FILTEREN31_16_DIN17` reader - Programmable counter length of digital glitch filter for DIN17"]
pub type Filteren31_16Din17R = crate::FieldReader<Filteren31_16Din17>;
impl Filteren31_16Din17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din17 {
        match self.bits {
            0 => Filteren31_16Din17::Filteren31_16Din17Disable,
            1 => Filteren31_16Din17::Filteren31_16Din17OneCycle,
            2 => Filteren31_16Din17::Filteren31_16Din17ThreeCycle,
            3 => Filteren31_16Din17::Filteren31_16Din17EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din17_disable(&self) -> bool {
        *self == Filteren31_16Din17::Filteren31_16Din17Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din17_one_cycle(&self) -> bool {
        *self == Filteren31_16Din17::Filteren31_16Din17OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din17_three_cycle(&self) -> bool {
        *self == Filteren31_16Din17::Filteren31_16Din17ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din17_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din17::Filteren31_16Din17EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN17` writer - Programmable counter length of digital glitch filter for DIN17"]
pub type Filteren31_16Din17W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din17, crate::Safe>;
impl<'a, REG> Filteren31_16Din17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din17_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din17::Filteren31_16Din17Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din17_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din17::Filteren31_16Din17OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din17_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din17::Filteren31_16Din17ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din17_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din17::Filteren31_16Din17EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din18 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din18Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din18OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din18ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din18EightCycle = 3,
}
impl From<Filteren31_16Din18> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din18 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din18 {}
#[doc = "Field `FILTEREN31_16_DIN18` reader - Programmable counter length of digital glitch filter for DIN18"]
pub type Filteren31_16Din18R = crate::FieldReader<Filteren31_16Din18>;
impl Filteren31_16Din18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din18 {
        match self.bits {
            0 => Filteren31_16Din18::Filteren31_16Din18Disable,
            1 => Filteren31_16Din18::Filteren31_16Din18OneCycle,
            2 => Filteren31_16Din18::Filteren31_16Din18ThreeCycle,
            3 => Filteren31_16Din18::Filteren31_16Din18EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din18_disable(&self) -> bool {
        *self == Filteren31_16Din18::Filteren31_16Din18Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din18_one_cycle(&self) -> bool {
        *self == Filteren31_16Din18::Filteren31_16Din18OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din18_three_cycle(&self) -> bool {
        *self == Filteren31_16Din18::Filteren31_16Din18ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din18_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din18::Filteren31_16Din18EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN18` writer - Programmable counter length of digital glitch filter for DIN18"]
pub type Filteren31_16Din18W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din18, crate::Safe>;
impl<'a, REG> Filteren31_16Din18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din18_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din18::Filteren31_16Din18Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din18_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din18::Filteren31_16Din18OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din18_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din18::Filteren31_16Din18ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din18_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din18::Filteren31_16Din18EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din19 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din19Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din19OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din19ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din19EightCycle = 3,
}
impl From<Filteren31_16Din19> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din19 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din19 {}
#[doc = "Field `FILTEREN31_16_DIN19` reader - Programmable counter length of digital glitch filter for DIN19"]
pub type Filteren31_16Din19R = crate::FieldReader<Filteren31_16Din19>;
impl Filteren31_16Din19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din19 {
        match self.bits {
            0 => Filteren31_16Din19::Filteren31_16Din19Disable,
            1 => Filteren31_16Din19::Filteren31_16Din19OneCycle,
            2 => Filteren31_16Din19::Filteren31_16Din19ThreeCycle,
            3 => Filteren31_16Din19::Filteren31_16Din19EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din19_disable(&self) -> bool {
        *self == Filteren31_16Din19::Filteren31_16Din19Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din19_one_cycle(&self) -> bool {
        *self == Filteren31_16Din19::Filteren31_16Din19OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din19_three_cycle(&self) -> bool {
        *self == Filteren31_16Din19::Filteren31_16Din19ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din19_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din19::Filteren31_16Din19EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN19` writer - Programmable counter length of digital glitch filter for DIN19"]
pub type Filteren31_16Din19W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din19, crate::Safe>;
impl<'a, REG> Filteren31_16Din19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din19_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din19::Filteren31_16Din19Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din19_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din19::Filteren31_16Din19OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din19_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din19::Filteren31_16Din19ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din19_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din19::Filteren31_16Din19EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din20 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din20Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din20OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din20ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din20EightCycle = 3,
}
impl From<Filteren31_16Din20> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din20 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din20 {}
#[doc = "Field `FILTEREN31_16_DIN20` reader - Programmable counter length of digital glitch filter for DIN20"]
pub type Filteren31_16Din20R = crate::FieldReader<Filteren31_16Din20>;
impl Filteren31_16Din20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din20 {
        match self.bits {
            0 => Filteren31_16Din20::Filteren31_16Din20Disable,
            1 => Filteren31_16Din20::Filteren31_16Din20OneCycle,
            2 => Filteren31_16Din20::Filteren31_16Din20ThreeCycle,
            3 => Filteren31_16Din20::Filteren31_16Din20EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din20_disable(&self) -> bool {
        *self == Filteren31_16Din20::Filteren31_16Din20Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din20_one_cycle(&self) -> bool {
        *self == Filteren31_16Din20::Filteren31_16Din20OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din20_three_cycle(&self) -> bool {
        *self == Filteren31_16Din20::Filteren31_16Din20ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din20_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din20::Filteren31_16Din20EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN20` writer - Programmable counter length of digital glitch filter for DIN20"]
pub type Filteren31_16Din20W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din20, crate::Safe>;
impl<'a, REG> Filteren31_16Din20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din20_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din20::Filteren31_16Din20Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din20_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din20::Filteren31_16Din20OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din20_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din20::Filteren31_16Din20ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din20_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din20::Filteren31_16Din20EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din21 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din21Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din21OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din21ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din21EightCycle = 3,
}
impl From<Filteren31_16Din21> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din21) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din21 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din21 {}
#[doc = "Field `FILTEREN31_16_DIN21` reader - Programmable counter length of digital glitch filter for DIN21"]
pub type Filteren31_16Din21R = crate::FieldReader<Filteren31_16Din21>;
impl Filteren31_16Din21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din21 {
        match self.bits {
            0 => Filteren31_16Din21::Filteren31_16Din21Disable,
            1 => Filteren31_16Din21::Filteren31_16Din21OneCycle,
            2 => Filteren31_16Din21::Filteren31_16Din21ThreeCycle,
            3 => Filteren31_16Din21::Filteren31_16Din21EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din21_disable(&self) -> bool {
        *self == Filteren31_16Din21::Filteren31_16Din21Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din21_one_cycle(&self) -> bool {
        *self == Filteren31_16Din21::Filteren31_16Din21OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din21_three_cycle(&self) -> bool {
        *self == Filteren31_16Din21::Filteren31_16Din21ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din21_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din21::Filteren31_16Din21EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN21` writer - Programmable counter length of digital glitch filter for DIN21"]
pub type Filteren31_16Din21W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din21, crate::Safe>;
impl<'a, REG> Filteren31_16Din21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din21_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din21::Filteren31_16Din21Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din21_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din21::Filteren31_16Din21OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din21_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din21::Filteren31_16Din21ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din21_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din21::Filteren31_16Din21EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din22 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din22Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din22OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din22ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din22EightCycle = 3,
}
impl From<Filteren31_16Din22> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din22 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din22 {}
#[doc = "Field `FILTEREN31_16_DIN22` reader - Programmable counter length of digital glitch filter for DIN22"]
pub type Filteren31_16Din22R = crate::FieldReader<Filteren31_16Din22>;
impl Filteren31_16Din22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din22 {
        match self.bits {
            0 => Filteren31_16Din22::Filteren31_16Din22Disable,
            1 => Filteren31_16Din22::Filteren31_16Din22OneCycle,
            2 => Filteren31_16Din22::Filteren31_16Din22ThreeCycle,
            3 => Filteren31_16Din22::Filteren31_16Din22EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din22_disable(&self) -> bool {
        *self == Filteren31_16Din22::Filteren31_16Din22Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din22_one_cycle(&self) -> bool {
        *self == Filteren31_16Din22::Filteren31_16Din22OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din22_three_cycle(&self) -> bool {
        *self == Filteren31_16Din22::Filteren31_16Din22ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din22_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din22::Filteren31_16Din22EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN22` writer - Programmable counter length of digital glitch filter for DIN22"]
pub type Filteren31_16Din22W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din22, crate::Safe>;
impl<'a, REG> Filteren31_16Din22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din22_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din22::Filteren31_16Din22Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din22_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din22::Filteren31_16Din22OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din22_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din22::Filteren31_16Din22ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din22_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din22::Filteren31_16Din22EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din23 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din23Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din23OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din23ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din23EightCycle = 3,
}
impl From<Filteren31_16Din23> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din23 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din23 {}
#[doc = "Field `FILTEREN31_16_DIN23` reader - Programmable counter length of digital glitch filter for DIN23"]
pub type Filteren31_16Din23R = crate::FieldReader<Filteren31_16Din23>;
impl Filteren31_16Din23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din23 {
        match self.bits {
            0 => Filteren31_16Din23::Filteren31_16Din23Disable,
            1 => Filteren31_16Din23::Filteren31_16Din23OneCycle,
            2 => Filteren31_16Din23::Filteren31_16Din23ThreeCycle,
            3 => Filteren31_16Din23::Filteren31_16Din23EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din23_disable(&self) -> bool {
        *self == Filteren31_16Din23::Filteren31_16Din23Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din23_one_cycle(&self) -> bool {
        *self == Filteren31_16Din23::Filteren31_16Din23OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din23_three_cycle(&self) -> bool {
        *self == Filteren31_16Din23::Filteren31_16Din23ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din23_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din23::Filteren31_16Din23EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN23` writer - Programmable counter length of digital glitch filter for DIN23"]
pub type Filteren31_16Din23W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din23, crate::Safe>;
impl<'a, REG> Filteren31_16Din23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din23_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din23::Filteren31_16Din23Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din23_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din23::Filteren31_16Din23OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din23_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din23::Filteren31_16Din23ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din23_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din23::Filteren31_16Din23EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din24 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din24Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din24OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din24ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din24EightCycle = 3,
}
impl From<Filteren31_16Din24> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din24) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din24 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din24 {}
#[doc = "Field `FILTEREN31_16_DIN24` reader - Programmable counter length of digital glitch filter for DIN24"]
pub type Filteren31_16Din24R = crate::FieldReader<Filteren31_16Din24>;
impl Filteren31_16Din24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din24 {
        match self.bits {
            0 => Filteren31_16Din24::Filteren31_16Din24Disable,
            1 => Filteren31_16Din24::Filteren31_16Din24OneCycle,
            2 => Filteren31_16Din24::Filteren31_16Din24ThreeCycle,
            3 => Filteren31_16Din24::Filteren31_16Din24EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din24_disable(&self) -> bool {
        *self == Filteren31_16Din24::Filteren31_16Din24Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din24_one_cycle(&self) -> bool {
        *self == Filteren31_16Din24::Filteren31_16Din24OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din24_three_cycle(&self) -> bool {
        *self == Filteren31_16Din24::Filteren31_16Din24ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din24_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din24::Filteren31_16Din24EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN24` writer - Programmable counter length of digital glitch filter for DIN24"]
pub type Filteren31_16Din24W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din24, crate::Safe>;
impl<'a, REG> Filteren31_16Din24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din24_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din24::Filteren31_16Din24Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din24_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din24::Filteren31_16Din24OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din24_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din24::Filteren31_16Din24ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din24_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din24::Filteren31_16Din24EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din25 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din25Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din25OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din25ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din25EightCycle = 3,
}
impl From<Filteren31_16Din25> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din25 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din25 {}
#[doc = "Field `FILTEREN31_16_DIN25` reader - Programmable counter length of digital glitch filter for DIN25"]
pub type Filteren31_16Din25R = crate::FieldReader<Filteren31_16Din25>;
impl Filteren31_16Din25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din25 {
        match self.bits {
            0 => Filteren31_16Din25::Filteren31_16Din25Disable,
            1 => Filteren31_16Din25::Filteren31_16Din25OneCycle,
            2 => Filteren31_16Din25::Filteren31_16Din25ThreeCycle,
            3 => Filteren31_16Din25::Filteren31_16Din25EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din25_disable(&self) -> bool {
        *self == Filteren31_16Din25::Filteren31_16Din25Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din25_one_cycle(&self) -> bool {
        *self == Filteren31_16Din25::Filteren31_16Din25OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din25_three_cycle(&self) -> bool {
        *self == Filteren31_16Din25::Filteren31_16Din25ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din25_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din25::Filteren31_16Din25EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN25` writer - Programmable counter length of digital glitch filter for DIN25"]
pub type Filteren31_16Din25W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din25, crate::Safe>;
impl<'a, REG> Filteren31_16Din25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din25_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din25::Filteren31_16Din25Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din25_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din25::Filteren31_16Din25OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din25_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din25::Filteren31_16Din25ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din25_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din25::Filteren31_16Din25EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din26 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din26Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din26OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din26ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din26EightCycle = 3,
}
impl From<Filteren31_16Din26> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din26 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din26 {}
#[doc = "Field `FILTEREN31_16_DIN26` reader - Programmable counter length of digital glitch filter for DIN26"]
pub type Filteren31_16Din26R = crate::FieldReader<Filteren31_16Din26>;
impl Filteren31_16Din26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din26 {
        match self.bits {
            0 => Filteren31_16Din26::Filteren31_16Din26Disable,
            1 => Filteren31_16Din26::Filteren31_16Din26OneCycle,
            2 => Filteren31_16Din26::Filteren31_16Din26ThreeCycle,
            3 => Filteren31_16Din26::Filteren31_16Din26EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din26_disable(&self) -> bool {
        *self == Filteren31_16Din26::Filteren31_16Din26Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din26_one_cycle(&self) -> bool {
        *self == Filteren31_16Din26::Filteren31_16Din26OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din26_three_cycle(&self) -> bool {
        *self == Filteren31_16Din26::Filteren31_16Din26ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din26_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din26::Filteren31_16Din26EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN26` writer - Programmable counter length of digital glitch filter for DIN26"]
pub type Filteren31_16Din26W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din26, crate::Safe>;
impl<'a, REG> Filteren31_16Din26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din26_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din26::Filteren31_16Din26Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din26_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din26::Filteren31_16Din26OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din26_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din26::Filteren31_16Din26ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din26_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din26::Filteren31_16Din26EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din27 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din27Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din27OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din27ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din27EightCycle = 3,
}
impl From<Filteren31_16Din27> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din27) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din27 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din27 {}
#[doc = "Field `FILTEREN31_16_DIN27` reader - Programmable counter length of digital glitch filter for DIN27"]
pub type Filteren31_16Din27R = crate::FieldReader<Filteren31_16Din27>;
impl Filteren31_16Din27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din27 {
        match self.bits {
            0 => Filteren31_16Din27::Filteren31_16Din27Disable,
            1 => Filteren31_16Din27::Filteren31_16Din27OneCycle,
            2 => Filteren31_16Din27::Filteren31_16Din27ThreeCycle,
            3 => Filteren31_16Din27::Filteren31_16Din27EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din27_disable(&self) -> bool {
        *self == Filteren31_16Din27::Filteren31_16Din27Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din27_one_cycle(&self) -> bool {
        *self == Filteren31_16Din27::Filteren31_16Din27OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din27_three_cycle(&self) -> bool {
        *self == Filteren31_16Din27::Filteren31_16Din27ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din27_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din27::Filteren31_16Din27EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN27` writer - Programmable counter length of digital glitch filter for DIN27"]
pub type Filteren31_16Din27W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din27, crate::Safe>;
impl<'a, REG> Filteren31_16Din27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din27_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din27::Filteren31_16Din27Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din27_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din27::Filteren31_16Din27OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din27_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din27::Filteren31_16Din27ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din27_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din27::Filteren31_16Din27EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din28 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din28Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din28OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din28ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din28EightCycle = 3,
}
impl From<Filteren31_16Din28> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din28) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din28 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din28 {}
#[doc = "Field `FILTEREN31_16_DIN28` reader - Programmable counter length of digital glitch filter for DIN28"]
pub type Filteren31_16Din28R = crate::FieldReader<Filteren31_16Din28>;
impl Filteren31_16Din28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din28 {
        match self.bits {
            0 => Filteren31_16Din28::Filteren31_16Din28Disable,
            1 => Filteren31_16Din28::Filteren31_16Din28OneCycle,
            2 => Filteren31_16Din28::Filteren31_16Din28ThreeCycle,
            3 => Filteren31_16Din28::Filteren31_16Din28EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din28_disable(&self) -> bool {
        *self == Filteren31_16Din28::Filteren31_16Din28Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din28_one_cycle(&self) -> bool {
        *self == Filteren31_16Din28::Filteren31_16Din28OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din28_three_cycle(&self) -> bool {
        *self == Filteren31_16Din28::Filteren31_16Din28ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din28_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din28::Filteren31_16Din28EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN28` writer - Programmable counter length of digital glitch filter for DIN28"]
pub type Filteren31_16Din28W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din28, crate::Safe>;
impl<'a, REG> Filteren31_16Din28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din28_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din28::Filteren31_16Din28Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din28_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din28::Filteren31_16Din28OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din28_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din28::Filteren31_16Din28ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din28_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din28::Filteren31_16Din28EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din29 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din29Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din29OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din29ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din29EightCycle = 3,
}
impl From<Filteren31_16Din29> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din29 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din29 {}
#[doc = "Field `FILTEREN31_16_DIN29` reader - Programmable counter length of digital glitch filter for DIN29"]
pub type Filteren31_16Din29R = crate::FieldReader<Filteren31_16Din29>;
impl Filteren31_16Din29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din29 {
        match self.bits {
            0 => Filteren31_16Din29::Filteren31_16Din29Disable,
            1 => Filteren31_16Din29::Filteren31_16Din29OneCycle,
            2 => Filteren31_16Din29::Filteren31_16Din29ThreeCycle,
            3 => Filteren31_16Din29::Filteren31_16Din29EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din29_disable(&self) -> bool {
        *self == Filteren31_16Din29::Filteren31_16Din29Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din29_one_cycle(&self) -> bool {
        *self == Filteren31_16Din29::Filteren31_16Din29OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din29_three_cycle(&self) -> bool {
        *self == Filteren31_16Din29::Filteren31_16Din29ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din29_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din29::Filteren31_16Din29EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN29` writer - Programmable counter length of digital glitch filter for DIN29"]
pub type Filteren31_16Din29W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din29, crate::Safe>;
impl<'a, REG> Filteren31_16Din29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din29_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din29::Filteren31_16Din29Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din29_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din29::Filteren31_16Din29OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din29_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din29::Filteren31_16Din29ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din29_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din29::Filteren31_16Din29EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din30 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din30Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din30OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din30ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din30EightCycle = 3,
}
impl From<Filteren31_16Din30> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din30 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din30 {}
#[doc = "Field `FILTEREN31_16_DIN30` reader - Programmable counter length of digital glitch filter for DIN30"]
pub type Filteren31_16Din30R = crate::FieldReader<Filteren31_16Din30>;
impl Filteren31_16Din30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din30 {
        match self.bits {
            0 => Filteren31_16Din30::Filteren31_16Din30Disable,
            1 => Filteren31_16Din30::Filteren31_16Din30OneCycle,
            2 => Filteren31_16Din30::Filteren31_16Din30ThreeCycle,
            3 => Filteren31_16Din30::Filteren31_16Din30EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din30_disable(&self) -> bool {
        *self == Filteren31_16Din30::Filteren31_16Din30Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din30_one_cycle(&self) -> bool {
        *self == Filteren31_16Din30::Filteren31_16Din30OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din30_three_cycle(&self) -> bool {
        *self == Filteren31_16Din30::Filteren31_16Din30ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din30_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din30::Filteren31_16Din30EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN30` writer - Programmable counter length of digital glitch filter for DIN30"]
pub type Filteren31_16Din30W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din30, crate::Safe>;
impl<'a, REG> Filteren31_16Din30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din30_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din30::Filteren31_16Din30Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din30_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din30::Filteren31_16Din30OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din30_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din30::Filteren31_16Din30ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din30_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din30::Filteren31_16Din30EightCycle)
    }
}
#[doc = "Programmable counter length of digital glitch filter for DIN31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filteren31_16Din31 {
    #[doc = "0: DISABLE"]
    Filteren31_16Din31Disable = 0,
    #[doc = "1: ONE_CYCLE"]
    Filteren31_16Din31OneCycle = 1,
    #[doc = "2: THREE_CYCLE"]
    Filteren31_16Din31ThreeCycle = 2,
    #[doc = "3: EIGHT_CYCLE"]
    Filteren31_16Din31EightCycle = 3,
}
impl From<Filteren31_16Din31> for u8 {
    #[inline(always)]
    fn from(variant: Filteren31_16Din31) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filteren31_16Din31 {
    type Ux = u8;
}
impl crate::IsEnum for Filteren31_16Din31 {}
#[doc = "Field `FILTEREN31_16_DIN31` reader - Programmable counter length of digital glitch filter for DIN31"]
pub type Filteren31_16Din31R = crate::FieldReader<Filteren31_16Din31>;
impl Filteren31_16Din31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filteren31_16Din31 {
        match self.bits {
            0 => Filteren31_16Din31::Filteren31_16Din31Disable,
            1 => Filteren31_16Din31::Filteren31_16Din31OneCycle,
            2 => Filteren31_16Din31::Filteren31_16Din31ThreeCycle,
            3 => Filteren31_16Din31::Filteren31_16Din31EightCycle,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din31_disable(&self) -> bool {
        *self == Filteren31_16Din31::Filteren31_16Din31Disable
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din31_one_cycle(&self) -> bool {
        *self == Filteren31_16Din31::Filteren31_16Din31OneCycle
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din31_three_cycle(&self) -> bool {
        *self == Filteren31_16Din31::Filteren31_16Din31ThreeCycle
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn is_filteren31_16_din31_eight_cycle(&self) -> bool {
        *self == Filteren31_16Din31::Filteren31_16Din31EightCycle
    }
}
#[doc = "Field `FILTEREN31_16_DIN31` writer - Programmable counter length of digital glitch filter for DIN31"]
pub type Filteren31_16Din31W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Filteren31_16Din31, crate::Safe>;
impl<'a, REG> Filteren31_16Din31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn filteren31_16_din31_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din31::Filteren31_16Din31Disable)
    }
    #[doc = "ONE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din31_one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din31::Filteren31_16Din31OneCycle)
    }
    #[doc = "THREE_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din31_three_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din31::Filteren31_16Din31ThreeCycle)
    }
    #[doc = "EIGHT_CYCLE"]
    #[inline(always)]
    pub fn filteren31_16_din31_eight_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Filteren31_16Din31::Filteren31_16Din31EightCycle)
    }
}
impl R {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN16"]
    #[inline(always)]
    pub fn filteren31_16_din16(&self) -> Filteren31_16Din16R {
        Filteren31_16Din16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN17"]
    #[inline(always)]
    pub fn filteren31_16_din17(&self) -> Filteren31_16Din17R {
        Filteren31_16Din17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN18"]
    #[inline(always)]
    pub fn filteren31_16_din18(&self) -> Filteren31_16Din18R {
        Filteren31_16Din18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN19"]
    #[inline(always)]
    pub fn filteren31_16_din19(&self) -> Filteren31_16Din19R {
        Filteren31_16Din19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN20"]
    #[inline(always)]
    pub fn filteren31_16_din20(&self) -> Filteren31_16Din20R {
        Filteren31_16Din20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN21"]
    #[inline(always)]
    pub fn filteren31_16_din21(&self) -> Filteren31_16Din21R {
        Filteren31_16Din21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN22"]
    #[inline(always)]
    pub fn filteren31_16_din22(&self) -> Filteren31_16Din22R {
        Filteren31_16Din22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN23"]
    #[inline(always)]
    pub fn filteren31_16_din23(&self) -> Filteren31_16Din23R {
        Filteren31_16Din23R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN24"]
    #[inline(always)]
    pub fn filteren31_16_din24(&self) -> Filteren31_16Din24R {
        Filteren31_16Din24R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN25"]
    #[inline(always)]
    pub fn filteren31_16_din25(&self) -> Filteren31_16Din25R {
        Filteren31_16Din25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN26"]
    #[inline(always)]
    pub fn filteren31_16_din26(&self) -> Filteren31_16Din26R {
        Filteren31_16Din26R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN27"]
    #[inline(always)]
    pub fn filteren31_16_din27(&self) -> Filteren31_16Din27R {
        Filteren31_16Din27R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN28"]
    #[inline(always)]
    pub fn filteren31_16_din28(&self) -> Filteren31_16Din28R {
        Filteren31_16Din28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN29"]
    #[inline(always)]
    pub fn filteren31_16_din29(&self) -> Filteren31_16Din29R {
        Filteren31_16Din29R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN30"]
    #[inline(always)]
    pub fn filteren31_16_din30(&self) -> Filteren31_16Din30R {
        Filteren31_16Din30R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN31"]
    #[inline(always)]
    pub fn filteren31_16_din31(&self) -> Filteren31_16Din31R {
        Filteren31_16Din31R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable counter length of digital glitch filter for DIN16"]
    #[inline(always)]
    pub fn filteren31_16_din16(&mut self) -> Filteren31_16Din16W<Filteren31_16Spec> {
        Filteren31_16Din16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Programmable counter length of digital glitch filter for DIN17"]
    #[inline(always)]
    pub fn filteren31_16_din17(&mut self) -> Filteren31_16Din17W<Filteren31_16Spec> {
        Filteren31_16Din17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Programmable counter length of digital glitch filter for DIN18"]
    #[inline(always)]
    pub fn filteren31_16_din18(&mut self) -> Filteren31_16Din18W<Filteren31_16Spec> {
        Filteren31_16Din18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Programmable counter length of digital glitch filter for DIN19"]
    #[inline(always)]
    pub fn filteren31_16_din19(&mut self) -> Filteren31_16Din19W<Filteren31_16Spec> {
        Filteren31_16Din19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Programmable counter length of digital glitch filter for DIN20"]
    #[inline(always)]
    pub fn filteren31_16_din20(&mut self) -> Filteren31_16Din20W<Filteren31_16Spec> {
        Filteren31_16Din20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Programmable counter length of digital glitch filter for DIN21"]
    #[inline(always)]
    pub fn filteren31_16_din21(&mut self) -> Filteren31_16Din21W<Filteren31_16Spec> {
        Filteren31_16Din21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Programmable counter length of digital glitch filter for DIN22"]
    #[inline(always)]
    pub fn filteren31_16_din22(&mut self) -> Filteren31_16Din22W<Filteren31_16Spec> {
        Filteren31_16Din22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Programmable counter length of digital glitch filter for DIN23"]
    #[inline(always)]
    pub fn filteren31_16_din23(&mut self) -> Filteren31_16Din23W<Filteren31_16Spec> {
        Filteren31_16Din23W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Programmable counter length of digital glitch filter for DIN24"]
    #[inline(always)]
    pub fn filteren31_16_din24(&mut self) -> Filteren31_16Din24W<Filteren31_16Spec> {
        Filteren31_16Din24W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Programmable counter length of digital glitch filter for DIN25"]
    #[inline(always)]
    pub fn filteren31_16_din25(&mut self) -> Filteren31_16Din25W<Filteren31_16Spec> {
        Filteren31_16Din25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Programmable counter length of digital glitch filter for DIN26"]
    #[inline(always)]
    pub fn filteren31_16_din26(&mut self) -> Filteren31_16Din26W<Filteren31_16Spec> {
        Filteren31_16Din26W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Programmable counter length of digital glitch filter for DIN27"]
    #[inline(always)]
    pub fn filteren31_16_din27(&mut self) -> Filteren31_16Din27W<Filteren31_16Spec> {
        Filteren31_16Din27W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Programmable counter length of digital glitch filter for DIN28"]
    #[inline(always)]
    pub fn filteren31_16_din28(&mut self) -> Filteren31_16Din28W<Filteren31_16Spec> {
        Filteren31_16Din28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Programmable counter length of digital glitch filter for DIN29"]
    #[inline(always)]
    pub fn filteren31_16_din29(&mut self) -> Filteren31_16Din29W<Filteren31_16Spec> {
        Filteren31_16Din29W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Programmable counter length of digital glitch filter for DIN30"]
    #[inline(always)]
    pub fn filteren31_16_din30(&mut self) -> Filteren31_16Din30W<Filteren31_16Spec> {
        Filteren31_16Din30W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Programmable counter length of digital glitch filter for DIN31"]
    #[inline(always)]
    pub fn filteren31_16_din31(&mut self) -> Filteren31_16Din31W<Filteren31_16Spec> {
        Filteren31_16Din31W::new(self, 30)
    }
}
#[doc = "Filter Enable 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`filteren31_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filteren31_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filteren31_16Spec;
impl crate::RegisterSpec for Filteren31_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filteren31_16::R`](R) reader structure"]
impl crate::Readable for Filteren31_16Spec {}
#[doc = "`write(|w| ..)` method takes [`filteren31_16::W`](W) writer structure"]
impl crate::Writable for Filteren31_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTEREN31_16 to value 0"]
impl crate::Resettable for Filteren31_16Spec {
    const RESET_VALUE: u32 = 0;
}

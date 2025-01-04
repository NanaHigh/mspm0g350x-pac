#[doc = "Register `FASTWAKE` reader"]
pub type R = crate::R<FastwakeSpec>;
#[doc = "Register `FASTWAKE` writer"]
pub type W = crate::W<FastwakeSpec>;
#[doc = "Enable fastwake feature for DIN0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin0 {
    #[doc = "0: DISABLE"]
    FastwakeDin0Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin0Enable = 1,
}
impl From<FastwakeDin0> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN0` reader - Enable fastwake feature for DIN0"]
pub type FastwakeDin0R = crate::BitReader<FastwakeDin0>;
impl FastwakeDin0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin0 {
        match self.bits {
            false => FastwakeDin0::FastwakeDin0Disable,
            true => FastwakeDin0::FastwakeDin0Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din0_disable(&self) -> bool {
        *self == FastwakeDin0::FastwakeDin0Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din0_enable(&self) -> bool {
        *self == FastwakeDin0::FastwakeDin0Enable
    }
}
#[doc = "Field `FASTWAKE_DIN0` writer - Enable fastwake feature for DIN0"]
pub type FastwakeDin0W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin0>;
impl<'a, REG> FastwakeDin0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din0_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin0::FastwakeDin0Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din0_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin0::FastwakeDin0Enable)
    }
}
#[doc = "Enable fastwake feature for DIN1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin1 {
    #[doc = "0: DISABLE"]
    FastwakeDin1Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin1Enable = 1,
}
impl From<FastwakeDin1> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN1` reader - Enable fastwake feature for DIN1"]
pub type FastwakeDin1R = crate::BitReader<FastwakeDin1>;
impl FastwakeDin1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin1 {
        match self.bits {
            false => FastwakeDin1::FastwakeDin1Disable,
            true => FastwakeDin1::FastwakeDin1Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din1_disable(&self) -> bool {
        *self == FastwakeDin1::FastwakeDin1Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din1_enable(&self) -> bool {
        *self == FastwakeDin1::FastwakeDin1Enable
    }
}
#[doc = "Field `FASTWAKE_DIN1` writer - Enable fastwake feature for DIN1"]
pub type FastwakeDin1W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin1>;
impl<'a, REG> FastwakeDin1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin1::FastwakeDin1Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din1_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin1::FastwakeDin1Enable)
    }
}
#[doc = "Enable fastwake feature for DIN2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin2 {
    #[doc = "0: DISABLE"]
    FastwakeDin2Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin2Enable = 1,
}
impl From<FastwakeDin2> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN2` reader - Enable fastwake feature for DIN2"]
pub type FastwakeDin2R = crate::BitReader<FastwakeDin2>;
impl FastwakeDin2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin2 {
        match self.bits {
            false => FastwakeDin2::FastwakeDin2Disable,
            true => FastwakeDin2::FastwakeDin2Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din2_disable(&self) -> bool {
        *self == FastwakeDin2::FastwakeDin2Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din2_enable(&self) -> bool {
        *self == FastwakeDin2::FastwakeDin2Enable
    }
}
#[doc = "Field `FASTWAKE_DIN2` writer - Enable fastwake feature for DIN2"]
pub type FastwakeDin2W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin2>;
impl<'a, REG> FastwakeDin2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din2_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin2::FastwakeDin2Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din2_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin2::FastwakeDin2Enable)
    }
}
#[doc = "Enable fastwake feature for DIN3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin3 {
    #[doc = "0: DISABLE"]
    FastwakeDin3Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin3Enable = 1,
}
impl From<FastwakeDin3> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN3` reader - Enable fastwake feature for DIN3"]
pub type FastwakeDin3R = crate::BitReader<FastwakeDin3>;
impl FastwakeDin3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin3 {
        match self.bits {
            false => FastwakeDin3::FastwakeDin3Disable,
            true => FastwakeDin3::FastwakeDin3Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din3_disable(&self) -> bool {
        *self == FastwakeDin3::FastwakeDin3Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din3_enable(&self) -> bool {
        *self == FastwakeDin3::FastwakeDin3Enable
    }
}
#[doc = "Field `FASTWAKE_DIN3` writer - Enable fastwake feature for DIN3"]
pub type FastwakeDin3W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin3>;
impl<'a, REG> FastwakeDin3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din3_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin3::FastwakeDin3Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din3_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin3::FastwakeDin3Enable)
    }
}
#[doc = "Enable fastwake feature for DIN4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin4 {
    #[doc = "0: DISABLE"]
    FastwakeDin4Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin4Enable = 1,
}
impl From<FastwakeDin4> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN4` reader - Enable fastwake feature for DIN4"]
pub type FastwakeDin4R = crate::BitReader<FastwakeDin4>;
impl FastwakeDin4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin4 {
        match self.bits {
            false => FastwakeDin4::FastwakeDin4Disable,
            true => FastwakeDin4::FastwakeDin4Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din4_disable(&self) -> bool {
        *self == FastwakeDin4::FastwakeDin4Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din4_enable(&self) -> bool {
        *self == FastwakeDin4::FastwakeDin4Enable
    }
}
#[doc = "Field `FASTWAKE_DIN4` writer - Enable fastwake feature for DIN4"]
pub type FastwakeDin4W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin4>;
impl<'a, REG> FastwakeDin4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din4_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin4::FastwakeDin4Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din4_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin4::FastwakeDin4Enable)
    }
}
#[doc = "Enable fastwake feature for DIN5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin5 {
    #[doc = "0: DISABLE"]
    FastwakeDin5Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin5Enable = 1,
}
impl From<FastwakeDin5> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN5` reader - Enable fastwake feature for DIN5"]
pub type FastwakeDin5R = crate::BitReader<FastwakeDin5>;
impl FastwakeDin5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin5 {
        match self.bits {
            false => FastwakeDin5::FastwakeDin5Disable,
            true => FastwakeDin5::FastwakeDin5Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din5_disable(&self) -> bool {
        *self == FastwakeDin5::FastwakeDin5Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din5_enable(&self) -> bool {
        *self == FastwakeDin5::FastwakeDin5Enable
    }
}
#[doc = "Field `FASTWAKE_DIN5` writer - Enable fastwake feature for DIN5"]
pub type FastwakeDin5W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin5>;
impl<'a, REG> FastwakeDin5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din5_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin5::FastwakeDin5Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din5_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin5::FastwakeDin5Enable)
    }
}
#[doc = "Enable fastwake feature for DIN6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin6 {
    #[doc = "0: DISABLE"]
    FastwakeDin6Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin6Enable = 1,
}
impl From<FastwakeDin6> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN6` reader - Enable fastwake feature for DIN6"]
pub type FastwakeDin6R = crate::BitReader<FastwakeDin6>;
impl FastwakeDin6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin6 {
        match self.bits {
            false => FastwakeDin6::FastwakeDin6Disable,
            true => FastwakeDin6::FastwakeDin6Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din6_disable(&self) -> bool {
        *self == FastwakeDin6::FastwakeDin6Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din6_enable(&self) -> bool {
        *self == FastwakeDin6::FastwakeDin6Enable
    }
}
#[doc = "Field `FASTWAKE_DIN6` writer - Enable fastwake feature for DIN6"]
pub type FastwakeDin6W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin6>;
impl<'a, REG> FastwakeDin6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din6_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin6::FastwakeDin6Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din6_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin6::FastwakeDin6Enable)
    }
}
#[doc = "Enable fastwake feature for DIN7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin7 {
    #[doc = "0: DISABLE"]
    FastwakeDin7Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin7Enable = 1,
}
impl From<FastwakeDin7> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN7` reader - Enable fastwake feature for DIN7"]
pub type FastwakeDin7R = crate::BitReader<FastwakeDin7>;
impl FastwakeDin7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin7 {
        match self.bits {
            false => FastwakeDin7::FastwakeDin7Disable,
            true => FastwakeDin7::FastwakeDin7Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din7_disable(&self) -> bool {
        *self == FastwakeDin7::FastwakeDin7Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din7_enable(&self) -> bool {
        *self == FastwakeDin7::FastwakeDin7Enable
    }
}
#[doc = "Field `FASTWAKE_DIN7` writer - Enable fastwake feature for DIN7"]
pub type FastwakeDin7W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin7>;
impl<'a, REG> FastwakeDin7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din7_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin7::FastwakeDin7Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din7_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin7::FastwakeDin7Enable)
    }
}
#[doc = "Enable fastwake feature for DIN8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin8 {
    #[doc = "0: DISABLE"]
    FastwakeDin8Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin8Enable = 1,
}
impl From<FastwakeDin8> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN8` reader - Enable fastwake feature for DIN8"]
pub type FastwakeDin8R = crate::BitReader<FastwakeDin8>;
impl FastwakeDin8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin8 {
        match self.bits {
            false => FastwakeDin8::FastwakeDin8Disable,
            true => FastwakeDin8::FastwakeDin8Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din8_disable(&self) -> bool {
        *self == FastwakeDin8::FastwakeDin8Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din8_enable(&self) -> bool {
        *self == FastwakeDin8::FastwakeDin8Enable
    }
}
#[doc = "Field `FASTWAKE_DIN8` writer - Enable fastwake feature for DIN8"]
pub type FastwakeDin8W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin8>;
impl<'a, REG> FastwakeDin8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din8_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin8::FastwakeDin8Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din8_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin8::FastwakeDin8Enable)
    }
}
#[doc = "Enable fastwake feature for DIN9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin9 {
    #[doc = "0: DISABLE"]
    FastwakeDin9Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin9Enable = 1,
}
impl From<FastwakeDin9> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN9` reader - Enable fastwake feature for DIN9"]
pub type FastwakeDin9R = crate::BitReader<FastwakeDin9>;
impl FastwakeDin9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin9 {
        match self.bits {
            false => FastwakeDin9::FastwakeDin9Disable,
            true => FastwakeDin9::FastwakeDin9Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din9_disable(&self) -> bool {
        *self == FastwakeDin9::FastwakeDin9Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din9_enable(&self) -> bool {
        *self == FastwakeDin9::FastwakeDin9Enable
    }
}
#[doc = "Field `FASTWAKE_DIN9` writer - Enable fastwake feature for DIN9"]
pub type FastwakeDin9W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin9>;
impl<'a, REG> FastwakeDin9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din9_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin9::FastwakeDin9Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din9_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin9::FastwakeDin9Enable)
    }
}
#[doc = "Enable fastwake feature for DIN10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin10 {
    #[doc = "0: DISABLE"]
    FastwakeDin10Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin10Enable = 1,
}
impl From<FastwakeDin10> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN10` reader - Enable fastwake feature for DIN10"]
pub type FastwakeDin10R = crate::BitReader<FastwakeDin10>;
impl FastwakeDin10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin10 {
        match self.bits {
            false => FastwakeDin10::FastwakeDin10Disable,
            true => FastwakeDin10::FastwakeDin10Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din10_disable(&self) -> bool {
        *self == FastwakeDin10::FastwakeDin10Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din10_enable(&self) -> bool {
        *self == FastwakeDin10::FastwakeDin10Enable
    }
}
#[doc = "Field `FASTWAKE_DIN10` writer - Enable fastwake feature for DIN10"]
pub type FastwakeDin10W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin10>;
impl<'a, REG> FastwakeDin10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din10_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin10::FastwakeDin10Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din10_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin10::FastwakeDin10Enable)
    }
}
#[doc = "Enable fastwake feature for DIN11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin11 {
    #[doc = "0: DISABLE"]
    FastwakeDin11Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin11Enable = 1,
}
impl From<FastwakeDin11> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN11` reader - Enable fastwake feature for DIN11"]
pub type FastwakeDin11R = crate::BitReader<FastwakeDin11>;
impl FastwakeDin11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin11 {
        match self.bits {
            false => FastwakeDin11::FastwakeDin11Disable,
            true => FastwakeDin11::FastwakeDin11Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din11_disable(&self) -> bool {
        *self == FastwakeDin11::FastwakeDin11Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din11_enable(&self) -> bool {
        *self == FastwakeDin11::FastwakeDin11Enable
    }
}
#[doc = "Field `FASTWAKE_DIN11` writer - Enable fastwake feature for DIN11"]
pub type FastwakeDin11W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin11>;
impl<'a, REG> FastwakeDin11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din11_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin11::FastwakeDin11Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din11_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin11::FastwakeDin11Enable)
    }
}
#[doc = "Enable fastwake feature for DIN12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin12 {
    #[doc = "0: DISABLE"]
    FastwakeDin12Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin12Enable = 1,
}
impl From<FastwakeDin12> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN12` reader - Enable fastwake feature for DIN12"]
pub type FastwakeDin12R = crate::BitReader<FastwakeDin12>;
impl FastwakeDin12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin12 {
        match self.bits {
            false => FastwakeDin12::FastwakeDin12Disable,
            true => FastwakeDin12::FastwakeDin12Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din12_disable(&self) -> bool {
        *self == FastwakeDin12::FastwakeDin12Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din12_enable(&self) -> bool {
        *self == FastwakeDin12::FastwakeDin12Enable
    }
}
#[doc = "Field `FASTWAKE_DIN12` writer - Enable fastwake feature for DIN12"]
pub type FastwakeDin12W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin12>;
impl<'a, REG> FastwakeDin12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din12_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin12::FastwakeDin12Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din12_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin12::FastwakeDin12Enable)
    }
}
#[doc = "Enable fastwake feature for DIN13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin13 {
    #[doc = "0: DISABLE"]
    FastwakeDin13Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin13Enable = 1,
}
impl From<FastwakeDin13> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN13` reader - Enable fastwake feature for DIN13"]
pub type FastwakeDin13R = crate::BitReader<FastwakeDin13>;
impl FastwakeDin13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin13 {
        match self.bits {
            false => FastwakeDin13::FastwakeDin13Disable,
            true => FastwakeDin13::FastwakeDin13Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din13_disable(&self) -> bool {
        *self == FastwakeDin13::FastwakeDin13Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din13_enable(&self) -> bool {
        *self == FastwakeDin13::FastwakeDin13Enable
    }
}
#[doc = "Field `FASTWAKE_DIN13` writer - Enable fastwake feature for DIN13"]
pub type FastwakeDin13W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin13>;
impl<'a, REG> FastwakeDin13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din13_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin13::FastwakeDin13Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din13_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin13::FastwakeDin13Enable)
    }
}
#[doc = "Enable fastwake feature for DIN14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin14 {
    #[doc = "0: DISABLE"]
    FastwakeDin14Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin14Enable = 1,
}
impl From<FastwakeDin14> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN14` reader - Enable fastwake feature for DIN14"]
pub type FastwakeDin14R = crate::BitReader<FastwakeDin14>;
impl FastwakeDin14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin14 {
        match self.bits {
            false => FastwakeDin14::FastwakeDin14Disable,
            true => FastwakeDin14::FastwakeDin14Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din14_disable(&self) -> bool {
        *self == FastwakeDin14::FastwakeDin14Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din14_enable(&self) -> bool {
        *self == FastwakeDin14::FastwakeDin14Enable
    }
}
#[doc = "Field `FASTWAKE_DIN14` writer - Enable fastwake feature for DIN14"]
pub type FastwakeDin14W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin14>;
impl<'a, REG> FastwakeDin14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din14_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin14::FastwakeDin14Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din14_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin14::FastwakeDin14Enable)
    }
}
#[doc = "Enable fastwake feature for DIN15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin15 {
    #[doc = "0: DISABLE"]
    FastwakeDin15Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin15Enable = 1,
}
impl From<FastwakeDin15> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN15` reader - Enable fastwake feature for DIN15"]
pub type FastwakeDin15R = crate::BitReader<FastwakeDin15>;
impl FastwakeDin15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin15 {
        match self.bits {
            false => FastwakeDin15::FastwakeDin15Disable,
            true => FastwakeDin15::FastwakeDin15Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din15_disable(&self) -> bool {
        *self == FastwakeDin15::FastwakeDin15Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din15_enable(&self) -> bool {
        *self == FastwakeDin15::FastwakeDin15Enable
    }
}
#[doc = "Field `FASTWAKE_DIN15` writer - Enable fastwake feature for DIN15"]
pub type FastwakeDin15W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin15>;
impl<'a, REG> FastwakeDin15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din15_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin15::FastwakeDin15Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din15_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin15::FastwakeDin15Enable)
    }
}
#[doc = "Enable fastwake feature for DIN16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin16 {
    #[doc = "0: DISABLE"]
    FastwakeDin16Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin16Enable = 1,
}
impl From<FastwakeDin16> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN16` reader - Enable fastwake feature for DIN16"]
pub type FastwakeDin16R = crate::BitReader<FastwakeDin16>;
impl FastwakeDin16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin16 {
        match self.bits {
            false => FastwakeDin16::FastwakeDin16Disable,
            true => FastwakeDin16::FastwakeDin16Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din16_disable(&self) -> bool {
        *self == FastwakeDin16::FastwakeDin16Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din16_enable(&self) -> bool {
        *self == FastwakeDin16::FastwakeDin16Enable
    }
}
#[doc = "Field `FASTWAKE_DIN16` writer - Enable fastwake feature for DIN16"]
pub type FastwakeDin16W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin16>;
impl<'a, REG> FastwakeDin16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din16_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin16::FastwakeDin16Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din16_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin16::FastwakeDin16Enable)
    }
}
#[doc = "Enable fastwake feature for DIN17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin17 {
    #[doc = "0: DISABLE"]
    FastwakeDin17Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin17Enable = 1,
}
impl From<FastwakeDin17> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN17` reader - Enable fastwake feature for DIN17"]
pub type FastwakeDin17R = crate::BitReader<FastwakeDin17>;
impl FastwakeDin17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin17 {
        match self.bits {
            false => FastwakeDin17::FastwakeDin17Disable,
            true => FastwakeDin17::FastwakeDin17Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din17_disable(&self) -> bool {
        *self == FastwakeDin17::FastwakeDin17Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din17_enable(&self) -> bool {
        *self == FastwakeDin17::FastwakeDin17Enable
    }
}
#[doc = "Field `FASTWAKE_DIN17` writer - Enable fastwake feature for DIN17"]
pub type FastwakeDin17W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin17>;
impl<'a, REG> FastwakeDin17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din17_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin17::FastwakeDin17Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din17_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin17::FastwakeDin17Enable)
    }
}
#[doc = "Enable fastwake feature for DIN18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin18 {
    #[doc = "0: DISABLE"]
    FastwakeDin18Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin18Enable = 1,
}
impl From<FastwakeDin18> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN18` reader - Enable fastwake feature for DIN18"]
pub type FastwakeDin18R = crate::BitReader<FastwakeDin18>;
impl FastwakeDin18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin18 {
        match self.bits {
            false => FastwakeDin18::FastwakeDin18Disable,
            true => FastwakeDin18::FastwakeDin18Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din18_disable(&self) -> bool {
        *self == FastwakeDin18::FastwakeDin18Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din18_enable(&self) -> bool {
        *self == FastwakeDin18::FastwakeDin18Enable
    }
}
#[doc = "Field `FASTWAKE_DIN18` writer - Enable fastwake feature for DIN18"]
pub type FastwakeDin18W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin18>;
impl<'a, REG> FastwakeDin18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din18_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin18::FastwakeDin18Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din18_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin18::FastwakeDin18Enable)
    }
}
#[doc = "Enable fastwake feature for DIN19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin19 {
    #[doc = "0: DISABLE"]
    FastwakeDin19Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin19Enable = 1,
}
impl From<FastwakeDin19> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN19` reader - Enable fastwake feature for DIN19"]
pub type FastwakeDin19R = crate::BitReader<FastwakeDin19>;
impl FastwakeDin19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin19 {
        match self.bits {
            false => FastwakeDin19::FastwakeDin19Disable,
            true => FastwakeDin19::FastwakeDin19Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din19_disable(&self) -> bool {
        *self == FastwakeDin19::FastwakeDin19Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din19_enable(&self) -> bool {
        *self == FastwakeDin19::FastwakeDin19Enable
    }
}
#[doc = "Field `FASTWAKE_DIN19` writer - Enable fastwake feature for DIN19"]
pub type FastwakeDin19W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin19>;
impl<'a, REG> FastwakeDin19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din19_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin19::FastwakeDin19Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din19_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin19::FastwakeDin19Enable)
    }
}
#[doc = "Enable fastwake feature for DIN20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin20 {
    #[doc = "0: DISABLE"]
    FastwakeDin20Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin20Enable = 1,
}
impl From<FastwakeDin20> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN20` reader - Enable fastwake feature for DIN20"]
pub type FastwakeDin20R = crate::BitReader<FastwakeDin20>;
impl FastwakeDin20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin20 {
        match self.bits {
            false => FastwakeDin20::FastwakeDin20Disable,
            true => FastwakeDin20::FastwakeDin20Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din20_disable(&self) -> bool {
        *self == FastwakeDin20::FastwakeDin20Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din20_enable(&self) -> bool {
        *self == FastwakeDin20::FastwakeDin20Enable
    }
}
#[doc = "Field `FASTWAKE_DIN20` writer - Enable fastwake feature for DIN20"]
pub type FastwakeDin20W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin20>;
impl<'a, REG> FastwakeDin20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din20_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin20::FastwakeDin20Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din20_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin20::FastwakeDin20Enable)
    }
}
#[doc = "Enable fastwake feature for DIN21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin21 {
    #[doc = "0: DISABLE"]
    FastwakeDin21Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin21Enable = 1,
}
impl From<FastwakeDin21> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN21` reader - Enable fastwake feature for DIN21"]
pub type FastwakeDin21R = crate::BitReader<FastwakeDin21>;
impl FastwakeDin21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin21 {
        match self.bits {
            false => FastwakeDin21::FastwakeDin21Disable,
            true => FastwakeDin21::FastwakeDin21Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din21_disable(&self) -> bool {
        *self == FastwakeDin21::FastwakeDin21Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din21_enable(&self) -> bool {
        *self == FastwakeDin21::FastwakeDin21Enable
    }
}
#[doc = "Field `FASTWAKE_DIN21` writer - Enable fastwake feature for DIN21"]
pub type FastwakeDin21W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin21>;
impl<'a, REG> FastwakeDin21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din21_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin21::FastwakeDin21Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din21_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin21::FastwakeDin21Enable)
    }
}
#[doc = "Enable fastwake feature for DIN22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin22 {
    #[doc = "0: DISABLE"]
    FastwakeDin22Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin22Enable = 1,
}
impl From<FastwakeDin22> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN22` reader - Enable fastwake feature for DIN22"]
pub type FastwakeDin22R = crate::BitReader<FastwakeDin22>;
impl FastwakeDin22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin22 {
        match self.bits {
            false => FastwakeDin22::FastwakeDin22Disable,
            true => FastwakeDin22::FastwakeDin22Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din22_disable(&self) -> bool {
        *self == FastwakeDin22::FastwakeDin22Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din22_enable(&self) -> bool {
        *self == FastwakeDin22::FastwakeDin22Enable
    }
}
#[doc = "Field `FASTWAKE_DIN22` writer - Enable fastwake feature for DIN22"]
pub type FastwakeDin22W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin22>;
impl<'a, REG> FastwakeDin22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din22_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin22::FastwakeDin22Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din22_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin22::FastwakeDin22Enable)
    }
}
#[doc = "Enable fastwake feature for DIN23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin23 {
    #[doc = "0: DISABLE"]
    FastwakeDin23Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin23Enable = 1,
}
impl From<FastwakeDin23> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN23` reader - Enable fastwake feature for DIN23"]
pub type FastwakeDin23R = crate::BitReader<FastwakeDin23>;
impl FastwakeDin23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin23 {
        match self.bits {
            false => FastwakeDin23::FastwakeDin23Disable,
            true => FastwakeDin23::FastwakeDin23Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din23_disable(&self) -> bool {
        *self == FastwakeDin23::FastwakeDin23Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din23_enable(&self) -> bool {
        *self == FastwakeDin23::FastwakeDin23Enable
    }
}
#[doc = "Field `FASTWAKE_DIN23` writer - Enable fastwake feature for DIN23"]
pub type FastwakeDin23W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin23>;
impl<'a, REG> FastwakeDin23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din23_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin23::FastwakeDin23Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din23_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin23::FastwakeDin23Enable)
    }
}
#[doc = "Enable fastwake feature for DIN24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin24 {
    #[doc = "0: DISABLE"]
    FastwakeDin24Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin24Enable = 1,
}
impl From<FastwakeDin24> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN24` reader - Enable fastwake feature for DIN24"]
pub type FastwakeDin24R = crate::BitReader<FastwakeDin24>;
impl FastwakeDin24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin24 {
        match self.bits {
            false => FastwakeDin24::FastwakeDin24Disable,
            true => FastwakeDin24::FastwakeDin24Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din24_disable(&self) -> bool {
        *self == FastwakeDin24::FastwakeDin24Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din24_enable(&self) -> bool {
        *self == FastwakeDin24::FastwakeDin24Enable
    }
}
#[doc = "Field `FASTWAKE_DIN24` writer - Enable fastwake feature for DIN24"]
pub type FastwakeDin24W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin24>;
impl<'a, REG> FastwakeDin24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din24_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin24::FastwakeDin24Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din24_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin24::FastwakeDin24Enable)
    }
}
#[doc = "Enable fastwake feature for DIN25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin25 {
    #[doc = "0: DISABLE"]
    FastwakeDin25Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin25Enable = 1,
}
impl From<FastwakeDin25> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN25` reader - Enable fastwake feature for DIN25"]
pub type FastwakeDin25R = crate::BitReader<FastwakeDin25>;
impl FastwakeDin25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin25 {
        match self.bits {
            false => FastwakeDin25::FastwakeDin25Disable,
            true => FastwakeDin25::FastwakeDin25Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din25_disable(&self) -> bool {
        *self == FastwakeDin25::FastwakeDin25Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din25_enable(&self) -> bool {
        *self == FastwakeDin25::FastwakeDin25Enable
    }
}
#[doc = "Field `FASTWAKE_DIN25` writer - Enable fastwake feature for DIN25"]
pub type FastwakeDin25W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin25>;
impl<'a, REG> FastwakeDin25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din25_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin25::FastwakeDin25Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din25_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin25::FastwakeDin25Enable)
    }
}
#[doc = "Enable fastwake feature for DIN26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin26 {
    #[doc = "0: DISABLE"]
    FastwakeDin26Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin26Enable = 1,
}
impl From<FastwakeDin26> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN26` reader - Enable fastwake feature for DIN26"]
pub type FastwakeDin26R = crate::BitReader<FastwakeDin26>;
impl FastwakeDin26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin26 {
        match self.bits {
            false => FastwakeDin26::FastwakeDin26Disable,
            true => FastwakeDin26::FastwakeDin26Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din26_disable(&self) -> bool {
        *self == FastwakeDin26::FastwakeDin26Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din26_enable(&self) -> bool {
        *self == FastwakeDin26::FastwakeDin26Enable
    }
}
#[doc = "Field `FASTWAKE_DIN26` writer - Enable fastwake feature for DIN26"]
pub type FastwakeDin26W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin26>;
impl<'a, REG> FastwakeDin26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din26_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin26::FastwakeDin26Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din26_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin26::FastwakeDin26Enable)
    }
}
#[doc = "Enable fastwake feature for DIN27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin27 {
    #[doc = "0: DISABLE"]
    FastwakeDin27Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin27Enable = 1,
}
impl From<FastwakeDin27> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN27` reader - Enable fastwake feature for DIN27"]
pub type FastwakeDin27R = crate::BitReader<FastwakeDin27>;
impl FastwakeDin27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin27 {
        match self.bits {
            false => FastwakeDin27::FastwakeDin27Disable,
            true => FastwakeDin27::FastwakeDin27Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din27_disable(&self) -> bool {
        *self == FastwakeDin27::FastwakeDin27Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din27_enable(&self) -> bool {
        *self == FastwakeDin27::FastwakeDin27Enable
    }
}
#[doc = "Field `FASTWAKE_DIN27` writer - Enable fastwake feature for DIN27"]
pub type FastwakeDin27W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin27>;
impl<'a, REG> FastwakeDin27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din27_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin27::FastwakeDin27Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din27_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin27::FastwakeDin27Enable)
    }
}
#[doc = "Enable fastwake feature for DIN29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin28 {
    #[doc = "0: DISABLE"]
    FastwakeDin28Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin28Enable = 1,
}
impl From<FastwakeDin28> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN28` reader - Enable fastwake feature for DIN29"]
pub type FastwakeDin28R = crate::BitReader<FastwakeDin28>;
impl FastwakeDin28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin28 {
        match self.bits {
            false => FastwakeDin28::FastwakeDin28Disable,
            true => FastwakeDin28::FastwakeDin28Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din28_disable(&self) -> bool {
        *self == FastwakeDin28::FastwakeDin28Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din28_enable(&self) -> bool {
        *self == FastwakeDin28::FastwakeDin28Enable
    }
}
#[doc = "Field `FASTWAKE_DIN28` writer - Enable fastwake feature for DIN29"]
pub type FastwakeDin28W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin28>;
impl<'a, REG> FastwakeDin28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din28_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin28::FastwakeDin28Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din28_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin28::FastwakeDin28Enable)
    }
}
#[doc = "Enable fastwake feature for DIN29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin29 {
    #[doc = "0: DISABLE"]
    FastwakeDin29Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin29Enable = 1,
}
impl From<FastwakeDin29> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN29` reader - Enable fastwake feature for DIN29"]
pub type FastwakeDin29R = crate::BitReader<FastwakeDin29>;
impl FastwakeDin29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin29 {
        match self.bits {
            false => FastwakeDin29::FastwakeDin29Disable,
            true => FastwakeDin29::FastwakeDin29Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din29_disable(&self) -> bool {
        *self == FastwakeDin29::FastwakeDin29Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din29_enable(&self) -> bool {
        *self == FastwakeDin29::FastwakeDin29Enable
    }
}
#[doc = "Field `FASTWAKE_DIN29` writer - Enable fastwake feature for DIN29"]
pub type FastwakeDin29W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin29>;
impl<'a, REG> FastwakeDin29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din29_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin29::FastwakeDin29Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din29_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin29::FastwakeDin29Enable)
    }
}
#[doc = "Enable fastwake feature for DIN30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin30 {
    #[doc = "0: DISABLE"]
    FastwakeDin30Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin30Enable = 1,
}
impl From<FastwakeDin30> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN30` reader - Enable fastwake feature for DIN30"]
pub type FastwakeDin30R = crate::BitReader<FastwakeDin30>;
impl FastwakeDin30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin30 {
        match self.bits {
            false => FastwakeDin30::FastwakeDin30Disable,
            true => FastwakeDin30::FastwakeDin30Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din30_disable(&self) -> bool {
        *self == FastwakeDin30::FastwakeDin30Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din30_enable(&self) -> bool {
        *self == FastwakeDin30::FastwakeDin30Enable
    }
}
#[doc = "Field `FASTWAKE_DIN30` writer - Enable fastwake feature for DIN30"]
pub type FastwakeDin30W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin30>;
impl<'a, REG> FastwakeDin30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din30_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin30::FastwakeDin30Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din30_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin30::FastwakeDin30Enable)
    }
}
#[doc = "Enable fastwake feature for DIN31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastwakeDin31 {
    #[doc = "0: DISABLE"]
    FastwakeDin31Disable = 0,
    #[doc = "1: ENABLE"]
    FastwakeDin31Enable = 1,
}
impl From<FastwakeDin31> for bool {
    #[inline(always)]
    fn from(variant: FastwakeDin31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTWAKE_DIN31` reader - Enable fastwake feature for DIN31"]
pub type FastwakeDin31R = crate::BitReader<FastwakeDin31>;
impl FastwakeDin31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastwakeDin31 {
        match self.bits {
            false => FastwakeDin31::FastwakeDin31Disable,
            true => FastwakeDin31::FastwakeDin31Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_fastwake_din31_disable(&self) -> bool {
        *self == FastwakeDin31::FastwakeDin31Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_fastwake_din31_enable(&self) -> bool {
        *self == FastwakeDin31::FastwakeDin31Enable
    }
}
#[doc = "Field `FASTWAKE_DIN31` writer - Enable fastwake feature for DIN31"]
pub type FastwakeDin31W<'a, REG> = crate::BitWriter<'a, REG, FastwakeDin31>;
impl<'a, REG> FastwakeDin31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn fastwake_din31_disable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin31::FastwakeDin31Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn fastwake_din31_enable(self) -> &'a mut crate::W<REG> {
        self.variant(FastwakeDin31::FastwakeDin31Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable fastwake feature for DIN0"]
    #[inline(always)]
    pub fn fastwake_din0(&self) -> FastwakeDin0R {
        FastwakeDin0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable fastwake feature for DIN1"]
    #[inline(always)]
    pub fn fastwake_din1(&self) -> FastwakeDin1R {
        FastwakeDin1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable fastwake feature for DIN2"]
    #[inline(always)]
    pub fn fastwake_din2(&self) -> FastwakeDin2R {
        FastwakeDin2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable fastwake feature for DIN3"]
    #[inline(always)]
    pub fn fastwake_din3(&self) -> FastwakeDin3R {
        FastwakeDin3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable fastwake feature for DIN4"]
    #[inline(always)]
    pub fn fastwake_din4(&self) -> FastwakeDin4R {
        FastwakeDin4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable fastwake feature for DIN5"]
    #[inline(always)]
    pub fn fastwake_din5(&self) -> FastwakeDin5R {
        FastwakeDin5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable fastwake feature for DIN6"]
    #[inline(always)]
    pub fn fastwake_din6(&self) -> FastwakeDin6R {
        FastwakeDin6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable fastwake feature for DIN7"]
    #[inline(always)]
    pub fn fastwake_din7(&self) -> FastwakeDin7R {
        FastwakeDin7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable fastwake feature for DIN8"]
    #[inline(always)]
    pub fn fastwake_din8(&self) -> FastwakeDin8R {
        FastwakeDin8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable fastwake feature for DIN9"]
    #[inline(always)]
    pub fn fastwake_din9(&self) -> FastwakeDin9R {
        FastwakeDin9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable fastwake feature for DIN10"]
    #[inline(always)]
    pub fn fastwake_din10(&self) -> FastwakeDin10R {
        FastwakeDin10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable fastwake feature for DIN11"]
    #[inline(always)]
    pub fn fastwake_din11(&self) -> FastwakeDin11R {
        FastwakeDin11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable fastwake feature for DIN12"]
    #[inline(always)]
    pub fn fastwake_din12(&self) -> FastwakeDin12R {
        FastwakeDin12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable fastwake feature for DIN13"]
    #[inline(always)]
    pub fn fastwake_din13(&self) -> FastwakeDin13R {
        FastwakeDin13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable fastwake feature for DIN14"]
    #[inline(always)]
    pub fn fastwake_din14(&self) -> FastwakeDin14R {
        FastwakeDin14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable fastwake feature for DIN15"]
    #[inline(always)]
    pub fn fastwake_din15(&self) -> FastwakeDin15R {
        FastwakeDin15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable fastwake feature for DIN16"]
    #[inline(always)]
    pub fn fastwake_din16(&self) -> FastwakeDin16R {
        FastwakeDin16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable fastwake feature for DIN17"]
    #[inline(always)]
    pub fn fastwake_din17(&self) -> FastwakeDin17R {
        FastwakeDin17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable fastwake feature for DIN18"]
    #[inline(always)]
    pub fn fastwake_din18(&self) -> FastwakeDin18R {
        FastwakeDin18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable fastwake feature for DIN19"]
    #[inline(always)]
    pub fn fastwake_din19(&self) -> FastwakeDin19R {
        FastwakeDin19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable fastwake feature for DIN20"]
    #[inline(always)]
    pub fn fastwake_din20(&self) -> FastwakeDin20R {
        FastwakeDin20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable fastwake feature for DIN21"]
    #[inline(always)]
    pub fn fastwake_din21(&self) -> FastwakeDin21R {
        FastwakeDin21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable fastwake feature for DIN22"]
    #[inline(always)]
    pub fn fastwake_din22(&self) -> FastwakeDin22R {
        FastwakeDin22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable fastwake feature for DIN23"]
    #[inline(always)]
    pub fn fastwake_din23(&self) -> FastwakeDin23R {
        FastwakeDin23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable fastwake feature for DIN24"]
    #[inline(always)]
    pub fn fastwake_din24(&self) -> FastwakeDin24R {
        FastwakeDin24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable fastwake feature for DIN25"]
    #[inline(always)]
    pub fn fastwake_din25(&self) -> FastwakeDin25R {
        FastwakeDin25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable fastwake feature for DIN26"]
    #[inline(always)]
    pub fn fastwake_din26(&self) -> FastwakeDin26R {
        FastwakeDin26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable fastwake feature for DIN27"]
    #[inline(always)]
    pub fn fastwake_din27(&self) -> FastwakeDin27R {
        FastwakeDin27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn fastwake_din28(&self) -> FastwakeDin28R {
        FastwakeDin28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn fastwake_din29(&self) -> FastwakeDin29R {
        FastwakeDin29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable fastwake feature for DIN30"]
    #[inline(always)]
    pub fn fastwake_din30(&self) -> FastwakeDin30R {
        FastwakeDin30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable fastwake feature for DIN31"]
    #[inline(always)]
    pub fn fastwake_din31(&self) -> FastwakeDin31R {
        FastwakeDin31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable fastwake feature for DIN0"]
    #[inline(always)]
    pub fn fastwake_din0(&mut self) -> FastwakeDin0W<FastwakeSpec> {
        FastwakeDin0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable fastwake feature for DIN1"]
    #[inline(always)]
    pub fn fastwake_din1(&mut self) -> FastwakeDin1W<FastwakeSpec> {
        FastwakeDin1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable fastwake feature for DIN2"]
    #[inline(always)]
    pub fn fastwake_din2(&mut self) -> FastwakeDin2W<FastwakeSpec> {
        FastwakeDin2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable fastwake feature for DIN3"]
    #[inline(always)]
    pub fn fastwake_din3(&mut self) -> FastwakeDin3W<FastwakeSpec> {
        FastwakeDin3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable fastwake feature for DIN4"]
    #[inline(always)]
    pub fn fastwake_din4(&mut self) -> FastwakeDin4W<FastwakeSpec> {
        FastwakeDin4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable fastwake feature for DIN5"]
    #[inline(always)]
    pub fn fastwake_din5(&mut self) -> FastwakeDin5W<FastwakeSpec> {
        FastwakeDin5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable fastwake feature for DIN6"]
    #[inline(always)]
    pub fn fastwake_din6(&mut self) -> FastwakeDin6W<FastwakeSpec> {
        FastwakeDin6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable fastwake feature for DIN7"]
    #[inline(always)]
    pub fn fastwake_din7(&mut self) -> FastwakeDin7W<FastwakeSpec> {
        FastwakeDin7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable fastwake feature for DIN8"]
    #[inline(always)]
    pub fn fastwake_din8(&mut self) -> FastwakeDin8W<FastwakeSpec> {
        FastwakeDin8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable fastwake feature for DIN9"]
    #[inline(always)]
    pub fn fastwake_din9(&mut self) -> FastwakeDin9W<FastwakeSpec> {
        FastwakeDin9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable fastwake feature for DIN10"]
    #[inline(always)]
    pub fn fastwake_din10(&mut self) -> FastwakeDin10W<FastwakeSpec> {
        FastwakeDin10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable fastwake feature for DIN11"]
    #[inline(always)]
    pub fn fastwake_din11(&mut self) -> FastwakeDin11W<FastwakeSpec> {
        FastwakeDin11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable fastwake feature for DIN12"]
    #[inline(always)]
    pub fn fastwake_din12(&mut self) -> FastwakeDin12W<FastwakeSpec> {
        FastwakeDin12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable fastwake feature for DIN13"]
    #[inline(always)]
    pub fn fastwake_din13(&mut self) -> FastwakeDin13W<FastwakeSpec> {
        FastwakeDin13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable fastwake feature for DIN14"]
    #[inline(always)]
    pub fn fastwake_din14(&mut self) -> FastwakeDin14W<FastwakeSpec> {
        FastwakeDin14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable fastwake feature for DIN15"]
    #[inline(always)]
    pub fn fastwake_din15(&mut self) -> FastwakeDin15W<FastwakeSpec> {
        FastwakeDin15W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable fastwake feature for DIN16"]
    #[inline(always)]
    pub fn fastwake_din16(&mut self) -> FastwakeDin16W<FastwakeSpec> {
        FastwakeDin16W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable fastwake feature for DIN17"]
    #[inline(always)]
    pub fn fastwake_din17(&mut self) -> FastwakeDin17W<FastwakeSpec> {
        FastwakeDin17W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable fastwake feature for DIN18"]
    #[inline(always)]
    pub fn fastwake_din18(&mut self) -> FastwakeDin18W<FastwakeSpec> {
        FastwakeDin18W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable fastwake feature for DIN19"]
    #[inline(always)]
    pub fn fastwake_din19(&mut self) -> FastwakeDin19W<FastwakeSpec> {
        FastwakeDin19W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable fastwake feature for DIN20"]
    #[inline(always)]
    pub fn fastwake_din20(&mut self) -> FastwakeDin20W<FastwakeSpec> {
        FastwakeDin20W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable fastwake feature for DIN21"]
    #[inline(always)]
    pub fn fastwake_din21(&mut self) -> FastwakeDin21W<FastwakeSpec> {
        FastwakeDin21W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable fastwake feature for DIN22"]
    #[inline(always)]
    pub fn fastwake_din22(&mut self) -> FastwakeDin22W<FastwakeSpec> {
        FastwakeDin22W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable fastwake feature for DIN23"]
    #[inline(always)]
    pub fn fastwake_din23(&mut self) -> FastwakeDin23W<FastwakeSpec> {
        FastwakeDin23W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable fastwake feature for DIN24"]
    #[inline(always)]
    pub fn fastwake_din24(&mut self) -> FastwakeDin24W<FastwakeSpec> {
        FastwakeDin24W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable fastwake feature for DIN25"]
    #[inline(always)]
    pub fn fastwake_din25(&mut self) -> FastwakeDin25W<FastwakeSpec> {
        FastwakeDin25W::new(self, 25)
    }
    #[doc = "Bit 26 - Enable fastwake feature for DIN26"]
    #[inline(always)]
    pub fn fastwake_din26(&mut self) -> FastwakeDin26W<FastwakeSpec> {
        FastwakeDin26W::new(self, 26)
    }
    #[doc = "Bit 27 - Enable fastwake feature for DIN27"]
    #[inline(always)]
    pub fn fastwake_din27(&mut self) -> FastwakeDin27W<FastwakeSpec> {
        FastwakeDin27W::new(self, 27)
    }
    #[doc = "Bit 28 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn fastwake_din28(&mut self) -> FastwakeDin28W<FastwakeSpec> {
        FastwakeDin28W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable fastwake feature for DIN29"]
    #[inline(always)]
    pub fn fastwake_din29(&mut self) -> FastwakeDin29W<FastwakeSpec> {
        FastwakeDin29W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable fastwake feature for DIN30"]
    #[inline(always)]
    pub fn fastwake_din30(&mut self) -> FastwakeDin30W<FastwakeSpec> {
        FastwakeDin30W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable fastwake feature for DIN31"]
    #[inline(always)]
    pub fn fastwake_din31(&mut self) -> FastwakeDin31W<FastwakeSpec> {
        FastwakeDin31W::new(self, 31)
    }
}
#[doc = "FAST WAKE ENABLE\n\nYou can [`read`](crate::Reg::read) this register and get [`fastwake::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fastwake::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastwakeSpec;
impl crate::RegisterSpec for FastwakeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fastwake::R`](R) reader structure"]
impl crate::Readable for FastwakeSpec {}
#[doc = "`write(|w| ..)` method takes [`fastwake::W`](W) writer structure"]
impl crate::Writable for FastwakeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FASTWAKE to value 0"]
impl crate::Resettable for FastwakeSpec {
    const RESET_VALUE: u32 = 0;
}

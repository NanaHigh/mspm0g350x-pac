#[doc = "Register `QDIR` reader"]
pub type R = crate::R<QdirSpec>;
#[doc = "Direction of count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QdirDir {
    #[doc = "0: DOWN"]
    QdirDirDown = 0,
    #[doc = "1: UP"]
    QdirDirUp = 1,
}
impl From<QdirDir> for bool {
    #[inline(always)]
    fn from(variant: QdirDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `QDIR_DIR` reader - Direction of count"]
pub type QdirDirR = crate::BitReader<QdirDir>;
impl QdirDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> QdirDir {
        match self.bits {
            false => QdirDir::QdirDirDown,
            true => QdirDir::QdirDirUp,
        }
    }
    #[doc = "DOWN"]
    #[inline(always)]
    pub fn is_qdir_dir_down(&self) -> bool {
        *self == QdirDir::QdirDirDown
    }
    #[doc = "UP"]
    #[inline(always)]
    pub fn is_qdir_dir_up(&self) -> bool {
        *self == QdirDir::QdirDirUp
    }
}
impl R {
    #[doc = "Bit 0 - Direction of count"]
    #[inline(always)]
    pub fn qdir_dir(&self) -> QdirDirR {
        QdirDirR::new((self.bits & 1) != 0)
    }
}
#[doc = "Count Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qdir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdirSpec;
impl crate::RegisterSpec for QdirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdir::R`](R) reader structure"]
impl crate::Readable for QdirSpec {}
#[doc = "`reset()` method sets QDIR to value 0"]
impl crate::Resettable for QdirSpec {
    const RESET_VALUE: u32 = 0;
}

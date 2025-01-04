#[doc = "Register `MCAN_TXBAR` reader"]
pub type R = crate::R<McanTxbarSpec>;
#[doc = "Register `MCAN_TXBAR` writer"]
pub type W = crate::W<McanTxbarSpec>;
#[doc = "Field `MCAN_TXBAR_AR0` reader - Add Request 0. Each Tx Buffer has its own Add Request bit. Writing a '1' will set the corresponding Add Request bit; writing a '0' has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0 No transmission request added 1 Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored. Qualified Write is possible only with CCCR.CCE='0'"]
pub type McanTxbarAr0R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR0` writer - Add Request 0. Each Tx Buffer has its own Add Request bit. Writing a '1' will set the corresponding Add Request bit; writing a '0' has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0 No transmission request added 1 Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored. Qualified Write is possible only with CCCR.CCE='0'"]
pub type McanTxbarAr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR1` reader - Add Request 1. See description for bit 0."]
pub type McanTxbarAr1R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR1` writer - Add Request 1. See description for bit 0."]
pub type McanTxbarAr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR2` reader - Add Request 2. See description for bit 0."]
pub type McanTxbarAr2R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR2` writer - Add Request 2. See description for bit 0."]
pub type McanTxbarAr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR3` reader - Add Request 3. See description for bit 0."]
pub type McanTxbarAr3R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR3` writer - Add Request 3. See description for bit 0."]
pub type McanTxbarAr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR4` reader - Add Request 4. See description for bit 0."]
pub type McanTxbarAr4R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR4` writer - Add Request 4. See description for bit 0."]
pub type McanTxbarAr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR5` reader - Add Request 5. See description for bit 0."]
pub type McanTxbarAr5R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR5` writer - Add Request 5. See description for bit 0."]
pub type McanTxbarAr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR6` reader - Add Request 6. See description for bit 0."]
pub type McanTxbarAr6R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR6` writer - Add Request 6. See description for bit 0."]
pub type McanTxbarAr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR7` reader - Add Request 7. See description for bit 0."]
pub type McanTxbarAr7R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR7` writer - Add Request 7. See description for bit 0."]
pub type McanTxbarAr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR8` reader - Add Request 8. See description for bit 0."]
pub type McanTxbarAr8R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR8` writer - Add Request 8. See description for bit 0."]
pub type McanTxbarAr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR9` reader - Add Request 9. See description for bit 0."]
pub type McanTxbarAr9R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR9` writer - Add Request 9. See description for bit 0."]
pub type McanTxbarAr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR10` reader - Add Request 10. See description for bit 0."]
pub type McanTxbarAr10R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR10` writer - Add Request 10. See description for bit 0."]
pub type McanTxbarAr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR11` reader - Add Request 11. See description for bit 0."]
pub type McanTxbarAr11R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR11` writer - Add Request 11. See description for bit 0."]
pub type McanTxbarAr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR12` reader - Add Request 12. See description for bit 0."]
pub type McanTxbarAr12R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR12` writer - Add Request 12. See description for bit 0."]
pub type McanTxbarAr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR13` reader - Add Request 13. See description for bit 0."]
pub type McanTxbarAr13R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR13` writer - Add Request 13. See description for bit 0."]
pub type McanTxbarAr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR14` reader - Add Request 14. See description for bit 0."]
pub type McanTxbarAr14R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR14` writer - Add Request 14. See description for bit 0."]
pub type McanTxbarAr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR15` reader - Add Request 15. See description for bit 0."]
pub type McanTxbarAr15R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR15` writer - Add Request 15. See description for bit 0."]
pub type McanTxbarAr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR16` reader - Add Request 16. See description for bit 0."]
pub type McanTxbarAr16R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR16` writer - Add Request 16. See description for bit 0."]
pub type McanTxbarAr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR17` reader - Add Request 17. See description for bit 0."]
pub type McanTxbarAr17R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR17` writer - Add Request 17. See description for bit 0."]
pub type McanTxbarAr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR18` reader - Add Request 18. See description for bit 0."]
pub type McanTxbarAr18R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR18` writer - Add Request 18. See description for bit 0."]
pub type McanTxbarAr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR19` reader - Add Request 19. See description for bit 0."]
pub type McanTxbarAr19R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR19` writer - Add Request 19. See description for bit 0."]
pub type McanTxbarAr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR20` reader - Add Request 20. See description for bit 0."]
pub type McanTxbarAr20R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR20` writer - Add Request 20. See description for bit 0."]
pub type McanTxbarAr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR21` reader - Add Request 21. See description for bit 0."]
pub type McanTxbarAr21R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR21` writer - Add Request 21. See description for bit 0."]
pub type McanTxbarAr21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR22` reader - Add Request 22. See description for bit 0."]
pub type McanTxbarAr22R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR22` writer - Add Request 22. See description for bit 0."]
pub type McanTxbarAr22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR23` reader - Add Request 23. See description for bit 0."]
pub type McanTxbarAr23R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR23` writer - Add Request 23. See description for bit 0."]
pub type McanTxbarAr23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR24` reader - Add Request 24. See description for bit 0."]
pub type McanTxbarAr24R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR24` writer - Add Request 24. See description for bit 0."]
pub type McanTxbarAr24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR25` reader - Add Request 25. See description for bit 0."]
pub type McanTxbarAr25R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR25` writer - Add Request 25. See description for bit 0."]
pub type McanTxbarAr25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR26` reader - Add Request 26. See description for bit 0."]
pub type McanTxbarAr26R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR26` writer - Add Request 26. See description for bit 0."]
pub type McanTxbarAr26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR27` reader - Add Request 27. See description for bit 0."]
pub type McanTxbarAr27R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR27` writer - Add Request 27. See description for bit 0."]
pub type McanTxbarAr27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR28` reader - Add Request 28. See description for bit 0."]
pub type McanTxbarAr28R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR28` writer - Add Request 28. See description for bit 0."]
pub type McanTxbarAr28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR29` reader - Add Request 29. See description for bit 0."]
pub type McanTxbarAr29R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR29` writer - Add Request 29. See description for bit 0."]
pub type McanTxbarAr29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR30` reader - Add Request 30. See description for bit 0."]
pub type McanTxbarAr30R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR30` writer - Add Request 30. See description for bit 0."]
pub type McanTxbarAr30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBAR_AR31` reader - Add Request 31. See description for bit 0."]
pub type McanTxbarAr31R = crate::BitReader;
#[doc = "Field `MCAN_TXBAR_AR31` writer - Add Request 31. See description for bit 0."]
pub type McanTxbarAr31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Add Request 0. Each Tx Buffer has its own Add Request bit. Writing a '1' will set the corresponding Add Request bit; writing a '0' has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0 No transmission request added 1 Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored. Qualified Write is possible only with CCCR.CCE='0'"]
    #[inline(always)]
    pub fn mcan_txbar_ar0(&self) -> McanTxbarAr0R {
        McanTxbarAr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Add Request 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar1(&self) -> McanTxbarAr1R {
        McanTxbarAr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Add Request 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar2(&self) -> McanTxbarAr2R {
        McanTxbarAr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Add Request 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar3(&self) -> McanTxbarAr3R {
        McanTxbarAr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Add Request 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar4(&self) -> McanTxbarAr4R {
        McanTxbarAr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Add Request 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar5(&self) -> McanTxbarAr5R {
        McanTxbarAr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Add Request 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar6(&self) -> McanTxbarAr6R {
        McanTxbarAr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Add Request 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar7(&self) -> McanTxbarAr7R {
        McanTxbarAr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Add Request 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar8(&self) -> McanTxbarAr8R {
        McanTxbarAr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Add Request 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar9(&self) -> McanTxbarAr9R {
        McanTxbarAr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Add Request 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar10(&self) -> McanTxbarAr10R {
        McanTxbarAr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Add Request 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar11(&self) -> McanTxbarAr11R {
        McanTxbarAr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Add Request 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar12(&self) -> McanTxbarAr12R {
        McanTxbarAr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Add Request 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar13(&self) -> McanTxbarAr13R {
        McanTxbarAr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Add Request 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar14(&self) -> McanTxbarAr14R {
        McanTxbarAr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Add Request 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar15(&self) -> McanTxbarAr15R {
        McanTxbarAr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add Request 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar16(&self) -> McanTxbarAr16R {
        McanTxbarAr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Add Request 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar17(&self) -> McanTxbarAr17R {
        McanTxbarAr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Add Request 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar18(&self) -> McanTxbarAr18R {
        McanTxbarAr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Add Request 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar19(&self) -> McanTxbarAr19R {
        McanTxbarAr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Add Request 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar20(&self) -> McanTxbarAr20R {
        McanTxbarAr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Add Request 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar21(&self) -> McanTxbarAr21R {
        McanTxbarAr21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Add Request 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar22(&self) -> McanTxbarAr22R {
        McanTxbarAr22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Add Request 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar23(&self) -> McanTxbarAr23R {
        McanTxbarAr23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Add Request 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar24(&self) -> McanTxbarAr24R {
        McanTxbarAr24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Add Request 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar25(&self) -> McanTxbarAr25R {
        McanTxbarAr25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Add Request 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar26(&self) -> McanTxbarAr26R {
        McanTxbarAr26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Add Request 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar27(&self) -> McanTxbarAr27R {
        McanTxbarAr27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Add Request 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar28(&self) -> McanTxbarAr28R {
        McanTxbarAr28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Add Request 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar29(&self) -> McanTxbarAr29R {
        McanTxbarAr29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Add Request 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar30(&self) -> McanTxbarAr30R {
        McanTxbarAr30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Add Request 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar31(&self) -> McanTxbarAr31R {
        McanTxbarAr31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Add Request 0. Each Tx Buffer has its own Add Request bit. Writing a '1' will set the corresponding Add Request bit; writing a '0' has no impact. This enables the Host to set transmission requests for multiple Tx Buffers with one write to TXBAR. TXBAR bits are set only for those Tx Buffers configured via TXBC. When no Tx scan is running, the bits are reset immediately, else the bits remain set until the Tx scan process has completed. 0 No transmission request added 1 Transmission requested added Note: If an add request is applied for a Tx Buffer with pending transmission request (corresponding TXBRP bit already set), this add request is ignored. Qualified Write is possible only with CCCR.CCE='0'"]
    #[inline(always)]
    pub fn mcan_txbar_ar0(&mut self) -> McanTxbarAr0W<McanTxbarSpec> {
        McanTxbarAr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Add Request 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar1(&mut self) -> McanTxbarAr1W<McanTxbarSpec> {
        McanTxbarAr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Add Request 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar2(&mut self) -> McanTxbarAr2W<McanTxbarSpec> {
        McanTxbarAr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Add Request 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar3(&mut self) -> McanTxbarAr3W<McanTxbarSpec> {
        McanTxbarAr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Add Request 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar4(&mut self) -> McanTxbarAr4W<McanTxbarSpec> {
        McanTxbarAr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Add Request 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar5(&mut self) -> McanTxbarAr5W<McanTxbarSpec> {
        McanTxbarAr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Add Request 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar6(&mut self) -> McanTxbarAr6W<McanTxbarSpec> {
        McanTxbarAr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Add Request 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar7(&mut self) -> McanTxbarAr7W<McanTxbarSpec> {
        McanTxbarAr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Add Request 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar8(&mut self) -> McanTxbarAr8W<McanTxbarSpec> {
        McanTxbarAr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Add Request 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar9(&mut self) -> McanTxbarAr9W<McanTxbarSpec> {
        McanTxbarAr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Add Request 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar10(&mut self) -> McanTxbarAr10W<McanTxbarSpec> {
        McanTxbarAr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Add Request 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar11(&mut self) -> McanTxbarAr11W<McanTxbarSpec> {
        McanTxbarAr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Add Request 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar12(&mut self) -> McanTxbarAr12W<McanTxbarSpec> {
        McanTxbarAr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Add Request 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar13(&mut self) -> McanTxbarAr13W<McanTxbarSpec> {
        McanTxbarAr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Add Request 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar14(&mut self) -> McanTxbarAr14W<McanTxbarSpec> {
        McanTxbarAr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Add Request 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar15(&mut self) -> McanTxbarAr15W<McanTxbarSpec> {
        McanTxbarAr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Add Request 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar16(&mut self) -> McanTxbarAr16W<McanTxbarSpec> {
        McanTxbarAr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Add Request 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar17(&mut self) -> McanTxbarAr17W<McanTxbarSpec> {
        McanTxbarAr17W::new(self, 17)
    }
    #[doc = "Bit 18 - Add Request 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar18(&mut self) -> McanTxbarAr18W<McanTxbarSpec> {
        McanTxbarAr18W::new(self, 18)
    }
    #[doc = "Bit 19 - Add Request 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar19(&mut self) -> McanTxbarAr19W<McanTxbarSpec> {
        McanTxbarAr19W::new(self, 19)
    }
    #[doc = "Bit 20 - Add Request 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar20(&mut self) -> McanTxbarAr20W<McanTxbarSpec> {
        McanTxbarAr20W::new(self, 20)
    }
    #[doc = "Bit 21 - Add Request 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar21(&mut self) -> McanTxbarAr21W<McanTxbarSpec> {
        McanTxbarAr21W::new(self, 21)
    }
    #[doc = "Bit 22 - Add Request 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar22(&mut self) -> McanTxbarAr22W<McanTxbarSpec> {
        McanTxbarAr22W::new(self, 22)
    }
    #[doc = "Bit 23 - Add Request 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar23(&mut self) -> McanTxbarAr23W<McanTxbarSpec> {
        McanTxbarAr23W::new(self, 23)
    }
    #[doc = "Bit 24 - Add Request 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar24(&mut self) -> McanTxbarAr24W<McanTxbarSpec> {
        McanTxbarAr24W::new(self, 24)
    }
    #[doc = "Bit 25 - Add Request 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar25(&mut self) -> McanTxbarAr25W<McanTxbarSpec> {
        McanTxbarAr25W::new(self, 25)
    }
    #[doc = "Bit 26 - Add Request 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar26(&mut self) -> McanTxbarAr26W<McanTxbarSpec> {
        McanTxbarAr26W::new(self, 26)
    }
    #[doc = "Bit 27 - Add Request 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar27(&mut self) -> McanTxbarAr27W<McanTxbarSpec> {
        McanTxbarAr27W::new(self, 27)
    }
    #[doc = "Bit 28 - Add Request 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar28(&mut self) -> McanTxbarAr28W<McanTxbarSpec> {
        McanTxbarAr28W::new(self, 28)
    }
    #[doc = "Bit 29 - Add Request 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar29(&mut self) -> McanTxbarAr29W<McanTxbarSpec> {
        McanTxbarAr29W::new(self, 29)
    }
    #[doc = "Bit 30 - Add Request 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar30(&mut self) -> McanTxbarAr30W<McanTxbarSpec> {
        McanTxbarAr30W::new(self, 30)
    }
    #[doc = "Bit 31 - Add Request 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbar_ar31(&mut self) -> McanTxbarAr31W<McanTxbarSpec> {
        McanTxbarAr31W::new(self, 31)
    }
}
#[doc = "MCAN Tx Buffer Add Request\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxbarSpec;
impl crate::RegisterSpec for McanTxbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txbar::R`](R) reader structure"]
impl crate::Readable for McanTxbarSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_txbar::W`](W) writer structure"]
impl crate::Writable for McanTxbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TXBAR to value 0"]
impl crate::Resettable for McanTxbarSpec {
    const RESET_VALUE: u32 = 0;
}

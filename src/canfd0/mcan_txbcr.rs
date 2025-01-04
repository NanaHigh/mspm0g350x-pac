#[doc = "Register `MCAN_TXBCR` reader"]
pub type R = crate::R<McanTxbcrSpec>;
#[doc = "Register `MCAN_TXBCR` writer"]
pub type W = crate::W<McanTxbcrSpec>;
#[doc = "Field `MCAN_TXBCR_CR0` reader - Cancellation Request 0. Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0 No cancellation pending 1 Cancellation pending Qualified Write is possible only with CCCR.CCE='0'"]
pub type McanTxbcrCr0R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR0` writer - Cancellation Request 0. Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0 No cancellation pending 1 Cancellation pending Qualified Write is possible only with CCCR.CCE='0'"]
pub type McanTxbcrCr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR1` reader - Cancellation Request 1. See description for bit 0."]
pub type McanTxbcrCr1R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR1` writer - Cancellation Request 1. See description for bit 0."]
pub type McanTxbcrCr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR2` reader - Cancellation Request 2. See description for bit 0."]
pub type McanTxbcrCr2R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR2` writer - Cancellation Request 2. See description for bit 0."]
pub type McanTxbcrCr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR3` reader - Cancellation Request 3. See description for bit 0."]
pub type McanTxbcrCr3R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR3` writer - Cancellation Request 3. See description for bit 0."]
pub type McanTxbcrCr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR4` reader - Cancellation Request 4. See description for bit 0."]
pub type McanTxbcrCr4R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR4` writer - Cancellation Request 4. See description for bit 0."]
pub type McanTxbcrCr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR5` reader - Cancellation Request 5. See description for bit 0."]
pub type McanTxbcrCr5R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR5` writer - Cancellation Request 5. See description for bit 0."]
pub type McanTxbcrCr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR6` reader - Cancellation Request 6. See description for bit 0."]
pub type McanTxbcrCr6R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR6` writer - Cancellation Request 6. See description for bit 0."]
pub type McanTxbcrCr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR7` reader - Cancellation Request 7. See description for bit 0."]
pub type McanTxbcrCr7R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR7` writer - Cancellation Request 7. See description for bit 0."]
pub type McanTxbcrCr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR8` reader - Cancellation Request 8. See description for bit 0."]
pub type McanTxbcrCr8R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR8` writer - Cancellation Request 8. See description for bit 0."]
pub type McanTxbcrCr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR9` reader - Cancellation Request 9. See description for bit 0."]
pub type McanTxbcrCr9R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR9` writer - Cancellation Request 9. See description for bit 0."]
pub type McanTxbcrCr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR10` reader - Cancellation Request 10. See description for bit 0."]
pub type McanTxbcrCr10R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR10` writer - Cancellation Request 10. See description for bit 0."]
pub type McanTxbcrCr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR11` reader - Cancellation Request 11. See description for bit 0."]
pub type McanTxbcrCr11R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR11` writer - Cancellation Request 11. See description for bit 0."]
pub type McanTxbcrCr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR12` reader - Cancellation Request 12. See description for bit 0."]
pub type McanTxbcrCr12R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR12` writer - Cancellation Request 12. See description for bit 0."]
pub type McanTxbcrCr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR13` reader - Cancellation Request 13. See description for bit 0."]
pub type McanTxbcrCr13R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR13` writer - Cancellation Request 13. See description for bit 0."]
pub type McanTxbcrCr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR14` reader - Cancellation Request 14. See description for bit 0."]
pub type McanTxbcrCr14R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR14` writer - Cancellation Request 14. See description for bit 0."]
pub type McanTxbcrCr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR15` reader - Cancellation Request 15. See description for bit 0."]
pub type McanTxbcrCr15R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR15` writer - Cancellation Request 15. See description for bit 0."]
pub type McanTxbcrCr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR16` reader - Cancellation Request 16. See description for bit 0."]
pub type McanTxbcrCr16R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR16` writer - Cancellation Request 16. See description for bit 0."]
pub type McanTxbcrCr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR17` reader - Cancellation Request 17. See description for bit 0."]
pub type McanTxbcrCr17R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR17` writer - Cancellation Request 17. See description for bit 0."]
pub type McanTxbcrCr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR18` reader - Cancellation Request 18. See description for bit 0."]
pub type McanTxbcrCr18R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR18` writer - Cancellation Request 18. See description for bit 0."]
pub type McanTxbcrCr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR19` reader - Cancellation Request 19. See description for bit 0."]
pub type McanTxbcrCr19R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR19` writer - Cancellation Request 19. See description for bit 0."]
pub type McanTxbcrCr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR20` reader - Cancellation Request 20. See description for bit 0."]
pub type McanTxbcrCr20R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR20` writer - Cancellation Request 20. See description for bit 0."]
pub type McanTxbcrCr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR21` reader - Cancellation Request 21. See description for bit 0."]
pub type McanTxbcrCr21R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR21` writer - Cancellation Request 21. See description for bit 0."]
pub type McanTxbcrCr21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR22` reader - Cancellation Request 22. See description for bit 0."]
pub type McanTxbcrCr22R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR22` writer - Cancellation Request 22. See description for bit 0."]
pub type McanTxbcrCr22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR23` reader - Cancellation Request 23. See description for bit 0."]
pub type McanTxbcrCr23R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR23` writer - Cancellation Request 23. See description for bit 0."]
pub type McanTxbcrCr23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR24` reader - Cancellation Request 24. See description for bit 0."]
pub type McanTxbcrCr24R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR24` writer - Cancellation Request 24. See description for bit 0."]
pub type McanTxbcrCr24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR25` reader - Cancellation Request 25. See description for bit 0."]
pub type McanTxbcrCr25R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR25` writer - Cancellation Request 25. See description for bit 0."]
pub type McanTxbcrCr25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR26` reader - Cancellation Request 26. See description for bit 0."]
pub type McanTxbcrCr26R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR26` writer - Cancellation Request 26. See description for bit 0."]
pub type McanTxbcrCr26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR27` reader - Cancellation Request 27. See description for bit 0."]
pub type McanTxbcrCr27R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR27` writer - Cancellation Request 27. See description for bit 0."]
pub type McanTxbcrCr27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR28` reader - Cancellation Request 28. See description for bit 0."]
pub type McanTxbcrCr28R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR28` writer - Cancellation Request 28. See description for bit 0."]
pub type McanTxbcrCr28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR29` reader - Cancellation Request 29. See description for bit 0."]
pub type McanTxbcrCr29R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR29` writer - Cancellation Request 29. See description for bit 0."]
pub type McanTxbcrCr29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR30` reader - Cancellation Request 30. See description for bit 0."]
pub type McanTxbcrCr30R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR30` writer - Cancellation Request 30. See description for bit 0."]
pub type McanTxbcrCr30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TXBCR_CR31` reader - Cancellation Request 31. See description for bit 0."]
pub type McanTxbcrCr31R = crate::BitReader;
#[doc = "Field `MCAN_TXBCR_CR31` writer - Cancellation Request 31. See description for bit 0."]
pub type McanTxbcrCr31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cancellation Request 0. Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0 No cancellation pending 1 Cancellation pending Qualified Write is possible only with CCCR.CCE='0'"]
    #[inline(always)]
    pub fn mcan_txbcr_cr0(&self) -> McanTxbcrCr0R {
        McanTxbcrCr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancellation Request 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr1(&self) -> McanTxbcrCr1R {
        McanTxbcrCr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cancellation Request 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr2(&self) -> McanTxbcrCr2R {
        McanTxbcrCr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cancellation Request 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr3(&self) -> McanTxbcrCr3R {
        McanTxbcrCr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cancellation Request 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr4(&self) -> McanTxbcrCr4R {
        McanTxbcrCr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cancellation Request 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr5(&self) -> McanTxbcrCr5R {
        McanTxbcrCr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cancellation Request 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr6(&self) -> McanTxbcrCr6R {
        McanTxbcrCr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cancellation Request 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr7(&self) -> McanTxbcrCr7R {
        McanTxbcrCr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cancellation Request 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr8(&self) -> McanTxbcrCr8R {
        McanTxbcrCr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cancellation Request 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr9(&self) -> McanTxbcrCr9R {
        McanTxbcrCr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cancellation Request 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr10(&self) -> McanTxbcrCr10R {
        McanTxbcrCr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cancellation Request 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr11(&self) -> McanTxbcrCr11R {
        McanTxbcrCr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cancellation Request 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr12(&self) -> McanTxbcrCr12R {
        McanTxbcrCr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cancellation Request 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr13(&self) -> McanTxbcrCr13R {
        McanTxbcrCr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cancellation Request 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr14(&self) -> McanTxbcrCr14R {
        McanTxbcrCr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cancellation Request 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr15(&self) -> McanTxbcrCr15R {
        McanTxbcrCr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Cancellation Request 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr16(&self) -> McanTxbcrCr16R {
        McanTxbcrCr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancellation Request 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr17(&self) -> McanTxbcrCr17R {
        McanTxbcrCr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Cancellation Request 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr18(&self) -> McanTxbcrCr18R {
        McanTxbcrCr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Cancellation Request 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr19(&self) -> McanTxbcrCr19R {
        McanTxbcrCr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cancellation Request 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr20(&self) -> McanTxbcrCr20R {
        McanTxbcrCr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cancellation Request 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr21(&self) -> McanTxbcrCr21R {
        McanTxbcrCr21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cancellation Request 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr22(&self) -> McanTxbcrCr22R {
        McanTxbcrCr22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cancellation Request 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr23(&self) -> McanTxbcrCr23R {
        McanTxbcrCr23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cancellation Request 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr24(&self) -> McanTxbcrCr24R {
        McanTxbcrCr24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cancellation Request 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr25(&self) -> McanTxbcrCr25R {
        McanTxbcrCr25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Cancellation Request 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr26(&self) -> McanTxbcrCr26R {
        McanTxbcrCr26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Cancellation Request 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr27(&self) -> McanTxbcrCr27R {
        McanTxbcrCr27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cancellation Request 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr28(&self) -> McanTxbcrCr28R {
        McanTxbcrCr28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cancellation Request 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr29(&self) -> McanTxbcrCr29R {
        McanTxbcrCr29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Cancellation Request 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr30(&self) -> McanTxbcrCr30R {
        McanTxbcrCr30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cancellation Request 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr31(&self) -> McanTxbcrCr31R {
        McanTxbcrCr31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cancellation Request 0. Each Tx Buffer has its own Cancellation Request bit. Writing a '1' will set the corresponding Cancellation Request bit; writing a '0' has no impact. This enables the Host to set cancellation requests for multiple Tx Buffers with one write to TXBCR. TXBCR bits are set only for those Tx Buffers configured via TXBC. The bits remain set until the corresponding bit of TXBRP is reset. 0 No cancellation pending 1 Cancellation pending Qualified Write is possible only with CCCR.CCE='0'"]
    #[inline(always)]
    pub fn mcan_txbcr_cr0(&mut self) -> McanTxbcrCr0W<McanTxbcrSpec> {
        McanTxbcrCr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Cancellation Request 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr1(&mut self) -> McanTxbcrCr1W<McanTxbcrSpec> {
        McanTxbcrCr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Cancellation Request 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr2(&mut self) -> McanTxbcrCr2W<McanTxbcrSpec> {
        McanTxbcrCr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Cancellation Request 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr3(&mut self) -> McanTxbcrCr3W<McanTxbcrSpec> {
        McanTxbcrCr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Cancellation Request 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr4(&mut self) -> McanTxbcrCr4W<McanTxbcrSpec> {
        McanTxbcrCr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Cancellation Request 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr5(&mut self) -> McanTxbcrCr5W<McanTxbcrSpec> {
        McanTxbcrCr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Cancellation Request 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr6(&mut self) -> McanTxbcrCr6W<McanTxbcrSpec> {
        McanTxbcrCr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Cancellation Request 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr7(&mut self) -> McanTxbcrCr7W<McanTxbcrSpec> {
        McanTxbcrCr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Cancellation Request 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr8(&mut self) -> McanTxbcrCr8W<McanTxbcrSpec> {
        McanTxbcrCr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Cancellation Request 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr9(&mut self) -> McanTxbcrCr9W<McanTxbcrSpec> {
        McanTxbcrCr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Cancellation Request 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr10(&mut self) -> McanTxbcrCr10W<McanTxbcrSpec> {
        McanTxbcrCr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Cancellation Request 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr11(&mut self) -> McanTxbcrCr11W<McanTxbcrSpec> {
        McanTxbcrCr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Cancellation Request 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr12(&mut self) -> McanTxbcrCr12W<McanTxbcrSpec> {
        McanTxbcrCr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Cancellation Request 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr13(&mut self) -> McanTxbcrCr13W<McanTxbcrSpec> {
        McanTxbcrCr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Cancellation Request 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr14(&mut self) -> McanTxbcrCr14W<McanTxbcrSpec> {
        McanTxbcrCr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Cancellation Request 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr15(&mut self) -> McanTxbcrCr15W<McanTxbcrSpec> {
        McanTxbcrCr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Cancellation Request 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr16(&mut self) -> McanTxbcrCr16W<McanTxbcrSpec> {
        McanTxbcrCr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Cancellation Request 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr17(&mut self) -> McanTxbcrCr17W<McanTxbcrSpec> {
        McanTxbcrCr17W::new(self, 17)
    }
    #[doc = "Bit 18 - Cancellation Request 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr18(&mut self) -> McanTxbcrCr18W<McanTxbcrSpec> {
        McanTxbcrCr18W::new(self, 18)
    }
    #[doc = "Bit 19 - Cancellation Request 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr19(&mut self) -> McanTxbcrCr19W<McanTxbcrSpec> {
        McanTxbcrCr19W::new(self, 19)
    }
    #[doc = "Bit 20 - Cancellation Request 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr20(&mut self) -> McanTxbcrCr20W<McanTxbcrSpec> {
        McanTxbcrCr20W::new(self, 20)
    }
    #[doc = "Bit 21 - Cancellation Request 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr21(&mut self) -> McanTxbcrCr21W<McanTxbcrSpec> {
        McanTxbcrCr21W::new(self, 21)
    }
    #[doc = "Bit 22 - Cancellation Request 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr22(&mut self) -> McanTxbcrCr22W<McanTxbcrSpec> {
        McanTxbcrCr22W::new(self, 22)
    }
    #[doc = "Bit 23 - Cancellation Request 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr23(&mut self) -> McanTxbcrCr23W<McanTxbcrSpec> {
        McanTxbcrCr23W::new(self, 23)
    }
    #[doc = "Bit 24 - Cancellation Request 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr24(&mut self) -> McanTxbcrCr24W<McanTxbcrSpec> {
        McanTxbcrCr24W::new(self, 24)
    }
    #[doc = "Bit 25 - Cancellation Request 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr25(&mut self) -> McanTxbcrCr25W<McanTxbcrSpec> {
        McanTxbcrCr25W::new(self, 25)
    }
    #[doc = "Bit 26 - Cancellation Request 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr26(&mut self) -> McanTxbcrCr26W<McanTxbcrSpec> {
        McanTxbcrCr26W::new(self, 26)
    }
    #[doc = "Bit 27 - Cancellation Request 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr27(&mut self) -> McanTxbcrCr27W<McanTxbcrSpec> {
        McanTxbcrCr27W::new(self, 27)
    }
    #[doc = "Bit 28 - Cancellation Request 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr28(&mut self) -> McanTxbcrCr28W<McanTxbcrSpec> {
        McanTxbcrCr28W::new(self, 28)
    }
    #[doc = "Bit 29 - Cancellation Request 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr29(&mut self) -> McanTxbcrCr29W<McanTxbcrSpec> {
        McanTxbcrCr29W::new(self, 29)
    }
    #[doc = "Bit 30 - Cancellation Request 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr30(&mut self) -> McanTxbcrCr30W<McanTxbcrSpec> {
        McanTxbcrCr30W::new(self, 30)
    }
    #[doc = "Bit 31 - Cancellation Request 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcr_cr31(&mut self) -> McanTxbcrCr31W<McanTxbcrSpec> {
        McanTxbcrCr31W::new(self, 31)
    }
}
#[doc = "MCAN Tx Buffer Cancellation Request\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_txbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxbcrSpec;
impl crate::RegisterSpec for McanTxbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txbcr::R`](R) reader structure"]
impl crate::Readable for McanTxbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_txbcr::W`](W) writer structure"]
impl crate::Writable for McanTxbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TXBCR to value 0"]
impl crate::Resettable for McanTxbcrSpec {
    const RESET_VALUE: u32 = 0;
}

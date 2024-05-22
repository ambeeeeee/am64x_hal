#[doc = "Register `CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_SET_REG` reader"]
pub type R = crate::R<CpswNussVbuspTxGOflowThreshSetRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_SET_REG` writer"]
pub type W = crate::W<CpswNussVbuspTxGOflowThreshSetRegSpec>;
#[doc = "Field `PRI0` reader - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
pub type Pri0R = crate::FieldReader;
#[doc = "Field `PRI0` writer - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
pub type Pri0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI1` reader - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
pub type Pri1R = crate::FieldReader;
#[doc = "Field `PRI1` writer - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
pub type Pri1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI2` reader - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
pub type Pri2R = crate::FieldReader;
#[doc = "Field `PRI2` writer - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
pub type Pri2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI3` reader - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
pub type Pri3R = crate::FieldReader;
#[doc = "Field `PRI3` writer - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
pub type Pri3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI4` reader - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
pub type Pri4R = crate::FieldReader;
#[doc = "Field `PRI4` writer - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
pub type Pri4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI5` reader - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
pub type Pri5R = crate::FieldReader;
#[doc = "Field `PRI5` writer - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
pub type Pri5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI6` reader - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
pub type Pri6R = crate::FieldReader;
#[doc = "Field `PRI6` writer - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
pub type Pri6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRI7` reader - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
pub type Pri7R = crate::FieldReader;
#[doc = "Field `PRI7` writer - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
pub type Pri7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
    #[inline(always)]
    pub fn pri0(&self) -> Pri0R {
        Pri0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
    #[inline(always)]
    pub fn pri1(&self) -> Pri1R {
        Pri1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
    #[inline(always)]
    pub fn pri2(&self) -> Pri2R {
        Pri2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
    #[inline(always)]
    pub fn pri3(&self) -> Pri3R {
        Pri3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
    #[inline(always)]
    pub fn pri4(&self) -> Pri4R {
        Pri4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
    #[inline(always)]
    pub fn pri5(&self) -> Pri5R {
        Pri5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
    #[inline(always)]
    pub fn pri6(&self) -> Pri6R {
        Pri6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
    #[inline(always)]
    pub fn pri7(&self) -> Pri7R {
        Pri7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 0"]
    #[inline(always)]
    #[must_use]
    pub fn pri0(&mut self) -> Pri0W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 1"]
    #[inline(always)]
    #[must_use]
    pub fn pri1(&mut self) -> Pri1W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 2"]
    #[inline(always)]
    #[must_use]
    pub fn pri2(&mut self) -> Pri2W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 3"]
    #[inline(always)]
    #[must_use]
    pub fn pri3(&mut self) -> Pri3W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 4"]
    #[inline(always)]
    #[must_use]
    pub fn pri4(&mut self) -> Pri4W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 5"]
    #[inline(always)]
    #[must_use]
    pub fn pri5(&mut self) -> Pri5W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 6"]
    #[inline(always)]
    #[must_use]
    pub fn pri6(&mut self) -> Pri6W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Priority Based Flow Control Global Outflow Usage Threshold for Pri 7"]
    #[inline(always)]
    #[must_use]
    pub fn pri7(&mut self) -> Pri7W<CpswNussVbuspTxGOflowThreshSetRegSpec> {
        Pri7W::new(self, 28)
    }
}
#[doc = "CPSW PFC Tx Global Out Flow Threshold Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspTxGOflowThreshSetRegSpec;
impl crate::RegisterSpec for CpswNussVbuspTxGOflowThreshSetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspTxGOflowThreshSetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_tx_g_oflow_thresh_set_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspTxGOflowThreshSetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_TX_G_OFLOW_THRESH_SET_REG to value 0x1555_5555"]
impl crate::Resettable for CpswNussVbuspTxGOflowThreshSetRegSpec {
    const RESET_VALUE: u32 = 0x1555_5555;
}

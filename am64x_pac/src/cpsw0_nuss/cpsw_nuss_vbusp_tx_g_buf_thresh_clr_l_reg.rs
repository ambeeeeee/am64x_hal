#[doc = "Register `CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_L_REG` reader"]
pub type R = crate::R<CpswNussVbuspTxGBufThreshClrLRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_L_REG` writer"]
pub type W = crate::W<CpswNussVbuspTxGBufThreshClrLRegSpec>;
#[doc = "Field `PRI0` reader - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
pub type Pri0R = crate::FieldReader;
#[doc = "Field `PRI0` writer - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
pub type Pri0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI1` reader - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
pub type Pri1R = crate::FieldReader;
#[doc = "Field `PRI1` writer - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
pub type Pri1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI2` reader - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
pub type Pri2R = crate::FieldReader;
#[doc = "Field `PRI2` writer - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
pub type Pri2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI3` reader - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
pub type Pri3R = crate::FieldReader;
#[doc = "Field `PRI3` writer - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
pub type Pri3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
    #[inline(always)]
    pub fn pri0(&self) -> Pri0R {
        Pri0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
    #[inline(always)]
    pub fn pri1(&self) -> Pri1R {
        Pri1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
    #[inline(always)]
    pub fn pri2(&self) -> Pri2R {
        Pri2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
    #[inline(always)]
    pub fn pri3(&self) -> Pri3R {
        Pri3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 0"]
    #[inline(always)]
    #[must_use]
    pub fn pri0(&mut self) -> Pri0W<CpswNussVbuspTxGBufThreshClrLRegSpec> {
        Pri0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 1"]
    #[inline(always)]
    #[must_use]
    pub fn pri1(&mut self) -> Pri1W<CpswNussVbuspTxGBufThreshClrLRegSpec> {
        Pri1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 2"]
    #[inline(always)]
    #[must_use]
    pub fn pri2(&mut self) -> Pri2W<CpswNussVbuspTxGBufThreshClrLRegSpec> {
        Pri2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 3"]
    #[inline(always)]
    #[must_use]
    pub fn pri3(&mut self) -> Pri3W<CpswNussVbuspTxGBufThreshClrLRegSpec> {
        Pri3W::new(self, 24)
    }
}
#[doc = "CPSW PFC Global Tx Buffer Threshold Clear Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspTxGBufThreshClrLRegSpec;
impl crate::RegisterSpec for CpswNussVbuspTxGBufThreshClrLRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspTxGBufThreshClrLRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_tx_g_buf_thresh_clr_l_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspTxGBufThreshClrLRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_CLR_L_REG to value 0"]
impl crate::Resettable for CpswNussVbuspTxGBufThreshClrLRegSpec {
    const RESET_VALUE: u32 = 0;
}

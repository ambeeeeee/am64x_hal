#[doc = "Register `CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_H_REG` reader"]
pub type R = crate::R<CpswNussVbuspTxGBufThreshSetHRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_H_REG` writer"]
pub type W = crate::W<CpswNussVbuspTxGBufThreshSetHRegSpec>;
#[doc = "Field `PRI4` reader - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 4"]
pub type Pri4R = crate::FieldReader;
#[doc = "Field `PRI4` writer - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 4"]
pub type Pri4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI5` reader - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 5"]
pub type Pri5R = crate::FieldReader;
#[doc = "Field `PRI5` writer - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 5"]
pub type Pri5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI6` reader - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 6"]
pub type Pri6R = crate::FieldReader;
#[doc = "Field `PRI6` writer - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 6"]
pub type Pri6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI7` reader - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 7"]
pub type Pri7R = crate::FieldReader;
#[doc = "Field `PRI7` writer - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 7"]
pub type Pri7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 4"]
    #[inline(always)]
    pub fn pri4(&self) -> Pri4R {
        Pri4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 5"]
    #[inline(always)]
    pub fn pri5(&self) -> Pri5R {
        Pri5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 6"]
    #[inline(always)]
    pub fn pri6(&self) -> Pri6R {
        Pri6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 7"]
    #[inline(always)]
    pub fn pri7(&self) -> Pri7R {
        Pri7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 4"]
    #[inline(always)]
    #[must_use]
    pub fn pri4(&mut self) -> Pri4W<CpswNussVbuspTxGBufThreshSetHRegSpec> {
        Pri4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 5"]
    #[inline(always)]
    #[must_use]
    pub fn pri5(&mut self) -> Pri5W<CpswNussVbuspTxGBufThreshSetHRegSpec> {
        Pri5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 6"]
    #[inline(always)]
    #[must_use]
    pub fn pri6(&mut self) -> Pri6W<CpswNussVbuspTxGBufThreshSetHRegSpec> {
        Pri6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority Based Flow Control Global Buffer Usage Threshold for Priority 7"]
    #[inline(always)]
    #[must_use]
    pub fn pri7(&mut self) -> Pri7W<CpswNussVbuspTxGBufThreshSetHRegSpec> {
        Pri7W::new(self, 24)
    }
}
#[doc = "CPSW PFC Global Tx Buffer Threshold Set High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspTxGBufThreshSetHRegSpec;
impl crate::RegisterSpec for CpswNussVbuspTxGBufThreshSetHRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspTxGBufThreshSetHRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_tx_g_buf_thresh_set_h_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspTxGBufThreshSetHRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_TX_G_BUF_THRESH_SET_H_REG to value 0x0257_5755"]
impl crate::Resettable for CpswNussVbuspTxGBufThreshSetHRegSpec {
    const RESET_VALUE: u32 = 0x0257_5755;
}

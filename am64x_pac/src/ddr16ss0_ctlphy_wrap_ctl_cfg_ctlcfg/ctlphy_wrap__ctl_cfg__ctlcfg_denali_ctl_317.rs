#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_317` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_317` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec>;
#[doc = "Field `ROW_DIFF_0` reader - 2:0\\]
Difference between number of address pins available and number being used."]
pub type RowDiff0R = crate::FieldReader;
#[doc = "Field `ROW_DIFF_0` writer - 2:0\\]
Difference between number of address pins available and number being used."]
pub type RowDiff0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ROW_DIFF_1` reader - 10:8\\]
Difference between number of address pins available and number being used."]
pub type RowDiff1R = crate::FieldReader;
#[doc = "Field `ROW_DIFF_1` writer - 10:8\\]
Difference between number of address pins available and number being used."]
pub type RowDiff1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COL_DIFF_0` reader - 19:16\\]
Difference between number of column pins available and number being used."]
pub type ColDiff0R = crate::FieldReader;
#[doc = "Field `COL_DIFF_0` writer - 19:16\\]
Difference between number of column pins available and number being used."]
pub type ColDiff0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COL_DIFF_1` reader - 27:24\\]
Difference between number of column pins available and number being used."]
pub type ColDiff1R = crate::FieldReader;
#[doc = "Field `COL_DIFF_1` writer - 27:24\\]
Difference between number of column pins available and number being used."]
pub type ColDiff1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Difference between number of address pins available and number being used."]
    #[inline(always)]
    pub fn row_diff_0(&self) -> RowDiff0R {
        RowDiff0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Difference between number of address pins available and number being used."]
    #[inline(always)]
    pub fn row_diff_1(&self) -> RowDiff1R {
        RowDiff1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Difference between number of column pins available and number being used."]
    #[inline(always)]
    pub fn col_diff_0(&self) -> ColDiff0R {
        ColDiff0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Difference between number of column pins available and number being used."]
    #[inline(always)]
    pub fn col_diff_1(&self) -> ColDiff1R {
        ColDiff1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Difference between number of address pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn row_diff_0(&mut self) -> RowDiff0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec> {
        RowDiff0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Difference between number of address pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn row_diff_1(&mut self) -> RowDiff1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec> {
        RowDiff1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Difference between number of column pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn col_diff_0(&mut self) -> ColDiff0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec> {
        ColDiff0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Difference between number of column pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn col_diff_1(&mut self) -> ColDiff1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec> {
        ColDiff1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_317\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_317::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_317::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_317::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_317::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_317 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl317Spec {
    const RESET_VALUE: u32 = 0;
}

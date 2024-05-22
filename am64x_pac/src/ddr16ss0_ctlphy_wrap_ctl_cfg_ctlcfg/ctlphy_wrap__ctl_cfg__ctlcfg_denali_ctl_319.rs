#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_319` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_319` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec>;
#[doc = "Field `ROW_START_VAL_0` reader - 1:0\\]
Row start value for chip select 0."]
pub type RowStartVal0R = crate::FieldReader;
#[doc = "Field `ROW_START_VAL_0` writer - 1:0\\]
Row start value for chip select 0."]
pub type RowStartVal0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_MSK_0` reader - 23:8\\]
Mask applied to the address decode for chip select 0."]
pub type CsMsk0R = crate::FieldReader<u16>;
#[doc = "Field `CS_MSK_0` writer - 23:8\\]
Mask applied to the address decode for chip select 0."]
pub type CsMsk0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Row start value for chip select 0."]
    #[inline(always)]
    pub fn row_start_val_0(&self) -> RowStartVal0R {
        RowStartVal0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Mask applied to the address decode for chip select 0."]
    #[inline(always)]
    pub fn cs_msk_0(&self) -> CsMsk0R {
        CsMsk0R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Row start value for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn row_start_val_0(&mut self) -> RowStartVal0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec> {
        RowStartVal0W::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Mask applied to the address decode for chip select 0."]
    #[inline(always)]
    #[must_use]
    pub fn cs_msk_0(&mut self) -> CsMsk0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec> {
        CsMsk0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_319\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_319::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_319::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_319::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_319::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_319 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl319Spec {
    const RESET_VALUE: u32 = 0;
}

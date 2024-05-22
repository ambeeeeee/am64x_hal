#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_321` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_321` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec>;
#[doc = "Field `ROW_START_VAL_1` reader - 1:0\\]
Row start value for chip select 1."]
pub type RowStartVal1R = crate::FieldReader;
#[doc = "Field `ROW_START_VAL_1` writer - 1:0\\]
Row start value for chip select 1."]
pub type RowStartVal1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CS_MSK_1` reader - 23:8\\]
Mask applied to the address decode for chip select 1."]
pub type CsMsk1R = crate::FieldReader<u16>;
#[doc = "Field `CS_MSK_1` writer - 23:8\\]
Mask applied to the address decode for chip select 1."]
pub type CsMsk1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CS_MAP_NON_POW2` reader - 25:24\\]
Defines which chip selects are non-power-of-2 memory sizes."]
pub type CsMapNonPow2R = crate::FieldReader;
#[doc = "Field `CS_MAP_NON_POW2` writer - 25:24\\]
Defines which chip selects are non-power-of-2 memory sizes."]
pub type CsMapNonPow2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Row start value for chip select 1."]
    #[inline(always)]
    pub fn row_start_val_1(&self) -> RowStartVal1R {
        RowStartVal1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Mask applied to the address decode for chip select 1."]
    #[inline(always)]
    pub fn cs_msk_1(&self) -> CsMsk1R {
        CsMsk1R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines which chip selects are non-power-of-2 memory sizes."]
    #[inline(always)]
    pub fn cs_map_non_pow2(&self) -> CsMapNonPow2R {
        CsMapNonPow2R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Row start value for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn row_start_val_1(&mut self) -> RowStartVal1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec> {
        RowStartVal1W::new(self, 0)
    }
    #[doc = "Bits 8:23 - 23:8\\]
Mask applied to the address decode for chip select 1."]
    #[inline(always)]
    #[must_use]
    pub fn cs_msk_1(&mut self) -> CsMsk1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec> {
        CsMsk1W::new(self, 8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Defines which chip selects are non-power-of-2 memory sizes."]
    #[inline(always)]
    #[must_use]
    pub fn cs_map_non_pow2(&mut self) -> CsMapNonPow2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec> {
        CsMapNonPow2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_321\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_321::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_321::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_321::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_321::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_321 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl321Spec {
    const RESET_VALUE: u32 = 0;
}

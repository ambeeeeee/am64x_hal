#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_277` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_277` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec>;
#[doc = "Field `MR_FSP_DATA_VALID_F1` reader - 0:0\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=1"]
pub type MrFspDataValidF1R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F1` writer - 0:0\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=1"]
pub type MrFspDataValidF1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR_FSP_DATA_VALID_F2` reader - 8:8\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=2"]
pub type MrFspDataValidF2R = crate::BitReader;
#[doc = "Field `MR_FSP_DATA_VALID_F2` writer - 8:8\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=2"]
pub type MrFspDataValidF2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_FSP_INSYNC_ACTIVE` reader - 16:16\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
pub type DfsFspInsyncActiveR = crate::BitReader;
#[doc = "Field `DFS_FSP_INSYNC_ACTIVE` writer - 16:16\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
pub type DfsFspInsyncActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFS_FSP_INSYNC_INACTIVE` reader - 24:24\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
pub type DfsFspInsyncInactiveR = crate::BitReader;
#[doc = "Field `DFS_FSP_INSYNC_INACTIVE` writer - 24:24\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
pub type DfsFspInsyncInactiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=1"]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f1(&self) -> MrFspDataValidF1R {
        MrFspDataValidF1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=2"]
    #[inline(always)]
    pub fn mr_fsp_data_valid_f2(&self) -> MrFspDataValidF2R {
        MrFspDataValidF2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
    #[inline(always)]
    pub fn dfs_fsp_insync_active(&self) -> DfsFspInsyncActiveR {
        DfsFspInsyncActiveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
    #[inline(always)]
    pub fn dfs_fsp_insync_inactive(&self) -> DfsFspInsyncInactiveR {
        DfsFspInsyncInactiveR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f1(
        &mut self,
    ) -> MrFspDataValidF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec> {
        MrFspDataValidF1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Indicates that, at this frequency, memory was trained and the associated data has been loaded into the MRx_DATA parameter\\[s\\]. Value of 1 means memory was trained. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn mr_fsp_data_valid_f2(
        &mut self,
    ) -> MrFspDataValidF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec> {
        MrFspDataValidF2W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dfs_fsp_insync_active(
        &mut self,
    ) -> DfsFspInsyncActiveW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec> {
        DfsFspInsyncActiveW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
When cleared, this indicates that the contents in memcd_param is new that the values in memory's MRx FSP reg and if a dfs occurs, they need to be updated. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dfs_fsp_insync_inactive(
        &mut self,
    ) -> DfsFspInsyncInactiveW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec> {
        DfsFspInsyncInactiveW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_277\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_277::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_277::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_277::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_277::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_277 to value 0x0001_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl277Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}

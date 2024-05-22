#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_20` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_20` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec>;
#[doc = "Field `DFI_CMD_RATIO` reader - 0:0\\]
Indicates the controller clock to DFI PHY clock ratio for the DFI command interface. For LPDDR5, this is always a 1:1 ratio. For all other memory classes, this will be the same as the dfi_freq_ratio. Zero specifies a 1:1 ratio and one specifies a 1:2 ratio. READ-ONLY"]
pub type DfiCmdRatioR = crate::BitReader;
#[doc = "Field `DFI_CMD_RATIO` writer - 0:0\\]
Indicates the controller clock to DFI PHY clock ratio for the DFI command interface. For LPDDR5, this is always a 1:1 ratio. For all other memory classes, this will be the same as the dfi_freq_ratio. Zero specifies a 1:1 ratio and one specifies a 1:2 ratio. READ-ONLY"]
pub type DfiCmdRatioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_MRW_INIT` reader - 8:8\\]
Disable MRW commands during initialization. Set to 1 to disable."]
pub type NoMrwInitR = crate::BitReader;
#[doc = "Field `NO_MRW_INIT` writer - 8:8\\]
Disable MRW commands during initialization. Set to 1 to disable."]
pub type NoMrwInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT_VALUE` reader - 17:16\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
pub type OdtValueR = crate::FieldReader;
#[doc = "Field `ODT_VALUE` writer - 17:16\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
pub type OdtValueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_INDEP_TRAIN_MODE` reader - 24:24\\]
Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
pub type PhyIndepTrainModeR = crate::BitReader;
#[doc = "Field `PHY_INDEP_TRAIN_MODE` writer - 24:24\\]
Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
pub type PhyIndepTrainModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates the controller clock to DFI PHY clock ratio for the DFI command interface. For LPDDR5, this is always a 1:1 ratio. For all other memory classes, this will be the same as the dfi_freq_ratio. Zero specifies a 1:1 ratio and one specifies a 1:2 ratio. READ-ONLY"]
    #[inline(always)]
    pub fn dfi_cmd_ratio(&self) -> DfiCmdRatioR {
        DfiCmdRatioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable MRW commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_mrw_init(&self) -> NoMrwInitR {
        NoMrwInitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
    #[inline(always)]
    pub fn odt_value(&self) -> OdtValueR {
        OdtValueR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
    #[inline(always)]
    pub fn phy_indep_train_mode(&self) -> PhyIndepTrainModeR {
        PhyIndepTrainModeR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates the controller clock to DFI PHY clock ratio for the DFI command interface. For LPDDR5, this is always a 1:1 ratio. For all other memory classes, this will be the same as the dfi_freq_ratio. Zero specifies a 1:1 ratio and one specifies a 1:2 ratio. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn dfi_cmd_ratio(&mut self) -> DfiCmdRatioW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec> {
        DfiCmdRatioW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Disable MRW commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_mrw_init(&mut self) -> NoMrwInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec> {
        NoMrwInitW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
    #[inline(always)]
    #[must_use]
    pub fn odt_value(&mut self) -> OdtValueW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec> {
        OdtValueW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable PHY independent training mode commands during initialization. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_indep_train_mode(
        &mut self,
    ) -> PhyIndepTrainModeW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec> {
        PhyIndepTrainModeW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_20::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_20::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_20 to value 0x01"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl20Spec {
    const RESET_VALUE: u32 = 0x01;
}

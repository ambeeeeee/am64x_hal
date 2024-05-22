#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1352` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1352` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec>;
#[doc = "Field `PHY_PAD_ATB_CTRL` reader - 15:0\\]
Pad ATB control settings. Bit \\[0\\]
is the enable signal. Bits \\[5:1\\]
are the ATB data signals. Bits \\[15:8\\]
are the 1 hot select for which pad is selected."]
pub type PhyPadAtbCtrlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_ATB_CTRL` writer - 15:0\\]
Pad ATB control settings. Bit \\[0\\]
is the enable signal. Bits \\[5:1\\]
are the ATB data signals. Bits \\[15:8\\]
are the 1 hot select for which pad is selected."]
pub type PhyPadAtbCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_ADRCTL_MANUAL_UPDATE` reader - 16:16\\]
Address/control manual update of slave delay lines. Set to 1 to update. WRITE-ONLY"]
pub type PhyAdrctlManualUpdateR = crate::BitReader;
#[doc = "Field `PHY_ADRCTL_MANUAL_UPDATE` writer - 16:16\\]
Address/control manual update of slave delay lines. Set to 1 to update. WRITE-ONLY"]
pub type PhyAdrctlManualUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_AC_LPBK_ERR_CLEAR` reader - 24:24\\]
Address/control loopback error clear. Set to 1 to clear error. WRITE-ONLY"]
pub type PhyAcLpbkErrClearR = crate::BitReader;
#[doc = "Field `PHY_AC_LPBK_ERR_CLEAR` writer - 24:24\\]
Address/control loopback error clear. Set to 1 to clear error. WRITE-ONLY"]
pub type PhyAcLpbkErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Pad ATB control settings. Bit \\[0\\]
is the enable signal. Bits \\[5:1\\]
are the ATB data signals. Bits \\[15:8\\]
are the 1 hot select for which pad is selected."]
    #[inline(always)]
    pub fn phy_pad_atb_ctrl(&self) -> PhyPadAtbCtrlR {
        PhyPadAtbCtrlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Address/control manual update of slave delay lines. Set to 1 to update. WRITE-ONLY"]
    #[inline(always)]
    pub fn phy_adrctl_manual_update(&self) -> PhyAdrctlManualUpdateR {
        PhyAdrctlManualUpdateR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Address/control loopback error clear. Set to 1 to clear error. WRITE-ONLY"]
    #[inline(always)]
    pub fn phy_ac_lpbk_err_clear(&self) -> PhyAcLpbkErrClearR {
        PhyAcLpbkErrClearR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Pad ATB control settings. Bit \\[0\\]
is the enable signal. Bits \\[5:1\\]
are the ATB data signals. Bits \\[15:8\\]
are the 1 hot select for which pad is selected."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_atb_ctrl(
        &mut self,
    ) -> PhyPadAtbCtrlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec> {
        PhyPadAtbCtrlW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Address/control manual update of slave delay lines. Set to 1 to update. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_manual_update(
        &mut self,
    ) -> PhyAdrctlManualUpdateW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec> {
        PhyAdrctlManualUpdateW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Address/control loopback error clear. Set to 1 to clear error. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_err_clear(
        &mut self,
    ) -> PhyAcLpbkErrClearW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec> {
        PhyAcLpbkErrClearW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1352\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1352::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1352::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1352::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1352::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1352 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1352Spec {
    const RESET_VALUE: u32 = 0;
}

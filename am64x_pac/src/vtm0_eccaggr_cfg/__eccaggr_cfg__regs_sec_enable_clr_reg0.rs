#[doc = "Register `__ECCAGGR_CFG__REGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<_EccaggrCfg_RegsSecEnableClrReg0Spec>;
#[doc = "Register `__ECCAGGR_CFG__REGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<_EccaggrCfg_RegsSecEnableClrReg0Spec>;
#[doc = "Field `ECCAGG_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for eccagg_pend"]
pub type EccaggEnableClrR = crate::BitReader;
#[doc = "Field `ECCAGG_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for eccagg_pend"]
pub type EccaggEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `K3VTM_N16FFC_MMR_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_mmr_edc_ctrl_busecc_pend"]
pub type K3vtmN16ffcMmrEdcCtrlBuseccEnableClrR = crate::BitReader;
#[doc = "Field `K3VTM_N16FFC_MMR_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_mmr_edc_ctrl_busecc_pend"]
pub type K3vtmN16ffcMmrEdcCtrlBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `K3VTM_N16FFC_CFG_CBASS_VBUSP_P2P_BRIDGE_VBUSP_BRIDGE_BUSECC_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_vbusp_p2p_bridge_vbusp_bridge_busecc_pend"]
pub type K3vtmN16ffcCfgCbassVbuspP2pBridgeVbuspBridgeBuseccEnableClrR = crate::BitReader;
#[doc = "Field `K3VTM_N16FFC_CFG_CBASS_VBUSP_P2P_BRIDGE_VBUSP_BRIDGE_BUSECC_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_vbusp_p2p_bridge_vbusp_bridge_busecc_pend"]
pub type K3vtmN16ffcCfgCbassVbuspP2pBridgeVbuspBridgeBuseccEnableClrW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `K3VTM_N16FFC_CFG_CBASS_CFG_SCR_SCR_K3VTM_N16FFC_CFG_CBASS_CFG_SCR_SCR_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 3:3\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_edc_ctrl_busecc_pend"]
pub type K3vtmN16ffcCfgCbassCfgScrScrK3vtmN16ffcCfgCbassCfgScrScrEdcCtrlBuseccEnableClrR =
    crate::BitReader;
#[doc = "Field `K3VTM_N16FFC_CFG_CBASS_CFG_SCR_SCR_K3VTM_N16FFC_CFG_CBASS_CFG_SCR_SCR_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 3:3\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_edc_ctrl_busecc_pend"]
pub type K3vtmN16ffcCfgCbassCfgScrScrK3vtmN16ffcCfgCbassCfgScrScrEdcCtrlBuseccEnableClrW<'a, REG> =
    crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for eccagg_pend"]
    #[inline(always)]
    pub fn eccagg_enable_clr(&self) -> EccaggEnableClrR {
        EccaggEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_mmr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn k3vtm_n16ffc_mmr_edc_ctrl_busecc_enable_clr(
        &self,
    ) -> K3vtmN16ffcMmrEdcCtrlBuseccEnableClrR {
        K3vtmN16ffcMmrEdcCtrlBuseccEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_vbusp_p2p_bridge_vbusp_bridge_busecc_pend"]
    #[inline(always)]
    pub fn k3vtm_n16ffc_cfg_cbass_vbusp_p2p_bridge_vbusp_bridge_busecc_enable_clr(
        &self,
    ) -> K3vtmN16ffcCfgCbassVbuspP2pBridgeVbuspBridgeBuseccEnableClrR {
        K3vtmN16ffcCfgCbassVbuspP2pBridgeVbuspBridgeBuseccEnableClrR::new(
            ((self.bits >> 2) & 1) != 0,
        )
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_edc_ctrl_busecc_enable_clr(
        &self,
    ) -> K3vtmN16ffcCfgCbassCfgScrScrK3vtmN16ffcCfgCbassCfgScrScrEdcCtrlBuseccEnableClrR {
        K3vtmN16ffcCfgCbassCfgScrScrK3vtmN16ffcCfgCbassCfgScrScrEdcCtrlBuseccEnableClrR::new(
            ((self.bits >> 3) & 1) != 0,
        )
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for eccagg_pend"]
    #[inline(always)]
    #[must_use]
    pub fn eccagg_enable_clr(&mut self) -> EccaggEnableClrW<_EccaggrCfg_RegsSecEnableClrReg0Spec> {
        EccaggEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_mmr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn k3vtm_n16ffc_mmr_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> K3vtmN16ffcMmrEdcCtrlBuseccEnableClrW<_EccaggrCfg_RegsSecEnableClrReg0Spec> {
        K3vtmN16ffcMmrEdcCtrlBuseccEnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_vbusp_p2p_bridge_vbusp_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn k3vtm_n16ffc_cfg_cbass_vbusp_p2p_bridge_vbusp_bridge_busecc_enable_clr(
        &mut self,
    ) -> K3vtmN16ffcCfgCbassVbuspP2pBridgeVbuspBridgeBuseccEnableClrW<
        _EccaggrCfg_RegsSecEnableClrReg0Spec,
    > {
        K3vtmN16ffcCfgCbassVbuspP2pBridgeVbuspBridgeBuseccEnableClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_k3vtm_n16ffc_cfg_cbass_cfg_scr_scr_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> K3vtmN16ffcCfgCbassCfgScrScrK3vtmN16ffcCfgCbassCfgScrScrEdcCtrlBuseccEnableClrW<
        _EccaggrCfg_RegsSecEnableClrReg0Spec,
    > {
        K3vtmN16ffcCfgCbassCfgScrScrK3vtmN16ffcCfgCbassCfgScrScrEdcCtrlBuseccEnableClrW::new(
            self, 3,
        )
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`__eccaggr_cfg__regs_sec_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`__eccaggr_cfg__regs_sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _EccaggrCfg_RegsSecEnableClrReg0Spec;
impl crate::RegisterSpec for _EccaggrCfg_RegsSecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`__eccaggr_cfg__regs_sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for _EccaggrCfg_RegsSecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`__eccaggr_cfg__regs_sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for _EccaggrCfg_RegsSecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets __ECCAGGR_CFG__REGS_sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for _EccaggrCfg_RegsSecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}

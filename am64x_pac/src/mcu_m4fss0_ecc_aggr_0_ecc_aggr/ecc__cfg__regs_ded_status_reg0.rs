#[doc = "Register `ECC__CFG__REGS_ded_status_reg0` reader"]
pub type R = crate::R<Ecc_Cfg_RegsDedStatusReg0Spec>;
#[doc = "Register `ECC__CFG__REGS_ded_status_reg0` writer"]
pub type W = crate::W<Ecc_Cfg_RegsDedStatusReg0Spec>;
#[doc = "Field `IRAM_RAMECC_PEND` reader - 0:0\\]
Interrupt Pending Status for iram_ramecc_pend"]
pub type IramRameccPendR = crate::BitReader;
#[doc = "Field `IRAM_RAMECC_PEND` writer - 0:0\\]
Interrupt Pending Status for iram_ramecc_pend"]
pub type IramRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_RAMECC_PEND` reader - 1:1\\]
Interrupt Pending Status for dram_ramecc_pend"]
pub type DramRameccPendR = crate::BitReader;
#[doc = "Field `DRAM_RAMECC_PEND` writer - 1:1\\]
Interrupt Pending Status for dram_ramecc_pend"]
pub type DramRameccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRAM_BUSECC_PEND` reader - 2:2\\]
Interrupt Pending Status for iram_busecc_pend"]
pub type IramBuseccPendR = crate::BitReader;
#[doc = "Field `IRAM_BUSECC_PEND` writer - 2:2\\]
Interrupt Pending Status for iram_busecc_pend"]
pub type IramBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_BUSECC_PEND` reader - 3:3\\]
Interrupt Pending Status for dram_busecc_pend"]
pub type DramBuseccPendR = crate::BitReader;
#[doc = "Field `DRAM_BUSECC_PEND` writer - 3:3\\]
Interrupt Pending Status for dram_busecc_pend"]
pub type DramBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_RAT_EDC_CTRL_BUSECC_PEND` reader - 4:4\\]
Interrupt Pending Status for blazar_rat_edc_ctrl_busecc_pend"]
pub type BlazarRatEdcCtrlBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_RAT_EDC_CTRL_BUSECC_PEND` writer - 4:4\\]
Interrupt Pending Status for blazar_rat_edc_ctrl_busecc_pend"]
pub type BlazarRatEdcCtrlBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_VBUSP_S_P2P_BRIDGE_VBUSP_S_BRIDGE_BUSECC_PEND` reader - 5:5\\]
Interrupt Pending Status for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
pub type BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_VBUSP_S_P2P_BRIDGE_VBUSP_S_BRIDGE_BUSECC_PEND` writer - 5:5\\]
Interrupt Pending Status for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
pub type BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_IECC_S_P2P_BRIDGE_IECC_S_BRIDGE_BUSECC_PEND` reader - 6:6\\]
Interrupt Pending Status for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
pub type BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_IECC_S_P2P_BRIDGE_IECC_S_BRIDGE_BUSECC_PEND` writer - 6:6\\]
Interrupt Pending Status for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
pub type BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_BLAZAR_SCR_SCR_BLAZAR_CBASS_BLAZAR_SCR_SCR_EDC_CTRL_BUSECC_PEND` reader - 7:7\\]
Interrupt Pending Status for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
pub type BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_BLAZAR_SCR_SCR_BLAZAR_CBASS_BLAZAR_SCR_SCR_EDC_CTRL_BUSECC_PEND` writer - 7:7\\]
Interrupt Pending Status for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
pub type BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccPendW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_PEND` reader - 8:8\\]
Interrupt Pending Status for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_PEND` writer - 8:8\\]
Interrupt Pending Status for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_SYS_SCR_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_PEND` reader - 9:9\\]
Interrupt Pending Status for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_SYS_SCR_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_PEND` writer - 9:9\\]
Interrupt Pending Status for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_I_EDC_CTRL_BUSECC_PEND` reader - 10:10\\]
Interrupt Pending Status for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
pub type BlazarIa2vIEdcCtrlBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_I_EDC_CTRL_BUSECC_PEND` writer - 10:10\\]
Interrupt Pending Status for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
pub type BlazarIa2vIEdcCtrlBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_D_EDC_CTRL_BUSECC_PEND` reader - 11:11\\]
Interrupt Pending Status for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
pub type BlazarIa2vDEdcCtrlBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_D_EDC_CTRL_BUSECC_PEND` writer - 11:11\\]
Interrupt Pending Status for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
pub type BlazarIa2vDEdcCtrlBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_S_EDC_CTRL_BUSECC_PEND` reader - 12:12\\]
Interrupt Pending Status for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
pub type BlazarIa2vSEdcCtrlBuseccPendR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_S_EDC_CTRL_BUSECC_PEND` writer - 12:12\\]
Interrupt Pending Status for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
pub type BlazarIa2vSEdcCtrlBuseccPendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCAGG_PEND` reader - 13:13\\]
Interrupt Pending Status for eccagg_pend"]
pub type EccaggPendR = crate::BitReader;
#[doc = "Field `ECCAGG_PEND` writer - 13:13\\]
Interrupt Pending Status for eccagg_pend"]
pub type EccaggPendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for iram_ramecc_pend"]
    #[inline(always)]
    pub fn iram_ramecc_pend(&self) -> IramRameccPendR {
        IramRameccPendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for dram_ramecc_pend"]
    #[inline(always)]
    pub fn dram_ramecc_pend(&self) -> DramRameccPendR {
        DramRameccPendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for iram_busecc_pend"]
    #[inline(always)]
    pub fn iram_busecc_pend(&self) -> IramBuseccPendR {
        IramBuseccPendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for dram_busecc_pend"]
    #[inline(always)]
    pub fn dram_busecc_pend(&self) -> DramBuseccPendR {
        DramBuseccPendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for blazar_rat_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_rat_edc_ctrl_busecc_pend(&self) -> BlazarRatEdcCtrlBuseccPendR {
        BlazarRatEdcCtrlBuseccPendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend(
        &self,
    ) -> BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccPendR {
        BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccPendR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_iecc_s_p2p_bridge_iecc_s_bridge_busecc_pend(
        &self,
    ) -> BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccPendR {
        BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccPendR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend(
        &self,
    ) -> BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccPendR {
        BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccPendR::new(
            ((self.bits >> 7) & 1) != 0,
        )
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend(
        &self,
    ) -> BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccPendR {
        BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccPendR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    pub fn blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend(
        &self,
    ) -> BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccPendR {
        BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccPendR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_i_edc_ctrl_busecc_pend(&self) -> BlazarIa2vIEdcCtrlBuseccPendR {
        BlazarIa2vIEdcCtrlBuseccPendR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_d_edc_ctrl_busecc_pend(&self) -> BlazarIa2vDEdcCtrlBuseccPendR {
        BlazarIa2vDEdcCtrlBuseccPendR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_s_edc_ctrl_busecc_pend(&self) -> BlazarIa2vSEdcCtrlBuseccPendR {
        BlazarIa2vSEdcCtrlBuseccPendR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for eccagg_pend"]
    #[inline(always)]
    pub fn eccagg_pend(&self) -> EccaggPendR {
        EccaggPendR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Pending Status for iram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn iram_ramecc_pend(&mut self) -> IramRameccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        IramRameccPendW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Pending Status for dram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn dram_ramecc_pend(&mut self) -> DramRameccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        DramRameccPendW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Pending Status for iram_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn iram_busecc_pend(&mut self) -> IramBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        IramBuseccPendW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Pending Status for dram_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn dram_busecc_pend(&mut self) -> DramBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        DramBuseccPendW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Pending Status for blazar_rat_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_rat_edc_ctrl_busecc_pend(
        &mut self,
    ) -> BlazarRatEdcCtrlBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarRatEdcCtrlBuseccPendW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Pending Status for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend(
        &mut self,
    ) -> BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccPendW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Pending Status for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_iecc_s_p2p_bridge_iecc_s_bridge_busecc_pend(
        &mut self,
    ) -> BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccPendW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Pending Status for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend(
        &mut self,
    ) -> BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccPendW<
        Ecc_Cfg_RegsDedStatusReg0Spec,
    > {
        BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccPendW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Pending Status for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend(
        &mut self,
    ) -> BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccPendW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Pending Status for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend(
        &mut self,
    ) -> BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccPendW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Pending Status for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_i_edc_ctrl_busecc_pend(
        &mut self,
    ) -> BlazarIa2vIEdcCtrlBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarIa2vIEdcCtrlBuseccPendW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Pending Status for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_d_edc_ctrl_busecc_pend(
        &mut self,
    ) -> BlazarIa2vDEdcCtrlBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarIa2vDEdcCtrlBuseccPendW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Pending Status for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_s_edc_ctrl_busecc_pend(
        &mut self,
    ) -> BlazarIa2vSEdcCtrlBuseccPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        BlazarIa2vSEdcCtrlBuseccPendW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Pending Status for eccagg_pend"]
    #[inline(always)]
    #[must_use]
    pub fn eccagg_pend(&mut self) -> EccaggPendW<Ecc_Cfg_RegsDedStatusReg0Spec> {
        EccaggPendW::new(self, 13)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_ded_status_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_ded_status_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecc_Cfg_RegsDedStatusReg0Spec;
impl crate::RegisterSpec for Ecc_Cfg_RegsDedStatusReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc__cfg__regs_ded_status_reg0::R`](R) reader structure"]
impl crate::Readable for Ecc_Cfg_RegsDedStatusReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc__cfg__regs_ded_status_reg0::W`](W) writer structure"]
impl crate::Writable for Ecc_Cfg_RegsDedStatusReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC__CFG__REGS_ded_status_reg0 to value 0"]
impl crate::Resettable for Ecc_Cfg_RegsDedStatusReg0Spec {
    const RESET_VALUE: u32 = 0;
}

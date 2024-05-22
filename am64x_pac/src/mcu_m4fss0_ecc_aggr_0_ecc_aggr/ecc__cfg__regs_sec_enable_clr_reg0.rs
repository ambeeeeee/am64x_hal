#[doc = "Register `ECC__CFG__REGS_sec_enable_clr_reg0` reader"]
pub type R = crate::R<Ecc_Cfg_RegsSecEnableClrReg0Spec>;
#[doc = "Register `ECC__CFG__REGS_sec_enable_clr_reg0` writer"]
pub type W = crate::W<Ecc_Cfg_RegsSecEnableClrReg0Spec>;
#[doc = "Field `IRAM_RAMECC_ENABLE_CLR` reader - 0:0\\]
Interrupt Enable Clear Register for iram_ramecc_pend"]
pub type IramRameccEnableClrR = crate::BitReader;
#[doc = "Field `IRAM_RAMECC_ENABLE_CLR` writer - 0:0\\]
Interrupt Enable Clear Register for iram_ramecc_pend"]
pub type IramRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_RAMECC_ENABLE_CLR` reader - 1:1\\]
Interrupt Enable Clear Register for dram_ramecc_pend"]
pub type DramRameccEnableClrR = crate::BitReader;
#[doc = "Field `DRAM_RAMECC_ENABLE_CLR` writer - 1:1\\]
Interrupt Enable Clear Register for dram_ramecc_pend"]
pub type DramRameccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRAM_BUSECC_ENABLE_CLR` reader - 2:2\\]
Interrupt Enable Clear Register for iram_busecc_pend"]
pub type IramBuseccEnableClrR = crate::BitReader;
#[doc = "Field `IRAM_BUSECC_ENABLE_CLR` writer - 2:2\\]
Interrupt Enable Clear Register for iram_busecc_pend"]
pub type IramBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_BUSECC_ENABLE_CLR` reader - 3:3\\]
Interrupt Enable Clear Register for dram_busecc_pend"]
pub type DramBuseccEnableClrR = crate::BitReader;
#[doc = "Field `DRAM_BUSECC_ENABLE_CLR` writer - 3:3\\]
Interrupt Enable Clear Register for dram_busecc_pend"]
pub type DramBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_RAT_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 4:4\\]
Interrupt Enable Clear Register for blazar_rat_edc_ctrl_busecc_pend"]
pub type BlazarRatEdcCtrlBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_RAT_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 4:4\\]
Interrupt Enable Clear Register for blazar_rat_edc_ctrl_busecc_pend"]
pub type BlazarRatEdcCtrlBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_VBUSP_S_P2P_BRIDGE_VBUSP_S_BRIDGE_BUSECC_ENABLE_CLR` reader - 5:5\\]
Interrupt Enable Clear Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
pub type BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_VBUSP_S_P2P_BRIDGE_VBUSP_S_BRIDGE_BUSECC_ENABLE_CLR` writer - 5:5\\]
Interrupt Enable Clear Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
pub type BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableClrW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_IECC_S_P2P_BRIDGE_IECC_S_BRIDGE_BUSECC_ENABLE_CLR` reader - 6:6\\]
Interrupt Enable Clear Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
pub type BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_IECC_S_P2P_BRIDGE_IECC_S_BRIDGE_BUSECC_ENABLE_CLR` writer - 6:6\\]
Interrupt Enable Clear Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
pub type BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_BLAZAR_SCR_SCR_BLAZAR_CBASS_BLAZAR_SCR_SCR_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 7:7\\]
Interrupt Enable Clear Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
pub type BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_BLAZAR_SCR_SCR_BLAZAR_CBASS_BLAZAR_SCR_SCR_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 7:7\\]
Interrupt Enable Clear Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
pub type BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableClrW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_CLR` reader - 8:8\\]
Interrupt Enable Clear Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_CLR` writer - 8:8\\]
Interrupt Enable Clear Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_SYS_SCR_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_CLR` reader - 9:9\\]
Interrupt Enable Clear Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_SYS_SCR_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_CLR` writer - 9:9\\]
Interrupt Enable Clear Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableClrW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_I_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 10:10\\]
Interrupt Enable Clear Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
pub type BlazarIa2vIEdcCtrlBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_I_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 10:10\\]
Interrupt Enable Clear Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
pub type BlazarIa2vIEdcCtrlBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_D_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 11:11\\]
Interrupt Enable Clear Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
pub type BlazarIa2vDEdcCtrlBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_D_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 11:11\\]
Interrupt Enable Clear Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
pub type BlazarIa2vDEdcCtrlBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_S_EDC_CTRL_BUSECC_ENABLE_CLR` reader - 12:12\\]
Interrupt Enable Clear Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
pub type BlazarIa2vSEdcCtrlBuseccEnableClrR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_S_EDC_CTRL_BUSECC_ENABLE_CLR` writer - 12:12\\]
Interrupt Enable Clear Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
pub type BlazarIa2vSEdcCtrlBuseccEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCAGG_ENABLE_CLR` reader - 13:13\\]
Interrupt Enable Clear Register for eccagg_pend"]
pub type EccaggEnableClrR = crate::BitReader;
#[doc = "Field `ECCAGG_ENABLE_CLR` writer - 13:13\\]
Interrupt Enable Clear Register for eccagg_pend"]
pub type EccaggEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for iram_ramecc_pend"]
    #[inline(always)]
    pub fn iram_ramecc_enable_clr(&self) -> IramRameccEnableClrR {
        IramRameccEnableClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for dram_ramecc_pend"]
    #[inline(always)]
    pub fn dram_ramecc_enable_clr(&self) -> DramRameccEnableClrR {
        DramRameccEnableClrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for iram_busecc_pend"]
    #[inline(always)]
    pub fn iram_busecc_enable_clr(&self) -> IramBuseccEnableClrR {
        IramBuseccEnableClrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for dram_busecc_pend"]
    #[inline(always)]
    pub fn dram_busecc_enable_clr(&self) -> DramBuseccEnableClrR {
        DramBuseccEnableClrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for blazar_rat_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_rat_edc_ctrl_busecc_enable_clr(&self) -> BlazarRatEdcCtrlBuseccEnableClrR {
        BlazarRatEdcCtrlBuseccEnableClrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_enable_clr(
        &self,
    ) -> BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableClrR {
        BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableClrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_iecc_s_p2p_bridge_iecc_s_bridge_busecc_enable_clr(
        &self,
    ) -> BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableClrR {
        BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableClrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_enable_clr(
        &self,
    ) -> BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableClrR {
        BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableClrR::new(
            ((self.bits >> 7) & 1) != 0,
        )
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_clr(
        &self,
    ) -> BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableClrR {
        BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableClrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Clear Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    pub fn blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_clr(
        &self,
    ) -> BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableClrR {
        BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableClrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Clear Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_i_edc_ctrl_busecc_enable_clr(&self) -> BlazarIa2vIEdcCtrlBuseccEnableClrR {
        BlazarIa2vIEdcCtrlBuseccEnableClrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Clear Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_d_edc_ctrl_busecc_enable_clr(&self) -> BlazarIa2vDEdcCtrlBuseccEnableClrR {
        BlazarIa2vDEdcCtrlBuseccEnableClrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Clear Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_s_edc_ctrl_busecc_enable_clr(&self) -> BlazarIa2vSEdcCtrlBuseccEnableClrR {
        BlazarIa2vSEdcCtrlBuseccEnableClrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Clear Register for eccagg_pend"]
    #[inline(always)]
    pub fn eccagg_enable_clr(&self) -> EccaggEnableClrR {
        EccaggEnableClrR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Clear Register for iram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn iram_ramecc_enable_clr(
        &mut self,
    ) -> IramRameccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        IramRameccEnableClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Clear Register for dram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn dram_ramecc_enable_clr(
        &mut self,
    ) -> DramRameccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        DramRameccEnableClrW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Clear Register for iram_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn iram_busecc_enable_clr(
        &mut self,
    ) -> IramBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        IramBuseccEnableClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Clear Register for dram_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn dram_busecc_enable_clr(
        &mut self,
    ) -> DramBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        DramBuseccEnableClrW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Clear Register for blazar_rat_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_rat_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> BlazarRatEdcCtrlBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        BlazarRatEdcCtrlBuseccEnableClrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Clear Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_enable_clr(
        &mut self,
    ) -> BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec>
    {
        BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableClrW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Clear Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_iecc_s_p2p_bridge_iecc_s_bridge_busecc_enable_clr(
        &mut self,
    ) -> BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec>
    {
        BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Clear Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableClrW<
        Ecc_Cfg_RegsSecEnableClrReg0Spec,
    > {
        BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableClrW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Clear Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_clr(
        &mut self,
    ) -> BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec>
    {
        BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableClrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Clear Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_clr(
        &mut self,
    ) -> BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec>
    {
        BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableClrW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Clear Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_i_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> BlazarIa2vIEdcCtrlBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        BlazarIa2vIEdcCtrlBuseccEnableClrW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Clear Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_d_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> BlazarIa2vDEdcCtrlBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        BlazarIa2vDEdcCtrlBuseccEnableClrW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Clear Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_s_edc_ctrl_busecc_enable_clr(
        &mut self,
    ) -> BlazarIa2vSEdcCtrlBuseccEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        BlazarIa2vSEdcCtrlBuseccEnableClrW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Clear Register for eccagg_pend"]
    #[inline(always)]
    #[must_use]
    pub fn eccagg_enable_clr(&mut self) -> EccaggEnableClrW<Ecc_Cfg_RegsSecEnableClrReg0Spec> {
        EccaggEnableClrW::new(self, 13)
    }
}
#[doc = "Interrupt Enable Clear Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_sec_enable_clr_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_sec_enable_clr_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecc_Cfg_RegsSecEnableClrReg0Spec;
impl crate::RegisterSpec for Ecc_Cfg_RegsSecEnableClrReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc__cfg__regs_sec_enable_clr_reg0::R`](R) reader structure"]
impl crate::Readable for Ecc_Cfg_RegsSecEnableClrReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc__cfg__regs_sec_enable_clr_reg0::W`](W) writer structure"]
impl crate::Writable for Ecc_Cfg_RegsSecEnableClrReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC__CFG__REGS_sec_enable_clr_reg0 to value 0"]
impl crate::Resettable for Ecc_Cfg_RegsSecEnableClrReg0Spec {
    const RESET_VALUE: u32 = 0;
}

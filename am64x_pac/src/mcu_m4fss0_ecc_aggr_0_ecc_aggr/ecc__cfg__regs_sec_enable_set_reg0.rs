#[doc = "Register `ECC__CFG__REGS_sec_enable_set_reg0` reader"]
pub type R = crate::R<Ecc_Cfg_RegsSecEnableSetReg0Spec>;
#[doc = "Register `ECC__CFG__REGS_sec_enable_set_reg0` writer"]
pub type W = crate::W<Ecc_Cfg_RegsSecEnableSetReg0Spec>;
#[doc = "Field `IRAM_RAMECC_ENABLE_SET` reader - 0:0\\]
Interrupt Enable Set Register for iram_ramecc_pend"]
pub type IramRameccEnableSetR = crate::BitReader;
#[doc = "Field `IRAM_RAMECC_ENABLE_SET` writer - 0:0\\]
Interrupt Enable Set Register for iram_ramecc_pend"]
pub type IramRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_RAMECC_ENABLE_SET` reader - 1:1\\]
Interrupt Enable Set Register for dram_ramecc_pend"]
pub type DramRameccEnableSetR = crate::BitReader;
#[doc = "Field `DRAM_RAMECC_ENABLE_SET` writer - 1:1\\]
Interrupt Enable Set Register for dram_ramecc_pend"]
pub type DramRameccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRAM_BUSECC_ENABLE_SET` reader - 2:2\\]
Interrupt Enable Set Register for iram_busecc_pend"]
pub type IramBuseccEnableSetR = crate::BitReader;
#[doc = "Field `IRAM_BUSECC_ENABLE_SET` writer - 2:2\\]
Interrupt Enable Set Register for iram_busecc_pend"]
pub type IramBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_BUSECC_ENABLE_SET` reader - 3:3\\]
Interrupt Enable Set Register for dram_busecc_pend"]
pub type DramBuseccEnableSetR = crate::BitReader;
#[doc = "Field `DRAM_BUSECC_ENABLE_SET` writer - 3:3\\]
Interrupt Enable Set Register for dram_busecc_pend"]
pub type DramBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_RAT_EDC_CTRL_BUSECC_ENABLE_SET` reader - 4:4\\]
Interrupt Enable Set Register for blazar_rat_edc_ctrl_busecc_pend"]
pub type BlazarRatEdcCtrlBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_RAT_EDC_CTRL_BUSECC_ENABLE_SET` writer - 4:4\\]
Interrupt Enable Set Register for blazar_rat_edc_ctrl_busecc_pend"]
pub type BlazarRatEdcCtrlBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_VBUSP_S_P2P_BRIDGE_VBUSP_S_BRIDGE_BUSECC_ENABLE_SET` reader - 5:5\\]
Interrupt Enable Set Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
pub type BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_VBUSP_S_P2P_BRIDGE_VBUSP_S_BRIDGE_BUSECC_ENABLE_SET` writer - 5:5\\]
Interrupt Enable Set Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
pub type BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableSetW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_IECC_S_P2P_BRIDGE_IECC_S_BRIDGE_BUSECC_ENABLE_SET` reader - 6:6\\]
Interrupt Enable Set Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
pub type BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_IECC_S_P2P_BRIDGE_IECC_S_BRIDGE_BUSECC_ENABLE_SET` writer - 6:6\\]
Interrupt Enable Set Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
pub type BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_BLAZAR_SCR_SCR_BLAZAR_CBASS_BLAZAR_SCR_SCR_EDC_CTRL_BUSECC_ENABLE_SET` reader - 7:7\\]
Interrupt Enable Set Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
pub type BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_BLAZAR_SCR_SCR_BLAZAR_CBASS_BLAZAR_SCR_SCR_EDC_CTRL_BUSECC_ENABLE_SET` writer - 7:7\\]
Interrupt Enable Set Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
pub type BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableSetW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_CBASS_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_SET` reader - 8:8\\]
Interrupt Enable Set Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_CBASS_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_SET` writer - 8:8\\]
Interrupt Enable Set Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_SYS_SCR_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_SET` reader - 9:9\\]
Interrupt Enable Set Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_SYS_SCR_VBUS_CLK_EDC_CTRL_CBASS_INT_VBUS_BUSECC_ENABLE_SET` writer - 9:9\\]
Interrupt Enable Set Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
pub type BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableSetW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_I_EDC_CTRL_BUSECC_ENABLE_SET` reader - 10:10\\]
Interrupt Enable Set Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
pub type BlazarIa2vIEdcCtrlBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_I_EDC_CTRL_BUSECC_ENABLE_SET` writer - 10:10\\]
Interrupt Enable Set Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
pub type BlazarIa2vIEdcCtrlBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_D_EDC_CTRL_BUSECC_ENABLE_SET` reader - 11:11\\]
Interrupt Enable Set Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
pub type BlazarIa2vDEdcCtrlBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_D_EDC_CTRL_BUSECC_ENABLE_SET` writer - 11:11\\]
Interrupt Enable Set Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
pub type BlazarIa2vDEdcCtrlBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLAZAR_IA2V_S_EDC_CTRL_BUSECC_ENABLE_SET` reader - 12:12\\]
Interrupt Enable Set Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
pub type BlazarIa2vSEdcCtrlBuseccEnableSetR = crate::BitReader;
#[doc = "Field `BLAZAR_IA2V_S_EDC_CTRL_BUSECC_ENABLE_SET` writer - 12:12\\]
Interrupt Enable Set Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
pub type BlazarIa2vSEdcCtrlBuseccEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCAGG_ENABLE_SET` reader - 13:13\\]
Interrupt Enable Set Register for eccagg_pend"]
pub type EccaggEnableSetR = crate::BitReader;
#[doc = "Field `ECCAGG_ENABLE_SET` writer - 13:13\\]
Interrupt Enable Set Register for eccagg_pend"]
pub type EccaggEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for iram_ramecc_pend"]
    #[inline(always)]
    pub fn iram_ramecc_enable_set(&self) -> IramRameccEnableSetR {
        IramRameccEnableSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for dram_ramecc_pend"]
    #[inline(always)]
    pub fn dram_ramecc_enable_set(&self) -> DramRameccEnableSetR {
        DramRameccEnableSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for iram_busecc_pend"]
    #[inline(always)]
    pub fn iram_busecc_enable_set(&self) -> IramBuseccEnableSetR {
        IramBuseccEnableSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for dram_busecc_pend"]
    #[inline(always)]
    pub fn dram_busecc_enable_set(&self) -> DramBuseccEnableSetR {
        DramBuseccEnableSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for blazar_rat_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_rat_edc_ctrl_busecc_enable_set(&self) -> BlazarRatEdcCtrlBuseccEnableSetR {
        BlazarRatEdcCtrlBuseccEnableSetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_enable_set(
        &self,
    ) -> BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableSetR {
        BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableSetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_iecc_s_p2p_bridge_iecc_s_bridge_busecc_enable_set(
        &self,
    ) -> BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableSetR {
        BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableSetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_enable_set(
        &self,
    ) -> BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableSetR {
        BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableSetR::new(
            ((self.bits >> 7) & 1) != 0,
        )
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Set Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    pub fn blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_set(
        &self,
    ) -> BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableSetR {
        BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableSetR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Set Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    pub fn blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_set(
        &self,
    ) -> BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableSetR {
        BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableSetR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Set Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_i_edc_ctrl_busecc_enable_set(&self) -> BlazarIa2vIEdcCtrlBuseccEnableSetR {
        BlazarIa2vIEdcCtrlBuseccEnableSetR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Set Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_d_edc_ctrl_busecc_enable_set(&self) -> BlazarIa2vDEdcCtrlBuseccEnableSetR {
        BlazarIa2vDEdcCtrlBuseccEnableSetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Set Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
    #[inline(always)]
    pub fn blazar_ia2v_s_edc_ctrl_busecc_enable_set(&self) -> BlazarIa2vSEdcCtrlBuseccEnableSetR {
        BlazarIa2vSEdcCtrlBuseccEnableSetR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Set Register for eccagg_pend"]
    #[inline(always)]
    pub fn eccagg_enable_set(&self) -> EccaggEnableSetR {
        EccaggEnableSetR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for iram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn iram_ramecc_enable_set(
        &mut self,
    ) -> IramRameccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        IramRameccEnableSetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for dram_ramecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn dram_ramecc_enable_set(
        &mut self,
    ) -> DramRameccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        DramRameccEnableSetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt Enable Set Register for iram_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn iram_busecc_enable_set(
        &mut self,
    ) -> IramBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        IramBuseccEnableSetW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt Enable Set Register for dram_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn dram_busecc_enable_set(
        &mut self,
    ) -> DramBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        DramBuseccEnableSetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt Enable Set Register for blazar_rat_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_rat_edc_ctrl_busecc_enable_set(
        &mut self,
    ) -> BlazarRatEdcCtrlBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        BlazarRatEdcCtrlBuseccEnableSetW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Enable Set Register for blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_vbusp_s_p2p_bridge_vbusp_s_bridge_busecc_enable_set(
        &mut self,
    ) -> BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec>
    {
        BlazarCbassVbuspSP2pBridgeVbuspSBridgeBuseccEnableSetW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt Enable Set Register for blazar_cbass_Iecc_s_p2p_bridge_Iecc_s_bridge_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_iecc_s_p2p_bridge_iecc_s_bridge_busecc_enable_set(
        &mut self,
    ) -> BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec>
    {
        BlazarCbassIeccSP2pBridgeIeccSBridgeBuseccEnableSetW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Interrupt Enable Set Register for blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_blazar_scr_scr_blazar_cbass_blazar_scr_scr_edc_ctrl_busecc_enable_set(
        &mut self,
    ) -> BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableSetW<
        Ecc_Cfg_RegsSecEnableSetReg0Spec,
    > {
        BlazarCbassBlazarScrScrBlazarCbassBlazarScrScrEdcCtrlBuseccEnableSetW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt Enable Set Register for blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_cbass_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_set(
        &mut self,
    ) -> BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec>
    {
        BlazarCbassVbusClkEdcCtrlCbassIntVbusBuseccEnableSetW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt Enable Set Register for blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_sys_scr_vbus_clk_edc_ctrl_cbass_int_vbus_busecc_enable_set(
        &mut self,
    ) -> BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec>
    {
        BlazarSysScrVbusClkEdcCtrlCbassIntVbusBuseccEnableSetW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt Enable Set Register for blazar_Ia2v_i_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_i_edc_ctrl_busecc_enable_set(
        &mut self,
    ) -> BlazarIa2vIEdcCtrlBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        BlazarIa2vIEdcCtrlBuseccEnableSetW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt Enable Set Register for blazar_Ia2v_d_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_d_edc_ctrl_busecc_enable_set(
        &mut self,
    ) -> BlazarIa2vDEdcCtrlBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        BlazarIa2vDEdcCtrlBuseccEnableSetW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt Enable Set Register for blazar_Ia2v_s_edc_ctrl_busecc_pend"]
    #[inline(always)]
    #[must_use]
    pub fn blazar_ia2v_s_edc_ctrl_busecc_enable_set(
        &mut self,
    ) -> BlazarIa2vSEdcCtrlBuseccEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        BlazarIa2vSEdcCtrlBuseccEnableSetW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt Enable Set Register for eccagg_pend"]
    #[inline(always)]
    #[must_use]
    pub fn eccagg_enable_set(&mut self) -> EccaggEnableSetW<Ecc_Cfg_RegsSecEnableSetReg0Spec> {
        EccaggEnableSetW::new(self, 13)
    }
}
#[doc = "Interrupt Enable Set Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc__cfg__regs_sec_enable_set_reg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc__cfg__regs_sec_enable_set_reg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ecc_Cfg_RegsSecEnableSetReg0Spec;
impl crate::RegisterSpec for Ecc_Cfg_RegsSecEnableSetReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc__cfg__regs_sec_enable_set_reg0::R`](R) reader structure"]
impl crate::Readable for Ecc_Cfg_RegsSecEnableSetReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`ecc__cfg__regs_sec_enable_set_reg0::W`](W) writer structure"]
impl crate::Writable for Ecc_Cfg_RegsSecEnableSetReg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC__CFG__REGS_sec_enable_set_reg0 to value 0"]
impl crate::Resettable for Ecc_Cfg_RegsSecEnableSetReg0Spec {
    const RESET_VALUE: u32 = 0;
}

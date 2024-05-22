#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec>;
#[doc = "Field `JTAG_DATAOUT_PHY_DSLICE_PAD_RX_CTLE_SETTING` reader - 5:0\\]
Controls jtag_dataout_phy_dslice_pad_rx_ctle_setting port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPhyDslicePadRxCtleSettingR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_PHY_DSLICE_PAD_RX_CTLE_SETTING` writer - 5:0\\]
Controls jtag_dataout_phy_dslice_pad_rx_ctle_setting port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPhyDslicePadRxCtleSettingW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `JTAG_DATAOUT_PHY_RX_CAL_CODE` reader - 14:6\\]
Controls jtag_dataout_phy_rx_cal_code port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPhyRxCalCodeR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_PHY_RX_CAL_CODE` writer - 14:6\\]
Controls jtag_dataout_phy_rx_cal_code port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPhyRxCalCodeW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `JTAG_DATAOUT_VREF_CTRL_DQ` reader - 26:15\\]
Controls jtag_dataout_vref_ctrl_dq port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutVrefCtrlDqR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_VREF_CTRL_DQ` writer - 26:15\\]
Controls jtag_dataout_vref_ctrl_dq port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutVrefCtrlDqW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `JTAG_DATAOUT_ATB_EN` reader - 27:27\\]
Controls jtag_dataout_atb_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAtbEnR = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_ATB_EN` writer - 27:27\\]
Controls jtag_dataout_atb_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAtbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_PAD_DSLICE_IO_CFG0` reader - 28:28\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadDsliceIoCfg0R = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PAD_DSLICE_IO_CFG0` writer - 28:28\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadDsliceIoCfg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_PAD_DSLICE_IO_CFG2` reader - 29:29\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[2\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadDsliceIoCfg2R = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PAD_DSLICE_IO_CFG2` writer - 29:29\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[2\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadDsliceIoCfg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_PAD_ACS_IO_CFG0` reader - 30:30\\]
Controls jtag_dataout_pad_acs_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadAcsIoCfg0R = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PAD_ACS_IO_CFG0` writer - 30:30\\]
Controls jtag_dataout_pad_acs_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadAcsIoCfg0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAG_DATAOUT_PAD_ADR_IO_CFG0` reader - 31:31\\]
Controls jtag_dataout_pad_adr_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadAdrIoCfg0R = crate::BitReader;
#[doc = "Field `JTAG_DATAOUT_PAD_ADR_IO_CFG0` writer - 31:31\\]
Controls jtag_dataout_pad_adr_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadAdrIoCfg0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Controls jtag_dataout_phy_dslice_pad_rx_ctle_setting port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_phy_dslice_pad_rx_ctle_setting(
        &self,
    ) -> JtagDataoutPhyDslicePadRxCtleSettingR {
        JtagDataoutPhyDslicePadRxCtleSettingR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - 14:6\\]
Controls jtag_dataout_phy_rx_cal_code port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_phy_rx_cal_code(&self) -> JtagDataoutPhyRxCalCodeR {
        JtagDataoutPhyRxCalCodeR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 15:26 - 26:15\\]
Controls jtag_dataout_vref_ctrl_dq port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_vref_ctrl_dq(&self) -> JtagDataoutVrefCtrlDqR {
        JtagDataoutVrefCtrlDqR::new(((self.bits >> 15) & 0x0fff) as u16)
    }
    #[doc = "Bit 27 - 27:27\\]
Controls jtag_dataout_atb_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_atb_en(&self) -> JtagDataoutAtbEnR {
        JtagDataoutAtbEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_pad_dslice_io_cfg0(&self) -> JtagDataoutPadDsliceIoCfg0R {
        JtagDataoutPadDsliceIoCfg0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[2\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_pad_dslice_io_cfg2(&self) -> JtagDataoutPadDsliceIoCfg2R {
        JtagDataoutPadDsliceIoCfg2R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_pad_acs_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_pad_acs_io_cfg0(&self) -> JtagDataoutPadAcsIoCfg0R {
        JtagDataoutPadAcsIoCfg0R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_pad_adr_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_pad_adr_io_cfg0(&self) -> JtagDataoutPadAdrIoCfg0R {
        JtagDataoutPadAdrIoCfg0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Controls jtag_dataout_phy_dslice_pad_rx_ctle_setting port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_phy_dslice_pad_rx_ctle_setting(
        &mut self,
    ) -> JtagDataoutPhyDslicePadRxCtleSettingW<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutPhyDslicePadRxCtleSettingW::new(self, 0)
    }
    #[doc = "Bits 6:14 - 14:6\\]
Controls jtag_dataout_phy_rx_cal_code port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_phy_rx_cal_code(
        &mut self,
    ) -> JtagDataoutPhyRxCalCodeW<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutPhyRxCalCodeW::new(self, 6)
    }
    #[doc = "Bits 15:26 - 26:15\\]
Controls jtag_dataout_vref_ctrl_dq port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_vref_ctrl_dq(
        &mut self,
    ) -> JtagDataoutVrefCtrlDqW<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutVrefCtrlDqW::new(self, 15)
    }
    #[doc = "Bit 27 - 27:27\\]
Controls jtag_dataout_atb_en port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_atb_en(
        &mut self,
    ) -> JtagDataoutAtbEnW<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutAtbEnW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_pad_dslice_io_cfg0(
        &mut self,
    ) -> JtagDataoutPadDsliceIoCfg0W<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutPadDsliceIoCfg0W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Controls jtag_dataout_pad_dslice_io_cfg\\[2\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_pad_dslice_io_cfg2(
        &mut self,
    ) -> JtagDataoutPadDsliceIoCfg2W<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutPadDsliceIoCfg2W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Controls jtag_dataout_pad_acs_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_pad_acs_io_cfg0(
        &mut self,
    ) -> JtagDataoutPadAcsIoCfg0W<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutPadAcsIoCfg0W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls jtag_dataout_pad_adr_io_cfg\\[0\\]
port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_pad_adr_io_cfg0(
        &mut self,
    ) -> JtagDataoutPadAdrIoCfg0W<Regs_SsCfg_SscfgPhyTestCtrl2RegSpec> {
        JtagDataoutPadAdrIoCfg0W::new(self, 31)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl2RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl2_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl2_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL2_REG to value 0x2381_9580"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl2RegSpec {
    const RESET_VALUE: u32 = 0x2381_9580;
}

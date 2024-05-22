#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl9RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl9RegSpec>;
#[doc = "Field `JTAG_DATAOUT_DATA_IE` reader - 15:0\\]
Controls jtag_dataout_data_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDataIeR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_DATA_IE` writer - 15:0\\]
Controls jtag_dataout_data_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDataIeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAG_DATAOUT_DM_IE` reader - 17:16\\]
Controls jtag_dataout_dm_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDmIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_DM_IE` writer - 17:16\\]
Controls jtag_dataout_dm_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDmIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_DQS_IE` reader - 21:20\\]
Controls jtag_dataout_dqs_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDqsIeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_DQS_IE` writer - 21:20\\]
Controls jtag_dataout_dqs_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDqsIeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_data_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_data_ie(&self) -> JtagDataoutDataIeR {
        JtagDataoutDataIeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls jtag_dataout_dm_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_dm_ie(&self) -> JtagDataoutDmIeR {
        JtagDataoutDmIeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Controls jtag_dataout_dqs_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_dqs_ie(&self) -> JtagDataoutDqsIeR {
        JtagDataoutDqsIeR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_data_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_data_ie(
        &mut self,
    ) -> JtagDataoutDataIeW<Regs_SsCfg_SscfgPhyTestCtrl9RegSpec> {
        JtagDataoutDataIeW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls jtag_dataout_dm_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_dm_ie(&mut self) -> JtagDataoutDmIeW<Regs_SsCfg_SscfgPhyTestCtrl9RegSpec> {
        JtagDataoutDmIeW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Controls jtag_dataout_dqs_ie port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_dqs_ie(
        &mut self,
    ) -> JtagDataoutDqsIeW<Regs_SsCfg_SscfgPhyTestCtrl9RegSpec> {
        JtagDataoutDqsIeW::new(self, 20)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl9_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl9_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl9RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl9RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl9_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl9RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl9_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl9RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL9_REG to value 0x0037_5535"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl9RegSpec {
    const RESET_VALUE: u32 = 0x0037_5535;
}

#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl5RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl5RegSpec>;
#[doc = "Field `JTAG_DATAOUT_DATA_OE` reader - 15:0\\]
Controls jtag_dataout_data_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDataOeR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_DATA_OE` writer - 15:0\\]
Controls jtag_dataout_data_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDataOeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAG_DATAOUT_DM_OE` reader - 17:16\\]
Controls jtag_dataout_dm_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDmOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_DM_OE` writer - 17:16\\]
Controls jtag_dataout_dm_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDmOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_DQS_OE` reader - 21:20\\]
Controls jtag_dataout_dqs_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDqsOeR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_DQS_OE` writer - 21:20\\]
Controls jtag_dataout_dqs_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDqsOeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_data_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_data_oe(&self) -> JtagDataoutDataOeR {
        JtagDataoutDataOeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls jtag_dataout_dm_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_dm_oe(&self) -> JtagDataoutDmOeR {
        JtagDataoutDmOeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Controls jtag_dataout_dqs_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_dqs_oe(&self) -> JtagDataoutDqsOeR {
        JtagDataoutDqsOeR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_data_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_data_oe(
        &mut self,
    ) -> JtagDataoutDataOeW<Regs_SsCfg_SscfgPhyTestCtrl5RegSpec> {
        JtagDataoutDataOeW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls jtag_dataout_dm_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_dm_oe(&mut self) -> JtagDataoutDmOeW<Regs_SsCfg_SscfgPhyTestCtrl5RegSpec> {
        JtagDataoutDmOeW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Controls jtag_dataout_dqs_oe port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_dqs_oe(
        &mut self,
    ) -> JtagDataoutDqsOeW<Regs_SsCfg_SscfgPhyTestCtrl5RegSpec> {
        JtagDataoutDqsOeW::new(self, 20)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl5_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl5_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl5RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl5RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl5_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl5RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl5_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl5RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL5_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl5RegSpec {
    const RESET_VALUE: u32 = 0;
}

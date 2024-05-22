#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl7RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl7RegSpec>;
#[doc = "Field `JTAG_DATAOUT_DATA` reader - 15:0\\]
Controls jtag_dataout_data port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDataR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_DATA` writer - 15:0\\]
Controls jtag_dataout_data port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAG_DATAOUT_DM` reader - 17:16\\]
Controls jtag_dataout_dm port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDmR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_DM` writer - 17:16\\]
Controls jtag_dataout_dm port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAOUT_DQS` reader - 21:20\\]
Controls jtag_dataout_dqs port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDqsR = crate::FieldReader;
#[doc = "Field `JTAG_DATAOUT_DQS` writer - 21:20\\]
Controls jtag_dataout_dqs port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutDqsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_data port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_data(&self) -> JtagDataoutDataR {
        JtagDataoutDataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls jtag_dataout_dm port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_dm(&self) -> JtagDataoutDmR {
        JtagDataoutDmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Controls jtag_dataout_dqs port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_dqs(&self) -> JtagDataoutDqsR {
        JtagDataoutDqsR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_data port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_data(&mut self) -> JtagDataoutDataW<Regs_SsCfg_SscfgPhyTestCtrl7RegSpec> {
        JtagDataoutDataW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Controls jtag_dataout_dm port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_dm(&mut self) -> JtagDataoutDmW<Regs_SsCfg_SscfgPhyTestCtrl7RegSpec> {
        JtagDataoutDmW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Controls jtag_dataout_dqs port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_dqs(&mut self) -> JtagDataoutDqsW<Regs_SsCfg_SscfgPhyTestCtrl7RegSpec> {
        JtagDataoutDqsW::new(self, 20)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl7_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl7_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl7RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl7RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl7_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl7RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl7_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl7RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL7_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl7RegSpec {
    const RESET_VALUE: u32 = 0;
}

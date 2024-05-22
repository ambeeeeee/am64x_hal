#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl10RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl10RegSpec>;
#[doc = "Field `HVM_CLK_DIV` reader - 7:0\\]
Divfactor to divide ddrss_ddr_pll_clk to generate PCLK for HVM tests when ddrss_bs_mode=0 and hvm_test_en=1. 0=div by 1, 1=div by 2, and so on."]
pub type HvmClkDivR = crate::FieldReader;
#[doc = "Field `HVM_CLK_DIV` writer - 7:0\\]
Divfactor to divide ddrss_ddr_pll_clk to generate PCLK for HVM tests when ddrss_bs_mode=0 and hvm_test_en=1. 0=div by 1, 1=div by 2, and so on."]
pub type HvmClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Divfactor to divide ddrss_ddr_pll_clk to generate PCLK for HVM tests when ddrss_bs_mode=0 and hvm_test_en=1. 0=div by 1, 1=div by 2, and so on."]
    #[inline(always)]
    pub fn hvm_clk_div(&self) -> HvmClkDivR {
        HvmClkDivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Divfactor to divide ddrss_ddr_pll_clk to generate PCLK for HVM tests when ddrss_bs_mode=0 and hvm_test_en=1. 0=div by 1, 1=div by 2, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn hvm_clk_div(&mut self) -> HvmClkDivW<Regs_SsCfg_SscfgPhyTestCtrl10RegSpec> {
        HvmClkDivW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl10_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl10_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl10RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl10RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl10_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl10RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl10_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl10RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL10_REG to value 0x07"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl10RegSpec {
    const RESET_VALUE: u32 = 0x07;
}

#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestCtrl3RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestCtrl3RegSpec>;
#[doc = "Field `JTAG_DATAOUT_PAD_ATB_CTRL` reader - 15:0\\]
Controls jtag_dataout_pad_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadAtbCtrlR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_PAD_ATB_CTRL` writer - 15:0\\]
Controls jtag_dataout_pad_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutPadAtbCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAG_DATAOUT_ATB_CTRL` reader - 31:16\\]
Controls jtag_dataout_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAtbCtrlR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAOUT_ATB_CTRL` writer - 31:16\\]
Controls jtag_dataout_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
pub type JtagDataoutAtbCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_pad_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_pad_atb_ctrl(&self) -> JtagDataoutPadAtbCtrlR {
        JtagDataoutPadAtbCtrlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Controls jtag_dataout_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    pub fn jtag_dataout_atb_ctrl(&self) -> JtagDataoutAtbCtrlR {
        JtagDataoutAtbCtrlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Controls jtag_dataout_pad_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_pad_atb_ctrl(
        &mut self,
    ) -> JtagDataoutPadAtbCtrlW<Regs_SsCfg_SscfgPhyTestCtrl3RegSpec> {
        JtagDataoutPadAtbCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Controls jtag_dataout_atb_ctrl port on the PHY when ddrss_bs_mode=0 and hvm_test_en=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_dataout_atb_ctrl(
        &mut self,
    ) -> JtagDataoutAtbCtrlW<Regs_SsCfg_SscfgPhyTestCtrl3RegSpec> {
        JtagDataoutAtbCtrlW::new(self, 16)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_ctrl3_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_ctrl3_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestCtrl3RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestCtrl3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_ctrl3_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestCtrl3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_ctrl3_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestCtrl3RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_CTRL3_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestCtrl3RegSpec {
    const RESET_VALUE: u32 = 0;
}

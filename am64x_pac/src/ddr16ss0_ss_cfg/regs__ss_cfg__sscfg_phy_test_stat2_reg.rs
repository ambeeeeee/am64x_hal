#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyTestStat2RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyTestStat2RegSpec>;
#[doc = "Field `JTAG_DATAIN_DATA` reader - 15:0\\]
Displays value of jtag_datain_data port on the PHY."]
pub type JtagDatainDataR = crate::FieldReader<u16>;
#[doc = "Field `JTAG_DATAIN_DATA` writer - 15:0\\]
Displays value of jtag_datain_data port on the PHY."]
pub type JtagDatainDataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `JTAG_DATAIN_DM` reader - 17:16\\]
Displays value of jtag_datain_dm port on the PHY."]
pub type JtagDatainDmR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_DM` writer - 17:16\\]
Displays value of jtag_datain_dm port on the PHY."]
pub type JtagDatainDmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JTAG_DATAIN_DQS` reader - 21:20\\]
Displays value of jtag_datain_dqs port on the PHY."]
pub type JtagDatainDqsR = crate::FieldReader;
#[doc = "Field `JTAG_DATAIN_DQS` writer - 21:20\\]
Displays value of jtag_datain_dqs port on the PHY."]
pub type JtagDatainDqsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Displays value of jtag_datain_data port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_data(&self) -> JtagDatainDataR {
        JtagDatainDataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Displays value of jtag_datain_dm port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_dm(&self) -> JtagDatainDmR {
        JtagDatainDmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Displays value of jtag_datain_dqs port on the PHY."]
    #[inline(always)]
    pub fn jtag_datain_dqs(&self) -> JtagDatainDqsR {
        JtagDatainDqsR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Displays value of jtag_datain_data port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_data(&mut self) -> JtagDatainDataW<Regs_SsCfg_SscfgPhyTestStat2RegSpec> {
        JtagDatainDataW::new(self, 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Displays value of jtag_datain_dm port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_dm(&mut self) -> JtagDatainDmW<Regs_SsCfg_SscfgPhyTestStat2RegSpec> {
        JtagDatainDmW::new(self, 16)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Displays value of jtag_datain_dqs port on the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_datain_dqs(&mut self) -> JtagDatainDqsW<Regs_SsCfg_SscfgPhyTestStat2RegSpec> {
        JtagDatainDqsW::new(self, 20)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_test_stat2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_test_stat2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyTestStat2RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyTestStat2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_test_stat2_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyTestStat2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_test_stat2_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyTestStat2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_TEST_STAT2_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyTestStat2RegSpec {
    const RESET_VALUE: u32 = 0;
}

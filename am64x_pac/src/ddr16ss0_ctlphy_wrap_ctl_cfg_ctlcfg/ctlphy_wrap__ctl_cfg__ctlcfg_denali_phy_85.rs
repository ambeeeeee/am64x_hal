#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_85` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_85` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_0` reader - 9:0\\]
Read DQ4 slave delay setting for slice 0."]
pub type PhyRddq4SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ4_SLAVE_DELAY_0` writer - 9:0\\]
Read DQ4 slave delay setting for slice 0."]
pub type PhyRddq4SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_0` reader - 25:16\\]
Read DQ5 slave delay setting for slice 0."]
pub type PhyRddq5SlaveDelay0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ5_SLAVE_DELAY_0` writer - 25:16\\]
Read DQ5 slave delay setting for slice 0."]
pub type PhyRddq5SlaveDelay0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Read DQ4 slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddq4_slave_delay_0(&self) -> PhyRddq4SlaveDelay0R {
        PhyRddq4SlaveDelay0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Read DQ5 slave delay setting for slice 0."]
    #[inline(always)]
    pub fn phy_rddq5_slave_delay_0(&self) -> PhyRddq5SlaveDelay0R {
        PhyRddq5SlaveDelay0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Read DQ4 slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq4_slave_delay_0(
        &mut self,
    ) -> PhyRddq4SlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec> {
        PhyRddq4SlaveDelay0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Read DQ5 slave delay setting for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq5_slave_delay_0(
        &mut self,
    ) -> PhyRddq5SlaveDelay0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec> {
        PhyRddq5SlaveDelay0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_85\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_85::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_85::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_85::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_85::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_85 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy85Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1380` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1380` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec>;
#[doc = "Field `PHY_GRP2_SLAVE_DELAY_1` reader - 10:0\\]
Address slice slave delay setting for address slice 2."]
pub type PhyGrp2SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP2_SLAVE_DELAY_1` writer - 10:0\\]
Address slice slave delay setting for address slice 2."]
pub type PhyGrp2SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_GRP3_SLAVE_DELAY_1` reader - 26:16\\]
Address slice slave delay setting for address slice 3."]
pub type PhyGrp3SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP3_SLAVE_DELAY_1` writer - 26:16\\]
Address slice slave delay setting for address slice 3."]
pub type PhyGrp3SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Address slice slave delay setting for address slice 2."]
    #[inline(always)]
    pub fn phy_grp2_slave_delay_1(&self) -> PhyGrp2SlaveDelay1R {
        PhyGrp2SlaveDelay1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Address slice slave delay setting for address slice 3."]
    #[inline(always)]
    pub fn phy_grp3_slave_delay_1(&self) -> PhyGrp3SlaveDelay1R {
        PhyGrp3SlaveDelay1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Address slice slave delay setting for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp2_slave_delay_1(
        &mut self,
    ) -> PhyGrp2SlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec> {
        PhyGrp2SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Address slice slave delay setting for address slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp3_slave_delay_1(
        &mut self,
    ) -> PhyGrp3SlaveDelay1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec> {
        PhyGrp3SlaveDelay1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1380\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1380::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1380::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1380::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1380::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1380 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1380Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1381` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1381` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec>;
#[doc = "Field `PHY_GRP0_SLAVE_DELAY_2` reader - 10:0\\]
Address slice slave delay setting for address slice 0."]
pub type PhyGrp0SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP0_SLAVE_DELAY_2` writer - 10:0\\]
Address slice slave delay setting for address slice 0."]
pub type PhyGrp0SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_GRP1_SLAVE_DELAY_2` reader - 26:16\\]
Address slice slave delay setting for address slice 1."]
pub type PhyGrp1SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_GRP1_SLAVE_DELAY_2` writer - 26:16\\]
Address slice slave delay setting for address slice 1."]
pub type PhyGrp1SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Address slice slave delay setting for address slice 0."]
    #[inline(always)]
    pub fn phy_grp0_slave_delay_2(&self) -> PhyGrp0SlaveDelay2R {
        PhyGrp0SlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    pub fn phy_grp1_slave_delay_2(&self) -> PhyGrp1SlaveDelay2R {
        PhyGrp1SlaveDelay2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Address slice slave delay setting for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp0_slave_delay_2(
        &mut self,
    ) -> PhyGrp0SlaveDelay2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec> {
        PhyGrp0SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - 26:16\\]
Address slice slave delay setting for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_grp1_slave_delay_2(
        &mut self,
    ) -> PhyGrp1SlaveDelay2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec> {
        PhyGrp1SlaveDelay2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1381\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1381::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1381::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1381::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1381::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1381 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1381Spec {
    const RESET_VALUE: u32 = 0;
}

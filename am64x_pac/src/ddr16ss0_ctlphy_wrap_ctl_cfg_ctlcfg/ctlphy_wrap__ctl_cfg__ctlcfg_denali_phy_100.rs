#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_100` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_100` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec>;
#[doc = "Field `PHY_RDLVL_MAX_EDGE_0` reader - 9:0\\]
The maximun rdlvl slave delay search window for read eye training for slice 0."]
pub type PhyRdlvlMaxEdge0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDLVL_MAX_EDGE_0` writer - 9:0\\]
The maximun rdlvl slave delay search window for read eye training for slice 0."]
pub type PhyRdlvlMaxEdge0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
The maximun rdlvl slave delay search window for read eye training for slice 0."]
    #[inline(always)]
    pub fn phy_rdlvl_max_edge_0(&self) -> PhyRdlvlMaxEdge0R {
        PhyRdlvlMaxEdge0R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
The maximun rdlvl slave delay search window for read eye training for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rdlvl_max_edge_0(
        &mut self,
    ) -> PhyRdlvlMaxEdge0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec> {
        PhyRdlvlMaxEdge0W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_100::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_100::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_100 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy100Spec {
    const RESET_VALUE: u32 = 0;
}

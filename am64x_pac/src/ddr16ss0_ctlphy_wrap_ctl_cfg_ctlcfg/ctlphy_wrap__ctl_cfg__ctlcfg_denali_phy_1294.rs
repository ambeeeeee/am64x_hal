#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1294` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1294` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec>;
#[doc = "Field `PHY_CSLVL_CS_MAP` reader - 1:0\\]
CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
pub type PhyCslvlCsMapR = crate::FieldReader;
#[doc = "Field `PHY_CSLVL_CS_MAP` writer - 1:0\\]
CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
pub type PhyCslvlCsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_CSLVL_QTR` reader - 18:8\\]
Defines the CS training DDL 1/4 cycle delay value."]
pub type PhyCslvlQtrR = crate::FieldReader<u16>;
#[doc = "Field `PHY_CSLVL_QTR` writer - 18:8\\]
Defines the CS training DDL 1/4 cycle delay value."]
pub type PhyCslvlQtrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
    #[inline(always)]
    pub fn phy_cslvl_cs_map(&self) -> PhyCslvlCsMapR {
        PhyCslvlCsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Defines the CS training DDL 1/4 cycle delay value."]
    #[inline(always)]
    pub fn phy_cslvl_qtr(&self) -> PhyCslvlQtrR {
        PhyCslvlQtrR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
CS training map. Set each CS bit to 1 to allow that CS to participate in CS training results. NOT CURRENTLY USED."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_cs_map(
        &mut self,
    ) -> PhyCslvlCsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec> {
        PhyCslvlCsMapW::new(self, 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Defines the CS training DDL 1/4 cycle delay value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_cslvl_qtr(&mut self) -> PhyCslvlQtrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec> {
        PhyCslvlQtrW::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1294\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1294::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1294::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1294::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1294::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1294 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1294Spec {
    const RESET_VALUE: u32 = 0;
}

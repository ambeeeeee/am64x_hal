#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_324` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_324` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec>;
#[doc = "Field `BANK_SPLIT_EN` reader - 0:0\\]
Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
pub type BankSplitEnR = crate::BitReader;
#[doc = "Field `BANK_SPLIT_EN` writer - 0:0\\]
Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
pub type BankSplitEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLACEMENT_EN` reader - 8:8\\]
Enable placement logic for command queue. Set to 1 to enable."]
pub type PlacementEnR = crate::BitReader;
#[doc = "Field `PLACEMENT_EN` writer - 8:8\\]
Enable placement logic for command queue. Set to 1 to enable."]
pub type PlacementEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIORITY_EN` reader - 16:16\\]
Enable priority as a rule for command queue placement. Set to 1 to enable."]
pub type PriorityEnR = crate::BitReader;
#[doc = "Field `PRIORITY_EN` writer - 16:16\\]
Enable priority as a rule for command queue placement. Set to 1 to enable."]
pub type PriorityEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RW_SAME_EN` reader - 24:24\\]
Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
pub type RwSameEnR = crate::BitReader;
#[doc = "Field `RW_SAME_EN` writer - 24:24\\]
Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
pub type RwSameEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn bank_split_en(&self) -> BankSplitEnR {
        BankSplitEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable placement logic for command queue. Set to 1 to enable."]
    #[inline(always)]
    pub fn placement_en(&self) -> PlacementEnR {
        PlacementEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable priority as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn priority_en(&self) -> PriorityEnR {
        PriorityEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    pub fn rw_same_en(&self) -> RwSameEnR {
        RwSameEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable bank splitting as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn bank_split_en(&mut self) -> BankSplitEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec> {
        BankSplitEnW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable placement logic for command queue. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn placement_en(&mut self) -> PlacementEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec> {
        PlacementEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable priority as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn priority_en(&mut self) -> PriorityEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec> {
        PriorityEnW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable read/write grouping as a rule for command queue placement. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rw_same_en(&mut self) -> RwSameEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec> {
        RwSameEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_324\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_324::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_324::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_324::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_324::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_324 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl324Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_69` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_69` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec>;
#[doc = "Field `OPTIMAL_RMODW_EN` reader - 0:0\\]
Enables optimized RMODW logic in the controller. A value of 1 enables optimized RMODW operation. All RMODW operations are still supported in a non-optimal manner when the value is 0."]
pub type OptimalRmodwEnR = crate::BitReader;
#[doc = "Field `OPTIMAL_RMODW_EN` writer - 0:0\\]
Enables optimized RMODW logic in the controller. A value of 1 enables optimized RMODW operation. All RMODW operations are still supported in a non-optimal manner when the value is 0."]
pub type OptimalRmodwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NO_MEMORY_DM` reader - 16:16\\]
Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
pub type NoMemoryDmR = crate::BitReader;
#[doc = "Field `NO_MEMORY_DM` writer - 16:16\\]
Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
pub type NoMemoryDmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables optimized RMODW logic in the controller. A value of 1 enables optimized RMODW operation. All RMODW operations are still supported in a non-optimal manner when the value is 0."]
    #[inline(always)]
    pub fn optimal_rmodw_en(&self) -> OptimalRmodwEnR {
        OptimalRmodwEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
    #[inline(always)]
    pub fn no_memory_dm(&self) -> NoMemoryDmR {
        NoMemoryDmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables optimized RMODW logic in the controller. A value of 1 enables optimized RMODW operation. All RMODW operations are still supported in a non-optimal manner when the value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn optimal_rmodw_en(&mut self) -> OptimalRmodwEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec> {
        OptimalRmodwEnW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Indicates that the external DRAM does not support DM masking. Set to 1 for no DM masking at the DRAM."]
    #[inline(always)]
    #[must_use]
    pub fn no_memory_dm(&mut self) -> NoMemoryDmW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec> {
        NoMemoryDmW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_69\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_69::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_69::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_69::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_69::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_69 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl69Spec {
    const RESET_VALUE: u32 = 0;
}

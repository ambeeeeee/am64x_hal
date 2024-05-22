#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_74` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_74` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec>;
#[doc = "Field `PI_WDQLVL_DRAM_LVL_START_ADDR_1` reader - 0:0\\]
Start address of WDQ leveling, not for LPDDR4."]
pub type PiWdqlvlDramLvlStartAddr1R = crate::BitReader;
#[doc = "Field `PI_WDQLVL_DRAM_LVL_START_ADDR_1` writer - 0:0\\]
Start address of WDQ leveling, not for LPDDR4."]
pub type PiWdqlvlDramLvlStartAddr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_WDQLVL_DM_LEVEL_EN` reader - 8:8\\]
Enable for write DM training as part of the write DQ training, not for LPDDR4."]
pub type PiWdqlvlDmLevelEnR = crate::BitReader;
#[doc = "Field `PI_WDQLVL_DM_LEVEL_EN` writer - 8:8\\]
Enable for write DM training as part of the write DQ training, not for LPDDR4."]
pub type PiWdqlvlDmLevelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_NO_MEMORY_DM` reader - 16:16\\]
Defines if the attached memory supports the Data Mask function, 1 = not supported."]
pub type PiNoMemoryDmR = crate::BitReader;
#[doc = "Field `PI_NO_MEMORY_DM` writer - 16:16\\]
Defines if the attached memory supports the Data Mask function, 1 = not supported."]
pub type PiNoMemoryDmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Start address of WDQ leveling, not for LPDDR4."]
    #[inline(always)]
    pub fn pi_wdqlvl_dram_lvl_start_addr_1(&self) -> PiWdqlvlDramLvlStartAddr1R {
        PiWdqlvlDramLvlStartAddr1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable for write DM training as part of the write DQ training, not for LPDDR4."]
    #[inline(always)]
    pub fn pi_wdqlvl_dm_level_en(&self) -> PiWdqlvlDmLevelEnR {
        PiWdqlvlDmLevelEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines if the attached memory supports the Data Mask function, 1 = not supported."]
    #[inline(always)]
    pub fn pi_no_memory_dm(&self) -> PiNoMemoryDmR {
        PiNoMemoryDmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Start address of WDQ leveling, not for LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_dram_lvl_start_addr_1(
        &mut self,
    ) -> PiWdqlvlDramLvlStartAddr1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec> {
        PiWdqlvlDramLvlStartAddr1W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable for write DM training as part of the write DQ training, not for LPDDR4."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_dm_level_en(
        &mut self,
    ) -> PiWdqlvlDmLevelEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec> {
        PiWdqlvlDmLevelEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines if the attached memory supports the Data Mask function, 1 = not supported."]
    #[inline(always)]
    #[must_use]
    pub fn pi_no_memory_dm(&mut self) -> PiNoMemoryDmW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec> {
        PiNoMemoryDmW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_74\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_74::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_74::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_74::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_74::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_74 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi74Spec {
    const RESET_VALUE: u32 = 0;
}

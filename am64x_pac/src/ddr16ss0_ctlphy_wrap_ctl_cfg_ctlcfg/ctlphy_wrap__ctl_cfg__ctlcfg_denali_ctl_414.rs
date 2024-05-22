#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_414` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_414` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec>;
#[doc = "Field `TDFI_PARIN_LAT` reader - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
pub type TdfiParinLatR = crate::FieldReader;
#[doc = "Field `TDFI_PARIN_LAT` writer - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
pub type TdfiParinLatW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDFI_WRDATA_DELAY` reader - 15:8\\]
Defines the tWRDATA_DELAY timing parameter \\[in DFI PHY clocks\\], the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
pub type TdfiWrdataDelayR = crate::FieldReader;
#[doc = "Field `TDFI_WRDATA_DELAY` writer - 15:8\\]
Defines the tWRDATA_DELAY timing parameter \\[in DFI PHY clocks\\], the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
pub type TdfiWrdataDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DISABLE_MEMORY_MASKED_WRITE` reader - 16:16\\]
Restricts the controller from masked write commands. Set to 1 to not issue these commands. Only used if connected to an LPDDR4 device."]
pub type DisableMemoryMaskedWriteR = crate::BitReader;
#[doc = "Field `DISABLE_MEMORY_MASKED_WRITE` writer - 16:16\\]
Restricts the controller from masked write commands. Set to 1 to not issue these commands. Only used if connected to an LPDDR4 device."]
pub type DisableMemoryMaskedWriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRATEGY_2TICK_COUNT` reader - 26:24\\]
NEED TO FiLL IN ."]
pub type Strategy2tickCountR = crate::FieldReader;
#[doc = "Field `STRATEGY_2TICK_COUNT` writer - 26:24\\]
NEED TO FiLL IN ."]
pub type Strategy2tickCountW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
    #[inline(always)]
    pub fn tdfi_parin_lat(&self) -> TdfiParinLatR {
        TdfiParinLatR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the tWRDATA_DELAY timing parameter \\[in DFI PHY clocks\\], the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
    #[inline(always)]
    pub fn tdfi_wrdata_delay(&self) -> TdfiWrdataDelayR {
        TdfiWrdataDelayR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Restricts the controller from masked write commands. Set to 1 to not issue these commands. Only used if connected to an LPDDR4 device."]
    #[inline(always)]
    pub fn disable_memory_masked_write(&self) -> DisableMemoryMaskedWriteR {
        DisableMemoryMaskedWriteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    pub fn strategy_2tick_count(&self) -> Strategy2tickCountR {
        Strategy2tickCountR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the DFI tPARIN_LAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a DFI command and a dfi_parity_in signal assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_parin_lat(&mut self) -> TdfiParinLatW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec> {
        TdfiParinLatW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Defines the tWRDATA_DELAY timing parameter \\[in DFI PHY clocks\\], the maximum cycles between when the dfi_wrdata_en signal is asserted and when the corresponding write data transfer is completed on the DRAM bus."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrdata_delay(
        &mut self,
    ) -> TdfiWrdataDelayW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec> {
        TdfiWrdataDelayW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Restricts the controller from masked write commands. Set to 1 to not issue these commands. Only used if connected to an LPDDR4 device."]
    #[inline(always)]
    #[must_use]
    pub fn disable_memory_masked_write(
        &mut self,
    ) -> DisableMemoryMaskedWriteW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec> {
        DisableMemoryMaskedWriteW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
NEED TO FiLL IN ."]
    #[inline(always)]
    #[must_use]
    pub fn strategy_2tick_count(
        &mut self,
    ) -> Strategy2tickCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec> {
        Strategy2tickCountW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_414\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_414::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_414::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_414::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_414::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_414 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl414Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_327` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_327` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec>;
#[doc = "Field `CS_MAP` reader - 1:0\\]
Defines which chip selects are active."]
pub type CsMapR = crate::FieldReader;
#[doc = "Field `CS_MAP` writer - 1:0\\]
Defines which chip selects are active."]
pub type CsMapW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BURST_ON_FLY_BIT` reader - 11:8\\]
Identifies the burst-on-fly bit in the memory mode registers."]
pub type BurstOnFlyBitR = crate::FieldReader;
#[doc = "Field `BURST_ON_FLY_BIT` writer - 11:8\\]
Identifies the burst-on-fly bit in the memory mode registers."]
pub type BurstOnFlyBitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MEM_DP_REDUCTION` reader - 16:16\\]
Enable the half datapath feature of the controller. Set to 1 to enable."]
pub type MemDpReductionR = crate::BitReader;
#[doc = "Field `MEM_DP_REDUCTION` writer - 16:16\\]
Enable the half datapath feature of the controller. Set to 1 to enable."]
pub type MemDpReductionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEMDATA_RATIO_0` reader - 26:24\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=0"]
pub type MemdataRatio0R = crate::FieldReader;
#[doc = "Field `MEMDATA_RATIO_0` writer - 26:24\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=0"]
pub type MemdataRatio0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip selects are active."]
    #[inline(always)]
    pub fn cs_map(&self) -> CsMapR {
        CsMapR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Identifies the burst-on-fly bit in the memory mode registers."]
    #[inline(always)]
    pub fn burst_on_fly_bit(&self) -> BurstOnFlyBitR {
        BurstOnFlyBitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the half datapath feature of the controller. Set to 1 to enable."]
    #[inline(always)]
    pub fn mem_dp_reduction(&self) -> MemDpReductionR {
        MemDpReductionR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=0"]
    #[inline(always)]
    pub fn memdata_ratio_0(&self) -> MemdataRatio0R {
        MemdataRatio0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Defines which chip selects are active."]
    #[inline(always)]
    #[must_use]
    pub fn cs_map(&mut self) -> CsMapW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec> {
        CsMapW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Identifies the burst-on-fly bit in the memory mode registers."]
    #[inline(always)]
    #[must_use]
    pub fn burst_on_fly_bit(&mut self) -> BurstOnFlyBitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec> {
        BurstOnFlyBitW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable the half datapath feature of the controller. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn mem_dp_reduction(
        &mut self,
    ) -> MemDpReductionW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec> {
        MemDpReductionW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Defines the ratio of the DRAM device size on chip select 0 to the memory data width. Program with the log2 ratio of the memory data width to the device data width. CS=0"]
    #[inline(always)]
    #[must_use]
    pub fn memdata_ratio_0(&mut self) -> MemdataRatio0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec> {
        MemdataRatio0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_327\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_327::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_327::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_327::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_327::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_327 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl327Spec {
    const RESET_VALUE: u32 = 0;
}

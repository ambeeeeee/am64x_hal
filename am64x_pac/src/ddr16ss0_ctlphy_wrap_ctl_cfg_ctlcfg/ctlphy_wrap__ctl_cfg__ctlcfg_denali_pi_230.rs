#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_230` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_230` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec>;
#[doc = "Field `PI_WDQLVL_RDLAT_ADJ_F2` reader - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
pub type PiWdqlvlRdlatAdjF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_RDLAT_ADJ_F2` writer - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
pub type PiWdqlvlRdlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WDQLVL_WRLAT_ADJ_F2` reader - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
pub type PiWdqlvlWrlatAdjF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_WRLAT_ADJ_F2` writer - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
pub type PiWdqlvlWrlatAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_RD_DBI_LEVEL_EN_F0` reader - 17:16\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiRdDbiLevelEnF0R = crate::FieldReader;
#[doc = "Field `PI_RD_DBI_LEVEL_EN_F0` writer - 17:16\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiRdDbiLevelEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_RD_DBI_LEVEL_EN_F1` reader - 25:24\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiRdDbiLevelEnF1R = crate::FieldReader;
#[doc = "Field `PI_RD_DBI_LEVEL_EN_F1` writer - 25:24\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
pub type PiRdDbiLevelEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_rdlat_adj_f2(&self) -> PiWdqlvlRdlatAdjF2R {
        PiWdqlvlRdlatAdjF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_wrlat_adj_f2(&self) -> PiWdqlvlWrlatAdjF2R {
        PiWdqlvlWrlatAdjF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_rd_dbi_level_en_f0(&self) -> PiRdDbiLevelEnF0R {
        PiRdDbiLevelEnF0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    pub fn pi_rd_dbi_level_en_f1(&self) -> PiRdDbiLevelEnF1R {
        PiRdDbiLevelEnF1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_rdlat_adj_f2(
        &mut self,
    ) -> PiWdqlvlRdlatAdjF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec> {
        PiWdqlvlRdlatAdjF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 2, used for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_wrlat_adj_f2(
        &mut self,
    ) -> PiWdqlvlWrlatAdjF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec> {
        PiWdqlvlWrlatAdjF2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 0. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_dbi_level_en_f0(
        &mut self,
    ) -> PiRdDbiLevelEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec> {
        PiRdDbiLevelEnF0W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Read DBI leveling enable, only can be enabled when READ DBI supported for DDR4, and PI_WDQLVL_EN or PI_RDLVL_EN configured by 1 for frequency set 1. Bit\\[1\\]
represents the support when non-initialization. Bit\\[0\\]represents the support when initialization."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_dbi_level_en_f1(
        &mut self,
    ) -> PiRdDbiLevelEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec> {
        PiRdDbiLevelEnF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_230\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_230::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_230::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_230::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_230::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_230 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi230Spec {
    const RESET_VALUE: u32 = 0;
}

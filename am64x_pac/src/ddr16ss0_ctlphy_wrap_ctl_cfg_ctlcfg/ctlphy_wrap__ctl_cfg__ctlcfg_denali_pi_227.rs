#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_227` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_227` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec>;
#[doc = "Field `PI_WDQLVL_RDLAT_ADJ_F1` reader - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
pub type PiWdqlvlRdlatAdjF1R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_RDLAT_ADJ_F1` writer - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
pub type PiWdqlvlRdlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WDQLVL_WRLAT_ADJ_F1` reader - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
pub type PiWdqlvlWrlatAdjF1R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_WRLAT_ADJ_F1` writer - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
pub type PiWdqlvlWrlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_WDQLVL_WR_F2` reader - 25:16\\]
Switch time from write to read for frequency set 2."]
pub type PiTdfiWdqlvlWrF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_WR_F2` writer - 25:16\\]
Switch time from write to read for frequency set 2."]
pub type PiTdfiWdqlvlWrF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_rdlat_adj_f1(&self) -> PiWdqlvlRdlatAdjF1R {
        PiWdqlvlRdlatAdjF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_wrlat_adj_f1(&self) -> PiWdqlvlWrlatAdjF1R {
        PiWdqlvlWrlatAdjF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Switch time from write to read for frequency set 2."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_wr_f2(&self) -> PiTdfiWdqlvlWrF2R {
        PiTdfiWdqlvlWrF2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_rdlat_adj_f1(
        &mut self,
    ) -> PiWdqlvlRdlatAdjF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec> {
        PiWdqlvlRdlatAdjF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 1, used for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_wrlat_adj_f1(
        &mut self,
    ) -> PiWdqlvlWrlatAdjF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec> {
        PiWdqlvlWrlatAdjF1W::new(self, 8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Switch time from write to read for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_wr_f2(
        &mut self,
    ) -> PiTdfiWdqlvlWrF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec> {
        PiTdfiWdqlvlWrF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_227\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_227::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_227::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_227::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_227::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_227 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi227Spec {
    const RESET_VALUE: u32 = 0;
}

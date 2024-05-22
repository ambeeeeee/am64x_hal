#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_224` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_224` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec>;
#[doc = "Field `PI_WDQLVL_RDLAT_ADJ_F0` reader - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
pub type PiWdqlvlRdlatAdjF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_RDLAT_ADJ_F0` writer - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
pub type PiWdqlvlRdlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WDQLVL_WRLAT_ADJ_F0` reader - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
pub type PiWdqlvlWrlatAdjF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_WRLAT_ADJ_F0` writer - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
pub type PiWdqlvlWrlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_WDQLVL_WR_F1` reader - 25:16\\]
Switch time from write to read for frequency set 1."]
pub type PiTdfiWdqlvlWrF1R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_WR_F1` writer - 25:16\\]
Switch time from write to read for frequency set 1."]
pub type PiTdfiWdqlvlWrF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_rdlat_adj_f0(&self) -> PiWdqlvlRdlatAdjF0R {
        PiWdqlvlRdlatAdjF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
    #[inline(always)]
    pub fn pi_wdqlvl_wrlat_adj_f0(&self) -> PiWdqlvlWrlatAdjF0R {
        PiWdqlvlWrlatAdjF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Switch time from write to read for frequency set 1."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_wr_f1(&self) -> PiTdfiWdqlvlWrF1R {
        PiTdfiWdqlvlWrF1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Adjusted Tdfi_rddata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_rdlat_adj_f0(
        &mut self,
    ) -> PiWdqlvlRdlatAdjF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec> {
        PiWdqlvlRdlatAdjF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Adjusted Tdfi_wrdata_en value for PHY read timing when read dbi disabled for frequency set 0, used for WDQ training."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_wrlat_adj_f0(
        &mut self,
    ) -> PiWdqlvlWrlatAdjF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec> {
        PiWdqlvlWrlatAdjF0W::new(self, 8)
    }
    #[doc = "Bits 16:25 - 25:16\\]
Switch time from write to read for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_wr_f1(
        &mut self,
    ) -> PiTdfiWdqlvlWrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec> {
        PiTdfiWdqlvlWrF1W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_224\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_224::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_224::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_224::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_224::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_224 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi224Spec {
    const RESET_VALUE: u32 = 0;
}

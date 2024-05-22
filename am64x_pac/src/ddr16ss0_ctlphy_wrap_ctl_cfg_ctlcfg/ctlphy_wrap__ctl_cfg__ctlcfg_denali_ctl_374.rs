#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_374` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_374` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec>;
#[doc = "Field `WR_TO_ODTH_F0` reader - 5:0\\]
Defines the delay from a write command to ODT assertion. FC=0"]
pub type WrToOdthF0R = crate::FieldReader;
#[doc = "Field `WR_TO_ODTH_F0` writer - 5:0\\]
Defines the delay from a write command to ODT assertion. FC=0"]
pub type WrToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WR_TO_ODTH_F1` reader - 13:8\\]
Defines the delay from a write command to ODT assertion. FC=1"]
pub type WrToOdthF1R = crate::FieldReader;
#[doc = "Field `WR_TO_ODTH_F1` writer - 13:8\\]
Defines the delay from a write command to ODT assertion. FC=1"]
pub type WrToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WR_TO_ODTH_F2` reader - 21:16\\]
Defines the delay from a write command to ODT assertion. FC=2"]
pub type WrToOdthF2R = crate::FieldReader;
#[doc = "Field `WR_TO_ODTH_F2` writer - 21:16\\]
Defines the delay from a write command to ODT assertion. FC=2"]
pub type WrToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ODT_RD_MAP_CS0` reader - 25:24\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=0"]
pub type OdtRdMapCs0R = crate::FieldReader;
#[doc = "Field `ODT_RD_MAP_CS0` writer - 25:24\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=0"]
pub type OdtRdMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Defines the delay from a write command to ODT assertion. FC=0"]
    #[inline(always)]
    pub fn wr_to_odth_f0(&self) -> WrToOdthF0R {
        WrToOdthF0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the delay from a write command to ODT assertion. FC=1"]
    #[inline(always)]
    pub fn wr_to_odth_f1(&self) -> WrToOdthF1R {
        WrToOdthF1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Defines the delay from a write command to ODT assertion. FC=2"]
    #[inline(always)]
    pub fn wr_to_odth_f2(&self) -> WrToOdthF2R {
        WrToOdthF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=0"]
    #[inline(always)]
    pub fn odt_rd_map_cs0(&self) -> OdtRdMapCs0R {
        OdtRdMapCs0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Defines the delay from a write command to ODT assertion. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_odth_f0(&mut self) -> WrToOdthF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec> {
        WrToOdthF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the delay from a write command to ODT assertion. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_odth_f1(&mut self) -> WrToOdthF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec> {
        WrToOdthF1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Defines the delay from a write command to ODT assertion. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn wr_to_odth_f2(&mut self) -> WrToOdthF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec> {
        WrToOdthF2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=0"]
    #[inline(always)]
    #[must_use]
    pub fn odt_rd_map_cs0(&mut self) -> OdtRdMapCs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec> {
        OdtRdMapCs0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_374\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_374::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_374::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_374::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_374::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_374 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl374Spec {
    const RESET_VALUE: u32 = 0;
}

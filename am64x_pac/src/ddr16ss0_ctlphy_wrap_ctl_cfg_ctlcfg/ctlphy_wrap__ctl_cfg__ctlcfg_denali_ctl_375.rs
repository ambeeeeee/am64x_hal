#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_375` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_375` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec>;
#[doc = "Field `ODT_WR_MAP_CS0` reader - 1:0\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=0"]
pub type OdtWrMapCs0R = crate::FieldReader;
#[doc = "Field `ODT_WR_MAP_CS0` writer - 1:0\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=0"]
pub type OdtWrMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODT_RD_MAP_CS1` reader - 9:8\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=1"]
pub type OdtRdMapCs1R = crate::FieldReader;
#[doc = "Field `ODT_RD_MAP_CS1` writer - 9:8\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=1"]
pub type OdtRdMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODT_WR_MAP_CS1` reader - 17:16\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=1"]
pub type OdtWrMapCs1R = crate::FieldReader;
#[doc = "Field `ODT_WR_MAP_CS1` writer - 17:16\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=1"]
pub type OdtWrMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RD_TO_ODTH_F0` reader - 29:24\\]
Defines the delay from a read command to ODT assertion. FC=0"]
pub type RdToOdthF0R = crate::FieldReader;
#[doc = "Field `RD_TO_ODTH_F0` writer - 29:24\\]
Defines the delay from a read command to ODT assertion. FC=0"]
pub type RdToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=0"]
    #[inline(always)]
    pub fn odt_wr_map_cs0(&self) -> OdtWrMapCs0R {
        OdtWrMapCs0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=1"]
    #[inline(always)]
    pub fn odt_rd_map_cs1(&self) -> OdtRdMapCs1R {
        OdtRdMapCs1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=1"]
    #[inline(always)]
    pub fn odt_wr_map_cs1(&self) -> OdtWrMapCs1R {
        OdtWrMapCs1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Defines the delay from a read command to ODT assertion. FC=0"]
    #[inline(always)]
    pub fn rd_to_odth_f0(&self) -> RdToOdthF0R {
        RdToOdthF0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=0"]
    #[inline(always)]
    #[must_use]
    pub fn odt_wr_map_cs0(&mut self) -> OdtWrMapCs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec> {
        OdtWrMapCs0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Determines which chip\\[s\\]
will have termination when a read occurs. Set bit X to enable termination on csX when a read is performed. CS=1"]
    #[inline(always)]
    #[must_use]
    pub fn odt_rd_map_cs1(&mut self) -> OdtRdMapCs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec> {
        OdtRdMapCs1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Determines which chip\\[s\\]
will have termination when a write occurs. Set bit X to enable termination on csX when a write is performed. CS=1"]
    #[inline(always)]
    #[must_use]
    pub fn odt_wr_map_cs1(&mut self) -> OdtWrMapCs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec> {
        OdtWrMapCs1W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Defines the delay from a read command to ODT assertion. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn rd_to_odth_f0(&mut self) -> RdToOdthF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec> {
        RdToOdthF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_375\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_375::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_375::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_375::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_375::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_375 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl375Spec {
    const RESET_VALUE: u32 = 0;
}

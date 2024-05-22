#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_283` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_283` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec>;
#[doc = "Field `PI_ODT_RD_MAP_CS0` reader - 1:0\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
pub type PiOdtRdMapCs0R = crate::FieldReader;
#[doc = "Field `PI_ODT_RD_MAP_CS0` writer - 1:0\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
pub type PiOdtRdMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ODT_WR_MAP_CS0` reader - 9:8\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
pub type PiOdtWrMapCs0R = crate::FieldReader;
#[doc = "Field `PI_ODT_WR_MAP_CS0` writer - 9:8\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
pub type PiOdtWrMapCs0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ODT_RD_MAP_CS1` reader - 17:16\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
pub type PiOdtRdMapCs1R = crate::FieldReader;
#[doc = "Field `PI_ODT_RD_MAP_CS1` writer - 17:16\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
pub type PiOdtRdMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_ODT_WR_MAP_CS1` reader - 25:24\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
pub type PiOdtWrMapCs1R = crate::FieldReader;
#[doc = "Field `PI_ODT_WR_MAP_CS1` writer - 25:24\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
pub type PiOdtWrMapCs1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
    #[inline(always)]
    pub fn pi_odt_rd_map_cs0(&self) -> PiOdtRdMapCs0R {
        PiOdtRdMapCs0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
    #[inline(always)]
    pub fn pi_odt_wr_map_cs0(&self) -> PiOdtWrMapCs0R {
        PiOdtWrMapCs0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
    #[inline(always)]
    pub fn pi_odt_rd_map_cs1(&self) -> PiOdtRdMapCs1R {
        PiOdtRdMapCs1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
    #[inline(always)]
    pub fn pi_odt_wr_map_cs1(&self) -> PiOdtWrMapCs1R {
        PiOdtWrMapCs1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a read."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_rd_map_cs0(&mut self) -> PiOdtRdMapCs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec> {
        PiOdtRdMapCs0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 0. Set bit X to enable termination on csX when cs0 is performing a write."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_wr_map_cs0(&mut self) -> PiOdtWrMapCs0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec> {
        PiOdtWrMapCs0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Determines which chip\\[s\\]
will have termination when a read occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a read."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_rd_map_cs1(&mut self) -> PiOdtRdMapCs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec> {
        PiOdtRdMapCs1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Determines which chip\\[s\\]
will have termination when a write occurs on chip select 1. Set bit X to enable termination on csX when cs1 is performing a write."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_wr_map_cs1(&mut self) -> PiOdtWrMapCs1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec> {
        PiOdtWrMapCs1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_283\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_283::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_283::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_283::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_283::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_283 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi283Spec {
    const RESET_VALUE: u32 = 0;
}

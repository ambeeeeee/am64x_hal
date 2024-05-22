#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_18` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_18` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec>;
#[doc = "Field `PI_SW_WRLVL_RESP_1` reader - 0:0\\]
Write leveling response for data slice 1. READ-ONLY"]
pub type PiSwWrlvlResp1R = crate::BitReader;
#[doc = "Field `PI_SW_WRLVL_RESP_1` writer - 0:0\\]
Write leveling response for data slice 1. READ-ONLY"]
pub type PiSwWrlvlResp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SW_RDLVL_RESP_0` reader - 9:8\\]
Read leveling response for data slice 0. READ-ONLY"]
pub type PiSwRdlvlResp0R = crate::FieldReader;
#[doc = "Field `PI_SW_RDLVL_RESP_0` writer - 9:8\\]
Read leveling response for data slice 0. READ-ONLY"]
pub type PiSwRdlvlResp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_SW_RDLVL_RESP_1` reader - 17:16\\]
Read leveling response for data slice 1. READ-ONLY"]
pub type PiSwRdlvlResp1R = crate::FieldReader;
#[doc = "Field `PI_SW_RDLVL_RESP_1` writer - 17:16\\]
Read leveling response for data slice 1. READ-ONLY"]
pub type PiSwRdlvlResp1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PI_SW_CALVL_RESP_0` reader - 25:24\\]
CA leveling response for address slice 0. READ-ONLY"]
pub type PiSwCalvlResp0R = crate::FieldReader;
#[doc = "Field `PI_SW_CALVL_RESP_0` writer - 25:24\\]
CA leveling response for address slice 0. READ-ONLY"]
pub type PiSwCalvlResp0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn pi_sw_wrlvl_resp_1(&self) -> PiSwWrlvlResp1R {
        PiSwWrlvlResp1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Read leveling response for data slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn pi_sw_rdlvl_resp_0(&self) -> PiSwRdlvlResp0R {
        PiSwRdlvlResp0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Read leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    pub fn pi_sw_rdlvl_resp_1(&self) -> PiSwRdlvlResp1R {
        PiSwRdlvlResp1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
CA leveling response for address slice 0. READ-ONLY"]
    #[inline(always)]
    pub fn pi_sw_calvl_resp_0(&self) -> PiSwCalvlResp0R {
        PiSwCalvlResp0R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_wrlvl_resp_1(
        &mut self,
    ) -> PiSwWrlvlResp1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec> {
        PiSwWrlvlResp1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Read leveling response for data slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_rdlvl_resp_0(
        &mut self,
    ) -> PiSwRdlvlResp0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec> {
        PiSwRdlvlResp0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Read leveling response for data slice 1. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_rdlvl_resp_1(
        &mut self,
    ) -> PiSwRdlvlResp1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec> {
        PiSwRdlvlResp1W::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
CA leveling response for address slice 0. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn pi_sw_calvl_resp_0(
        &mut self,
    ) -> PiSwCalvlResp0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec> {
        PiSwCalvlResp0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_18::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_18::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_18 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi18Spec {
    const RESET_VALUE: u32 = 0;
}

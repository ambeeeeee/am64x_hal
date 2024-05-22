#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_133` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_133` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec>;
#[doc = "Field `PI_COL_DIFF` reader - 3:0\\]
Difference between number of column pins available and number being used."]
pub type PiColDiffR = crate::FieldReader;
#[doc = "Field `PI_COL_DIFF` writer - 3:0\\]
Difference between number of column pins available and number being used."]
pub type PiColDiffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_BG_ROTATE_EN` reader - 8:8\\]
Enable bank group rotation. Set to 1 to enable."]
pub type PiBgRotateEnR = crate::BitReader;
#[doc = "Field `PI_BG_ROTATE_EN` writer - 8:8\\]
Enable bank group rotation. Set to 1 to enable."]
pub type PiBgRotateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_CRC_CALC` reader - 16:16\\]
Defines where CRC is performed; set to 1 for PI responsibility or clear to 0 for PHY responsibility."]
pub type PiCrcCalcR = crate::BitReader;
#[doc = "Field `PI_CRC_CALC` writer - 16:16\\]
Defines where CRC is performed; set to 1 for PI responsibility or clear to 0 for PHY responsibility."]
pub type PiCrcCalcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PI_SELF_REFRESH_EN` reader - 24:24\\]
Control for PI to enable self refresh mode. Set to 1 to enable."]
pub type PiSelfRefreshEnR = crate::BitReader;
#[doc = "Field `PI_SELF_REFRESH_EN` writer - 24:24\\]
Control for PI to enable self refresh mode. Set to 1 to enable."]
pub type PiSelfRefreshEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Difference between number of column pins available and number being used."]
    #[inline(always)]
    pub fn pi_col_diff(&self) -> PiColDiffR {
        PiColDiffR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable bank group rotation. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_bg_rotate_en(&self) -> PiBgRotateEnR {
        PiBgRotateEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines where CRC is performed; set to 1 for PI responsibility or clear to 0 for PHY responsibility."]
    #[inline(always)]
    pub fn pi_crc_calc(&self) -> PiCrcCalcR {
        PiCrcCalcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Control for PI to enable self refresh mode. Set to 1 to enable."]
    #[inline(always)]
    pub fn pi_self_refresh_en(&self) -> PiSelfRefreshEnR {
        PiSelfRefreshEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Difference between number of column pins available and number being used."]
    #[inline(always)]
    #[must_use]
    pub fn pi_col_diff(&mut self) -> PiColDiffW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec> {
        PiColDiffW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable bank group rotation. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_bg_rotate_en(&mut self) -> PiBgRotateEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec> {
        PiBgRotateEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines where CRC is performed; set to 1 for PI responsibility or clear to 0 for PHY responsibility."]
    #[inline(always)]
    #[must_use]
    pub fn pi_crc_calc(&mut self) -> PiCrcCalcW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec> {
        PiCrcCalcW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Control for PI to enable self refresh mode. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pi_self_refresh_en(
        &mut self,
    ) -> PiSelfRefreshEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec> {
        PiSelfRefreshEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_133\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_133::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_133::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_133::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_133::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_133 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi133Spec {
    const RESET_VALUE: u32 = 0;
}

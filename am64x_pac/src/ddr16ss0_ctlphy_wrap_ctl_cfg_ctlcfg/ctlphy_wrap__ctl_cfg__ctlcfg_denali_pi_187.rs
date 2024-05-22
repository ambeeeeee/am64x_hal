#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_187` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_187` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec>;
#[doc = "Field `PI_WR_TO_ODTH_F0` reader - 5:0\\]
Defines the delay from a write command to ODT assertion for frequency set 0."]
pub type PiWrToOdthF0R = crate::FieldReader;
#[doc = "Field `PI_WR_TO_ODTH_F0` writer - 5:0\\]
Defines the delay from a write command to ODT assertion for frequency set 0."]
pub type PiWrToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_WR_TO_ODTH_F1` reader - 13:8\\]
Defines the delay from a write command to ODT assertion for frequency set 1."]
pub type PiWrToOdthF1R = crate::FieldReader;
#[doc = "Field `PI_WR_TO_ODTH_F1` writer - 13:8\\]
Defines the delay from a write command to ODT assertion for frequency set 1."]
pub type PiWrToOdthF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_WR_TO_ODTH_F2` reader - 21:16\\]
Defines the delay from a write command to ODT assertion for frequency set 2."]
pub type PiWrToOdthF2R = crate::FieldReader;
#[doc = "Field `PI_WR_TO_ODTH_F2` writer - 21:16\\]
Defines the delay from a write command to ODT assertion for frequency set 2."]
pub type PiWrToOdthF2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PI_RD_TO_ODTH_F0` reader - 29:24\\]
Defines the delay from a read command to ODT assertion for frequency set 0."]
pub type PiRdToOdthF0R = crate::FieldReader;
#[doc = "Field `PI_RD_TO_ODTH_F0` writer - 29:24\\]
Defines the delay from a read command to ODT assertion for frequency set 0."]
pub type PiRdToOdthF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Defines the delay from a write command to ODT assertion for frequency set 0."]
    #[inline(always)]
    pub fn pi_wr_to_odth_f0(&self) -> PiWrToOdthF0R {
        PiWrToOdthF0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the delay from a write command to ODT assertion for frequency set 1."]
    #[inline(always)]
    pub fn pi_wr_to_odth_f1(&self) -> PiWrToOdthF1R {
        PiWrToOdthF1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Defines the delay from a write command to ODT assertion for frequency set 2."]
    #[inline(always)]
    pub fn pi_wr_to_odth_f2(&self) -> PiWrToOdthF2R {
        PiWrToOdthF2R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Defines the delay from a read command to ODT assertion for frequency set 0."]
    #[inline(always)]
    pub fn pi_rd_to_odth_f0(&self) -> PiRdToOdthF0R {
        PiRdToOdthF0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Defines the delay from a write command to ODT assertion for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wr_to_odth_f0(&mut self) -> PiWrToOdthF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec> {
        PiWrToOdthF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Defines the delay from a write command to ODT assertion for frequency set 1."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wr_to_odth_f1(&mut self) -> PiWrToOdthF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec> {
        PiWrToOdthF1W::new(self, 8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Defines the delay from a write command to ODT assertion for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wr_to_odth_f2(&mut self) -> PiWrToOdthF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec> {
        PiWrToOdthF2W::new(self, 16)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Defines the delay from a read command to ODT assertion for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rd_to_odth_f0(&mut self) -> PiRdToOdthF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec> {
        PiRdToOdthF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_187\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_187::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_187::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_187::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_187::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_187 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi187Spec {
    const RESET_VALUE: u32 = 0;
}

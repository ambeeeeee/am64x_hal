#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_222` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_222` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_RW_F0` reader - 9:0\\]
Switch time from read to write for frequency set 0."]
pub type PiTdfiWdqlvlRwF0R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_RW_F0` writer - 9:0\\]
Switch time from read to write for frequency set 0."]
pub type PiTdfiWdqlvlRwF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_START_POINT_F0` reader - 22:16\\]
Write DQ training vref initial training start value for frequency set 0."]
pub type PiWdqlvlVrefInitialStartPointF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_START_POINT_F0` writer - 22:16\\]
Write DQ training vref initial training start value for frequency set 0."]
pub type PiWdqlvlVrefInitialStartPointF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STOP_POINT_F0` reader - 30:24\\]
Write DQ training vref initial training stop value for frequency set 0."]
pub type PiWdqlvlVrefInitialStopPointF0R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STOP_POINT_F0` writer - 30:24\\]
Write DQ training vref initial training stop value for frequency set 0."]
pub type PiWdqlvlVrefInitialStopPointF0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Switch time from read to write for frequency set 0."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_rw_f0(&self) -> PiTdfiWdqlvlRwF0R {
        PiTdfiWdqlvlRwF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Write DQ training vref initial training start value for frequency set 0."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_start_point_f0(&self) -> PiWdqlvlVrefInitialStartPointF0R {
        PiWdqlvlVrefInitialStartPointF0R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Write DQ training vref initial training stop value for frequency set 0."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_stop_point_f0(&self) -> PiWdqlvlVrefInitialStopPointF0R {
        PiWdqlvlVrefInitialStopPointF0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Switch time from read to write for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_rw_f0(
        &mut self,
    ) -> PiTdfiWdqlvlRwF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec> {
        PiTdfiWdqlvlRwF0W::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Write DQ training vref initial training start value for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_start_point_f0(
        &mut self,
    ) -> PiWdqlvlVrefInitialStartPointF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec> {
        PiWdqlvlVrefInitialStartPointF0W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Write DQ training vref initial training stop value for frequency set 0."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_stop_point_f0(
        &mut self,
    ) -> PiWdqlvlVrefInitialStopPointF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec> {
        PiWdqlvlVrefInitialStopPointF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_222\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_222::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_222::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_222::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_222::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_222 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi222Spec {
    const RESET_VALUE: u32 = 0;
}

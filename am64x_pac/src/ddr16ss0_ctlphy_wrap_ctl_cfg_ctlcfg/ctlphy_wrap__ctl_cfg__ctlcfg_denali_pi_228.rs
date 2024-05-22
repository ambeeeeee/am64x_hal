#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_228` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_228` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec>;
#[doc = "Field `PI_TDFI_WDQLVL_RW_F2` reader - 9:0\\]
Switch time from read to write for frequency set 2."]
pub type PiTdfiWdqlvlRwF2R = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WDQLVL_RW_F2` writer - 9:0\\]
Switch time from read to write for frequency set 2."]
pub type PiTdfiWdqlvlRwF2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_START_POINT_F2` reader - 22:16\\]
Write DQ training vref initial training start value for frequency set 2."]
pub type PiWdqlvlVrefInitialStartPointF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_START_POINT_F2` writer - 22:16\\]
Write DQ training vref initial training start value for frequency set 2."]
pub type PiWdqlvlVrefInitialStartPointF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STOP_POINT_F2` reader - 30:24\\]
Write DQ training vref initial training stop value for frequency set 2."]
pub type PiWdqlvlVrefInitialStopPointF2R = crate::FieldReader;
#[doc = "Field `PI_WDQLVL_VREF_INITIAL_STOP_POINT_F2` writer - 30:24\\]
Write DQ training vref initial training stop value for frequency set 2."]
pub type PiWdqlvlVrefInitialStopPointF2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Switch time from read to write for frequency set 2."]
    #[inline(always)]
    pub fn pi_tdfi_wdqlvl_rw_f2(&self) -> PiTdfiWdqlvlRwF2R {
        PiTdfiWdqlvlRwF2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Write DQ training vref initial training start value for frequency set 2."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_start_point_f2(&self) -> PiWdqlvlVrefInitialStartPointF2R {
        PiWdqlvlVrefInitialStartPointF2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Write DQ training vref initial training stop value for frequency set 2."]
    #[inline(always)]
    pub fn pi_wdqlvl_vref_initial_stop_point_f2(&self) -> PiWdqlvlVrefInitialStopPointF2R {
        PiWdqlvlVrefInitialStopPointF2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Switch time from read to write for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wdqlvl_rw_f2(
        &mut self,
    ) -> PiTdfiWdqlvlRwF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec> {
        PiTdfiWdqlvlRwF2W::new(self, 0)
    }
    #[doc = "Bits 16:22 - 22:16\\]
Write DQ training vref initial training start value for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_start_point_f2(
        &mut self,
    ) -> PiWdqlvlVrefInitialStartPointF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec> {
        PiWdqlvlVrefInitialStartPointF2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - 30:24\\]
Write DQ training vref initial training stop value for frequency set 2."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wdqlvl_vref_initial_stop_point_f2(
        &mut self,
    ) -> PiWdqlvlVrefInitialStopPointF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec> {
        PiWdqlvlVrefInitialStopPointF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_228\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_228::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_228::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_228::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_228::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_228 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi228Spec {
    const RESET_VALUE: u32 = 0;
}

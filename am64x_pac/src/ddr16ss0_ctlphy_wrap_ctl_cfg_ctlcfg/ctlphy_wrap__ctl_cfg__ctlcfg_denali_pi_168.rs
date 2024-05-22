#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_168` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_168` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec>;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F1` reader - 7:0\\]
The delay from read or write to bus idle for frequency set 1. Recommend setting is: delay time from read command issued to last read data received."]
pub type PiTdelayRdwr2BusIdleF1R = crate::FieldReader;
#[doc = "Field `PI_TDELAY_RDWR_2_BUS_IDLE_F1` writer - 7:0\\]
The delay from read or write to bus idle for frequency set 1. Recommend setting is: delay time from read command issued to last read data received."]
pub type PiTdelayRdwr2BusIdleF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The delay from read or write to bus idle for frequency set 1. Recommend setting is: delay time from read command issued to last read data received."]
    #[inline(always)]
    pub fn pi_tdelay_rdwr_2_bus_idle_f1(&self) -> PiTdelayRdwr2BusIdleF1R {
        PiTdelayRdwr2BusIdleF1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The delay from read or write to bus idle for frequency set 1. Recommend setting is: delay time from read command issued to last read data received."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdelay_rdwr_2_bus_idle_f1(
        &mut self,
    ) -> PiTdelayRdwr2BusIdleF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec> {
        PiTdelayRdwr2BusIdleF1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_168\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_168::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_168::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_168::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_168::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_168 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi168Spec {
    const RESET_VALUE: u32 = 0;
}

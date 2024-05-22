#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_14` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_14` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec>;
#[doc = "Field `TINIT5_F1` reader - 23:0\\]
DRAM TINIT5 value in cycles. FC=1"]
pub type Tinit5F1R = crate::FieldReader<u32>;
#[doc = "Field `TINIT5_F1` writer - 23:0\\]
DRAM TINIT5 value in cycles. FC=1"]
pub type Tinit5F1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM TINIT5 value in cycles. FC=1"]
    #[inline(always)]
    pub fn tinit5_f1(&self) -> Tinit5F1R {
        Tinit5F1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM TINIT5 value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tinit5_f1(&mut self) -> Tinit5F1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec> {
        Tinit5F1W::new(self, 0)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_14::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_14::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_14 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl14Spec {
    const RESET_VALUE: u32 = 0;
}
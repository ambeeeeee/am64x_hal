#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_61` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_61` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec>;
#[doc = "Field `TRAS_MAX_F2` reader - 19:0\\]
DRAM TRAS_MAX value in cycles. FC=2"]
pub type TrasMaxF2R = crate::FieldReader<u32>;
#[doc = "Field `TRAS_MAX_F2` writer - 19:0\\]
DRAM TRAS_MAX value in cycles. FC=2"]
pub type TrasMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `TCKE_F2` reader - 28:24\\]
Minimum CKE pulse width. FC=2"]
pub type TckeF2R = crate::FieldReader;
#[doc = "Field `TCKE_F2` writer - 28:24\\]
Minimum CKE pulse width. FC=2"]
pub type TckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM TRAS_MAX value in cycles. FC=2"]
    #[inline(always)]
    pub fn tras_max_f2(&self) -> TrasMaxF2R {
        TrasMaxF2R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Minimum CKE pulse width. FC=2"]
    #[inline(always)]
    pub fn tcke_f2(&self) -> TckeF2R {
        TckeF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM TRAS_MAX value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tras_max_f2(&mut self) -> TrasMaxF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec> {
        TrasMaxF2W::new(self, 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Minimum CKE pulse width. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tcke_f2(&mut self) -> TckeF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec> {
        TckeF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_61\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_61::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_61::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_61::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_61::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_61 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl61Spec {
    const RESET_VALUE: u32 = 0;
}

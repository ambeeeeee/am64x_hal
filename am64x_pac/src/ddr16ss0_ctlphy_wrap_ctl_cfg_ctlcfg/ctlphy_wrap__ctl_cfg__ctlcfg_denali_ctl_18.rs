#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_18` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_18` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec>;
#[doc = "Field `TINIT5_F2` reader - 23:0\\]
DRAM TINIT5 value in cycles. FC=2"]
pub type Tinit5F2R = crate::FieldReader<u32>;
#[doc = "Field `TINIT5_F2` writer - 23:0\\]
DRAM TINIT5 value in cycles. FC=2"]
pub type Tinit5F2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NO_AUTO_MRR_INIT` reader - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
pub type NoAutoMrrInitR = crate::BitReader;
#[doc = "Field `NO_AUTO_MRR_INIT` writer - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
pub type NoAutoMrrInitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM TINIT5 value in cycles. FC=2"]
    #[inline(always)]
    pub fn tinit5_f2(&self) -> Tinit5F2R {
        Tinit5F2R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    pub fn no_auto_mrr_init(&self) -> NoAutoMrrInitR {
        NoAutoMrrInitR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
DRAM TINIT5 value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tinit5_f2(&mut self) -> Tinit5F2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec> {
        Tinit5F2W::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Disable MRR commands during initialization. Set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn no_auto_mrr_init(&mut self) -> NoAutoMrrInitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec> {
        NoAutoMrrInitW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_18::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_18::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_18::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_18::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_18 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl18Spec {
    const RESET_VALUE: u32 = 0;
}

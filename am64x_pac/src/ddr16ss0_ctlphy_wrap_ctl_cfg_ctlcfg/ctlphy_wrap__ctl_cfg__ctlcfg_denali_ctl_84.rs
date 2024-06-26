#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_84` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_84` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec>;
#[doc = "Field `TREFI_PB_F2` reader - 19:0\\]
DRAM TREFI_PB value in cycles. FC=2"]
pub type TrefiPbF2R = crate::FieldReader<u32>;
#[doc = "Field `TREFI_PB_F2` writer - 19:0\\]
DRAM TREFI_PB value in cycles. FC=2"]
pub type TrefiPbF2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `PBR_EN` reader - 24:24\\]
Enables the per-bank refresh feature. Set to 1 to enable."]
pub type PbrEnR = crate::BitReader;
#[doc = "Field `PBR_EN` writer - 24:24\\]
Enables the per-bank refresh feature. Set to 1 to enable."]
pub type PbrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM TREFI_PB value in cycles. FC=2"]
    #[inline(always)]
    pub fn trefi_pb_f2(&self) -> TrefiPbF2R {
        TrefiPbF2R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the per-bank refresh feature. Set to 1 to enable."]
    #[inline(always)]
    pub fn pbr_en(&self) -> PbrEnR {
        PbrEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
DRAM TREFI_PB value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trefi_pb_f2(&mut self) -> TrefiPbF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec> {
        TrefiPbF2W::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enables the per-bank refresh feature. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn pbr_en(&mut self) -> PbrEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec> {
        PbrEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_84\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_84::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_84::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_84::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_84::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_84 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl84Spec {
    const RESET_VALUE: u32 = 0;
}

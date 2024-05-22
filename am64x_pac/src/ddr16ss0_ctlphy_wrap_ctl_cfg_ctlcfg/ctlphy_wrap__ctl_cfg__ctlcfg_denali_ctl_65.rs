#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_65` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_65` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec>;
#[doc = "Field `TMRR` reader - 3:0\\]
DRAM TMRR value in cycles."]
pub type TmrrR = crate::FieldReader;
#[doc = "Field `TMRR` writer - 3:0\\]
DRAM TMRR value in cycles."]
pub type TmrrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AP` reader - 8:8\\]
Enable auto pre-charge mode of controller. Set to 1 to enable."]
pub type ApR = crate::BitReader;
#[doc = "Field `AP` writer - 8:8\\]
Enable auto pre-charge mode of controller. Set to 1 to enable."]
pub type ApW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONCURRENTAP` reader - 16:16\\]
IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
pub type ConcurrentapR = crate::BitReader;
#[doc = "Field `CONCURRENTAP` writer - 16:16\\]
IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
pub type ConcurrentapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRAS_LOCKOUT` reader - 24:24\\]
IF the DRAM supports it, this allows the controller to execute auto pre-charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
pub type TrasLockoutR = crate::BitReader;
#[doc = "Field `TRAS_LOCKOUT` writer - 24:24\\]
IF the DRAM supports it, this allows the controller to execute auto pre-charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
pub type TrasLockoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
DRAM TMRR value in cycles."]
    #[inline(always)]
    pub fn tmrr(&self) -> TmrrR {
        TmrrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable auto pre-charge mode of controller. Set to 1 to enable."]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
    #[inline(always)]
    pub fn concurrentap(&self) -> ConcurrentapR {
        ConcurrentapR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
IF the DRAM supports it, this allows the controller to execute auto pre-charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
    #[inline(always)]
    pub fn tras_lockout(&self) -> TrasLockoutR {
        TrasLockoutR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
DRAM TMRR value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrr(&mut self) -> TmrrW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec> {
        TmrrW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable auto pre-charge mode of controller. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec> {
        ApW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
IF the DRAM supports it, this allows the controller to issue commands to other banks while a bank is in auto pre-charge. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn concurrentap(&mut self) -> ConcurrentapW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec> {
        ConcurrentapW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
IF the DRAM supports it, this allows the controller to execute auto pre-charge commands before the TRAS_MIN parameter expires. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn tras_lockout(&mut self) -> TrasLockoutW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec> {
        TrasLockoutW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_65::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_65::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_65 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl65Spec {
    const RESET_VALUE: u32 = 0;
}

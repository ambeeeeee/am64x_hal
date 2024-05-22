#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_59` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_59` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec>;
#[doc = "Field `TCKESR_F1` reader - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=1"]
pub type TckesrF1R = crate::FieldReader;
#[doc = "Field `TCKESR_F1` writer - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=1"]
pub type TckesrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCCDMW_F1` reader - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=1"]
pub type TccdmwF1R = crate::FieldReader;
#[doc = "Field `TCCDMW_F1` writer - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=1"]
pub type TccdmwF1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRTP_F2` reader - 23:16\\]
DRAM TRTP value in cycles. FC=2"]
pub type TrtpF2R = crate::FieldReader;
#[doc = "Field `TRTP_F2` writer - 23:16\\]
DRAM TRTP value in cycles. FC=2"]
pub type TrtpF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRTP_AP_F2` reader - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=2"]
pub type TrtpApF2R = crate::FieldReader;
#[doc = "Field `TRTP_AP_F2` writer - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=2"]
pub type TrtpApF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=1"]
    #[inline(always)]
    pub fn tckesr_f1(&self) -> TckesrF1R {
        TckesrF1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=1"]
    #[inline(always)]
    pub fn tccdmw_f1(&self) -> TccdmwF1R {
        TccdmwF1R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRTP value in cycles. FC=2"]
    #[inline(always)]
    pub fn trtp_f2(&self) -> TrtpF2R {
        TrtpF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=2"]
    #[inline(always)]
    pub fn trtp_ap_f2(&self) -> TrtpApF2R {
        TrtpApF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckesr_f1(&mut self) -> TckesrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec> {
        TckesrF1W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tccdmw_f1(&mut self) -> TccdmwF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec> {
        TccdmwF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRTP value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trtp_f2(&mut self) -> TrtpF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec> {
        TrtpF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn trtp_ap_f2(&mut self) -> TrtpApF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec> {
        TrtpApF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_59\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_59::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_59::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_59::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_59::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_59 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl59Spec {
    const RESET_VALUE: u32 = 0;
}

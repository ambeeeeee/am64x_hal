#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_56` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_56` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec>;
#[doc = "Field `TCKESR_F0` reader - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=0"]
pub type TckesrF0R = crate::FieldReader;
#[doc = "Field `TCKESR_F0` writer - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=0"]
pub type TckesrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCCDMW_F0` reader - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=0"]
pub type TccdmwF0R = crate::FieldReader;
#[doc = "Field `TCCDMW_F0` writer - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=0"]
pub type TccdmwF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TRTP_F1` reader - 23:16\\]
DRAM TRTP value in cycles. FC=1"]
pub type TrtpF1R = crate::FieldReader;
#[doc = "Field `TRTP_F1` writer - 23:16\\]
DRAM TRTP value in cycles. FC=1"]
pub type TrtpF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TRTP_AP_F1` reader - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=1"]
pub type TrtpApF1R = crate::FieldReader;
#[doc = "Field `TRTP_AP_F1` writer - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=1"]
pub type TrtpApF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=0"]
    #[inline(always)]
    pub fn tckesr_f0(&self) -> TckesrF0R {
        TckesrF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=0"]
    #[inline(always)]
    pub fn tccdmw_f0(&self) -> TccdmwF0R {
        TccdmwF0R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRTP value in cycles. FC=1"]
    #[inline(always)]
    pub fn trtp_f1(&self) -> TrtpF1R {
        TrtpF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=1"]
    #[inline(always)]
    pub fn trtp_ap_f1(&self) -> TrtpApF1R {
        TrtpApF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Minimum CKE low pulse width during a self-refresh. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckesr_f0(&mut self) -> TckesrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec> {
        TckesrF0W::new(self, 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
DRAM CAS-to-CAS masked write value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tccdmw_f0(&mut self) -> TccdmwF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec> {
        TccdmwF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
DRAM TRTP value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn trtp_f1(&mut self) -> TrtpF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec> {
        TrtpF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TRTP for auto-precharge value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn trtp_ap_f1(&mut self) -> TrtpApF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec> {
        TrtpApF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_56::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_56::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_56 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl56Spec {
    const RESET_VALUE: u32 = 0;
}

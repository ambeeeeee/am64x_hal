#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_54` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_54` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec>;
#[doc = "Field `TRTP_AP_F0` reader - 7:0\\]
DRAM TRTP for auto-precharge value in cycles. FC=0"]
pub type TrtpApF0R = crate::FieldReader;
#[doc = "Field `TRTP_AP_F0` writer - 7:0\\]
DRAM TRTP for auto-precharge value in cycles. FC=0"]
pub type TrtpApF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRD_F0` reader - 15:8\\]
DRAM TMRD value in cycles. FC=0"]
pub type TmrdF0R = crate::FieldReader;
#[doc = "Field `TMRD_F0` writer - 15:8\\]
DRAM TMRD value in cycles. FC=0"]
pub type TmrdF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMOD_F0` reader - 23:16\\]
Number of cycles after MRS command and before any other command. FC=0"]
pub type TmodF0R = crate::FieldReader;
#[doc = "Field `TMOD_F0` writer - 23:16\\]
Number of cycles after MRS command and before any other command. FC=0"]
pub type TmodF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRTP for auto-precharge value in cycles. FC=0"]
    #[inline(always)]
    pub fn trtp_ap_f0(&self) -> TrtpApF0R {
        TrtpApF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value in cycles. FC=0"]
    #[inline(always)]
    pub fn tmrd_f0(&self) -> TmrdF0R {
        TmrdF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Number of cycles after MRS command and before any other command. FC=0"]
    #[inline(always)]
    pub fn tmod_f0(&self) -> TmodF0R {
        TmodF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TRTP for auto-precharge value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn trtp_ap_f0(&mut self) -> TrtpApF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec> {
        TrtpApF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
DRAM TMRD value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd_f0(&mut self) -> TmrdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec> {
        TmrdF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Number of cycles after MRS command and before any other command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tmod_f0(&mut self) -> TmodF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec> {
        TmodF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_54\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_54::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_54::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_54::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_54::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_54 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl54Spec {
    const RESET_VALUE: u32 = 0;
}

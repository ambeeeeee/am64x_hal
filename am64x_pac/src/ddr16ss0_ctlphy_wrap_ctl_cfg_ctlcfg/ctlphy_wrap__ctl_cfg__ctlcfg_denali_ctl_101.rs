#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_101` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_101` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec>;
#[doc = "Field `TCKEHCMD_F0` reader - 4:0\\]
DRAM TCKEHCMD value in cycles. FC=0"]
pub type TckehcmdF0R = crate::FieldReader;
#[doc = "Field `TCKEHCMD_F0` writer - 4:0\\]
DRAM TCKEHCMD value in cycles. FC=0"]
pub type TckehcmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKCKEL_F0` reader - 12:8\\]
DRAM TCKCKEL value in cycles. FC=0"]
pub type TckckelF0R = crate::FieldReader;
#[doc = "Field `TCKCKEL_F0` writer - 12:8\\]
DRAM TCKCKEL value in cycles. FC=0"]
pub type TckckelF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKELPD_F0` reader - 20:16\\]
DRAM TCKELPD value in cycles. FC=0"]
pub type TckelpdF0R = crate::FieldReader;
#[doc = "Field `TCKELPD_F0` writer - 20:16\\]
DRAM TCKELPD value in cycles. FC=0"]
pub type TckelpdF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSR_F1` reader - 31:24\\]
DRAM TSR value in cycles. FC=1"]
pub type TsrF1R = crate::FieldReader;
#[doc = "Field `TSR_F1` writer - 31:24\\]
DRAM TSR value in cycles. FC=1"]
pub type TsrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCMD value in cycles. FC=0"]
    #[inline(always)]
    pub fn tckehcmd_f0(&self) -> TckehcmdF0R {
        TckehcmdF0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCKCKEL value in cycles. FC=0"]
    #[inline(always)]
    pub fn tckckel_f0(&self) -> TckckelF0R {
        TckckelF0R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCKELPD value in cycles. FC=0"]
    #[inline(always)]
    pub fn tckelpd_f0(&self) -> TckelpdF0R {
        TckelpdF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TSR value in cycles. FC=1"]
    #[inline(always)]
    pub fn tsr_f1(&self) -> TsrF1R {
        TsrF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCKEHCMD value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckehcmd_f0(&mut self) -> TckehcmdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec> {
        TckehcmdF0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCKCKEL value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckckel_f0(&mut self) -> TckckelF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec> {
        TckckelF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCKELPD value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckelpd_f0(&mut self) -> TckelpdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec> {
        TckelpdF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DRAM TSR value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tsr_f1(&mut self) -> TsrF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec> {
        TsrF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_101\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_101::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_101::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_101::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_101::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_101 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl101Spec {
    const RESET_VALUE: u32 = 0;
}

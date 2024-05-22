#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_102` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_102` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec>;
#[doc = "Field `TESCKE_F1` reader - 2:0\\]
DRAM TESCKE value in cycles. FC=1"]
pub type TesckeF1R = crate::FieldReader;
#[doc = "Field `TESCKE_F1` writer - 2:0\\]
DRAM TESCKE value in cycles. FC=1"]
pub type TesckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCSCKEH_F1` reader - 12:8\\]
DRAM TCSCKEH value in cycles. FC=1"]
pub type TcsckehF1R = crate::FieldReader;
#[doc = "Field `TCSCKEH_F1` writer - 12:8\\]
DRAM TCSCKEH value in cycles. FC=1"]
pub type TcsckehF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKELCMD_F1` reader - 20:16\\]
DRAM TCKELCMD value in cycles. FC=1"]
pub type TckelcmdF1R = crate::FieldReader;
#[doc = "Field `TCKELCMD_F1` writer - 20:16\\]
DRAM TCKELCMD value in cycles. FC=1"]
pub type TckelcmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKEHCMD_F1` reader - 28:24\\]
DRAM TCKEHCMD value in cycles. FC=1"]
pub type TckehcmdF1R = crate::FieldReader;
#[doc = "Field `TCKEHCMD_F1` writer - 28:24\\]
DRAM TCKEHCMD value in cycles. FC=1"]
pub type TckehcmdF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DRAM TESCKE value in cycles. FC=1"]
    #[inline(always)]
    pub fn tescke_f1(&self) -> TesckeF1R {
        TesckeF1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCSCKEH value in cycles. FC=1"]
    #[inline(always)]
    pub fn tcsckeh_f1(&self) -> TcsckehF1R {
        TcsckehF1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCKELCMD value in cycles. FC=1"]
    #[inline(always)]
    pub fn tckelcmd_f1(&self) -> TckelcmdF1R {
        TckelcmdF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKEHCMD value in cycles. FC=1"]
    #[inline(always)]
    pub fn tckehcmd_f1(&self) -> TckehcmdF1R {
        TckehcmdF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DRAM TESCKE value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tescke_f1(&mut self) -> TesckeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec> {
        TesckeF1W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCSCKEH value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tcsckeh_f1(&mut self) -> TcsckehF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec> {
        TcsckehF1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCKELCMD value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckelcmd_f1(&mut self) -> TckelcmdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec> {
        TckelcmdF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKEHCMD value in cycles. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckehcmd_f1(&mut self) -> TckehcmdF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec> {
        TckehcmdF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_102\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_102::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_102::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_102::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_102::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_102 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl102Spec {
    const RESET_VALUE: u32 = 0;
}

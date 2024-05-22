#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_100` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_100` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec>;
#[doc = "Field `TSR_F0` reader - 7:0\\]
DRAM TSR value in cycles. FC=0"]
pub type TsrF0R = crate::FieldReader;
#[doc = "Field `TSR_F0` writer - 7:0\\]
DRAM TSR value in cycles. FC=0"]
pub type TsrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TESCKE_F0` reader - 10:8\\]
DRAM TESCKE value in cycles. FC=0"]
pub type TesckeF0R = crate::FieldReader;
#[doc = "Field `TESCKE_F0` writer - 10:8\\]
DRAM TESCKE value in cycles. FC=0"]
pub type TesckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCSCKEH_F0` reader - 20:16\\]
DRAM TCSCKEH value in cycles. FC=0"]
pub type TcsckehF0R = crate::FieldReader;
#[doc = "Field `TCSCKEH_F0` writer - 20:16\\]
DRAM TCSCKEH value in cycles. FC=0"]
pub type TcsckehF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKELCMD_F0` reader - 28:24\\]
DRAM TCKELCMD value in cycles. FC=0"]
pub type TckelcmdF0R = crate::FieldReader;
#[doc = "Field `TCKELCMD_F0` writer - 28:24\\]
DRAM TCKELCMD value in cycles. FC=0"]
pub type TckelcmdF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TSR value in cycles. FC=0"]
    #[inline(always)]
    pub fn tsr_f0(&self) -> TsrF0R {
        TsrF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DRAM TESCKE value in cycles. FC=0"]
    #[inline(always)]
    pub fn tescke_f0(&self) -> TesckeF0R {
        TesckeF0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCSCKEH value in cycles. FC=0"]
    #[inline(always)]
    pub fn tcsckeh_f0(&self) -> TcsckehF0R {
        TcsckehF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCMD value in cycles. FC=0"]
    #[inline(always)]
    pub fn tckelcmd_f0(&self) -> TckelcmdF0R {
        TckelcmdF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
DRAM TSR value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tsr_f0(&mut self) -> TsrF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec> {
        TsrF0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
DRAM TESCKE value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tescke_f0(&mut self) -> TesckeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec> {
        TesckeF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCSCKEH value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tcsckeh_f0(&mut self) -> TcsckehF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec> {
        TcsckehF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKELCMD value in cycles. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckelcmd_f0(&mut self) -> TckelcmdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec> {
        TckelcmdF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_100::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_100::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_100 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl100Spec {
    const RESET_VALUE: u32 = 0;
}

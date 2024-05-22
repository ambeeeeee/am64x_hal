#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_104` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_104` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec>;
#[doc = "Field `TCSCKEH_F2` reader - 4:0\\]
DRAM TCSCKEH value in cycles. FC=2"]
pub type TcsckehF2R = crate::FieldReader;
#[doc = "Field `TCSCKEH_F2` writer - 4:0\\]
DRAM TCSCKEH value in cycles. FC=2"]
pub type TcsckehF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKELCMD_F2` reader - 12:8\\]
DRAM TCKELCMD value in cycles. FC=2"]
pub type TckelcmdF2R = crate::FieldReader;
#[doc = "Field `TCKELCMD_F2` writer - 12:8\\]
DRAM TCKELCMD value in cycles. FC=2"]
pub type TckelcmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKEHCMD_F2` reader - 20:16\\]
DRAM TCKEHCMD value in cycles. FC=2"]
pub type TckehcmdF2R = crate::FieldReader;
#[doc = "Field `TCKEHCMD_F2` writer - 20:16\\]
DRAM TCKEHCMD value in cycles. FC=2"]
pub type TckehcmdF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKCKEL_F2` reader - 28:24\\]
DRAM TCKCKEL value in cycles. FC=2"]
pub type TckckelF2R = crate::FieldReader;
#[doc = "Field `TCKCKEL_F2` writer - 28:24\\]
DRAM TCKCKEL value in cycles. FC=2"]
pub type TckckelF2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCSCKEH value in cycles. FC=2"]
    #[inline(always)]
    pub fn tcsckeh_f2(&self) -> TcsckehF2R {
        TcsckehF2R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCKELCMD value in cycles. FC=2"]
    #[inline(always)]
    pub fn tckelcmd_f2(&self) -> TckelcmdF2R {
        TckelcmdF2R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCKEHCMD value in cycles. FC=2"]
    #[inline(always)]
    pub fn tckehcmd_f2(&self) -> TckehcmdF2R {
        TckehcmdF2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKCKEL value in cycles. FC=2"]
    #[inline(always)]
    pub fn tckckel_f2(&self) -> TckckelF2R {
        TckckelF2R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
DRAM TCSCKEH value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tcsckeh_f2(&mut self) -> TcsckehF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec> {
        TcsckehF2W::new(self, 0)
    }
    #[doc = "Bits 8:12 - 12:8\\]
DRAM TCKELCMD value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckelcmd_f2(&mut self) -> TckelcmdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec> {
        TckelcmdF2W::new(self, 8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
DRAM TCKEHCMD value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckehcmd_f2(&mut self) -> TckehcmdF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec> {
        TckehcmdF2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
DRAM TCKCKEL value in cycles. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tckckel_f2(&mut self) -> TckckelF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec> {
        TckckelF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_104\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_104::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_104::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_104::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_104::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_104 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl104Spec {
    const RESET_VALUE: u32 = 0;
}

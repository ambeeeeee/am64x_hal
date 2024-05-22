#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_193` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_193` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec>;
#[doc = "Field `TFC_F0` reader - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=0"]
pub type TfcF0R = crate::FieldReader<u16>;
#[doc = "Field `TFC_F0` writer - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=0"]
pub type TfcF0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCKFSPE_F0` reader - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=0"]
pub type TckfspeF0R = crate::FieldReader;
#[doc = "Field `TCKFSPE_F0` writer - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=0"]
pub type TckfspeF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKFSPX_F0` reader - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=0"]
pub type TckfspxF0R = crate::FieldReader;
#[doc = "Field `TCKFSPX_F0` writer - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=0"]
pub type TckfspxF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=0"]
    #[inline(always)]
    pub fn tfc_f0(&self) -> TfcF0R {
        TfcF0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=0"]
    #[inline(always)]
    pub fn tckfspe_f0(&self) -> TckfspeF0R {
        TckfspeF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=0"]
    #[inline(always)]
    pub fn tckfspx_f0(&self) -> TckfspxF0R {
        TckfspxF0R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tfc_f0(&mut self) -> TfcF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec> {
        TfcF0W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckfspe_f0(&mut self) -> TckfspeF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec> {
        TckfspeF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tckfspx_f0(&mut self) -> TckfspxF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec> {
        TckfspxF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_193\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_193::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_193::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_193::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_193::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_193 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl193Spec {
    const RESET_VALUE: u32 = 0;
}

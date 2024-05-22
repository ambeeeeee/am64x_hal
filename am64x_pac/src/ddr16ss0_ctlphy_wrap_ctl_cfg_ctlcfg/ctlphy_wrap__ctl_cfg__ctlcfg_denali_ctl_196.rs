#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_196` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_196` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec>;
#[doc = "Field `TFC_F1` reader - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=1"]
pub type TfcF1R = crate::FieldReader<u16>;
#[doc = "Field `TFC_F1` writer - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=1"]
pub type TfcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCKFSPE_F1` reader - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=1"]
pub type TckfspeF1R = crate::FieldReader;
#[doc = "Field `TCKFSPE_F1` writer - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=1"]
pub type TckfspeF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCKFSPX_F1` reader - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=1"]
pub type TckfspxF1R = crate::FieldReader;
#[doc = "Field `TCKFSPX_F1` writer - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=1"]
pub type TckfspxF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=1"]
    #[inline(always)]
    pub fn tfc_f1(&self) -> TfcF1R {
        TfcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:20 - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=1"]
    #[inline(always)]
    pub fn tckfspe_f1(&self) -> TckfspeF1R {
        TckfspeF1R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=1"]
    #[inline(always)]
    pub fn tckfspx_f1(&self) -> TckfspxF1R {
        TckfspxF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
JEDEC TFC, the frequency set point switching time. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tfc_f1(&mut self) -> TfcF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec> {
        TfcF1W::new(self, 0)
    }
    #[doc = "Bits 16:20 - 20:16\\]
JEDEC TCKFSPE, the frequency set point switching time. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckfspe_f1(&mut self) -> TckfspeF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec> {
        TckfspeF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
JEDEC TCKFSPX, the frequency set point switching time. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tckfspx_f1(&mut self) -> TckfspxF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec> {
        TckfspxF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_196\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_196::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_196::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_196::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_196::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_196 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl196Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_99` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_99` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec>;
#[doc = "Field `TXPR_F1` reader - 15:0\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=1"]
pub type TxprF1R = crate::FieldReader<u16>;
#[doc = "Field `TXPR_F1` writer - 15:0\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=1"]
pub type TxprF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXPR_F2` reader - 31:16\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=2"]
pub type TxprF2R = crate::FieldReader<u16>;
#[doc = "Field `TXPR_F2` writer - 31:16\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=2"]
pub type TxprF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=1"]
    #[inline(always)]
    pub fn txpr_f1(&self) -> TxprF1R {
        TxprF1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=2"]
    #[inline(always)]
    pub fn txpr_f2(&self) -> TxprF2R {
        TxprF2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn txpr_f1(&mut self) -> TxprF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec> {
        TxprF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
DRAM TXPR value in cycles. This parameter defines reset exit time from CKE HIGH to a valid command. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn txpr_f2(&mut self) -> TxprF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec> {
        TxprF2W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_99::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_99::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_99 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl99Spec {
    const RESET_VALUE: u32 = 0;
}

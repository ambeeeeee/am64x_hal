#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_401` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_401` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec>;
#[doc = "Field `TDFI_CTRL_DELAY_F1` reader - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=1"]
pub type TdfiCtrlDelayF1R = crate::FieldReader;
#[doc = "Field `TDFI_CTRL_DELAY_F1` writer - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=1"]
pub type TdfiCtrlDelayF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_PHY_WRDATA_F1` reader - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=1"]
pub type TdfiPhyWrdataF1R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRDATA_F1` writer - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=1"]
pub type TdfiPhyWrdataF1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDFI_RDCSLAT_F1` reader - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=1"]
pub type TdfiRdcslatF1R = crate::FieldReader;
#[doc = "Field `TDFI_RDCSLAT_F1` writer - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=1"]
pub type TdfiRdcslatF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_RDDATA_EN_F1` reader - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=1"]
pub type TdfiRddataEnF1R = crate::FieldReader;
#[doc = "Field `TDFI_RDDATA_EN_F1` writer - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=1"]
pub type TdfiRddataEnF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=1"]
    #[inline(always)]
    pub fn tdfi_ctrl_delay_f1(&self) -> TdfiCtrlDelayF1R {
        TdfiCtrlDelayF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=1"]
    #[inline(always)]
    pub fn tdfi_phy_wrdata_f1(&self) -> TdfiPhyWrdataF1R {
        TdfiPhyWrdataF1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=1"]
    #[inline(always)]
    pub fn tdfi_rdcslat_f1(&self) -> TdfiRdcslatF1R {
        TdfiRdcslatF1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=1"]
    #[inline(always)]
    pub fn tdfi_rddata_en_f1(&self) -> TdfiRddataEnF1R {
        TdfiRddataEnF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrl_delay_f1(
        &mut self,
    ) -> TdfiCtrlDelayF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec> {
        TdfiCtrlDelayF1W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_wrdata_f1(
        &mut self,
    ) -> TdfiPhyWrdataF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec> {
        TdfiPhyWrdataF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdcslat_f1(&mut self) -> TdfiRdcslatF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec> {
        TdfiRdcslatF1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=1"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rddata_en_f1(
        &mut self,
    ) -> TdfiRddataEnF1W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec> {
        TdfiRddataEnF1W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_401\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_401::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_401::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_401::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_401::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_401 to value 0x02"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl401Spec {
    const RESET_VALUE: u32 = 0x02;
}

#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_410` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_410` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec>;
#[doc = "Field `TDFI_CTRL_DELAY_F2` reader - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=2"]
pub type TdfiCtrlDelayF2R = crate::FieldReader;
#[doc = "Field `TDFI_CTRL_DELAY_F2` writer - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=2"]
pub type TdfiCtrlDelayF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_PHY_WRDATA_F2` reader - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=2"]
pub type TdfiPhyWrdataF2R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRDATA_F2` writer - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=2"]
pub type TdfiPhyWrdataF2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDFI_RDCSLAT_F2` reader - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=2"]
pub type TdfiRdcslatF2R = crate::FieldReader;
#[doc = "Field `TDFI_RDCSLAT_F2` writer - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=2"]
pub type TdfiRdcslatF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_RDDATA_EN_F2` reader - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=2"]
pub type TdfiRddataEnF2R = crate::FieldReader;
#[doc = "Field `TDFI_RDDATA_EN_F2` writer - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=2"]
pub type TdfiRddataEnF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=2"]
    #[inline(always)]
    pub fn tdfi_ctrl_delay_f2(&self) -> TdfiCtrlDelayF2R {
        TdfiCtrlDelayF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=2"]
    #[inline(always)]
    pub fn tdfi_phy_wrdata_f2(&self) -> TdfiPhyWrdataF2R {
        TdfiPhyWrdataF2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=2"]
    #[inline(always)]
    pub fn tdfi_rdcslat_f2(&self) -> TdfiRdcslatF2R {
        TdfiRdcslatF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=2"]
    #[inline(always)]
    pub fn tdfi_rddata_en_f2(&self) -> TdfiRddataEnF2R {
        TdfiRddataEnF2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrl_delay_f2(
        &mut self,
    ) -> TdfiCtrlDelayF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec> {
        TdfiCtrlDelayF2W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_wrdata_f2(
        &mut self,
    ) -> TdfiPhyWrdataF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec> {
        TdfiPhyWrdataF2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdcslat_f2(&mut self) -> TdfiRdcslatF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec> {
        TdfiRdcslatF2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rddata_en_f2(
        &mut self,
    ) -> TdfiRddataEnF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec> {
        TdfiRddataEnF2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_410\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_410::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_410::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_410::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_410::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_410 to value 0x02"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl410Spec {
    const RESET_VALUE: u32 = 0x02;
}

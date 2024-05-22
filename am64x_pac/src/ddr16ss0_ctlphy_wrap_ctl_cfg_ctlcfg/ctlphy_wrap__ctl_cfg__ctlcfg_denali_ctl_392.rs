#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_392` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_392` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec>;
#[doc = "Field `TDFI_CTRL_DELAY_F0` reader - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=0"]
pub type TdfiCtrlDelayF0R = crate::FieldReader;
#[doc = "Field `TDFI_CTRL_DELAY_F0` writer - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=0"]
pub type TdfiCtrlDelayF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TDFI_PHY_WRDATA_F0` reader - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=0"]
pub type TdfiPhyWrdataF0R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_WRDATA_F0` writer - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=0"]
pub type TdfiPhyWrdataF0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TDFI_RDCSLAT_F0` reader - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=0"]
pub type TdfiRdcslatF0R = crate::FieldReader;
#[doc = "Field `TDFI_RDCSLAT_F0` writer - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=0"]
pub type TdfiRdcslatF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TDFI_RDDATA_EN_F0` reader - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=0"]
pub type TdfiRddataEnF0R = crate::FieldReader;
#[doc = "Field `TDFI_RDDATA_EN_F0` writer - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=0"]
pub type TdfiRddataEnF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=0"]
    #[inline(always)]
    pub fn tdfi_ctrl_delay_f0(&self) -> TdfiCtrlDelayF0R {
        TdfiCtrlDelayF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=0"]
    #[inline(always)]
    pub fn tdfi_phy_wrdata_f0(&self) -> TdfiPhyWrdataF0R {
        TdfiPhyWrdataF0R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=0"]
    #[inline(always)]
    pub fn tdfi_rdcslat_f0(&self) -> TdfiRdcslatF0R {
        TdfiRdcslatF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=0"]
    #[inline(always)]
    pub fn tdfi_rddata_en_f0(&self) -> TdfiRddataEnF0R {
        TdfiRddataEnF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the DFI tCTRL_DELAY timing parameter \\[in DFI clocks\\], the delay between a DFI command change and a memory command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_ctrl_delay_f0(
        &mut self,
    ) -> TdfiCtrlDelayF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec> {
        TdfiCtrlDelayF0W::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Defines the DFI tPHY_WRDATA timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_wrdata_en assertion and a dfi_wrdata signal. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_wrdata_f0(
        &mut self,
    ) -> TdfiPhyWrdataF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec> {
        TdfiPhyWrdataF0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Defines the DFI tPHY_RDCSLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a read command and a dfi_rddata_cs_n assertion. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdcslat_f0(&mut self) -> TdfiRdcslatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec> {
        TdfiRdcslatF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
DFI tRDDATA_EN timing parameter. This is the number of DFI data phases between a read command and the first assertion of dfi_rddata_en_pN. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rddata_en_f0(
        &mut self,
    ) -> TdfiRddataEnF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec> {
        TdfiRddataEnF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_392\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_392::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_392::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_392::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_392::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_392 to value 0x02"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl392Spec {
    const RESET_VALUE: u32 = 0x02;
}

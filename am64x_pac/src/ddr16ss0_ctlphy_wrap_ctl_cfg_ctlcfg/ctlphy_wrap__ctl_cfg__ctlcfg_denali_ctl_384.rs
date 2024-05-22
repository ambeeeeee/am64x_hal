#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_384` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_384` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec>;
#[doc = "Field `AXI0_W_PRIORITY` reader - 2:0\\]
Priority of write commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
pub type Axi0WPriorityR = crate::FieldReader;
#[doc = "Field `AXI0_W_PRIORITY` writer - 2:0\\]
Priority of write commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
pub type Axi0WPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKE_STATUS` reader - 9:8\\]
Register access to cke_status signal. READ-ONLY"]
pub type CkeStatusR = crate::FieldReader;
#[doc = "Field `CKE_STATUS` writer - 9:8\\]
Register access to cke_status signal. READ-ONLY"]
pub type CkeStatusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM_RST_VALID` reader - 16:16\\]
Register access to mem_rst_valid signal. READ-ONLY"]
pub type MemRstValidR = crate::BitReader;
#[doc = "Field `MEM_RST_VALID` writer - 16:16\\]
Register access to mem_rst_valid signal. READ-ONLY"]
pub type MemRstValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDFI_PHY_RDLAT_F0` reader - 31:24\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=0"]
pub type TdfiPhyRdlatF0R = crate::FieldReader;
#[doc = "Field `TDFI_PHY_RDLAT_F0` writer - 31:24\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=0"]
pub type TdfiPhyRdlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Priority of write commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
    #[inline(always)]
    pub fn axi0_w_priority(&self) -> Axi0WPriorityR {
        Axi0WPriorityR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Register access to cke_status signal. READ-ONLY"]
    #[inline(always)]
    pub fn cke_status(&self) -> CkeStatusR {
        CkeStatusR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Register access to mem_rst_valid signal. READ-ONLY"]
    #[inline(always)]
    pub fn mem_rst_valid(&self) -> MemRstValidR {
        MemRstValidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=0"]
    #[inline(always)]
    pub fn tdfi_phy_rdlat_f0(&self) -> TdfiPhyRdlatF0R {
        TdfiPhyRdlatF0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Priority of write commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
    #[inline(always)]
    #[must_use]
    pub fn axi0_w_priority(&mut self) -> Axi0WPriorityW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec> {
        Axi0WPriorityW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Register access to cke_status signal. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn cke_status(&mut self) -> CkeStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec> {
        CkeStatusW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Register access to mem_rst_valid signal. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn mem_rst_valid(&mut self) -> MemRstValidW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec> {
        MemRstValidW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Defines the DFI tPHY_RDLAT timing parameter \\[in DFI PHY clocks\\], the maximum cycles between a dfi_rddata_en assertion and a dfi_rddata_valid assertion. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phy_rdlat_f0(
        &mut self,
    ) -> TdfiPhyRdlatF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec> {
        TdfiPhyRdlatF0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_384\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_384::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_384::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_384::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_384::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_384 to value 0x0600_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl384Spec {
    const RESET_VALUE: u32 = 0x0600_0000;
}

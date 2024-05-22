#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl1RegSpec>;
#[doc = "Field `PDB` reader - 0:0\\]
CALIO S/M power down bar. SOC asserts after power up sequence is completed."]
pub type PdbR = crate::BitReader;
#[doc = "Field `PDB` writer - 0:0\\]
CALIO S/M power down bar. SOC asserts after power up sequence is completed."]
pub type PdbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDLL` reader - 1:1\\]
Enable DLL. Enables the analog DLL circuits."]
pub type EndllR = crate::BitReader;
#[doc = "Field `ENDLL` writer - 1:1\\]
Enable DLL. Enables the analog DLL circuits."]
pub type EndllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_TRM_ICP` reader - 7:4\\]
Analog DLL's Charge Pump Current Trim. Programs the analog DLL loop gain."]
pub type DllTrmIcpR = crate::FieldReader;
#[doc = "Field `DLL_TRM_ICP` writer - 7:4\\]
Analog DLL's Charge Pump Current Trim. Programs the analog DLL loop gain."]
pub type DllTrmIcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EN_RTRIM` reader - 16:16\\]
CALIO enable. Enables CALIO, If enabled CALIO will start calibration cycle at phyctrl_pdb positive edge."]
pub type EnRtrimR = crate::BitReader;
#[doc = "Field `EN_RTRIM` writer - 16:16\\]
CALIO enable. Enables CALIO, If enabled CALIO will start calibration cycle at phyctrl_pdb positive edge."]
pub type EnRtrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRIM` reader - 17:17\\]
Start CALIO calibration cycle. At positive edge initiates CALIO calibration cycle."]
pub type RetrimR = crate::BitReader;
#[doc = "Field `RETRIM` writer - 17:17\\]
Start CALIO calibration cycle. At positive edge initiates CALIO calibration cycle."]
pub type RetrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DR_TY` reader - 22:20\\]
Drive Source/Sink impedance programming. 0 => 50 Ohms, 1 => 33 Ohms, 2 => 66 Ohms, 3 => 100 Ohms, 4 => 40 Ohms."]
pub type DrTyR = crate::FieldReader;
#[doc = "Field `DR_TY` writer - 22:20\\]
Drive Source/Sink impedance programming. 0 => 50 Ohms, 1 => 33 Ohms, 2 => 66 Ohms, 3 => 100 Ohms, 4 => 40 Ohms."]
pub type DrTyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
CALIO S/M power down bar. SOC asserts after power up sequence is completed."]
    #[inline(always)]
    pub fn pdb(&self) -> PdbR {
        PdbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable DLL. Enables the analog DLL circuits."]
    #[inline(always)]
    pub fn endll(&self) -> EndllR {
        EndllR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Analog DLL's Charge Pump Current Trim. Programs the analog DLL loop gain."]
    #[inline(always)]
    pub fn dll_trm_icp(&self) -> DllTrmIcpR {
        DllTrmIcpR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
CALIO enable. Enables CALIO, If enabled CALIO will start calibration cycle at phyctrl_pdb positive edge."]
    #[inline(always)]
    pub fn en_rtrim(&self) -> EnRtrimR {
        EnRtrimR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Start CALIO calibration cycle. At positive edge initiates CALIO calibration cycle."]
    #[inline(always)]
    pub fn retrim(&self) -> RetrimR {
        RetrimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Drive Source/Sink impedance programming. 0 => 50 Ohms, 1 => 33 Ohms, 2 => 66 Ohms, 3 => 100 Ohms, 4 => 40 Ohms."]
    #[inline(always)]
    pub fn dr_ty(&self) -> DrTyR {
        DrTyR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
CALIO S/M power down bar. SOC asserts after power up sequence is completed."]
    #[inline(always)]
    #[must_use]
    pub fn pdb(&mut self) -> PdbW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        PdbW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable DLL. Enables the analog DLL circuits."]
    #[inline(always)]
    #[must_use]
    pub fn endll(&mut self) -> EndllW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        EndllW::new(self, 1)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Analog DLL's Charge Pump Current Trim. Programs the analog DLL loop gain."]
    #[inline(always)]
    #[must_use]
    pub fn dll_trm_icp(&mut self) -> DllTrmIcpW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        DllTrmIcpW::new(self, 4)
    }
    #[doc = "Bit 16 - 16:16\\]
CALIO enable. Enables CALIO, If enabled CALIO will start calibration cycle at phyctrl_pdb positive edge."]
    #[inline(always)]
    #[must_use]
    pub fn en_rtrim(&mut self) -> EnRtrimW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        EnRtrimW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Start CALIO calibration cycle. At positive edge initiates CALIO calibration cycle."]
    #[inline(always)]
    #[must_use]
    pub fn retrim(&mut self) -> RetrimW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        RetrimW::new(self, 17)
    }
    #[doc = "Bits 20:22 - 22:20\\]
Drive Source/Sink impedance programming. 0 => 50 Ohms, 1 => 33 Ohms, 2 => 66 Ohms, 3 => 100 Ohms, 4 => 40 Ohms."]
    #[inline(always)]
    #[must_use]
    pub fn dr_ty(&mut self) -> DrTyW<Regs_SsCfg_SscfgPhyCtrl1RegSpec> {
        DrTyW::new(self, 20)
    }
}
#[doc = "The PHY Control 1 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_1_REG to value 0x0001_0000"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl1RegSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}

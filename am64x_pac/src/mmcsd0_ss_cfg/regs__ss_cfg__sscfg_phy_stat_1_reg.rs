#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_STAT_1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyStat1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_STAT_1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyStat1RegSpec>;
#[doc = "Field `DLLRDY` reader - 0:0\\]
DLL ready. Indicates that DLL loop is locked."]
pub type DllrdyR = crate::BitReader;
#[doc = "Field `DLLRDY` writer - 0:0\\]
DLL ready. Indicates that DLL loop is locked."]
pub type DllrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALDONE` reader - 1:1\\]
STATUS, indicate that CALIO Calibration is completed successfully."]
pub type CaldoneR = crate::BitReader;
#[doc = "Field `CALDONE` writer - 1:1\\]
STATUS, indicate that CALIO Calibration is completed successfully."]
pub type CaldoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXR_NINST` reader - 2:2\\]
External Resistor on CALIO absent. Indicates trim cycle started and external resistor is absent."]
pub type ExrNinstR = crate::BitReader;
#[doc = "Field `EXR_NINST` writer - 2:2\\]
External Resistor on CALIO absent. Indicates trim cycle started and external resistor is absent."]
pub type ExrNinstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BISTDONE` reader - 3:3\\]
Internal BIST completed test cycle. Indicates that the embedded BIST has completed the test cycle."]
pub type BistdoneR = crate::BitReader;
#[doc = "Field `BISTDONE` writer - 3:3\\]
Internal BIST completed test cycle. Indicates that the embedded BIST has completed the test cycle."]
pub type BistdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTRIM` reader - 7:4\\]
CALIO Calibration Result. Holds the content of CALIO Impedance Calibration Result."]
pub type RtrimR = crate::FieldReader;
#[doc = "Field `RTRIM` writer - 7:4\\]
CALIO Calibration Result. Holds the content of CALIO Impedance Calibration Result."]
pub type RtrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DLL ready. Indicates that DLL loop is locked."]
    #[inline(always)]
    pub fn dllrdy(&self) -> DllrdyR {
        DllrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
STATUS, indicate that CALIO Calibration is completed successfully."]
    #[inline(always)]
    pub fn caldone(&self) -> CaldoneR {
        CaldoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
External Resistor on CALIO absent. Indicates trim cycle started and external resistor is absent."]
    #[inline(always)]
    pub fn exr_ninst(&self) -> ExrNinstR {
        ExrNinstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal BIST completed test cycle. Indicates that the embedded BIST has completed the test cycle."]
    #[inline(always)]
    pub fn bistdone(&self) -> BistdoneR {
        BistdoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
CALIO Calibration Result. Holds the content of CALIO Impedance Calibration Result."]
    #[inline(always)]
    pub fn rtrim(&self) -> RtrimR {
        RtrimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DLL ready. Indicates that DLL loop is locked."]
    #[inline(always)]
    #[must_use]
    pub fn dllrdy(&mut self) -> DllrdyW<Regs_SsCfg_SscfgPhyStat1RegSpec> {
        DllrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
STATUS, indicate that CALIO Calibration is completed successfully."]
    #[inline(always)]
    #[must_use]
    pub fn caldone(&mut self) -> CaldoneW<Regs_SsCfg_SscfgPhyStat1RegSpec> {
        CaldoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
External Resistor on CALIO absent. Indicates trim cycle started and external resistor is absent."]
    #[inline(always)]
    #[must_use]
    pub fn exr_ninst(&mut self) -> ExrNinstW<Regs_SsCfg_SscfgPhyStat1RegSpec> {
        ExrNinstW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal BIST completed test cycle. Indicates that the embedded BIST has completed the test cycle."]
    #[inline(always)]
    #[must_use]
    pub fn bistdone(&mut self) -> BistdoneW<Regs_SsCfg_SscfgPhyStat1RegSpec> {
        BistdoneW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
CALIO Calibration Result. Holds the content of CALIO Impedance Calibration Result."]
    #[inline(always)]
    #[must_use]
    pub fn rtrim(&mut self) -> RtrimW<Regs_SsCfg_SscfgPhyStat1RegSpec> {
        RtrimW::new(self, 4)
    }
}
#[doc = "The PHY Status 1 Register contains various fields to reflect the status of the Arasan eMMC/SD PHY ports. For detailed functionality of the Arasan eMMC/SD PHY status ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_stat_1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_stat_1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyStat1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyStat1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_stat_1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyStat1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_stat_1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyStat1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_STAT_1_REG to value 0x0140"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyStat1RegSpec {
    const RESET_VALUE: u32 = 0x0140;
}

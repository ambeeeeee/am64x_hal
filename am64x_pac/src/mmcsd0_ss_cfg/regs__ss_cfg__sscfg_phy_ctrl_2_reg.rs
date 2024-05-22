#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_2_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl2RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_2_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl2RegSpec>;
#[doc = "Field `ODEN_DAT` reader - 7:0\\]
Open Drain Enable on DAT lines."]
pub type OdenDatR = crate::FieldReader;
#[doc = "Field `ODEN_DAT` writer - 7:0\\]
Open Drain Enable on DAT lines."]
pub type OdenDatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODEN_CMD` reader - 12:12\\]
Open Drain Enable on CMD line."]
pub type OdenCmdR = crate::BitReader;
#[doc = "Field `ODEN_CMD` writer - 12:12\\]
Open Drain Enable on CMD line."]
pub type OdenCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODEN_STRB` reader - 13:13\\]
Open Drain Enable on STRB line."]
pub type OdenStrbR = crate::BitReader;
#[doc = "Field `ODEN_STRB` writer - 13:13\\]
Open Drain Enable on STRB line."]
pub type OdenStrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD_RELEASE_DAT` reader - 23:16\\]
Disable an internal 4.7K pull up resistor on data lines in open drain mode."]
pub type OdReleaseDatR = crate::FieldReader;
#[doc = "Field `OD_RELEASE_DAT` writer - 23:16\\]
Disable an internal 4.7K pull up resistor on data lines in open drain mode."]
pub type OdReleaseDatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OD_RELEASE_CMD` reader - 28:28\\]
Disable an internal 4.7K pull up resistor on CMD line in open drain mode."]
pub type OdReleaseCmdR = crate::BitReader;
#[doc = "Field `OD_RELEASE_CMD` writer - 28:28\\]
Disable an internal 4.7K pull up resistor on CMD line in open drain mode."]
pub type OdReleaseCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OD_RELEASE_STRB` reader - 29:29\\]
Disable an internal 4.7K pull up resistor on STRB line in open drain mode."]
pub type OdReleaseStrbR = crate::BitReader;
#[doc = "Field `OD_RELEASE_STRB` writer - 29:29\\]
Disable an internal 4.7K pull up resistor on STRB line in open drain mode."]
pub type OdReleaseStrbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Open Drain Enable on DAT lines."]
    #[inline(always)]
    pub fn oden_dat(&self) -> OdenDatR {
        OdenDatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Open Drain Enable on CMD line."]
    #[inline(always)]
    pub fn oden_cmd(&self) -> OdenCmdR {
        OdenCmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Open Drain Enable on STRB line."]
    #[inline(always)]
    pub fn oden_strb(&self) -> OdenStrbR {
        OdenStrbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Disable an internal 4.7K pull up resistor on data lines in open drain mode."]
    #[inline(always)]
    pub fn od_release_dat(&self) -> OdReleaseDatR {
        OdReleaseDatR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Disable an internal 4.7K pull up resistor on CMD line in open drain mode."]
    #[inline(always)]
    pub fn od_release_cmd(&self) -> OdReleaseCmdR {
        OdReleaseCmdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Disable an internal 4.7K pull up resistor on STRB line in open drain mode."]
    #[inline(always)]
    pub fn od_release_strb(&self) -> OdReleaseStrbR {
        OdReleaseStrbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Open Drain Enable on DAT lines."]
    #[inline(always)]
    #[must_use]
    pub fn oden_dat(&mut self) -> OdenDatW<Regs_SsCfg_SscfgPhyCtrl2RegSpec> {
        OdenDatW::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Open Drain Enable on CMD line."]
    #[inline(always)]
    #[must_use]
    pub fn oden_cmd(&mut self) -> OdenCmdW<Regs_SsCfg_SscfgPhyCtrl2RegSpec> {
        OdenCmdW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Open Drain Enable on STRB line."]
    #[inline(always)]
    #[must_use]
    pub fn oden_strb(&mut self) -> OdenStrbW<Regs_SsCfg_SscfgPhyCtrl2RegSpec> {
        OdenStrbW::new(self, 13)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Disable an internal 4.7K pull up resistor on data lines in open drain mode."]
    #[inline(always)]
    #[must_use]
    pub fn od_release_dat(&mut self) -> OdReleaseDatW<Regs_SsCfg_SscfgPhyCtrl2RegSpec> {
        OdReleaseDatW::new(self, 16)
    }
    #[doc = "Bit 28 - 28:28\\]
Disable an internal 4.7K pull up resistor on CMD line in open drain mode."]
    #[inline(always)]
    #[must_use]
    pub fn od_release_cmd(&mut self) -> OdReleaseCmdW<Regs_SsCfg_SscfgPhyCtrl2RegSpec> {
        OdReleaseCmdW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Disable an internal 4.7K pull up resistor on STRB line in open drain mode."]
    #[inline(always)]
    #[must_use]
    pub fn od_release_strb(&mut self) -> OdReleaseStrbW<Regs_SsCfg_SscfgPhyCtrl2RegSpec> {
        OdReleaseStrbW::new(self, 29)
    }
}
#[doc = "The PHY Control 2 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl2RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_2_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_2_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_2_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl2RegSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_3_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl3RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_3_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl3RegSpec>;
#[doc = "Field `REN_DAT` reader - 7:0\\]
Enable pull up/down on the DAT lines. If pu_dat is high a week pull up is enabled on DATA lines, if low week pull down is enabled on DATA lines."]
pub type RenDatR = crate::FieldReader;
#[doc = "Field `REN_DAT` writer - 7:0\\]
Enable pull up/down on the DAT lines. If pu_dat is high a week pull up is enabled on DATA lines, if low week pull down is enabled on DATA lines."]
pub type RenDatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REN_CMD` reader - 12:12\\]
Enable pull up/down on the CMD line. If pu_cmd is high a week pull up is enabled on CMD line, if low week pull down is enabled on CMD line."]
pub type RenCmdR = crate::BitReader;
#[doc = "Field `REN_CMD` writer - 12:12\\]
Enable pull up/down on the CMD line. If pu_cmd is high a week pull up is enabled on CMD line, if low week pull down is enabled on CMD line."]
pub type RenCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN_STRB` reader - 13:13\\]
Enable pull up/down on the STRB line. If pu_strb is high a week pull up is enabled on STRB line, if low week pull down is enabled on STRB line."]
pub type RenStrbR = crate::BitReader;
#[doc = "Field `REN_STRB` writer - 13:13\\]
Enable pull up/down on the STRB line. If pu_strb is high a week pull up is enabled on STRB line, if low week pull down is enabled on STRB line."]
pub type RenStrbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU_DAT` reader - 23:16\\]
Enable pull up on DAT lines. If ren_dat is high week pull up is enabled on DATA lines."]
pub type PuDatR = crate::FieldReader;
#[doc = "Field `PU_DAT` writer - 23:16\\]
Enable pull up on DAT lines. If ren_dat is high week pull up is enabled on DATA lines."]
pub type PuDatW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PU_CMD` reader - 28:28\\]
Enable pull up on CMD line. If ren_cmd is high week pull up is enabled on CMD line."]
pub type PuCmdR = crate::BitReader;
#[doc = "Field `PU_CMD` writer - 28:28\\]
Enable pull up on CMD line. If ren_cmd is high week pull up is enabled on CMD line."]
pub type PuCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU_STRB` reader - 29:29\\]
Enable pull up on STRB line. If ren_strb is high week pull up is enabled on STRB line."]
pub type PuStrbR = crate::BitReader;
#[doc = "Field `PU_STRB` writer - 29:29\\]
Enable pull up on STRB line. If ren_strb is high week pull up is enabled on STRB line."]
pub type PuStrbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Enable pull up/down on the DAT lines. If pu_dat is high a week pull up is enabled on DATA lines, if low week pull down is enabled on DATA lines."]
    #[inline(always)]
    pub fn ren_dat(&self) -> RenDatR {
        RenDatR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable pull up/down on the CMD line. If pu_cmd is high a week pull up is enabled on CMD line, if low week pull down is enabled on CMD line."]
    #[inline(always)]
    pub fn ren_cmd(&self) -> RenCmdR {
        RenCmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable pull up/down on the STRB line. If pu_strb is high a week pull up is enabled on STRB line, if low week pull down is enabled on STRB line."]
    #[inline(always)]
    pub fn ren_strb(&self) -> RenStrbR {
        RenStrbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable pull up on DAT lines. If ren_dat is high week pull up is enabled on DATA lines."]
    #[inline(always)]
    pub fn pu_dat(&self) -> PuDatR {
        PuDatR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 28 - 28:28\\]
Enable pull up on CMD line. If ren_cmd is high week pull up is enabled on CMD line."]
    #[inline(always)]
    pub fn pu_cmd(&self) -> PuCmdR {
        PuCmdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable pull up on STRB line. If ren_strb is high week pull up is enabled on STRB line."]
    #[inline(always)]
    pub fn pu_strb(&self) -> PuStrbR {
        PuStrbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Enable pull up/down on the DAT lines. If pu_dat is high a week pull up is enabled on DATA lines, if low week pull down is enabled on DATA lines."]
    #[inline(always)]
    #[must_use]
    pub fn ren_dat(&mut self) -> RenDatW<Regs_SsCfg_SscfgPhyCtrl3RegSpec> {
        RenDatW::new(self, 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Enable pull up/down on the CMD line. If pu_cmd is high a week pull up is enabled on CMD line, if low week pull down is enabled on CMD line."]
    #[inline(always)]
    #[must_use]
    pub fn ren_cmd(&mut self) -> RenCmdW<Regs_SsCfg_SscfgPhyCtrl3RegSpec> {
        RenCmdW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Enable pull up/down on the STRB line. If pu_strb is high a week pull up is enabled on STRB line, if low week pull down is enabled on STRB line."]
    #[inline(always)]
    #[must_use]
    pub fn ren_strb(&mut self) -> RenStrbW<Regs_SsCfg_SscfgPhyCtrl3RegSpec> {
        RenStrbW::new(self, 13)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Enable pull up on DAT lines. If ren_dat is high week pull up is enabled on DATA lines."]
    #[inline(always)]
    #[must_use]
    pub fn pu_dat(&mut self) -> PuDatW<Regs_SsCfg_SscfgPhyCtrl3RegSpec> {
        PuDatW::new(self, 16)
    }
    #[doc = "Bit 28 - 28:28\\]
Enable pull up on CMD line. If ren_cmd is high week pull up is enabled on CMD line."]
    #[inline(always)]
    #[must_use]
    pub fn pu_cmd(&mut self) -> PuCmdW<Regs_SsCfg_SscfgPhyCtrl3RegSpec> {
        PuCmdW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Enable pull up on STRB line. If ren_strb is high week pull up is enabled on STRB line."]
    #[inline(always)]
    #[must_use]
    pub fn pu_strb(&mut self) -> PuStrbW<Regs_SsCfg_SscfgPhyCtrl3RegSpec> {
        PuStrbW::new(self, 29)
    }
}
#[doc = "The PHY Control 3 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_3_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_3_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl3RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_3_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_3_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl3RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_3_REG to value 0x1255_1255"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl3RegSpec {
    const RESET_VALUE: u32 = 0x1255_1255;
}

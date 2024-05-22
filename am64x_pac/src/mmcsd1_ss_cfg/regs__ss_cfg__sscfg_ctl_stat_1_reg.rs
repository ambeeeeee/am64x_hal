#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_STAT_1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlStat1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_STAT_1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlStat1RegSpec>;
#[doc = "Field `DMADEBUGBUS` reader - 15:0\\]
DMA_CTRL Debug Bus."]
pub type DmadebugbusR = crate::FieldReader<u16>;
#[doc = "Field `DMADEBUGBUS` writer - 15:0\\]
DMA_CTRL Debug Bus."]
pub type DmadebugbusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SDHC_CMDIDLE` reader - 31:31\\]
Idle signal to enable S/W to gate off the clocks."]
pub type SdhcCmdidleR = crate::BitReader;
#[doc = "Field `SDHC_CMDIDLE` writer - 31:31\\]
Idle signal to enable S/W to gate off the clocks."]
pub type SdhcCmdidleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
DMA_CTRL Debug Bus."]
    #[inline(always)]
    pub fn dmadebugbus(&self) -> DmadebugbusR {
        DmadebugbusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Idle signal to enable S/W to gate off the clocks."]
    #[inline(always)]
    pub fn sdhc_cmdidle(&self) -> SdhcCmdidleR {
        SdhcCmdidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
DMA_CTRL Debug Bus."]
    #[inline(always)]
    #[must_use]
    pub fn dmadebugbus(&mut self) -> DmadebugbusW<Regs_SsCfg_SscfgCtlStat1RegSpec> {
        DmadebugbusW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Idle signal to enable S/W to gate off the clocks."]
    #[inline(always)]
    #[must_use]
    pub fn sdhc_cmdidle(&mut self) -> SdhcCmdidleW<Regs_SsCfg_SscfgCtlStat1RegSpec> {
        SdhcCmdidleW::new(self, 31)
    }
}
#[doc = "The Controller Status 1 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlStat1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlStat1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_stat_1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlStat1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_stat_1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlStat1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_STAT_1_REG to value 0x8000_0000"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlStat1RegSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}

#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_STAT_2_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlStat2RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_STAT_2_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlStat2RegSpec>;
#[doc = "Field `CMDDEBUGBUS` reader - 15:0\\]
CMD_CTRL Debug Bus."]
pub type CmddebugbusR = crate::FieldReader<u16>;
#[doc = "Field `CMDDEBUGBUS` writer - 15:0\\]
CMD_CTRL Debug Bus."]
pub type CmddebugbusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
CMD_CTRL Debug Bus."]
    #[inline(always)]
    pub fn cmddebugbus(&self) -> CmddebugbusR {
        CmddebugbusR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
CMD_CTRL Debug Bus."]
    #[inline(always)]
    #[must_use]
    pub fn cmddebugbus(&mut self) -> CmddebugbusW<Regs_SsCfg_SscfgCtlStat2RegSpec> {
        CmddebugbusW::new(self, 0)
    }
}
#[doc = "The Controller Status 2 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_2_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_2_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlStat2RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlStat2RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_stat_2_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlStat2RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_stat_2_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlStat2RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_STAT_2_REG to value 0x16"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlStat2RegSpec {
    const RESET_VALUE: u32 = 0x16;
}

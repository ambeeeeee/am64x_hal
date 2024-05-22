#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_STAT_3_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlStat3RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_STAT_3_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlStat3RegSpec>;
#[doc = "Field `TXDDEBUGBUS` reader - 15:0\\]
TXD_CTRL Debug Bus."]
pub type TxddebugbusR = crate::FieldReader<u16>;
#[doc = "Field `TXDDEBUGBUS` writer - 15:0\\]
TXD_CTRL Debug Bus."]
pub type TxddebugbusW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
TXD_CTRL Debug Bus."]
    #[inline(always)]
    pub fn txddebugbus(&self) -> TxddebugbusR {
        TxddebugbusR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
TXD_CTRL Debug Bus."]
    #[inline(always)]
    #[must_use]
    pub fn txddebugbus(&mut self) -> TxddebugbusW<Regs_SsCfg_SscfgCtlStat3RegSpec> {
        TxddebugbusW::new(self, 0)
    }
}
#[doc = "The Controller Status 3 Register contains various fields to reflect the status of the debug ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller debug ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_stat_3_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_stat_3_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlStat3RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlStat3RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_stat_3_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlStat3RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_stat_3_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlStat3RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_STAT_3_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlStat3RegSpec {
    const RESET_VALUE: u32 = 0;
}

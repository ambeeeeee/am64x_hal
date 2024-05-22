#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_7_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlCfg7RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_7_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlCfg7RegSpec>;
#[doc = "Field `DSPDPRESETVAL` reader - 12:0\\]
Preset Value for Default Speed."]
pub type DspdpresetvalR = crate::FieldReader<u16>;
#[doc = "Field `DSPDPRESETVAL` writer - 12:0\\]
Preset Value for Default Speed."]
pub type DspdpresetvalW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - 12:0\\]
Preset Value for Default Speed."]
    #[inline(always)]
    pub fn dspdpresetval(&self) -> DspdpresetvalR {
        DspdpresetvalR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - 12:0\\]
Preset Value for Default Speed."]
    #[inline(always)]
    #[must_use]
    pub fn dspdpresetval(&mut self) -> DspdpresetvalW<Regs_SsCfg_SscfgCtlCfg7RegSpec> {
        DspdpresetvalW::new(self, 0)
    }
}
#[doc = "The Controller Config 7 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the fields in the Preset Values Register for Default Speed inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_7_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_7_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlCfg7RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlCfg7RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_cfg_7_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlCfg7RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_cfg_7_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlCfg7RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_CFG_7_REG to value 0x04"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlCfg7RegSpec {
    const RESET_VALUE: u32 = 0x04;
}

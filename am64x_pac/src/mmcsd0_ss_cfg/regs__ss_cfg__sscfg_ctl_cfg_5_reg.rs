#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_5_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlCfg5RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_5_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlCfg5RegSpec>;
#[doc = "Field `MAXCURRENTVDD2` reader - 7:0\\]
Maximum Current for 1.8 V (VDD2)."]
pub type Maxcurrentvdd2R = crate::FieldReader;
#[doc = "Field `MAXCURRENTVDD2` writer - 7:0\\]
Maximum Current for 1.8 V (VDD2)."]
pub type Maxcurrentvdd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum Current for 1.8 V (VDD2)."]
    #[inline(always)]
    pub fn maxcurrentvdd2(&self) -> Maxcurrentvdd2R {
        Maxcurrentvdd2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum Current for 1.8 V (VDD2)."]
    #[inline(always)]
    #[must_use]
    pub fn maxcurrentvdd2(&mut self) -> Maxcurrentvdd2W<Regs_SsCfg_SscfgCtlCfg5RegSpec> {
        Maxcurrentvdd2W::new(self, 0)
    }
}
#[doc = "The Controller Config 5 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the msb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_5_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_5_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlCfg5RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlCfg5RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_cfg_5_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlCfg5RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_cfg_5_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlCfg5RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_CFG_5_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlCfg5RegSpec {
    const RESET_VALUE: u32 = 0;
}

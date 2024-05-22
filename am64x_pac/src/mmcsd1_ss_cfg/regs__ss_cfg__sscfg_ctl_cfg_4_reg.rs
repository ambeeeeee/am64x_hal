#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_4_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlCfg4RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_4_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlCfg4RegSpec>;
#[doc = "Field `MAXCURRENT3P3V` reader - 7:0\\]
Maximum Current for 3.3V."]
pub type Maxcurrent3p3vR = crate::FieldReader;
#[doc = "Field `MAXCURRENT3P3V` writer - 7:0\\]
Maximum Current for 3.3V."]
pub type Maxcurrent3p3vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAXCURRENT3P0V` reader - 15:8\\]
Maximum Current for 3.0V."]
pub type Maxcurrent3p0vR = crate::FieldReader;
#[doc = "Field `MAXCURRENT3P0V` writer - 15:8\\]
Maximum Current for 3.0V."]
pub type Maxcurrent3p0vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MAXCURRENT1P8V` reader - 23:16\\]
Maximum Current for 1.8V."]
pub type Maxcurrent1p8vR = crate::FieldReader;
#[doc = "Field `MAXCURRENT1P8V` writer - 23:16\\]
Maximum Current for 1.8V."]
pub type Maxcurrent1p8vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum Current for 3.3V."]
    #[inline(always)]
    pub fn maxcurrent3p3v(&self) -> Maxcurrent3p3vR {
        Maxcurrent3p3vR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Maximum Current for 3.0V."]
    #[inline(always)]
    pub fn maxcurrent3p0v(&self) -> Maxcurrent3p0vR {
        Maxcurrent3p0vR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Maximum Current for 1.8V."]
    #[inline(always)]
    pub fn maxcurrent1p8v(&self) -> Maxcurrent1p8vR {
        Maxcurrent1p8vR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum Current for 3.3V."]
    #[inline(always)]
    #[must_use]
    pub fn maxcurrent3p3v(&mut self) -> Maxcurrent3p3vW<Regs_SsCfg_SscfgCtlCfg4RegSpec> {
        Maxcurrent3p3vW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Maximum Current for 3.0V."]
    #[inline(always)]
    #[must_use]
    pub fn maxcurrent3p0v(&mut self) -> Maxcurrent3p0vW<Regs_SsCfg_SscfgCtlCfg4RegSpec> {
        Maxcurrent3p0vW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Maximum Current for 1.8V."]
    #[inline(always)]
    #[must_use]
    pub fn maxcurrent1p8v(&mut self) -> Maxcurrent1p8vW<Regs_SsCfg_SscfgCtlCfg4RegSpec> {
        Maxcurrent1p8vW::new(self, 16)
    }
}
#[doc = "The Controller Config 4 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2. This register sets the lsb fields in the Maximum Current Capabilities Register inside the Arasan eMMC/SD Controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_4_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_4_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlCfg4RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlCfg4RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_cfg_4_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlCfg4RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_cfg_4_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlCfg4RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_CFG_4_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlCfg4RegSpec {
    const RESET_VALUE: u32 = 0;
}

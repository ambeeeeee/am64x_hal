#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_max_current_cap` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_max_current_cap` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec>;
#[doc = "Field `VDD1_3P3V` reader - 7:0\\]
Maximum Current for 3.3V VDD1"]
pub type Vdd1_3p3vR = crate::FieldReader;
#[doc = "Field `VDD1_3P3V` writer - 7:0\\]
Maximum Current for 3.3V VDD1"]
pub type Vdd1_3p3vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDD1_3P0V` reader - 15:8\\]
Maximum Current for 3.0V VDD1"]
pub type Vdd1_3p0vR = crate::FieldReader;
#[doc = "Field `VDD1_3P0V` writer - 15:8\\]
Maximum Current for 3.0V VDD1"]
pub type Vdd1_3p0vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDD1_1P8V` reader - 23:16\\]
Maximum Current for 1.8V VDD1"]
pub type Vdd1_1p8vR = crate::FieldReader;
#[doc = "Field `VDD1_1P8V` writer - 23:16\\]
Maximum Current for 1.8V VDD1"]
pub type Vdd1_1p8vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDD2_1P8V` reader - 39:32\\]
Maximum Current for 1.8V VDD2"]
pub type Vdd2_1p8vR = crate::FieldReader;
#[doc = "Field `VDD2_1P8V` writer - 39:32\\]
Maximum Current for 1.8V VDD2"]
pub type Vdd2_1p8vW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum Current for 3.3V VDD1"]
    #[inline(always)]
    pub fn vdd1_3p3v(&self) -> Vdd1_3p3vR {
        Vdd1_3p3vR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Maximum Current for 3.0V VDD1"]
    #[inline(always)]
    pub fn vdd1_3p0v(&self) -> Vdd1_3p0vR {
        Vdd1_3p0vR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Maximum Current for 1.8V VDD1"]
    #[inline(always)]
    pub fn vdd1_1p8v(&self) -> Vdd1_1p8vR {
        Vdd1_1p8vR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 32:39 - 39:32\\]
Maximum Current for 1.8V VDD2"]
    #[inline(always)]
    pub fn vdd2_1p8v(&self) -> Vdd2_1p8vR {
        Vdd2_1p8vR::new(((self.bits >> 32) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Maximum Current for 3.3V VDD1"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1_3p3v(&mut self) -> Vdd1_3p3vW<SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec> {
        Vdd1_3p3vW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Maximum Current for 3.0V VDD1"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1_3p0v(&mut self) -> Vdd1_3p0vW<SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec> {
        Vdd1_3p0vW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Maximum Current for 1.8V VDD1"]
    #[inline(always)]
    #[must_use]
    pub fn vdd1_1p8v(&mut self) -> Vdd1_1p8vW<SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec> {
        Vdd1_1p8vW::new(self, 16)
    }
    #[doc = "Bits 32:39 - 39:32\\]
Maximum Current for 1.8V VDD2"]
    #[inline(always)]
    #[must_use]
    pub fn vdd2_1p8v(&mut self) -> Vdd2_1p8vW<SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec> {
        Vdd2_1p8vW::new(self, 32)
    }
}
#[doc = "This register indicates maximum current capability for each voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_max_current_cap::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_max_current_cap to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgMaxCurrentCapSpec {
    const RESET_VALUE: u64 = 0;
}

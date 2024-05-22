#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_DEF_PRI_MAP_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aDefPriMapRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_DEF_PRI_MAP_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec>;
#[doc = "Field `PRIMAP7` reader - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7. 0=highest priority. 7=lowest priority"]
pub type Primap7R = crate::FieldReader;
#[doc = "Field `PRIMAP7` writer - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7. 0=highest priority. 7=lowest priority"]
pub type Primap7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP6` reader - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6. 0=highest priority. 7=lowest priority"]
pub type Primap6R = crate::FieldReader;
#[doc = "Field `PRIMAP6` writer - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6. 0=highest priority. 7=lowest priority"]
pub type Primap6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP5` reader - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5. 0=highest priority. 7=lowest priority"]
pub type Primap5R = crate::FieldReader;
#[doc = "Field `PRIMAP5` writer - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5. 0=highest priority. 7=lowest priority"]
pub type Primap5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP4` reader - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4. 0=highest priority. 7=lowest priority"]
pub type Primap4R = crate::FieldReader;
#[doc = "Field `PRIMAP4` writer - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4. 0=highest priority. 7=lowest priority"]
pub type Primap4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP3` reader - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3. 0=highest priority. 7=lowest priority"]
pub type Primap3R = crate::FieldReader;
#[doc = "Field `PRIMAP3` writer - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3. 0=highest priority. 7=lowest priority"]
pub type Primap3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP2` reader - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2. 0=highest priority. 7=lowest priority"]
pub type Primap2R = crate::FieldReader;
#[doc = "Field `PRIMAP2` writer - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2. 0=highest priority. 7=lowest priority"]
pub type Primap2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP1` reader - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1. 0=highest priority. 7=lowest priority"]
pub type Primap1R = crate::FieldReader;
#[doc = "Field `PRIMAP1` writer - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1. 0=highest priority. 7=lowest priority"]
pub type Primap1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PRIMAP0` reader - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0. 0=highest priority. 7=lowest priority"]
pub type Primap0R = crate::FieldReader;
#[doc = "Field `PRIMAP0` writer - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0. 0=highest priority. 7=lowest priority"]
pub type Primap0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap7(&self) -> Primap7R {
        Primap7R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap6(&self) -> Primap6R {
        Primap6R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap5(&self) -> Primap5R {
        Primap5R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap4(&self) -> Primap4R {
        Primap4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap3(&self) -> Primap3R {
        Primap3R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap2(&self) -> Primap2R {
        Primap2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap1(&self) -> Primap1R {
        Primap1R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn primap0(&self) -> Primap0R {
        Primap0R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap7(&mut self) -> Primap7W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap7W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap6(&mut self) -> Primap6W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap6W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap5(&mut self) -> Primap5W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap5W::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap4(&mut self) -> Primap4W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap4W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap3(&mut self) -> Primap3W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap3W::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap2(&mut self) -> Primap2W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap2W::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap1(&mut self) -> Primap1W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap1W::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn primap0(&mut self) -> Primap0W<Regs_SsCfg_SscfgV2aDefPriMapRegSpec> {
        Primap0W::new(self, 28)
    }
}
#[doc = "The Default Priority Mapping Register is the default map for the inbound VBUSM.C priority to AXI priority.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_def_pri_map_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_def_pri_map_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aDefPriMapRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aDefPriMapRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_def_pri_map_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aDefPriMapRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_def_pri_map_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aDefPriMapRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_DEF_PRI_MAP_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aDefPriMapRegSpec {
    const RESET_VALUE: u32 = 0;
}

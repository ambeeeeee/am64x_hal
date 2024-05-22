#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_R2_PRI_MAP_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aR2PriMapRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_R2_PRI_MAP_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec>;
#[doc = "Field `RANGE2_PRIMAP7` reader - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap7R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP7` writer - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP6` reader - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap6R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP6` writer - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP5` reader - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap5R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP5` writer - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP4` reader - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap4R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP4` writer - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP3` reader - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap3R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP3` writer - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP2` reader - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap2R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP2` writer - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP1` reader - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap1R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP1` writer - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_PRIMAP0` reader - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap0R = crate::FieldReader;
#[doc = "Field `RANGE2_PRIMAP0` writer - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0 for range match 2. 0=highest priority. 7=lowest priority"]
pub type Range2Primap0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap7(&self) -> Range2Primap7R {
        Range2Primap7R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap6(&self) -> Range2Primap6R {
        Range2Primap6R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap5(&self) -> Range2Primap5R {
        Range2Primap5R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap4(&self) -> Range2Primap4R {
        Range2Primap4R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap3(&self) -> Range2Primap3R {
        Range2Primap3R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap2(&self) -> Range2Primap2R {
        Range2Primap2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap1(&self) -> Range2Primap1R {
        Range2Primap1R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    pub fn range2_primap0(&self) -> Range2Primap0R {
        Range2Primap0R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
The field contains AXI priority value for VBUSM.C priority 7 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap7(&mut self) -> Range2Primap7W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap7W::new(self, 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
The field contains AXI priority value for VBUSM.C priority 6 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap6(&mut self) -> Range2Primap6W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap6W::new(self, 4)
    }
    #[doc = "Bits 8:10 - 10:8\\]
The field contains AXI priority value for VBUSM.C priority 5 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap5(&mut self) -> Range2Primap5W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap5W::new(self, 8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The field contains AXI priority value for VBUSM.C priority 4 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap4(&mut self) -> Range2Primap4W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap4W::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
The field contains AXI priority value for VBUSM.C priority 3 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap3(&mut self) -> Range2Primap3W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap3W::new(self, 16)
    }
    #[doc = "Bits 20:22 - 22:20\\]
The field contains AXI priority value for VBUSM.C priority 2 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap2(&mut self) -> Range2Primap2W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap2W::new(self, 20)
    }
    #[doc = "Bits 24:26 - 26:24\\]
The field contains AXI priority value for VBUSM.C priority 1 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap1(&mut self) -> Range2Primap1W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap1W::new(self, 24)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The field contains AXI priority value for VBUSM.C priority 0 for range match 2. 0=highest priority. 7=lowest priority"]
    #[inline(always)]
    #[must_use]
    pub fn range2_primap0(&mut self) -> Range2Primap0W<Regs_SsCfg_SscfgV2aR2PriMapRegSpec> {
        Range2Primap0W::new(self, 28)
    }
}
#[doc = "The Range 2 Priority Mapping Register is used to map the inbound VBUSM.C priority to AXI priority when a RouteID match 2 occurs. This allows the priority level to be changed from the Default Priority Mapping value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aR2PriMapRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aR2PriMapRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aR2PriMapRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_r2_pri_map_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aR2PriMapRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_R2_PRI_MAP_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aR2PriMapRegSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_R2_MAT_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aR2MatRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_R2_MAT_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aR2MatRegSpec>;
#[doc = "Field `RANGE2_ROUTEID_B` reader - 11:0\\]
The range2_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
pub type Range2RouteidBR = crate::FieldReader<u16>;
#[doc = "Field `RANGE2_ROUTEID_B` writer - 11:0\\]
The range2_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
pub type Range2RouteidBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RANGE2_MASK_B` reader - 14:12\\]
The range2_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
pub type Range2MaskBR = crate::FieldReader;
#[doc = "Field `RANGE2_MASK_B` writer - 14:12\\]
The range2_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
pub type Range2MaskBW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_RANGEEN_B` reader - 15:15\\]
The range2_rangeen_b enables the RouteID AND'd with range2_mask_b to match the range2_routeid_b"]
pub type Range2RangeenBR = crate::BitReader;
#[doc = "Field `RANGE2_RANGEEN_B` writer - 15:15\\]
The range2_rangeen_b enables the RouteID AND'd with range2_mask_b to match the range2_routeid_b"]
pub type Range2RangeenBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGE2_ROUTEID_A` reader - 27:16\\]
The range2_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
pub type Range2RouteidAR = crate::FieldReader<u16>;
#[doc = "Field `RANGE2_ROUTEID_A` writer - 27:16\\]
The range2_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
pub type Range2RouteidAW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RANGE2_MASK_A` reader - 30:28\\]
The range2_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
pub type Range2MaskAR = crate::FieldReader;
#[doc = "Field `RANGE2_MASK_A` writer - 30:28\\]
The range2_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
pub type Range2MaskAW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE2_RANGEEN_A` reader - 31:31\\]
The range2_rangeen_a enables the RouteID AND'd with range2_mask_a to match the range2_routeid_a"]
pub type Range2RangeenAR = crate::BitReader;
#[doc = "Field `RANGE2_RANGEEN_A` writer - 31:31\\]
The range2_rangeen_a enables the RouteID AND'd with range2_mask_a to match the range2_routeid_a"]
pub type Range2RangeenAW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
The range2_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    pub fn range2_routeid_b(&self) -> Range2RouteidBR {
        Range2RouteidBR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The range2_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
    #[inline(always)]
    pub fn range2_mask_b(&self) -> Range2MaskBR {
        Range2MaskBR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
The range2_rangeen_b enables the RouteID AND'd with range2_mask_b to match the range2_routeid_b"]
    #[inline(always)]
    pub fn range2_rangeen_b(&self) -> Range2RangeenBR {
        Range2RangeenBR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
The range2_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    pub fn range2_routeid_a(&self) -> Range2RouteidAR {
        Range2RouteidAR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The range2_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
    #[inline(always)]
    pub fn range2_mask_a(&self) -> Range2MaskAR {
        Range2MaskAR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
The range2_rangeen_a enables the RouteID AND'd with range2_mask_a to match the range2_routeid_a"]
    #[inline(always)]
    pub fn range2_rangeen_a(&self) -> Range2RangeenAR {
        Range2RangeenAR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
The range2_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    #[must_use]
    pub fn range2_routeid_b(&mut self) -> Range2RouteidBW<Regs_SsCfg_SscfgV2aR2MatRegSpec> {
        Range2RouteidBW::new(self, 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The range2_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
    #[inline(always)]
    #[must_use]
    pub fn range2_mask_b(&mut self) -> Range2MaskBW<Regs_SsCfg_SscfgV2aR2MatRegSpec> {
        Range2MaskBW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
The range2_rangeen_b enables the RouteID AND'd with range2_mask_b to match the range2_routeid_b"]
    #[inline(always)]
    #[must_use]
    pub fn range2_rangeen_b(&mut self) -> Range2RangeenBW<Regs_SsCfg_SscfgV2aR2MatRegSpec> {
        Range2RangeenBW::new(self, 15)
    }
    #[doc = "Bits 16:27 - 27:16\\]
The range2_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    #[must_use]
    pub fn range2_routeid_a(&mut self) -> Range2RouteidAW<Regs_SsCfg_SscfgV2aR2MatRegSpec> {
        Range2RouteidAW::new(self, 16)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The range2_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
    #[inline(always)]
    #[must_use]
    pub fn range2_mask_a(&mut self) -> Range2MaskAW<Regs_SsCfg_SscfgV2aR2MatRegSpec> {
        Range2MaskAW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
The range2_rangeen_a enables the RouteID AND'd with range2_mask_a to match the range2_routeid_a"]
    #[inline(always)]
    #[must_use]
    pub fn range2_rangeen_a(&mut self) -> Range2RangeenAW<Regs_SsCfg_SscfgV2aR2MatRegSpec> {
        Range2RangeenAW::new(self, 31)
    }
}
#[doc = "The Range 2 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 2 Match Register uses the associated Range 2 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r2_mat_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r2_mat_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aR2MatRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aR2MatRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_r2_mat_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aR2MatRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_r2_mat_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aR2MatRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_R2_MAT_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aR2MatRegSpec {
    const RESET_VALUE: u32 = 0;
}

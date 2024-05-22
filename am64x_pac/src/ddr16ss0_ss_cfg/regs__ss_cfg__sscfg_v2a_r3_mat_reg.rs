#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_R3_MAT_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aR3MatRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_R3_MAT_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aR3MatRegSpec>;
#[doc = "Field `RANGE3_ROUTEID_B` reader - 11:0\\]
The range3_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
pub type Range3RouteidBR = crate::FieldReader<u16>;
#[doc = "Field `RANGE3_ROUTEID_B` writer - 11:0\\]
The range3_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
pub type Range3RouteidBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RANGE3_MASK_B` reader - 14:12\\]
The range3_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
pub type Range3MaskBR = crate::FieldReader;
#[doc = "Field `RANGE3_MASK_B` writer - 14:12\\]
The range3_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
pub type Range3MaskBW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE3_RANGEEN_B` reader - 15:15\\]
The range3_rangeen_b enables the RouteID AND'd with range3_mask_b to match the range3_routeid_b"]
pub type Range3RangeenBR = crate::BitReader;
#[doc = "Field `RANGE3_RANGEEN_B` writer - 15:15\\]
The range3_rangeen_b enables the RouteID AND'd with range3_mask_b to match the range3_routeid_b"]
pub type Range3RangeenBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RANGE3_ROUTEID_A` reader - 27:16\\]
The range3_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
pub type Range3RouteidAR = crate::FieldReader<u16>;
#[doc = "Field `RANGE3_ROUTEID_A` writer - 27:16\\]
The range3_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
pub type Range3RouteidAW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RANGE3_MASK_A` reader - 30:28\\]
The range3_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
pub type Range3MaskAR = crate::FieldReader;
#[doc = "Field `RANGE3_MASK_A` writer - 30:28\\]
The range3_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
pub type Range3MaskAW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RANGE3_RANGEEN_A` reader - 31:31\\]
The range3_rangeen_a enables the RouteID AND'd with range3_mask_a to match the range3_routeid_a"]
pub type Range3RangeenAR = crate::BitReader;
#[doc = "Field `RANGE3_RANGEEN_A` writer - 31:31\\]
The range3_rangeen_a enables the RouteID AND'd with range3_mask_a to match the range3_routeid_a"]
pub type Range3RangeenAW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
The range3_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    pub fn range3_routeid_b(&self) -> Range3RouteidBR {
        Range3RouteidBR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The range3_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
    #[inline(always)]
    pub fn range3_mask_b(&self) -> Range3MaskBR {
        Range3MaskBR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
The range3_rangeen_b enables the RouteID AND'd with range3_mask_b to match the range3_routeid_b"]
    #[inline(always)]
    pub fn range3_rangeen_b(&self) -> Range3RangeenBR {
        Range3RangeenBR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
The range3_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    pub fn range3_routeid_a(&self) -> Range3RouteidAR {
        Range3RouteidAR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The range3_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
    #[inline(always)]
    pub fn range3_mask_a(&self) -> Range3MaskAR {
        Range3MaskAR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
The range3_rangeen_a enables the RouteID AND'd with range3_mask_a to match the range3_routeid_a"]
    #[inline(always)]
    pub fn range3_rangeen_a(&self) -> Range3RangeenAR {
        Range3RangeenAR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
The range3_routeid_b is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    #[must_use]
    pub fn range3_routeid_b(&mut self) -> Range3RouteidBW<Regs_SsCfg_SscfgV2aR3MatRegSpec> {
        Range3RouteidBW::new(self, 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
The range3_mask_b allows a number of least significant bits to be ignored prior to the match of the routeid_b"]
    #[inline(always)]
    #[must_use]
    pub fn range3_mask_b(&mut self) -> Range3MaskBW<Regs_SsCfg_SscfgV2aR3MatRegSpec> {
        Range3MaskBW::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
The range3_rangeen_b enables the RouteID AND'd with range3_mask_b to match the range3_routeid_b"]
    #[inline(always)]
    #[must_use]
    pub fn range3_rangeen_b(&mut self) -> Range3RangeenBW<Regs_SsCfg_SscfgV2aR3MatRegSpec> {
        Range3RangeenBW::new(self, 15)
    }
    #[doc = "Bits 16:27 - 27:16\\]
The range3_routeid_a is the value that is compared to the RouteID arriving on the command interface"]
    #[inline(always)]
    #[must_use]
    pub fn range3_routeid_a(&mut self) -> Range3RouteidAW<Regs_SsCfg_SscfgV2aR3MatRegSpec> {
        Range3RouteidAW::new(self, 16)
    }
    #[doc = "Bits 28:30 - 30:28\\]
The range3_mask_a allows a number of least significant bits to be ignored prior to the match of the routeid_a"]
    #[inline(always)]
    #[must_use]
    pub fn range3_mask_a(&mut self) -> Range3MaskAW<Regs_SsCfg_SscfgV2aR3MatRegSpec> {
        Range3MaskAW::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
The range3_rangeen_a enables the RouteID AND'd with range3_mask_a to match the range3_routeid_a"]
    #[inline(always)]
    #[must_use]
    pub fn range3_rangeen_a(&mut self) -> Range3RangeenAW<Regs_SsCfg_SscfgV2aR3MatRegSpec> {
        Range3RangeenAW::new(self, 31)
    }
}
#[doc = "The Range 3 Match Register allows a single master to a range of masters to change their priority mapping. This allows selective masters to be increased or decreased in effective priority. Range 3 Match Register uses the associated Range 3 Priority Map Register. The highest Range Match Register will take priority and will be used in case of multiple range matches.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_r3_mat_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_r3_mat_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aR3MatRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aR3MatRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_r3_mat_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aR3MatRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_r3_mat_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aR3MatRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_R3_MAT_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aR3MatRegSpec {
    const RESET_VALUE: u32 = 0;
}

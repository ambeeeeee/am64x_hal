#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_CTL_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aCtlRegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_CTL_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aCtlRegSpec>;
#[doc = "Field `REGION_IDX` reader - 4:0\\]
Region Index = log2(CBA region size) - 16. The region_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
pub type RegionIdxR = crate::FieldReader;
#[doc = "Field `REGION_IDX` writer - 4:0\\]
Region Index = log2(CBA region size) - 16. The region_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
pub type RegionIdxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SDRAM_IDX` reader - 9:5\\]
SDRAM Index = log2(connected SDRAM size) - 16. The sdram_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
pub type SdramIdxR = crate::FieldReader;
#[doc = "Field `SDRAM_IDX` writer - 9:5\\]
SDRAM Index = log2(connected SDRAM size) - 16. The sdram_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
pub type SdramIdxW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SDRAM_3QT` reader - 10:10\\]
Setting this field to a 1 will modify SDRAM Index to be 3/4 its programmed value to support 3, 6, 12 and 24 GB sizes."]
pub type Sdram3qtR = crate::BitReader;
#[doc = "Field `SDRAM_3QT` writer - 10:10\\]
Setting this field to a 1 will modify SDRAM Index to be 3/4 its programmed value to support 3, 6, 12 and 24 GB sizes."]
pub type Sdram3qtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Region Index = log2(CBA region size) - 16. The region_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
    #[inline(always)]
    pub fn region_idx(&self) -> RegionIdxR {
        RegionIdxR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 9:5\\]
SDRAM Index = log2(connected SDRAM size) - 16. The sdram_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
    #[inline(always)]
    pub fn sdram_idx(&self) -> SdramIdxR {
        SdramIdxR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - 10:10\\]
Setting this field to a 1 will modify SDRAM Index to be 3/4 its programmed value to support 3, 6, 12 and 24 GB sizes."]
    #[inline(always)]
    pub fn sdram_3qt(&self) -> Sdram3qtR {
        Sdram3qtR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Region Index = log2(CBA region size) - 16. The region_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
    #[inline(always)]
    #[must_use]
    pub fn region_idx(&mut self) -> RegionIdxW<Regs_SsCfg_SscfgV2aCtlRegSpec> {
        RegionIdxW::new(self, 0)
    }
    #[doc = "Bits 5:9 - 9:5\\]
SDRAM Index = log2(connected SDRAM size) - 16. The sdram_idx describes the number of address bits minus 16 that are used to determine the mask used to detect memory rollover and prevent aliasing and false coherency issues. Max size supported is 8GB. A programmed value greater than 0x11 will result in this field being reset to 0x11."]
    #[inline(always)]
    #[must_use]
    pub fn sdram_idx(&mut self) -> SdramIdxW<Regs_SsCfg_SscfgV2aCtlRegSpec> {
        SdramIdxW::new(self, 5)
    }
    #[doc = "Bit 10 - 10:10\\]
Setting this field to a 1 will modify SDRAM Index to be 3/4 its programmed value to support 3, 6, 12 and 24 GB sizes."]
    #[inline(always)]
    #[must_use]
    pub fn sdram_3qt(&mut self) -> Sdram3qtW<Regs_SsCfg_SscfgV2aCtlRegSpec> {
        Sdram3qtW::new(self, 10)
    }
}
#[doc = "The VBUSM2AXI Control register contains control functions required for the VBUSM2AXI submodule.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_ctl_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_ctl_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aCtlRegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aCtlRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_ctl_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aCtlRegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_ctl_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aCtlRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_CTL_REG to value 0x02f7"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aCtlRegSpec {
    const RESET_VALUE: u32 = 0x02f7;
}

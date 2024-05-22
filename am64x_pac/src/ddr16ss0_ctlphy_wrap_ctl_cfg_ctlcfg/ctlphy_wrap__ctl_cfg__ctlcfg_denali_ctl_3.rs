#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_3` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_3` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec>;
#[doc = "Field `MAX_ROW_REG` reader - 4:0\\]
Holds the maximum width of memory address bus. READ-ONLY"]
pub type MaxRowRegR = crate::FieldReader;
#[doc = "Field `MAX_ROW_REG` writer - 4:0\\]
Holds the maximum width of memory address bus. READ-ONLY"]
pub type MaxRowRegW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAX_COL_REG` reader - 11:8\\]
Holds the maximum width of column address in DRAMs. READ-ONLY"]
pub type MaxColRegR = crate::FieldReader;
#[doc = "Field `MAX_COL_REG` writer - 11:8\\]
Holds the maximum width of column address in DRAMs. READ-ONLY"]
pub type MaxColRegW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAX_CS_REG` reader - 17:16\\]
Holds the maximum number of chip selects available. READ-ONLY"]
pub type MaxCsRegR = crate::FieldReader;
#[doc = "Field `MAX_CS_REG` writer - 17:16\\]
Holds the maximum number of chip selects available. READ-ONLY"]
pub type MaxCsRegW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `READ_DATA_FIFO_DEPTH` reader - 31:24\\]
Reports the depth of the controller core read data queue. READ-ONLY"]
pub type ReadDataFifoDepthR = crate::FieldReader;
#[doc = "Field `READ_DATA_FIFO_DEPTH` writer - 31:24\\]
Reports the depth of the controller core read data queue. READ-ONLY"]
pub type ReadDataFifoDepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Holds the maximum width of memory address bus. READ-ONLY"]
    #[inline(always)]
    pub fn max_row_reg(&self) -> MaxRowRegR {
        MaxRowRegR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Holds the maximum width of column address in DRAMs. READ-ONLY"]
    #[inline(always)]
    pub fn max_col_reg(&self) -> MaxColRegR {
        MaxColRegR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Holds the maximum number of chip selects available. READ-ONLY"]
    #[inline(always)]
    pub fn max_cs_reg(&self) -> MaxCsRegR {
        MaxCsRegR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reports the depth of the controller core read data queue. READ-ONLY"]
    #[inline(always)]
    pub fn read_data_fifo_depth(&self) -> ReadDataFifoDepthR {
        ReadDataFifoDepthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Holds the maximum width of memory address bus. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn max_row_reg(&mut self) -> MaxRowRegW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec> {
        MaxRowRegW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Holds the maximum width of column address in DRAMs. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn max_col_reg(&mut self) -> MaxColRegW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec> {
        MaxColRegW::new(self, 8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Holds the maximum number of chip selects available. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn max_cs_reg(&mut self) -> MaxCsRegW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec> {
        MaxCsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reports the depth of the controller core read data queue. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn read_data_fifo_depth(
        &mut self,
    ) -> ReadDataFifoDepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec> {
        ReadDataFifoDepthW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_3::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_3::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_3 to value 0x6402_1017"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl3Spec {
    const RESET_VALUE: u32 = 0x6402_1017;
}

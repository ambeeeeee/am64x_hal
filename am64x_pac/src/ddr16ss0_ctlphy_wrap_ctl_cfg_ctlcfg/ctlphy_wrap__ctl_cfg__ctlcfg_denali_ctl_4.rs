#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_4` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_4` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec>;
#[doc = "Field `READ_DATA_FIFO_PTR_WIDTH` reader - 7:0\\]
Reports the width of the controller core read data queue pointer. READ-ONLY"]
pub type ReadDataFifoPtrWidthR = crate::FieldReader;
#[doc = "Field `READ_DATA_FIFO_PTR_WIDTH` writer - 7:0\\]
Reports the width of the controller core read data queue pointer. READ-ONLY"]
pub type ReadDataFifoPtrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRITE_DATA_FIFO_DEPTH` reader - 15:8\\]
Reports the depth of the controller core write data latency queue. READ-ONLY"]
pub type WriteDataFifoDepthR = crate::FieldReader;
#[doc = "Field `WRITE_DATA_FIFO_DEPTH` writer - 15:8\\]
Reports the depth of the controller core write data latency queue. READ-ONLY"]
pub type WriteDataFifoDepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WRITE_DATA_FIFO_PTR_WIDTH` reader - 23:16\\]
Reports the width of the controller core write data latency queue pointer. READ-ONLY"]
pub type WriteDataFifoPtrWidthR = crate::FieldReader;
#[doc = "Field `WRITE_DATA_FIFO_PTR_WIDTH` writer - 23:16\\]
Reports the width of the controller core write data latency queue pointer. READ-ONLY"]
pub type WriteDataFifoPtrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reports the width of the controller core read data queue pointer. READ-ONLY"]
    #[inline(always)]
    pub fn read_data_fifo_ptr_width(&self) -> ReadDataFifoPtrWidthR {
        ReadDataFifoPtrWidthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reports the depth of the controller core write data latency queue. READ-ONLY"]
    #[inline(always)]
    pub fn write_data_fifo_depth(&self) -> WriteDataFifoDepthR {
        WriteDataFifoDepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reports the width of the controller core write data latency queue pointer. READ-ONLY"]
    #[inline(always)]
    pub fn write_data_fifo_ptr_width(&self) -> WriteDataFifoPtrWidthR {
        WriteDataFifoPtrWidthR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reports the width of the controller core read data queue pointer. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn read_data_fifo_ptr_width(
        &mut self,
    ) -> ReadDataFifoPtrWidthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec> {
        ReadDataFifoPtrWidthW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reports the depth of the controller core write data latency queue. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn write_data_fifo_depth(
        &mut self,
    ) -> WriteDataFifoDepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec> {
        WriteDataFifoDepthW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reports the width of the controller core write data latency queue pointer. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn write_data_fifo_ptr_width(
        &mut self,
    ) -> WriteDataFifoPtrWidthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec> {
        WriteDataFifoPtrWidthW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_4::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_4::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_4 to value 0x0005_3206"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl4Spec {
    const RESET_VALUE: u32 = 0x0005_3206;
}

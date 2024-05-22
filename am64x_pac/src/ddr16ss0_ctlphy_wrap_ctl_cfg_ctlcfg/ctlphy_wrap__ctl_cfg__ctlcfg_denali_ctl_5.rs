#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_5` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_5` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec>;
#[doc = "Field `MEMCD_RMODW_FIFO_DEPTH` reader - 15:0\\]
Reports the depth of the controller core read/modify/write FIFO. READ-ONLY"]
pub type MemcdRmodwFifoDepthR = crate::FieldReader<u16>;
#[doc = "Field `MEMCD_RMODW_FIFO_DEPTH` writer - 15:0\\]
Reports the depth of the controller core read/modify/write FIFO. READ-ONLY"]
pub type MemcdRmodwFifoDepthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MEMCD_RMODW_FIFO_PTR_WIDTH` reader - 23:16\\]
Reports the width of the controller core read/modify/write FIFO pointer. READ-ONLY"]
pub type MemcdRmodwFifoPtrWidthR = crate::FieldReader;
#[doc = "Field `MEMCD_RMODW_FIFO_PTR_WIDTH` writer - 23:16\\]
Reports the width of the controller core read/modify/write FIFO pointer. READ-ONLY"]
pub type MemcdRmodwFifoPtrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ASYNC_CDC_STAGES` reader - 31:24\\]
Reports the number of synchronizer delays specified for the asynchronous boundary crossings. READ-ONLY"]
pub type AsyncCdcStagesR = crate::FieldReader;
#[doc = "Field `ASYNC_CDC_STAGES` writer - 31:24\\]
Reports the number of synchronizer delays specified for the asynchronous boundary crossings. READ-ONLY"]
pub type AsyncCdcStagesW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Reports the depth of the controller core read/modify/write FIFO. READ-ONLY"]
    #[inline(always)]
    pub fn memcd_rmodw_fifo_depth(&self) -> MemcdRmodwFifoDepthR {
        MemcdRmodwFifoDepthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reports the width of the controller core read/modify/write FIFO pointer. READ-ONLY"]
    #[inline(always)]
    pub fn memcd_rmodw_fifo_ptr_width(&self) -> MemcdRmodwFifoPtrWidthR {
        MemcdRmodwFifoPtrWidthR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reports the number of synchronizer delays specified for the asynchronous boundary crossings. READ-ONLY"]
    #[inline(always)]
    pub fn async_cdc_stages(&self) -> AsyncCdcStagesR {
        AsyncCdcStagesR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Reports the depth of the controller core read/modify/write FIFO. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn memcd_rmodw_fifo_depth(
        &mut self,
    ) -> MemcdRmodwFifoDepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec> {
        MemcdRmodwFifoDepthW::new(self, 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reports the width of the controller core read/modify/write FIFO pointer. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn memcd_rmodw_fifo_ptr_width(
        &mut self,
    ) -> MemcdRmodwFifoPtrWidthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec> {
        MemcdRmodwFifoPtrWidthW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reports the number of synchronizer delays specified for the asynchronous boundary crossings. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn async_cdc_stages(&mut self) -> AsyncCdcStagesW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec> {
        AsyncCdcStagesW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_5::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_5::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_5 to value 0x0205_0032"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl5Spec {
    const RESET_VALUE: u32 = 0x0205_0032;
}

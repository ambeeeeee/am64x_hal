#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_6` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_6` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec>;
#[doc = "Field `AXI0_CMDFIFO_LOG2_DEPTH` reader - 7:0\\]
Reports the depth of the AXI port 0 command FIFO. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0CmdfifoLog2DepthR = crate::FieldReader;
#[doc = "Field `AXI0_CMDFIFO_LOG2_DEPTH` writer - 7:0\\]
Reports the depth of the AXI port 0 command FIFO. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0CmdfifoLog2DepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AXI0_RDFIFO_LOG2_DEPTH` reader - 15:8\\]
Reports the depth of the AXI port 0 read data FIFO. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0RdfifoLog2DepthR = crate::FieldReader;
#[doc = "Field `AXI0_RDFIFO_LOG2_DEPTH` writer - 15:8\\]
Reports the depth of the AXI port 0 read data FIFO. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0RdfifoLog2DepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AXI0_WR_ARRAY_LOG2_DEPTH` reader - 23:16\\]
Reports the depth of the AXI port 0 write data array. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0WrArrayLog2DepthR = crate::FieldReader;
#[doc = "Field `AXI0_WR_ARRAY_LOG2_DEPTH` writer - 23:16\\]
Reports the depth of the AXI port 0 write data array. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0WrArrayLog2DepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AXI0_WRCMD_PROC_FIFO_LOG2_DEPTH` reader - 31:24\\]
Reports the depth of the AXI port 0 write command processing FIFO. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0WrcmdProcFifoLog2DepthR = crate::FieldReader;
#[doc = "Field `AXI0_WRCMD_PROC_FIFO_LOG2_DEPTH` writer - 31:24\\]
Reports the depth of the AXI port 0 write command processing FIFO. Value is the log2 value of the depth. READ-ONLY"]
pub type Axi0WrcmdProcFifoLog2DepthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reports the depth of the AXI port 0 command FIFO. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    pub fn axi0_cmdfifo_log2_depth(&self) -> Axi0CmdfifoLog2DepthR {
        Axi0CmdfifoLog2DepthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reports the depth of the AXI port 0 read data FIFO. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    pub fn axi0_rdfifo_log2_depth(&self) -> Axi0RdfifoLog2DepthR {
        Axi0RdfifoLog2DepthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reports the depth of the AXI port 0 write data array. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    pub fn axi0_wr_array_log2_depth(&self) -> Axi0WrArrayLog2DepthR {
        Axi0WrArrayLog2DepthR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reports the depth of the AXI port 0 write command processing FIFO. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    pub fn axi0_wrcmd_proc_fifo_log2_depth(&self) -> Axi0WrcmdProcFifoLog2DepthR {
        Axi0WrcmdProcFifoLog2DepthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reports the depth of the AXI port 0 command FIFO. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn axi0_cmdfifo_log2_depth(
        &mut self,
    ) -> Axi0CmdfifoLog2DepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec> {
        Axi0CmdfifoLog2DepthW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reports the depth of the AXI port 0 read data FIFO. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn axi0_rdfifo_log2_depth(
        &mut self,
    ) -> Axi0RdfifoLog2DepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec> {
        Axi0RdfifoLog2DepthW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reports the depth of the AXI port 0 write data array. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn axi0_wr_array_log2_depth(
        &mut self,
    ) -> Axi0WrArrayLog2DepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec> {
        Axi0WrArrayLog2DepthW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reports the depth of the AXI port 0 write command processing FIFO. Value is the log2 value of the depth. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn axi0_wrcmd_proc_fifo_log2_depth(
        &mut self,
    ) -> Axi0WrcmdProcFifoLog2DepthW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec> {
        Axi0WrcmdProcFifoLog2DepthW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_6::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_6::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_6 to value 0x0307_0101"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl6Spec {
    const RESET_VALUE: u32 = 0x0307_0101;
}

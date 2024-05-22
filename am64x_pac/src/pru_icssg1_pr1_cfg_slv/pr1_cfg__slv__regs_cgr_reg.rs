#[doc = "Register `PR1_CFG__SLV__REGS_cgr_reg` reader"]
pub type R = crate::R<Pr1Cfg_Slv_RegsCgrRegSpec>;
#[doc = "Register `PR1_CFG__SLV__REGS_cgr_reg` writer"]
pub type W = crate::W<Pr1Cfg_Slv_RegsCgrRegSpec>;
#[doc = "Field `INTC_CLK_STOP_REQ` reader - "]
pub type IntcClkStopReqR = crate::BitReader;
#[doc = "Field `INTC_CLK_STOP_REQ` writer - "]
pub type IntcClkStopReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC_CLK_STOP_ACK` reader - "]
pub type IntcClkStopAckR = crate::BitReader;
#[doc = "Field `INTC_CLK_STOP_ACK` writer - "]
pub type IntcClkStopAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC_CLK_EN` reader - "]
pub type IntcClkEnR = crate::BitReader;
#[doc = "Field `INTC_CLK_EN` writer - "]
pub type IntcClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CLK_STOP_REQ` reader - "]
pub type UartClkStopReqR = crate::BitReader;
#[doc = "Field `UART_CLK_STOP_REQ` writer - "]
pub type UartClkStopReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CLK_STOP_ACK` reader - "]
pub type UartClkStopAckR = crate::BitReader;
#[doc = "Field `UART_CLK_STOP_ACK` writer - "]
pub type UartClkStopAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_CLK_EN` reader - "]
pub type UartClkEnR = crate::BitReader;
#[doc = "Field `UART_CLK_EN` writer - "]
pub type UartClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP_CLK_STOP_REQ` reader - "]
pub type EcapClkStopReqR = crate::BitReader;
#[doc = "Field `ECAP_CLK_STOP_REQ` writer - "]
pub type EcapClkStopReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP_CLK_STOP_ACK` reader - "]
pub type EcapClkStopAckR = crate::BitReader;
#[doc = "Field `ECAP_CLK_STOP_ACK` writer - "]
pub type EcapClkStopAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECAP_CLK_EN` reader - "]
pub type EcapClkEnR = crate::BitReader;
#[doc = "Field `ECAP_CLK_EN` writer - "]
pub type EcapClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEP_CLK_STOP_REQ` reader - "]
pub type IepClkStopReqR = crate::BitReader;
#[doc = "Field `IEP_CLK_STOP_REQ` writer - "]
pub type IepClkStopReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEP_CLK_STOP_ACK` reader - "]
pub type IepClkStopAckR = crate::BitReader;
#[doc = "Field `IEP_CLK_STOP_ACK` writer - "]
pub type IepClkStopAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEP_CLK_EN` reader - "]
pub type IepClkEnR = crate::BitReader;
#[doc = "Field `IEP_CLK_EN` writer - "]
pub type IepClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_SLICE0_CLK_GATE_EN` reader - "]
pub type AutoSlice0ClkGateEnR = crate::BitReader;
#[doc = "Field `AUTO_SLICE0_CLK_GATE_EN` writer - "]
pub type AutoSlice0ClkGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_SLICE1_CLK_GATE_EN` reader - "]
pub type AutoSlice1ClkGateEnR = crate::BitReader;
#[doc = "Field `AUTO_SLICE1_CLK_GATE_EN` writer - "]
pub type AutoSlice1ClkGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOP_HALF_CLK_GATE_EN` reader - "]
pub type TopHalfClkGateEnR = crate::BitReader;
#[doc = "Field `TOP_HALF_CLK_GATE_EN` writer - "]
pub type TopHalfClkGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOTTOM_HALF_CLK_GATE_EN` reader - "]
pub type BottomHalfClkGateEnR = crate::BitReader;
#[doc = "Field `BOTTOM_HALF_CLK_GATE_EN` writer - "]
pub type BottomHalfClkGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSS_PWR_IDLE` reader - "]
pub type IcssPwrIdleR = crate::BitReader;
#[doc = "Field `ICSS_PWR_IDLE` writer - "]
pub type IcssPwrIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSS_STOP_REQ` reader - "]
pub type IcssStopReqR = crate::BitReader;
#[doc = "Field `ICSS_STOP_REQ` writer - "]
pub type IcssStopReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSS_STOP_ACK` reader - "]
pub type IcssStopAckR = crate::BitReader;
#[doc = "Field `ICSS_STOP_ACK` writer - "]
pub type IcssStopAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn intc_clk_stop_req(&self) -> IntcClkStopReqR {
        IntcClkStopReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn intc_clk_stop_ack(&self) -> IntcClkStopAckR {
        IntcClkStopAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn intc_clk_en(&self) -> IntcClkEnR {
        IntcClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn uart_clk_stop_req(&self) -> UartClkStopReqR {
        UartClkStopReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn uart_clk_stop_ack(&self) -> UartClkStopAckR {
        UartClkStopAckR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn uart_clk_en(&self) -> UartClkEnR {
        UartClkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ecap_clk_stop_req(&self) -> EcapClkStopReqR {
        EcapClkStopReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ecap_clk_stop_ack(&self) -> EcapClkStopAckR {
        EcapClkStopAckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ecap_clk_en(&self) -> EcapClkEnR {
        EcapClkEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn iep_clk_stop_req(&self) -> IepClkStopReqR {
        IepClkStopReqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn iep_clk_stop_ack(&self) -> IepClkStopAckR {
        IepClkStopAckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn iep_clk_en(&self) -> IepClkEnR {
        IepClkEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn auto_slice0_clk_gate_en(&self) -> AutoSlice0ClkGateEnR {
        AutoSlice0ClkGateEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn auto_slice1_clk_gate_en(&self) -> AutoSlice1ClkGateEnR {
        AutoSlice1ClkGateEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn top_half_clk_gate_en(&self) -> TopHalfClkGateEnR {
        TopHalfClkGateEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn bottom_half_clk_gate_en(&self) -> BottomHalfClkGateEnR {
        BottomHalfClkGateEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn icss_pwr_idle(&self) -> IcssPwrIdleR {
        IcssPwrIdleR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn icss_stop_req(&self) -> IcssStopReqR {
        IcssStopReqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn icss_stop_ack(&self) -> IcssStopAckR {
        IcssStopAckR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn intc_clk_stop_req(&mut self) -> IntcClkStopReqW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IntcClkStopReqW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn intc_clk_stop_ack(&mut self) -> IntcClkStopAckW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IntcClkStopAckW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn intc_clk_en(&mut self) -> IntcClkEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IntcClkEnW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_stop_req(&mut self) -> UartClkStopReqW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        UartClkStopReqW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_stop_ack(&mut self) -> UartClkStopAckW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        UartClkStopAckW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn uart_clk_en(&mut self) -> UartClkEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        UartClkEnW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ecap_clk_stop_req(&mut self) -> EcapClkStopReqW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        EcapClkStopReqW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn ecap_clk_stop_ack(&mut self) -> EcapClkStopAckW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        EcapClkStopAckW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn ecap_clk_en(&mut self) -> EcapClkEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        EcapClkEnW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iep_clk_stop_req(&mut self) -> IepClkStopReqW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IepClkStopReqW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn iep_clk_stop_ack(&mut self) -> IepClkStopAckW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IepClkStopAckW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn iep_clk_en(&mut self) -> IepClkEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IepClkEnW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn auto_slice0_clk_gate_en(&mut self) -> AutoSlice0ClkGateEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        AutoSlice0ClkGateEnW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn auto_slice1_clk_gate_en(&mut self) -> AutoSlice1ClkGateEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        AutoSlice1ClkGateEnW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn top_half_clk_gate_en(&mut self) -> TopHalfClkGateEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        TopHalfClkGateEnW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn bottom_half_clk_gate_en(&mut self) -> BottomHalfClkGateEnW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        BottomHalfClkGateEnW::new(self, 21)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn icss_pwr_idle(&mut self) -> IcssPwrIdleW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IcssPwrIdleW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn icss_stop_req(&mut self) -> IcssStopReqW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IcssStopReqW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn icss_stop_ack(&mut self) -> IcssStopAckW<Pr1Cfg_Slv_RegsCgrRegSpec> {
        IcssStopAckW::new(self, 31)
    }
}
#[doc = "PR1_CFG__SLV__REGS_cgr_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_cfg__slv__regs_cgr_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_cfg__slv__regs_cgr_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Cfg_Slv_RegsCgrRegSpec;
impl crate::RegisterSpec for Pr1Cfg_Slv_RegsCgrRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_cfg__slv__regs_cgr_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Cfg_Slv_RegsCgrRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_cfg__slv__regs_cgr_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Cfg_Slv_RegsCgrRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_CFG__SLV__REGS_cgr_reg to value 0xa032_4900"]
impl crate::Resettable for Pr1Cfg_Slv_RegsCgrRegSpec {
    const RESET_VALUE: u32 = 0xa032_4900;
}

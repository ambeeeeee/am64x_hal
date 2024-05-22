#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_1_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgCtlCfg1RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_CTL_CFG_1_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgCtlCfg1RegSpec>;
#[doc = "Field `CQFVAL` reader - 9:0\\]
FVAL for the CQ Internal Timer Clock Frequency"]
pub type CqfvalR = crate::FieldReader<u16>;
#[doc = "Field `CQFVAL` writer - 9:0\\]
FVAL for the CQ Internal Timer Clock Frequency"]
pub type CqfvalW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CQFMUL` reader - 15:12\\]
FMUL for the CQ Internal Timer Clock Frequency"]
pub type CqfmulR = crate::FieldReader;
#[doc = "Field `CQFMUL` writer - 15:12\\]
FMUL for the CQ Internal Timer Clock Frequency"]
pub type CqfmulW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASYNCWKUPENA` reader - 20:20\\]
Determines the Wakeup Signal Generation Mode. 0: Synchronous Wakeup Mode: The xin_clk has to be running for this mode. The Card Insertion/Removal/Interrupt events are detected synchronously on the xin_clk and the Wakeup Event is generated. The Assertion and deassertion of the wakeup Event signal synchronous to xin_clk. 1: Asyncrhonous Wakeup Mode: The xin_clk and the host_clk can be stopped in this mode and the Wake up Event is asynchronously generated based on the Card Insertion/Removal/Interrupt Events. The Assertion and deassertion of the wakeup Event signal is asynchronous."]
pub type AsyncwkupenaR = crate::BitReader;
#[doc = "Field `ASYNCWKUPENA` writer - 20:20\\]
Determines the Wakeup Signal Generation Mode. 0: Synchronous Wakeup Mode: The xin_clk has to be running for this mode. The Card Insertion/Removal/Interrupt events are detected synchronously on the xin_clk and the Wakeup Event is generated. The Assertion and deassertion of the wakeup Event signal synchronous to xin_clk. 1: Asyncrhonous Wakeup Mode: The xin_clk and the host_clk can be stopped in this mode and the Wake up Event is asynchronously generated based on the Card Insertion/Removal/Interrupt Events. The Assertion and deassertion of the wakeup Event signal is asynchronous."]
pub type AsyncwkupenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNINGCOUNT` reader - 29:24\\]
Configures the Number of Taps (Phases) of the RX clock that is supported. The Tuning State machine uses this information to select one of the Taps (Phases) of the RX clock during the Tuning Procedure."]
pub type TuningcountR = crate::FieldReader;
#[doc = "Field `TUNINGCOUNT` writer - 29:24\\]
Configures the Number of Taps (Phases) of the RX clock that is supported. The Tuning State machine uses this information to select one of the Taps (Phases) of the RX clock during the Tuning Procedure."]
pub type TuningcountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
FVAL for the CQ Internal Timer Clock Frequency"]
    #[inline(always)]
    pub fn cqfval(&self) -> CqfvalR {
        CqfvalR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
FMUL for the CQ Internal Timer Clock Frequency"]
    #[inline(always)]
    pub fn cqfmul(&self) -> CqfmulR {
        CqfmulR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Determines the Wakeup Signal Generation Mode. 0: Synchronous Wakeup Mode: The xin_clk has to be running for this mode. The Card Insertion/Removal/Interrupt events are detected synchronously on the xin_clk and the Wakeup Event is generated. The Assertion and deassertion of the wakeup Event signal synchronous to xin_clk. 1: Asyncrhonous Wakeup Mode: The xin_clk and the host_clk can be stopped in this mode and the Wake up Event is asynchronously generated based on the Card Insertion/Removal/Interrupt Events. The Assertion and deassertion of the wakeup Event signal is asynchronous."]
    #[inline(always)]
    pub fn asyncwkupena(&self) -> AsyncwkupenaR {
        AsyncwkupenaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Configures the Number of Taps (Phases) of the RX clock that is supported. The Tuning State machine uses this information to select one of the Taps (Phases) of the RX clock during the Tuning Procedure."]
    #[inline(always)]
    pub fn tuningcount(&self) -> TuningcountR {
        TuningcountR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
FVAL for the CQ Internal Timer Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn cqfval(&mut self) -> CqfvalW<Regs_SsCfg_SscfgCtlCfg1RegSpec> {
        CqfvalW::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
FMUL for the CQ Internal Timer Clock Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn cqfmul(&mut self) -> CqfmulW<Regs_SsCfg_SscfgCtlCfg1RegSpec> {
        CqfmulW::new(self, 12)
    }
    #[doc = "Bit 20 - 20:20\\]
Determines the Wakeup Signal Generation Mode. 0: Synchronous Wakeup Mode: The xin_clk has to be running for this mode. The Card Insertion/Removal/Interrupt events are detected synchronously on the xin_clk and the Wakeup Event is generated. The Assertion and deassertion of the wakeup Event signal synchronous to xin_clk. 1: Asyncrhonous Wakeup Mode: The xin_clk and the host_clk can be stopped in this mode and the Wake up Event is asynchronously generated based on the Card Insertion/Removal/Interrupt Events. The Assertion and deassertion of the wakeup Event signal is asynchronous."]
    #[inline(always)]
    #[must_use]
    pub fn asyncwkupena(&mut self) -> AsyncwkupenaW<Regs_SsCfg_SscfgCtlCfg1RegSpec> {
        AsyncwkupenaW::new(self, 20)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Configures the Number of Taps (Phases) of the RX clock that is supported. The Tuning State machine uses this information to select one of the Taps (Phases) of the RX clock during the Tuning Procedure."]
    #[inline(always)]
    #[must_use]
    pub fn tuningcount(&mut self) -> TuningcountW<Regs_SsCfg_SscfgCtlCfg1RegSpec> {
        TuningcountW::new(self, 24)
    }
}
#[doc = "The Controller Config 1 Register contains various fields to control the configuration ports on the Arasan eMMC/SD Controller. For detailed functionality of the Arasan eMMC/SD Controller configuration ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_ctl_cfg_1_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_ctl_cfg_1_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgCtlCfg1RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgCtlCfg1RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_ctl_cfg_1_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgCtlCfg1RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_ctl_cfg_1_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgCtlCfg1RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_CTL_CFG_1_REG to value 0x3210_3200"]
impl crate::Resettable for Regs_SsCfg_SscfgCtlCfg1RegSpec {
    const RESET_VALUE: u32 = 0x3210_3200;
}

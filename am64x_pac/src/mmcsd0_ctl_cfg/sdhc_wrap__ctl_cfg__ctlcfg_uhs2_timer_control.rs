#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_timer_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_timer_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec>;
#[doc = "Field `CMDRESP_TIMEOUT_CTR` reader - 3:0\\]
This value determines the interval between com-mand packet and response packet \\[5ms\\]. Timeout clock frequency will be generated by dividing the base clock TMCLK value by this value. When set-ting this register, prevent inadvertent timeout events by clearing the Timeout for CMD_RES \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
pub type CmdrespTimeoutCtrR = crate::FieldReader;
#[doc = "Field `CMDRESP_TIMEOUT_CTR` writer - 3:0\\]
This value determines the interval between com-mand packet and response packet \\[5ms\\]. Timeout clock frequency will be generated by dividing the base clock TMCLK value by this value. When set-ting this register, prevent inadvertent timeout events by clearing the Timeout for CMD_RES \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
pub type CmdrespTimeoutCtrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DEADLOCK_TIMEOUT_CTR` reader - 7:4\\]
This value determines the deadlock period while host expecting to receive a packet \\[1 second\\]. Tim-eout clock frequency will be generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Timeout for Deadlock \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
pub type DeadlockTimeoutCtrR = crate::FieldReader;
#[doc = "Field `DEADLOCK_TIMEOUT_CTR` writer - 7:4\\]
This value determines the deadlock period while host expecting to receive a packet \\[1 second\\]. Tim-eout clock frequency will be generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Timeout for Deadlock \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
pub type DeadlockTimeoutCtrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This value determines the interval between com-mand packet and response packet \\[5ms\\]. Timeout clock frequency will be generated by dividing the base clock TMCLK value by this value. When set-ting this register, prevent inadvertent timeout events by clearing the Timeout for CMD_RES \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
    #[inline(always)]
    pub fn cmdresp_timeout_ctr(&self) -> CmdrespTimeoutCtrR {
        CmdrespTimeoutCtrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This value determines the deadlock period while host expecting to receive a packet \\[1 second\\]. Tim-eout clock frequency will be generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Timeout for Deadlock \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
    #[inline(always)]
    pub fn deadlock_timeout_ctr(&self) -> DeadlockTimeoutCtrR {
        DeadlockTimeoutCtrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This value determines the interval between com-mand packet and response packet \\[5ms\\]. Timeout clock frequency will be generated by dividing the base clock TMCLK value by this value. When set-ting this register, prevent inadvertent timeout events by clearing the Timeout for CMD_RES \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
    #[inline(always)]
    #[must_use]
    pub fn cmdresp_timeout_ctr(
        &mut self,
    ) -> CmdrespTimeoutCtrW<SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec> {
        CmdrespTimeoutCtrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
This value determines the deadlock period while host expecting to receive a packet \\[1 second\\]. Tim-eout clock frequency will be generated by dividing the base clock TMCLK value by this value. When setting this register, prevent inadvertent timeout events by clearing the Timeout for Deadlock \\[in the UHS-II Error Interrupt Status Enable register\\]
1111b Reserved 1110b TMCLK x 2^27 .............. .................. 0001b TMCLK x 2^14 0000b TMCLK x 2^13"]
    #[inline(always)]
    #[must_use]
    pub fn deadlock_timeout_ctr(
        &mut self,
    ) -> DeadlockTimeoutCtrW<SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec> {
        DeadlockTimeoutCtrW::new(self, 4)
    }
}
#[doc = "UHS-II Timeout Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_timer_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_timer_control to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2TimerControlSpec {
    const RESET_VALUE: u16 = 0;
}
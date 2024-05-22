#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_305` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_305` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec>;
#[doc = "Field `TIMEOUT_TIMER_LOG` reader - 7:0\\]
Reflects which timers experienced a timeout error \\[or had an uncleared error\\]
when the timeout interrupt fired. Bit \\[0\\]
correlates to a ZQ cal init, cs, cl, or reset FM timeout. Bit \\[1\\]
correlates to the ZQ calstart FM timeout. Bit \\[2\\]
correlates to the ZQ callatch FM timeout. Bit \\[3\\]
correlates to the MRR temperature check FM timeout. Bit \\[4\\]
correlates to the DQS oscillator FM timeout. Bit \\[5\\]
correlates to the DFI update FM timeout. Bit \\[6\\]
correlates to the low power interface wakeup timeout. Bit \\[7\\]
correlates to the auto refresh max deficit timeout. READ-ONLY"]
pub type TimeoutTimerLogR = crate::FieldReader;
#[doc = "Field `TIMEOUT_TIMER_LOG` writer - 7:0\\]
Reflects which timers experienced a timeout error \\[or had an uncleared error\\]
when the timeout interrupt fired. Bit \\[0\\]
correlates to a ZQ cal init, cs, cl, or reset FM timeout. Bit \\[1\\]
correlates to the ZQ calstart FM timeout. Bit \\[2\\]
correlates to the ZQ callatch FM timeout. Bit \\[3\\]
correlates to the MRR temperature check FM timeout. Bit \\[4\\]
correlates to the DQS oscillator FM timeout. Bit \\[5\\]
correlates to the DFI update FM timeout. Bit \\[6\\]
correlates to the low power interface wakeup timeout. Bit \\[7\\]
correlates to the auto refresh max deficit timeout. READ-ONLY"]
pub type TimeoutTimerLogW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ZQINIT_F0` reader - 19:8\\]
Number of cycles needed for a ZQINIT command. FC=0"]
pub type ZqinitF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQINIT_F0` writer - 19:8\\]
Number of cycles needed for a ZQINIT command. FC=0"]
pub type ZqinitF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reflects which timers experienced a timeout error \\[or had an uncleared error\\]
when the timeout interrupt fired. Bit \\[0\\]
correlates to a ZQ cal init, cs, cl, or reset FM timeout. Bit \\[1\\]
correlates to the ZQ calstart FM timeout. Bit \\[2\\]
correlates to the ZQ callatch FM timeout. Bit \\[3\\]
correlates to the MRR temperature check FM timeout. Bit \\[4\\]
correlates to the DQS oscillator FM timeout. Bit \\[5\\]
correlates to the DFI update FM timeout. Bit \\[6\\]
correlates to the low power interface wakeup timeout. Bit \\[7\\]
correlates to the auto refresh max deficit timeout. READ-ONLY"]
    #[inline(always)]
    pub fn timeout_timer_log(&self) -> TimeoutTimerLogR {
        TimeoutTimerLogR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Number of cycles needed for a ZQINIT command. FC=0"]
    #[inline(always)]
    pub fn zqinit_f0(&self) -> ZqinitF0R {
        ZqinitF0R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reflects which timers experienced a timeout error \\[or had an uncleared error\\]
when the timeout interrupt fired. Bit \\[0\\]
correlates to a ZQ cal init, cs, cl, or reset FM timeout. Bit \\[1\\]
correlates to the ZQ calstart FM timeout. Bit \\[2\\]
correlates to the ZQ callatch FM timeout. Bit \\[3\\]
correlates to the MRR temperature check FM timeout. Bit \\[4\\]
correlates to the DQS oscillator FM timeout. Bit \\[5\\]
correlates to the DFI update FM timeout. Bit \\[6\\]
correlates to the low power interface wakeup timeout. Bit \\[7\\]
correlates to the auto refresh max deficit timeout. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_timer_log(
        &mut self,
    ) -> TimeoutTimerLogW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec> {
        TimeoutTimerLogW::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Number of cycles needed for a ZQINIT command. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zqinit_f0(&mut self) -> ZqinitF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec> {
        ZqinitF0W::new(self, 8)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_305\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_305::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_305::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_305::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_305::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_305 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl305Spec {
    const RESET_VALUE: u32 = 0;
}

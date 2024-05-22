#[doc = "Register `CFG_DCCGCTRL` reader"]
pub type R = crate::R<CfgDccgctrlSpec>;
#[doc = "Register `CFG_DCCGCTRL` writer"]
pub type W = crate::W<CfgDccgctrlSpec>;
#[doc = "Field `DCCENA` reader - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc. User, privilege, and debug mode (read): 0101 = counters are stopped others = counters are running Privilege and debug mode (write): 0101 = stop counters and error-checking others = load the counters with their seed values and begin counting"]
pub type DccenaR = crate::FieldReader;
#[doc = "Field `DCCENA` writer - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc. User, privilege, and debug mode (read): 0101 = counters are stopped others = counters are running Privilege and debug mode (write): 0101 = stop counters and error-checking others = load the counters with their seed values and begin counting"]
pub type DccenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ERRENA` reader - 7:4\\]
The ERRENA bit enables/disables the error signal. User, privilege, and debug mode (read): 0101 = the error signal is disabled others = the error signal is enabled Privilege and debug mode (write): 0101 = disable error signal generation others = enable error signal generation"]
pub type ErrenaR = crate::FieldReader;
#[doc = "Field `ERRENA` writer - 7:4\\]
The ERRENA bit enables/disables the error signal. User, privilege, and debug mode (read): 0101 = the error signal is disabled others = the error signal is enabled Privilege and debug mode (write): 0101 = disable error signal generation others = enable error signal generation"]
pub type ErrenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLESHOT` reader - 11:8\\]
The SINGLESHOT bit enables/disables repetitive operation of the DCC. User, privilege, and debug mode (read): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error) Privilege and debug mode (write): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error)"]
pub type SingleshotR = crate::FieldReader;
#[doc = "Field `SINGLESHOT` writer - 11:8\\]
The SINGLESHOT bit enables/disables repetitive operation of the DCC. User, privilege, and debug mode (read): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error) Privilege and debug mode (write): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error)"]
pub type SingleshotW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DONEENA` reader - 15:12\\]
The DONEENA bit enables/disables the done interrupt signal, but has no effect on the done status flag in DCCSTAT register. User, privilege, and debug mode (read): 0101 = the done signal is disabled others = the done signal is enabled Privilege and debug mode (write): 0101 = disable done signal generation others = enable done signal generation"]
pub type DoneenaR = crate::FieldReader;
#[doc = "Field `DONEENA` writer - 15:12\\]
The DONEENA bit enables/disables the done interrupt signal, but has no effect on the done status flag in DCCSTAT register. User, privilege, and debug mode (read): 0101 = the done signal is disabled others = the done signal is enabled Privilege and debug mode (write): 0101 = disable done signal generation others = enable done signal generation"]
pub type DoneenaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc. User, privilege, and debug mode (read): 0101 = counters are stopped others = counters are running Privilege and debug mode (write): 0101 = stop counters and error-checking others = load the counters with their seed values and begin counting"]
    #[inline(always)]
    pub fn dccena(&self) -> DccenaR {
        DccenaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The ERRENA bit enables/disables the error signal. User, privilege, and debug mode (read): 0101 = the error signal is disabled others = the error signal is enabled Privilege and debug mode (write): 0101 = disable error signal generation others = enable error signal generation"]
    #[inline(always)]
    pub fn errena(&self) -> ErrenaR {
        ErrenaR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
The SINGLESHOT bit enables/disables repetitive operation of the DCC. User, privilege, and debug mode (read): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error) Privilege and debug mode (write): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error)"]
    #[inline(always)]
    pub fn singleshot(&self) -> SingleshotR {
        SingleshotR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
The DONEENA bit enables/disables the done interrupt signal, but has no effect on the done status flag in DCCSTAT register. User, privilege, and debug mode (read): 0101 = the done signal is disabled others = the done signal is enabled Privilege and debug mode (write): 0101 = disable done signal generation others = enable done signal generation"]
    #[inline(always)]
    pub fn doneena(&self) -> DoneenaR {
        DoneenaR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
The DCCENA bit starts and stops the operation of the dcc. User, privilege, and debug mode (read): 0101 = counters are stopped others = counters are running Privilege and debug mode (write): 0101 = stop counters and error-checking others = load the counters with their seed values and begin counting"]
    #[inline(always)]
    #[must_use]
    pub fn dccena(&mut self) -> DccenaW<CfgDccgctrlSpec> {
        DccenaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
The ERRENA bit enables/disables the error signal. User, privilege, and debug mode (read): 0101 = the error signal is disabled others = the error signal is enabled Privilege and debug mode (write): 0101 = disable error signal generation others = enable error signal generation"]
    #[inline(always)]
    #[must_use]
    pub fn errena(&mut self) -> ErrenaW<CfgDccgctrlSpec> {
        ErrenaW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
The SINGLESHOT bit enables/disables repetitive operation of the DCC. User, privilege, and debug mode (read): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error) Privilege and debug mode (write): 1010 = stop counting when counter0 and valid0 both reach zero 1011 = stop counting when counter1 reaches zero others = continuously repeat (until error)"]
    #[inline(always)]
    #[must_use]
    pub fn singleshot(&mut self) -> SingleshotW<CfgDccgctrlSpec> {
        SingleshotW::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
The DONEENA bit enables/disables the done interrupt signal, but has no effect on the done status flag in DCCSTAT register. User, privilege, and debug mode (read): 0101 = the done signal is disabled others = the done signal is enabled Privilege and debug mode (write): 0101 = disable done signal generation others = enable done signal generation"]
    #[inline(always)]
    #[must_use]
    pub fn doneena(&mut self) -> DoneenaW<CfgDccgctrlSpec> {
        DoneenaW::new(self, 12)
    }
}
#[doc = "Starts / stops the counters. Clears the error signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dccgctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dccgctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDccgctrlSpec;
impl crate::RegisterSpec for CfgDccgctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dccgctrl::R`](R) reader structure"]
impl crate::Readable for CfgDccgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dccgctrl::W`](W) writer structure"]
impl crate::Writable for CfgDccgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DCCGCTRL to value 0x5555"]
impl crate::Resettable for CfgDccgctrlSpec {
    const RESET_VALUE: u32 = 0x5555;
}

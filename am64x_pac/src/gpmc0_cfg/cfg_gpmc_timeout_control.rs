#[doc = "Register `CFG_GPMC_TIMEOUT_CONTROL` reader"]
pub type R = crate::R<CfgGpmcTimeoutControlSpec>;
#[doc = "Register `CFG_GPMC_TIMEOUT_CONTROL` writer"]
pub type W = crate::W<CfgGpmcTimeoutControlSpec>;
#[doc = "Field `TIMEOUTENABLE` reader - 0:0\\]
Enable bit of the TimeOut feature"]
pub type TimeoutenableR = crate::BitReader;
#[doc = "Field `TIMEOUTENABLE` writer - 0:0\\]
Enable bit of the TimeOut feature"]
pub type TimeoutenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUTSTARTVALUE` reader - 12:4\\]
Start value of the time-out counter \\[0x000 corresponds to 0 GPMC.FCLK cycle, 0x001 corresponds to 1 GmpcClk cycle, &amp;, 0x1FF corresponds to 511 GPMC.FCLK cyles.\\]"]
pub type TimeoutstartvalueR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUTSTARTVALUE` writer - 12:4\\]
Start value of the time-out counter \\[0x000 corresponds to 0 GPMC.FCLK cycle, 0x001 corresponds to 1 GmpcClk cycle, &amp;, 0x1FF corresponds to 511 GPMC.FCLK cyles.\\]"]
pub type TimeoutstartvalueW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable bit of the TimeOut feature"]
    #[inline(always)]
    pub fn timeoutenable(&self) -> TimeoutenableR {
        TimeoutenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:12 - 12:4\\]
Start value of the time-out counter \\[0x000 corresponds to 0 GPMC.FCLK cycle, 0x001 corresponds to 1 GmpcClk cycle, &amp;, 0x1FF corresponds to 511 GPMC.FCLK cyles.\\]"]
    #[inline(always)]
    pub fn timeoutstartvalue(&self) -> TimeoutstartvalueR {
        TimeoutstartvalueR::new(((self.bits >> 4) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable bit of the TimeOut feature"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutenable(&mut self) -> TimeoutenableW<CfgGpmcTimeoutControlSpec> {
        TimeoutenableW::new(self, 0)
    }
    #[doc = "Bits 4:12 - 12:4\\]
Start value of the time-out counter \\[0x000 corresponds to 0 GPMC.FCLK cycle, 0x001 corresponds to 1 GmpcClk cycle, &amp;, 0x1FF corresponds to 511 GPMC.FCLK cyles.\\]"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutstartvalue(&mut self) -> TimeoutstartvalueW<CfgGpmcTimeoutControlSpec> {
        TimeoutstartvalueW::new(self, 4)
    }
}
#[doc = "The GPMC_TIMEOUT_CONTROL register allows the user to set the start value of the timeout counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_timeout_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_timeout_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcTimeoutControlSpec;
impl crate::RegisterSpec for CfgGpmcTimeoutControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_timeout_control::R`](R) reader structure"]
impl crate::Readable for CfgGpmcTimeoutControlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_timeout_control::W`](W) writer structure"]
impl crate::Writable for CfgGpmcTimeoutControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_TIMEOUT_CONTROL to value 0x5110"]
impl crate::Resettable for CfgGpmcTimeoutControlSpec {
    const RESET_VALUE: u32 = 0x5110;
}

#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0McuM4fss0LbistStatProxySpec>;
#[doc = "Register `CFG0_MCU_M4FSS0_LBIST_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0McuM4fss0LbistStatProxySpec>;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_MISR_MUX_CTL_PROXY` reader - 7:0\\]
Selects block of 32 MISR bits to read. A value of 0 selects a compacted 32-bit version of the full MISR. A value of 1-32 select a 32-bit segment of the MISR."]
pub type McuM4fss0LbistStatMisrMuxCtlProxyR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_MISR_MUX_CTL_PROXY` writer - 7:0\\]
Selects block of 32 MISR bits to read. A value of 0 selects a compacted 32-bit version of the full MISR. A value of 1-32 select a 32-bit segment of the MISR."]
pub type McuM4fss0LbistStatMisrMuxCtlProxyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_OUT_MUX_CTL_PROXY` reader - 9:8\\]
Selects source of LBIST output"]
pub type McuM4fss0LbistStatOutMuxCtlProxyR = crate::FieldReader;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_OUT_MUX_CTL_PROXY` writer - 9:8\\]
Selects source of LBIST output"]
pub type McuM4fss0LbistStatOutMuxCtlProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_BIST_RUNNING_PROXY` reader - 15:15\\]
LBIST is running"]
pub type McuM4fss0LbistStatBistRunningProxyR = crate::BitReader;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_BIST_RUNNING_PROXY` writer - 15:15\\]
LBIST is running"]
pub type McuM4fss0LbistStatBistRunningProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_BIST_DONE_PROXY` reader - 31:31\\]
LBIST is done"]
pub type McuM4fss0LbistStatBistDoneProxyR = crate::BitReader;
#[doc = "Field `MCU_M4FSS0_LBIST_STAT_BIST_DONE_PROXY` writer - 31:31\\]
LBIST is done"]
pub type McuM4fss0LbistStatBistDoneProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Selects block of 32 MISR bits to read. A value of 0 selects a compacted 32-bit version of the full MISR. A value of 1-32 select a 32-bit segment of the MISR."]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_stat_misr_mux_ctl_proxy(&self) -> McuM4fss0LbistStatMisrMuxCtlProxyR {
        McuM4fss0LbistStatMisrMuxCtlProxyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects source of LBIST output"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_stat_out_mux_ctl_proxy(&self) -> McuM4fss0LbistStatOutMuxCtlProxyR {
        McuM4fss0LbistStatOutMuxCtlProxyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
LBIST is running"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_stat_bist_running_proxy(&self) -> McuM4fss0LbistStatBistRunningProxyR {
        McuM4fss0LbistStatBistRunningProxyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
LBIST is done"]
    #[inline(always)]
    pub fn mcu_m4fss0_lbist_stat_bist_done_proxy(&self) -> McuM4fss0LbistStatBistDoneProxyR {
        McuM4fss0LbistStatBistDoneProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Selects block of 32 MISR bits to read. A value of 0 selects a compacted 32-bit version of the full MISR. A value of 1-32 select a 32-bit segment of the MISR."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_stat_misr_mux_ctl_proxy(
        &mut self,
    ) -> McuM4fss0LbistStatMisrMuxCtlProxyW<Cfg0McuM4fss0LbistStatProxySpec> {
        McuM4fss0LbistStatMisrMuxCtlProxyW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Selects source of LBIST output"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_stat_out_mux_ctl_proxy(
        &mut self,
    ) -> McuM4fss0LbistStatOutMuxCtlProxyW<Cfg0McuM4fss0LbistStatProxySpec> {
        McuM4fss0LbistStatOutMuxCtlProxyW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
LBIST is running"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_stat_bist_running_proxy(
        &mut self,
    ) -> McuM4fss0LbistStatBistRunningProxyW<Cfg0McuM4fss0LbistStatProxySpec> {
        McuM4fss0LbistStatBistRunningProxyW::new(self, 15)
    }
    #[doc = "Bit 31 - 31:31\\]
LBIST is done"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_m4fss0_lbist_stat_bist_done_proxy(
        &mut self,
    ) -> McuM4fss0LbistStatBistDoneProxyW<Cfg0McuM4fss0LbistStatProxySpec> {
        McuM4fss0LbistStatBistDoneProxyW::new(self, 31)
    }
}
#[doc = "CFG0_MCU_M4FSS0_LBIST_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_m4fss0_lbist_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_m4fss0_lbist_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuM4fss0LbistStatProxySpec;
impl crate::RegisterSpec for Cfg0McuM4fss0LbistStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_m4fss0_lbist_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0McuM4fss0LbistStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_m4fss0_lbist_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0McuM4fss0LbistStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_M4FSS0_LBIST_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0McuM4fss0LbistStatProxySpec {
    const RESET_VALUE: u32 = 0;
}

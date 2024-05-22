#[doc = "Register `APBADDR_ETM_CPU1_TRCAUXCTLR` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcauxctlrSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCAUXCTLR` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcauxctlrSpec>;
#[doc = "Field `AFREADY` reader - 0:0\\]
Always respond to AFREADY immediately. Does not have any interaction with FIFO draining even in WFI state."]
pub type AfreadyR = crate::BitReader;
#[doc = "Field `AFREADY` writer - 0:0\\]
Always respond to AFREADY immediately. Does not have any interaction with FIFO draining even in WFI state."]
pub type AfreadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEACK` reader - 1:1\\]
Force idle-drain acknowledge high CPU does not wait for trace to drain before entering WFX state. When this bit is set to 1 trace unit behavior deviates from architecturally-specified behavior."]
pub type IdleackR = crate::BitReader;
#[doc = "Field `IDLEACK` writer - 1:1\\]
Force idle-drain acknowledge high CPU does not wait for trace to drain before entering WFX state. When this bit is set to 1 trace unit behavior deviates from architecturally-specified behavior."]
pub type IdleackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFLW` reader - 2:2\\]
Force an overflow if synchronization is not completed when second synchronization becomes due. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
pub type OvflwR = crate::BitReader;
#[doc = "Field `OVFLW` writer - 2:2\\]
Force an overflow if synchronization is not completed when second synchronization becomes due. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
pub type OvflwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDELAY` reader - 3:3\\]
Delay periodic synchronization if FIFO is more than half-full."]
pub type SyncdelayR = crate::BitReader;
#[doc = "Field `SYNCDELAY` writer - 3:3\\]
Delay periodic synchronization if FIFO is more than half-full."]
pub type SyncdelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSNODELAY` reader - 4:4\\]
Do not delay timestamp insertion based on FIFO depth."]
pub type TsnodelayR = crate::BitReader;
#[doc = "Field `TSNODELAY` writer - 4:4\\]
Do not delay timestamp insertion based on FIFO depth."]
pub type TsnodelayW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTHNOFLUSH` reader - 5:5\\]
Do not flush trace on de-assertion of authentication inputs. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
pub type AuthnoflushR = crate::BitReader;
#[doc = "Field `AUTHNOFLUSH` writer - 5:5\\]
Do not flush trace on de-assertion of authentication inputs. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
pub type AuthnoflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCAUXCTLR_6_6` reader - 6:6\\]
Reserved RES0"]
pub type Res0Trcauxctlr6_6R = crate::BitReader;
#[doc = "Field `RES0_TRCAUXCTLR_6_6` writer - 6:6\\]
Reserved RES0"]
pub type Res0Trcauxctlr6_6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COREIFEN` reader - 7:7\\]
Keep core interface enabled regardless of trace enable register state"]
pub type CoreifenR = crate::BitReader;
#[doc = "Field `COREIFEN` writer - 7:7\\]
Keep core interface enabled regardless of trace enable register state"]
pub type CoreifenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCAUXCTLR_31_8` reader - 31:8\\]
Reserved RES0"]
pub type Res0Trcauxctlr31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCAUXCTLR_31_8` writer - 31:8\\]
Reserved RES0"]
pub type Res0Trcauxctlr31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Always respond to AFREADY immediately. Does not have any interaction with FIFO draining even in WFI state."]
    #[inline(always)]
    pub fn afready(&self) -> AfreadyR {
        AfreadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force idle-drain acknowledge high CPU does not wait for trace to drain before entering WFX state. When this bit is set to 1 trace unit behavior deviates from architecturally-specified behavior."]
    #[inline(always)]
    pub fn idleack(&self) -> IdleackR {
        IdleackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Force an overflow if synchronization is not completed when second synchronization becomes due. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
    #[inline(always)]
    pub fn ovflw(&self) -> OvflwR {
        OvflwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Delay periodic synchronization if FIFO is more than half-full."]
    #[inline(always)]
    pub fn syncdelay(&self) -> SyncdelayR {
        SyncdelayR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Do not delay timestamp insertion based on FIFO depth."]
    #[inline(always)]
    pub fn tsnodelay(&self) -> TsnodelayR {
        TsnodelayR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Do not flush trace on de-assertion of authentication inputs. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
    #[inline(always)]
    pub fn authnoflush(&self) -> AuthnoflushR {
        AuthnoflushR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reserved RES0"]
    #[inline(always)]
    pub fn res0_trcauxctlr_6_6(&self) -> Res0Trcauxctlr6_6R {
        Res0Trcauxctlr6_6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Keep core interface enabled regardless of trace enable register state"]
    #[inline(always)]
    pub fn coreifen(&self) -> CoreifenR {
        CoreifenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved RES0"]
    #[inline(always)]
    pub fn res0_trcauxctlr_31_8(&self) -> Res0Trcauxctlr31_8R {
        Res0Trcauxctlr31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Always respond to AFREADY immediately. Does not have any interaction with FIFO draining even in WFI state."]
    #[inline(always)]
    #[must_use]
    pub fn afready(&mut self) -> AfreadyW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        AfreadyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Force idle-drain acknowledge high CPU does not wait for trace to drain before entering WFX state. When this bit is set to 1 trace unit behavior deviates from architecturally-specified behavior."]
    #[inline(always)]
    #[must_use]
    pub fn idleack(&mut self) -> IdleackW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        IdleackW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Force an overflow if synchronization is not completed when second synchronization becomes due. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ovflw(&mut self) -> OvflwW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        OvflwW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Delay periodic synchronization if FIFO is more than half-full."]
    #[inline(always)]
    #[must_use]
    pub fn syncdelay(&mut self) -> SyncdelayW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        SyncdelayW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Do not delay timestamp insertion based on FIFO depth."]
    #[inline(always)]
    #[must_use]
    pub fn tsnodelay(&mut self) -> TsnodelayW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        TsnodelayW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Do not flush trace on de-assertion of authentication inputs. When this bit is set to 1 the trace unit behavior deviates from architecturally-specified behavior."]
    #[inline(always)]
    #[must_use]
    pub fn authnoflush(&mut self) -> AuthnoflushW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        AuthnoflushW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Reserved RES0"]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcauxctlr_6_6(&mut self) -> Res0Trcauxctlr6_6W<ApbaddrEtmCpu1TrcauxctlrSpec> {
        Res0Trcauxctlr6_6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Keep core interface enabled regardless of trace enable register state"]
    #[inline(always)]
    #[must_use]
    pub fn coreifen(&mut self) -> CoreifenW<ApbaddrEtmCpu1TrcauxctlrSpec> {
        CoreifenW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved RES0"]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcauxctlr_31_8(&mut self) -> Res0Trcauxctlr31_8W<ApbaddrEtmCpu1TrcauxctlrSpec> {
        Res0Trcauxctlr31_8W::new(self, 8)
    }
}
#[doc = "Auxiliary Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcauxctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcauxctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcauxctlrSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcauxctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcauxctlr::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcauxctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcauxctlr::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcauxctlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCAUXCTLR to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcauxctlrSpec {
    const RESET_VALUE: u32 = 0;
}

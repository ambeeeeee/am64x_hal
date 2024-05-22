#[doc = "Register `APBADDR_ETM_CPU0_TRCCNTCTLR0` reader"]
pub type R = crate::R<ApbaddrEtmCpu0Trccntctlr0Spec>;
#[doc = "Register `APBADDR_ETM_CPU0_TRCCNTCTLR0` writer"]
pub type W = crate::W<ApbaddrEtmCpu0Trccntctlr0Spec>;
#[doc = "Field `CNTEVENT_N` reader - 7:0\\]
Selects an event, that when it occurs causes counter &lt;n> to decrement."]
pub type CnteventNR = crate::FieldReader;
#[doc = "Field `CNTEVENT_N` writer - 7:0\\]
Selects an event, that when it occurs causes counter &lt;n> to decrement."]
pub type CnteventNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLDEVENT_N` reader - 15:8\\]
Selects an event, that when it occurs causes a reload event for counter &lt;n>."]
pub type RldeventNR = crate::FieldReader;
#[doc = "Field `RLDEVENT_N` writer - 15:8\\]
Selects an event, that when it occurs causes a reload event for counter &lt;n>."]
pub type RldeventNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLDSELF_N` reader - 16:16\\]
Controls whether a reload event occurs for counter &lt;n>, when counter &lt;n> reaches zero: 0 The trace unit does not generate a reload event. 1 The trace unit generates a reload event for counter &lt;n>, provided that the event resource that CNTEVENT&lt;n> specifies is active."]
pub type RldselfNR = crate::BitReader;
#[doc = "Field `RLDSELF_N` writer - 16:16\\]
Controls whether a reload event occurs for counter &lt;n>, when counter &lt;n> reaches zero: 0 The trace unit does not generate a reload event. 1 The trace unit generates a reload event for counter &lt;n>, provided that the event resource that CNTEVENT&lt;n> specifies is active."]
pub type RldselfNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTCHAIN_N` reader - 17:17\\]
For TRCCNTCTLR3 and TRCCNTCTLR1, controls whether counter &lt;n> decrements when a reload event occurs for counter &lt;n-1>: 0 1 For TRCCNTCTLR2 and TRCCNTCTLR0, this bit is RES0."]
pub type CntchainNR = crate::BitReader;
#[doc = "Field `CNTCHAIN_N` writer - 17:17\\]
For TRCCNTCTLR3 and TRCCNTCTLR1, controls whether counter &lt;n> decrements when a reload event occurs for counter &lt;n-1>: 0 1 For TRCCNTCTLR2 and TRCCNTCTLR0, this bit is RES0."]
pub type CntchainNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCCNTCTLR0_31_18` reader - 31:18\\]
Reserved, RES0."]
pub type Res0Trccntctlr0_31_18R = crate::FieldReader<u16>;
#[doc = "Field `RES0_TRCCNTCTLR0_31_18` writer - 31:18\\]
Reserved, RES0."]
pub type Res0Trccntctlr0_31_18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Selects an event, that when it occurs causes counter &lt;n> to decrement."]
    #[inline(always)]
    pub fn cntevent_n(&self) -> CnteventNR {
        CnteventNR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Selects an event, that when it occurs causes a reload event for counter &lt;n>."]
    #[inline(always)]
    pub fn rldevent_n(&self) -> RldeventNR {
        RldeventNR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Controls whether a reload event occurs for counter &lt;n>, when counter &lt;n> reaches zero: 0 The trace unit does not generate a reload event. 1 The trace unit generates a reload event for counter &lt;n>, provided that the event resource that CNTEVENT&lt;n> specifies is active."]
    #[inline(always)]
    pub fn rldself_n(&self) -> RldselfNR {
        RldselfNR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
For TRCCNTCTLR3 and TRCCNTCTLR1, controls whether counter &lt;n> decrements when a reload event occurs for counter &lt;n-1>: 0 1 For TRCCNTCTLR2 and TRCCNTCTLR0, this bit is RES0."]
    #[inline(always)]
    pub fn cntchain_n(&self) -> CntchainNR {
        CntchainNR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trccntctlr0_31_18(&self) -> Res0Trccntctlr0_31_18R {
        Res0Trccntctlr0_31_18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Selects an event, that when it occurs causes counter &lt;n> to decrement."]
    #[inline(always)]
    #[must_use]
    pub fn cntevent_n(&mut self) -> CnteventNW<ApbaddrEtmCpu0Trccntctlr0Spec> {
        CnteventNW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Selects an event, that when it occurs causes a reload event for counter &lt;n>."]
    #[inline(always)]
    #[must_use]
    pub fn rldevent_n(&mut self) -> RldeventNW<ApbaddrEtmCpu0Trccntctlr0Spec> {
        RldeventNW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Controls whether a reload event occurs for counter &lt;n>, when counter &lt;n> reaches zero: 0 The trace unit does not generate a reload event. 1 The trace unit generates a reload event for counter &lt;n>, provided that the event resource that CNTEVENT&lt;n> specifies is active."]
    #[inline(always)]
    #[must_use]
    pub fn rldself_n(&mut self) -> RldselfNW<ApbaddrEtmCpu0Trccntctlr0Spec> {
        RldselfNW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
For TRCCNTCTLR3 and TRCCNTCTLR1, controls whether counter &lt;n> decrements when a reload event occurs for counter &lt;n-1>: 0 1 For TRCCNTCTLR2 and TRCCNTCTLR0, this bit is RES0."]
    #[inline(always)]
    #[must_use]
    pub fn cntchain_n(&mut self) -> CntchainNW<ApbaddrEtmCpu0Trccntctlr0Spec> {
        CntchainNW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trccntctlr0_31_18(
        &mut self,
    ) -> Res0Trccntctlr0_31_18W<ApbaddrEtmCpu0Trccntctlr0Spec> {
        Res0Trccntctlr0_31_18W::new(self, 18)
    }
}
#[doc = "Counter Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu0_trccntctlr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu0_trccntctlr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu0Trccntctlr0Spec;
impl crate::RegisterSpec for ApbaddrEtmCpu0Trccntctlr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu0_trccntctlr0::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu0Trccntctlr0Spec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu0_trccntctlr0::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu0Trccntctlr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU0_TRCCNTCTLR0 to value 0"]
impl crate::Resettable for ApbaddrEtmCpu0Trccntctlr0Spec {
    const RESET_VALUE: u32 = 0;
}

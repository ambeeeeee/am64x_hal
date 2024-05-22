#[doc = "Register `APBADDR_ETM_CPU1_TRCITCTRL` reader"]
pub type R = crate::R<ApbaddrEtmCpu1TrcitctrlSpec>;
#[doc = "Register `APBADDR_ETM_CPU1_TRCITCTRL` writer"]
pub type W = crate::W<ApbaddrEtmCpu1TrcitctrlSpec>;
#[doc = "Field `ITEN` reader - 0:0\\]
Integration mode enable bit: 0 The trace unit is not in integration mode. 1 The trace unit is in integration mode. This mode enables a debug agent to perform topology detection, and System-on-Chip \\[SoC\\]
test software to perform integration testing."]
pub type ItenR = crate::BitReader;
#[doc = "Field `ITEN` writer - 0:0\\]
Integration mode enable bit: 0 The trace unit is not in integration mode. 1 The trace unit is in integration mode. This mode enables a debug agent to perform topology detection, and System-on-Chip \\[SoC\\]
test software to perform integration testing."]
pub type ItenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_TRCITCTRL_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Trcitctrl31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_TRCITCTRL_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Trcitctrl31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable bit: 0 The trace unit is not in integration mode. 1 The trace unit is in integration mode. This mode enables a debug agent to perform topology detection, and System-on-Chip \\[SoC\\]
test software to perform integration testing."]
    #[inline(always)]
    pub fn iten(&self) -> ItenR {
        ItenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_trcitctrl_31_1(&self) -> Res0Trcitctrl31_1R {
        Res0Trcitctrl31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable bit: 0 The trace unit is not in integration mode. 1 The trace unit is in integration mode. This mode enables a debug agent to perform topology detection, and System-on-Chip \\[SoC\\]
test software to perform integration testing."]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ItenW<ApbaddrEtmCpu1TrcitctrlSpec> {
        ItenW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_trcitctrl_31_1(&mut self) -> Res0Trcitctrl31_1W<ApbaddrEtmCpu1TrcitctrlSpec> {
        Res0Trcitctrl31_1W::new(self, 1)
    }
}
#[doc = "Integration Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_etm_cpu1_trcitctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_etm_cpu1_trcitctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrEtmCpu1TrcitctrlSpec;
impl crate::RegisterSpec for ApbaddrEtmCpu1TrcitctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_etm_cpu1_trcitctrl::R`](R) reader structure"]
impl crate::Readable for ApbaddrEtmCpu1TrcitctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_etm_cpu1_trcitctrl::W`](W) writer structure"]
impl crate::Writable for ApbaddrEtmCpu1TrcitctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_ETM_CPU1_TRCITCTRL to value 0"]
impl crate::Resettable for ApbaddrEtmCpu1TrcitctrlSpec {
    const RESET_VALUE: u32 = 0;
}

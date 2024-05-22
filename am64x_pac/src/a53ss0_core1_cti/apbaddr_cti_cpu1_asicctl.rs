#[doc = "Register `APBADDR_CTI_CPU1_ASICCTL` reader"]
pub type R = crate::R<ApbaddrCtiCpu1AsicctlSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_ASICCTL` writer"]
pub type W = crate::W<ApbaddrCtiCpu1AsicctlSpec>;
#[doc = "Field `ASICCTL` reader - 7:0\\]
IMPLEMENTATION DEFINED ASIC control. Provides a control for external multiplexing of additional triggers into the CTI.If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected in CTIDEVID.EXTMUXNUM.If CTIDEVID.EXTMUXNUM is zero, this field is RAZ."]
pub type AsicctlR = crate::FieldReader;
#[doc = "Field `ASICCTL` writer - 7:0\\]
IMPLEMENTATION DEFINED ASIC control. Provides a control for external multiplexing of additional triggers into the CTI.If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected in CTIDEVID.EXTMUXNUM.If CTIDEVID.EXTMUXNUM is zero, this field is RAZ."]
pub type AsicctlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RES0_ASICCTL_31_8` reader - 31:8\\]
Reserved, RES0."]
pub type Res0Asicctl31_8R = crate::FieldReader<u32>;
#[doc = "Field `RES0_ASICCTL_31_8` writer - 31:8\\]
Reserved, RES0."]
pub type Res0Asicctl31_8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
IMPLEMENTATION DEFINED ASIC control. Provides a control for external multiplexing of additional triggers into the CTI.If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected in CTIDEVID.EXTMUXNUM.If CTIDEVID.EXTMUXNUM is zero, this field is RAZ."]
    #[inline(always)]
    pub fn asicctl(&self) -> AsicctlR {
        AsicctlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_asicctl_31_8(&self) -> Res0Asicctl31_8R {
        Res0Asicctl31_8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
IMPLEMENTATION DEFINED ASIC control. Provides a control for external multiplexing of additional triggers into the CTI.If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected in CTIDEVID.EXTMUXNUM.If CTIDEVID.EXTMUXNUM is zero, this field is RAZ."]
    #[inline(always)]
    #[must_use]
    pub fn asicctl(&mut self) -> AsicctlW<ApbaddrCtiCpu1AsicctlSpec> {
        AsicctlW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_asicctl_31_8(&mut self) -> Res0Asicctl31_8W<ApbaddrCtiCpu1AsicctlSpec> {
        Res0Asicctl31_8W::new(self, 8)
    }
}
#[doc = "CTI External Multiplexor Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_asicctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_asicctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1AsicctlSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1AsicctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_asicctl::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1AsicctlSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_asicctl::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1AsicctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_ASICCTL to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1AsicctlSpec {
    const RESET_VALUE: u32 = 0;
}

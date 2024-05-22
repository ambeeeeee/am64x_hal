#[doc = "Register `APBADDR_CTI_CPU0_CTICONTROL` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CticontrolSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTICONTROL` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CticontrolSpec>;
#[doc = "Field `GLBEN` reader - 0:0\\]
Enables or disables the CTI mapping functions. Possible values of this field are: 0 CTI mapping functions disabled. 1 CTI mapping functions enabled. When the mapping functions are disabled, no new events are signaled on either output triggers or output channels. If a previously asserted output trigger has not been acknowledged, it remains asserted after the mapping functions are disabled. All output triggers are disabled by CTI reset."]
pub type GlbenR = crate::BitReader;
#[doc = "Field `GLBEN` writer - 0:0\\]
Enables or disables the CTI mapping functions. Possible values of this field are: 0 CTI mapping functions disabled. 1 CTI mapping functions enabled. When the mapping functions are disabled, no new events are signaled on either output triggers or output channels. If a previously asserted output trigger has not been acknowledged, it remains asserted after the mapping functions are disabled. All output triggers are disabled by CTI reset."]
pub type GlbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_CTICONTROL_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Cticontrol31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTICONTROL_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Cticontrol31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enables or disables the CTI mapping functions. Possible values of this field are: 0 CTI mapping functions disabled. 1 CTI mapping functions enabled. When the mapping functions are disabled, no new events are signaled on either output triggers or output channels. If a previously asserted output trigger has not been acknowledged, it remains asserted after the mapping functions are disabled. All output triggers are disabled by CTI reset."]
    #[inline(always)]
    pub fn glben(&self) -> GlbenR {
        GlbenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_cticontrol_31_1(&self) -> Res0Cticontrol31_1R {
        Res0Cticontrol31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enables or disables the CTI mapping functions. Possible values of this field are: 0 CTI mapping functions disabled. 1 CTI mapping functions enabled. When the mapping functions are disabled, no new events are signaled on either output triggers or output channels. If a previously asserted output trigger has not been acknowledged, it remains asserted after the mapping functions are disabled. All output triggers are disabled by CTI reset."]
    #[inline(always)]
    #[must_use]
    pub fn glben(&mut self) -> GlbenW<ApbaddrCtiCpu0CticontrolSpec> {
        GlbenW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_cticontrol_31_1(&mut self) -> Res0Cticontrol31_1W<ApbaddrCtiCpu0CticontrolSpec> {
        Res0Cticontrol31_1W::new(self, 1)
    }
}
#[doc = "CTI Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_cticontrol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_cticontrol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CticontrolSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CticontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_cticontrol::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CticontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_cticontrol::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CticontrolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTICONTROL to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CticontrolSpec {
    const RESET_VALUE: u32 = 0;
}

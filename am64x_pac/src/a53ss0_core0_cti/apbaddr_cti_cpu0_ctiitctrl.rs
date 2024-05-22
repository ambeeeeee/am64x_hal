#[doc = "Register `APBADDR_CTI_CPU0_CTIITCTRL` reader"]
pub type R = crate::R<ApbaddrCtiCpu0CtiitctrlSpec>;
#[doc = "Register `APBADDR_CTI_CPU0_CTIITCTRL` writer"]
pub type W = crate::W<ApbaddrCtiCpu0CtiitctrlSpec>;
#[doc = "Field `IME` reader - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
pub type ImeR = crate::BitReader;
#[doc = "Field `IME` writer - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
pub type ImeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES0_CTIITCTRL_31_1` reader - 31:1\\]
Reserved, RES0."]
pub type Res0Ctiitctrl31_1R = crate::FieldReader<u32>;
#[doc = "Field `RES0_CTIITCTRL_31_1` writer - 31:1\\]
Reserved, RES0."]
pub type Res0Ctiitctrl31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
    #[inline(always)]
    pub fn ime(&self) -> ImeR {
        ImeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    pub fn res0_ctiitctrl_31_1(&self) -> Res0Ctiitctrl31_1R {
        Res0Ctiitctrl31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Integration mode enable. When IME == 1, the device reverts to an integration mode to enable integration testing or topology detection. The integration mode behavior is IMPLEMENTATION DEFINED. 0 Normal operation. 1 Integration mode enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ime(&mut self) -> ImeW<ApbaddrCtiCpu0CtiitctrlSpec> {
        ImeW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved, RES0."]
    #[inline(always)]
    #[must_use]
    pub fn res0_ctiitctrl_31_1(&mut self) -> Res0Ctiitctrl31_1W<ApbaddrCtiCpu0CtiitctrlSpec> {
        Res0Ctiitctrl31_1W::new(self, 1)
    }
}
#[doc = "CTI Integration mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu0_ctiitctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu0_ctiitctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu0CtiitctrlSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu0CtiitctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu0_ctiitctrl::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu0CtiitctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu0_ctiitctrl::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu0CtiitctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU0_CTIITCTRL to value 0"]
impl crate::Resettable for ApbaddrCtiCpu0CtiitctrlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CFG_IRQWAKEEN` reader"]
pub type R = crate::R<CfgIrqwakeenSpec>;
#[doc = "Register `CFG_IRQWAKEEN` writer"]
pub type W = crate::W<CfgIrqwakeenSpec>;
#[doc = "Field `TCAR_WUP_ENA` reader - 0:0\\]
Capture Wakeup Enable"]
pub type TcarWupEnaR = crate::BitReader;
#[doc = "Field `TCAR_WUP_ENA` writer - 0:0\\]
Capture Wakeup Enable"]
pub type TcarWupEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_WUP_ENA` reader - 1:1\\]
Overflow Wakeup Enable"]
pub type OvfWupEnaR = crate::BitReader;
#[doc = "Field `OVF_WUP_ENA` writer - 1:1\\]
Overflow Wakeup Enable"]
pub type OvfWupEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAT_WUP_ENA` reader - 2:2\\]
Match Wakeup Enable"]
pub type MatWupEnaR = crate::BitReader;
#[doc = "Field `MAT_WUP_ENA` writer - 2:2\\]
Match Wakeup Enable"]
pub type MatWupEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Capture Wakeup Enable"]
    #[inline(always)]
    pub fn tcar_wup_ena(&self) -> TcarWupEnaR {
        TcarWupEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Overflow Wakeup Enable"]
    #[inline(always)]
    pub fn ovf_wup_ena(&self) -> OvfWupEnaR {
        OvfWupEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Match Wakeup Enable"]
    #[inline(always)]
    pub fn mat_wup_ena(&self) -> MatWupEnaR {
        MatWupEnaR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Capture Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcar_wup_ena(&mut self) -> TcarWupEnaW<CfgIrqwakeenSpec> {
        TcarWupEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Overflow Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_wup_ena(&mut self) -> OvfWupEnaW<CfgIrqwakeenSpec> {
        OvfWupEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Match Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mat_wup_ena(&mut self) -> MatWupEnaW<CfgIrqwakeenSpec> {
        MatWupEnaW::new(self, 2)
    }
}
#[doc = "Wakeup-enabled events taking place when module is idle shall generate an asynchronous wakeup\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqwakeen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqwakeen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIrqwakeenSpec;
impl crate::RegisterSpec for CfgIrqwakeenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_irqwakeen::R`](R) reader structure"]
impl crate::Readable for CfgIrqwakeenSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_irqwakeen::W`](W) writer structure"]
impl crate::Writable for CfgIrqwakeenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_IRQWAKEEN to value 0"]
impl crate::Resettable for CfgIrqwakeenSpec {
    const RESET_VALUE: u32 = 0;
}

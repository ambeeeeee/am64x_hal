#[doc = "Register `CFG0_PBIST_EN_PROXY` reader"]
pub type R = crate::R<Cfg0PbistEnProxySpec>;
#[doc = "Register `CFG0_PBIST_EN_PROXY` writer"]
pub type W = crate::W<Cfg0PbistEnProxySpec>;
#[doc = "Field `PBIST_EN_EMMC0_PROXY` reader - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
pub type PbistEnEmmc0ProxyR = crate::BitReader;
#[doc = "Field `PBIST_EN_EMMC0_PROXY` writer - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
pub type PbistEnEmmc0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIST_EN_EMMC1_PROXY` reader - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
pub type PbistEnEmmc1ProxyR = crate::BitReader;
#[doc = "Field `PBIST_EN_EMMC1_PROXY` writer - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
pub type PbistEnEmmc1ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIST_EN_USB0_PROXY` reader - 4:4\\]
Activates PBIST Access to USB0 Memories"]
pub type PbistEnUsb0ProxyR = crate::BitReader;
#[doc = "Field `PBIST_EN_USB0_PROXY` writer - 4:4\\]
Activates PBIST Access to USB0 Memories"]
pub type PbistEnUsb0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBIST_EN_PCIE0_PROXY` reader - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
pub type PbistEnPcie0ProxyR = crate::BitReader;
#[doc = "Field `PBIST_EN_PCIE0_PROXY` writer - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
pub type PbistEnPcie0ProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
    #[inline(always)]
    pub fn pbist_en_emmc0_proxy(&self) -> PbistEnEmmc0ProxyR {
        PbistEnEmmc0ProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
    #[inline(always)]
    pub fn pbist_en_emmc1_proxy(&self) -> PbistEnEmmc1ProxyR {
        PbistEnEmmc1ProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates PBIST Access to USB0 Memories"]
    #[inline(always)]
    pub fn pbist_en_usb0_proxy(&self) -> PbistEnUsb0ProxyR {
        PbistEnUsb0ProxyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
    #[inline(always)]
    pub fn pbist_en_pcie0_proxy(&self) -> PbistEnPcie0ProxyR {
        PbistEnPcie0ProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Activates PBIST Access to MMC0 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_emmc0_proxy(&mut self) -> PbistEnEmmc0ProxyW<Cfg0PbistEnProxySpec> {
        PbistEnEmmc0ProxyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Activates PBIST Access to MMC1 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_emmc1_proxy(&mut self) -> PbistEnEmmc1ProxyW<Cfg0PbistEnProxySpec> {
        PbistEnEmmc1ProxyW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
Activates PBIST Access to USB0 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_usb0_proxy(&mut self) -> PbistEnUsb0ProxyW<Cfg0PbistEnProxySpec> {
        PbistEnUsb0ProxyW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Activates PBIST Access to PCIE0 Memories"]
    #[inline(always)]
    #[must_use]
    pub fn pbist_en_pcie0_proxy(&mut self) -> PbistEnPcie0ProxyW<Cfg0PbistEnProxySpec> {
        PbistEnPcie0ProxyW::new(self, 8)
    }
}
#[doc = "CFG0_PBIST_EN_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pbist_en_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pbist_en_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PbistEnProxySpec;
impl crate::RegisterSpec for Cfg0PbistEnProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pbist_en_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0PbistEnProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pbist_en_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0PbistEnProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PBIST_EN_PROXY to value 0"]
impl crate::Resettable for Cfg0PbistEnProxySpec {
    const RESET_VALUE: u32 = 0;
}

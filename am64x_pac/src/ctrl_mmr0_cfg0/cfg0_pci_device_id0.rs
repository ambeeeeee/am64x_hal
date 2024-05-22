#[doc = "Register `CFG0_PCI_DEVICE_ID0` reader"]
pub type R = crate::R<Cfg0PciDeviceId0Spec>;
#[doc = "Register `CFG0_PCI_DEVICE_ID0` writer"]
pub type W = crate::W<Cfg0PciDeviceId0Spec>;
#[doc = "Field `PCI_DEVICE_ID0_ID0` reader - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
pub type PciDeviceId0Id0R = crate::FieldReader<u32>;
#[doc = "Field `PCI_DEVICE_ID0_ID0` writer - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
pub type PciDeviceId0Id0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
    #[inline(always)]
    pub fn pci_device_id0_id0(&self) -> PciDeviceId0Id0R {
        PciDeviceId0Id0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
    #[inline(always)]
    #[must_use]
    pub fn pci_device_id0_id0(&mut self) -> PciDeviceId0Id0W<Cfg0PciDeviceId0Spec> {
        PciDeviceId0Id0W::new(self, 0)
    }
}
#[doc = "CFG0_PCI_DEVICE_ID0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_pci_device_id0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_pci_device_id0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0PciDeviceId0Spec;
impl crate::RegisterSpec for Cfg0PciDeviceId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_pci_device_id0::R`](R) reader structure"]
impl crate::Readable for Cfg0PciDeviceId0Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_pci_device_id0::W`](W) writer structure"]
impl crate::Writable for Cfg0PciDeviceId0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_PCI_DEVICE_ID0 to value 0"]
impl crate::Resettable for Cfg0PciDeviceId0Spec {
    const RESET_VALUE: u32 = 0;
}

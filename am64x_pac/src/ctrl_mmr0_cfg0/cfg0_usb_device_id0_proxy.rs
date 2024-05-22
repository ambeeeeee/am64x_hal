#[doc = "Register `CFG0_USB_DEVICE_ID0_PROXY` reader"]
pub type R = crate::R<Cfg0UsbDeviceId0ProxySpec>;
#[doc = "Register `CFG0_USB_DEVICE_ID0_PROXY` writer"]
pub type W = crate::W<Cfg0UsbDeviceId0ProxySpec>;
#[doc = "Field `USB_DEVICE_ID0_ID0_PROXY` reader - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
pub type UsbDeviceId0Id0ProxyR = crate::FieldReader<u32>;
#[doc = "Field `USB_DEVICE_ID0_ID0_PROXY` writer - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
pub type UsbDeviceId0Id0ProxyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
    #[inline(always)]
    pub fn usb_device_id0_id0_proxy(&self) -> UsbDeviceId0Id0ProxyR {
        UsbDeviceId0Id0ProxyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_id0_id0_proxy(&mut self) -> UsbDeviceId0Id0ProxyW<Cfg0UsbDeviceId0ProxySpec> {
        UsbDeviceId0Id0ProxyW::new(self, 0)
    }
}
#[doc = "CFG0_USB_DEVICE_ID0_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb_device_id0_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb_device_id0_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0UsbDeviceId0ProxySpec;
impl crate::RegisterSpec for Cfg0UsbDeviceId0ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usb_device_id0_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0UsbDeviceId0ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usb_device_id0_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0UsbDeviceId0ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USB_DEVICE_ID0_PROXY to value 0"]
impl crate::Resettable for Cfg0UsbDeviceId0ProxySpec {
    const RESET_VALUE: u32 = 0;
}

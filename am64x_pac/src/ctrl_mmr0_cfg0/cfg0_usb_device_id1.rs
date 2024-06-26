#[doc = "Register `CFG0_USB_DEVICE_ID1` reader"]
pub type R = crate::R<Cfg0UsbDeviceId1Spec>;
#[doc = "Register `CFG0_USB_DEVICE_ID1` writer"]
pub type W = crate::W<Cfg0UsbDeviceId1Spec>;
#[doc = "Field `USB_DEVICE_ID1_ID1` reader - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
pub type UsbDeviceId1Id1R = crate::FieldReader<u32>;
#[doc = "Field `USB_DEVICE_ID1_ID1` writer - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
pub type UsbDeviceId1Id1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
    #[inline(always)]
    pub fn usb_device_id1_id1(&self) -> UsbDeviceId1Id1R {
        UsbDeviceId1Id1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
ROM can optionally update this register with a 32 bit value from Customer OTP."]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_id1_id1(&mut self) -> UsbDeviceId1Id1W<Cfg0UsbDeviceId1Spec> {
        UsbDeviceId1Id1W::new(self, 0)
    }
}
#[doc = "CFG0_USB_DEVICE_ID1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_usb_device_id1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_usb_device_id1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0UsbDeviceId1Spec;
impl crate::RegisterSpec for Cfg0UsbDeviceId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_usb_device_id1::R`](R) reader structure"]
impl crate::Readable for Cfg0UsbDeviceId1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_usb_device_id1::W`](W) writer structure"]
impl crate::Writable for Cfg0UsbDeviceId1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_USB_DEVICE_ID1 to value 0"]
impl crate::Resettable for Cfg0UsbDeviceId1Spec {
    const RESET_VALUE: u32 = 0;
}

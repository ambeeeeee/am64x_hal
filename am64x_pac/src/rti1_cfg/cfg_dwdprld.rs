#[doc = "Register `CFG_DWDPRLD` reader"]
pub type R = crate::R<CfgDwdprldSpec>;
#[doc = "Register `CFG_DWDPRLD` writer"]
pub type W = crate::W<CfgDwdprldSpec>;
#[doc = "Field `DWDPRLD` reader - 11:0\\]
User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left- justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 213 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
pub type DwdprldR = crate::FieldReader<u16>;
#[doc = "Field `DWDPRLD` writer - 11:0\\]
User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left- justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 213 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
pub type DwdprldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left- justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 213 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
    #[inline(always)]
    pub fn dwdprld(&self) -> DwdprldR {
        DwdprldR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
User and priviledge mode (read): A read from this register in any CPU mode returns the current preload value. Priviledge mode (write): If the DWD is always enabled after reset is released: The DWD starts counting down from the reset value of the counter, that is, 0x002DFFFF. The application can configure the DWD preload register any time before this down counter expires. When the application services the DWD, the preload register contents are copied left- justified into the DWD down counter and it starts counting down from that value. If the DWD is implemented such that the down counter is enabled by software: The DWD preload register can be configured only when the DWD is disabled. Therefore, the application can only configure the DWD preload register before it enables the DWD down counter. The expiration time of the DWD Down Counter can be determined with following equation: texp = (RTIDWDPRLD+1) x 213 / RTICLK1 where: RTIDWDPRLD = 0...4095"]
    #[inline(always)]
    #[must_use]
    pub fn dwdprld(&mut self) -> DwdprldW<CfgDwdprldSpec> {
        DwdprldW::new(self, 0)
    }
}
#[doc = "CFG_DWDPRLD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_dwdprld::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_dwdprld::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgDwdprldSpec;
impl crate::RegisterSpec for CfgDwdprldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_dwdprld::R`](R) reader structure"]
impl crate::Readable for CfgDwdprldSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_dwdprld::W`](W) writer structure"]
impl crate::Writable for CfgDwdprldSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DWDPRLD to value 0x4095"]
impl crate::Resettable for CfgDwdprldSpec {
    const RESET_VALUE: u32 = 0x4095;
}

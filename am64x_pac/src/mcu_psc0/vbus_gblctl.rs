#[doc = "Register `VBUS_GBLCTL` reader"]
pub type R = crate::R<VbusGblctlSpec>;
#[doc = "Register `VBUS_GBLCTL` writer"]
pub type W = crate::W<VbusGblctlSpec>;
#[doc = "Field `IO_ANA_CTL` reader - 15:8\\]
General purpose IO/Analog PowerDown control. Directly drives io_ana_pdctl_po\\[7:0\\]
outputs."]
pub type IoAnaCtlR = crate::FieldReader;
#[doc = "Field `IO_ANA_CTL` writer - 15:8\\]
General purpose IO/Analog PowerDown control. Directly drives io_ana_pdctl_po\\[7:0\\]
outputs."]
pub type IoAnaCtlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - 15:8\\]
General purpose IO/Analog PowerDown control. Directly drives io_ana_pdctl_po\\[7:0\\]
outputs."]
    #[inline(always)]
    pub fn io_ana_ctl(&self) -> IoAnaCtlR {
        IoAnaCtlR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - 15:8\\]
General purpose IO/Analog PowerDown control. Directly drives io_ana_pdctl_po\\[7:0\\]
outputs."]
    #[inline(always)]
    #[must_use]
    pub fn io_ana_ctl(&mut self) -> IoAnaCtlW<VbusGblctlSpec> {
        IoAnaCtlW::new(self, 8)
    }
}
#[doc = "This register contains global control to PSC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_gblctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_gblctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusGblctlSpec;
impl crate::RegisterSpec for VbusGblctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_gblctl::R`](R) reader structure"]
impl crate::Readable for VbusGblctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_gblctl::W`](W) writer structure"]
impl crate::Writable for VbusGblctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_GBLCTL to value 0"]
impl crate::Resettable for VbusGblctlSpec {
    const RESET_VALUE: u32 = 0;
}

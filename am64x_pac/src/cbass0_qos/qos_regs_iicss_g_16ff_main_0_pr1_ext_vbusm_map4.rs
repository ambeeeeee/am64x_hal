#[doc = "Register `QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map4` reader"]
pub type R = crate::R<QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec>;
#[doc = "Register `QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map4` writer"]
pub type W = crate::W<QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec>;
#[doc = "Field `ORDERID` reader - 7:4\\]
orderid signal for channel N."]
pub type OrderidR = crate::FieldReader;
#[doc = "Field `ORDERID` writer - 7:4\\]
orderid signal for channel N."]
pub type OrderidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPRIORITY` reader - 14:12\\]
epriority signal for channel N."]
pub type EpriorityR = crate::FieldReader;
#[doc = "Field `EPRIORITY` writer - 14:12\\]
epriority signal for channel N."]
pub type EpriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:7 - 7:4\\]
orderid signal for channel N."]
    #[inline(always)]
    pub fn orderid(&self) -> OrderidR {
        OrderidR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
epriority signal for channel N."]
    #[inline(always)]
    pub fn epriority(&self) -> EpriorityR {
        EpriorityR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:7 - 7:4\\]
orderid signal for channel N."]
    #[inline(always)]
    #[must_use]
    pub fn orderid(&mut self) -> OrderidW<QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec> {
        OrderidW::new(self, 4)
    }
    #[doc = "Bits 12:14 - 14:12\\]
epriority signal for channel N."]
    #[inline(always)]
    #[must_use]
    pub fn epriority(&mut self) -> EpriorityW<QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec> {
        EpriorityW::new(self, 12)
    }
}
#[doc = "The Map Register defines the fields for the master Iicss_g_16ff_main_0.pr1_ext_vbusm per channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec;
impl crate::RegisterSpec for QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::R`](R) reader structure"]
impl crate::Readable for QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec {}
#[doc = "`write(|w| ..)` method takes [`qos_regs_iicss_g_16ff_main_0_pr1_ext_vbusm_map4::W`](W) writer structure"]
impl crate::Writable for QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QOS_REGS_Iicss_g_16ff_main_0_pr1_ext_vbusm_map4 to value 0x7000"]
impl crate::Resettable for QosRegsIicssG16ffMain0Pr1ExtVbusmMap4Spec {
    const RESET_VALUE: u32 = 0x7000;
}

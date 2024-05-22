#[doc = "Register `APBADDR_CTI_CPU1_CTIINTACK` reader"]
pub type R = crate::R<ApbaddrCtiCpu1CtiintackSpec>;
#[doc = "Register `APBADDR_CTI_CPU1_CTIINTACK` writer"]
pub type W = crate::W<ApbaddrCtiCpu1CtiintackSpec>;
#[doc = "Field `ACK_N` reader - 7:0\\]
Can be used to create soft acknowledges for output triggers"]
pub type AckNR = crate::FieldReader;
#[doc = "Field `ACK_N` writer - 7:0\\]
Can be used to create soft acknowledges for output triggers"]
pub type AckNW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Can be used to create soft acknowledges for output triggers"]
    #[inline(always)]
    pub fn ack_n(&self) -> AckNR {
        AckNR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Can be used to create soft acknowledges for output triggers"]
    #[inline(always)]
    #[must_use]
    pub fn ack_n(&mut self) -> AckNW<ApbaddrCtiCpu1CtiintackSpec> {
        AckNW::new(self, 0)
    }
}
#[doc = "CTI Output Trigger Acknowledge Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbaddr_cti_cpu1_ctiintack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbaddr_cti_cpu1_ctiintack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbaddrCtiCpu1CtiintackSpec;
impl crate::RegisterSpec for ApbaddrCtiCpu1CtiintackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbaddr_cti_cpu1_ctiintack::R`](R) reader structure"]
impl crate::Readable for ApbaddrCtiCpu1CtiintackSpec {}
#[doc = "`write(|w| ..)` method takes [`apbaddr_cti_cpu1_ctiintack::W`](W) writer structure"]
impl crate::Writable for ApbaddrCtiCpu1CtiintackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBADDR_CTI_CPU1_CTIINTACK to value 0"]
impl crate::Resettable for ApbaddrCtiCpu1CtiintackSpec {
    const RESET_VALUE: u32 = 0;
}

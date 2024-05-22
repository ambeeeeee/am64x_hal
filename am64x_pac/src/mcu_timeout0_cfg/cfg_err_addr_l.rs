#[doc = "Register `CFG_ERR_ADDR_L` reader"]
pub type R = crate::R<CfgErrAddrLSpec>;
#[doc = "Register `CFG_ERR_ADDR_L` writer"]
pub type W = crate::W<CfgErrAddrLSpec>;
#[doc = "Field `ADDR` reader - 31:0\\]
Lower bits of the Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:0\\]
Lower bits of the Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Lower bits of the Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Lower bits of the Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<CfgErrAddrLSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "This register contains information about transaction that caused the interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_err_addr_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_err_addr_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgErrAddrLSpec;
impl crate::RegisterSpec for CfgErrAddrLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_err_addr_l::R`](R) reader structure"]
impl crate::Readable for CfgErrAddrLSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_err_addr_l::W`](W) writer structure"]
impl crate::Writable for CfgErrAddrLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_ERR_ADDR_L to value 0"]
impl crate::Resettable for CfgErrAddrLSpec {
    const RESET_VALUE: u32 = 0;
}

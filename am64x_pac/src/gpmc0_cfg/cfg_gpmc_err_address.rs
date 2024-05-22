#[doc = "Register `CFG_GPMC_ERR_ADDRESS` reader"]
pub type R = crate::R<CfgGpmcErrAddressSpec>;
#[doc = "Register `CFG_GPMC_ERR_ADDRESS` writer"]
pub type W = crate::W<CfgGpmcErrAddressSpec>;
#[doc = "Field `ILLEGALADD` reader - 30:0\\]
Address of illegal access : A30\\[0 for memory region, 1 for GPMC register region\\]
and A29-A0\\[1 GBytes maximum\\]"]
pub type IllegaladdR = crate::FieldReader<u32>;
#[doc = "Field `ILLEGALADD` writer - 30:0\\]
Address of illegal access : A30\\[0 for memory region, 1 for GPMC register region\\]
and A29-A0\\[1 GBytes maximum\\]"]
pub type IllegaladdW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - 30:0\\]
Address of illegal access : A30\\[0 for memory region, 1 for GPMC register region\\]
and A29-A0\\[1 GBytes maximum\\]"]
    #[inline(always)]
    pub fn illegaladd(&self) -> IllegaladdR {
        IllegaladdR::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - 30:0\\]
Address of illegal access : A30\\[0 for memory region, 1 for GPMC register region\\]
and A29-A0\\[1 GBytes maximum\\]"]
    #[inline(always)]
    #[must_use]
    pub fn illegaladd(&mut self) -> IllegaladdW<CfgGpmcErrAddressSpec> {
        IllegaladdW::new(self, 0)
    }
}
#[doc = "The GPMC_ERR_ADDRESS register stores the address of the illegal access when an error occurs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_gpmc_err_address::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_gpmc_err_address::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgGpmcErrAddressSpec;
impl crate::RegisterSpec for CfgGpmcErrAddressSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_gpmc_err_address::R`](R) reader structure"]
impl crate::Readable for CfgGpmcErrAddressSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_gpmc_err_address::W`](W) writer structure"]
impl crate::Writable for CfgGpmcErrAddressSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_GPMC_ERR_ADDRESS to value 0"]
impl crate::Resettable for CfgGpmcErrAddressSpec {
    const RESET_VALUE: u32 = 0;
}
